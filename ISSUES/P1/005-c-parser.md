# Issue #005: C Language Parser (SoftEtherVPN Case Study)

**Priority**: P1 - High  
**Milestone**: Foundation Reality Check  
**Assignee**: TBD  
**Status**: 🔴 Not Started  

## Problem
SoftEtherVPN inspired this project, but we have no C language support. Need to parse complex C code to demonstrate real-world legacy modernization.

## Context: SoftEtherVPN Challenges
- 30-year-old C codebase
- Complex memory management
- Platform-specific code
- Intricate networking protocols
- Time-consuming maintenance

## Goal
Create C parser that can handle SoftEtherVPN-style code patterns:
- Manual memory management
- Pointer arithmetic  
- Preprocessor macros
- Platform conditionals
- Complex struct definitions

## Technical Requirements

### 1. Tree-sitter C Integration
```toml
tree-sitter-c = "0.20"
```

### 2. C-Specific UIR Patterns
```rust
// Handle C-specific constructs
pub enum CSpecificPattern {
    ManualMemoryManagement {
        allocator: String,      // malloc, calloc, etc.
        deallocator: String,    // free
        size_expr: String,
    },
    PointerArithmetic {
        base_ptr: String,
        offset: String,
        operation: String,      // +, -, ++, --
    },
    PreprocessorMacro {
        name: String,
        definition: String,
        usage_locations: Vec<SourceLocation>,
    },
}
```

### 3. Legacy Pattern Recognition
```rust
impl CParser {
    fn detect_legacy_patterns(&self, node: &UIRNode) -> Vec<LegacyPattern> {
        let mut patterns = Vec::new();
        
        // Detect manual memory management
        if self.is_malloc_pattern(node) {
            patterns.push(LegacyPattern {
                pattern_type: "manual_memory_management".to_string(),
                original_construct: self.extract_code(node),
                modernization_hint: Some("Use smart pointers or RAII".to_string()),
                preserve_exactly: true,
            });
        }
        
        patterns
    }
}
```

## SoftEtherVPN Code Examples

### Session Management (Target Code)
```c
typedef struct SESSION {
    SOCKET socket;
    SSL* ssl;
    THREAD* recv_thread;
    THREAD* send_thread;
    QUEUE* send_queue;
    bool halt_flag;
    LOCK* lock;
} SESSION;

SESSION* CreateSession(SOCKET socket) {
    SESSION* s = (SESSION*)malloc(sizeof(SESSION));
    if (s == NULL) return NULL;
    
    s->socket = socket;
    s->ssl = NULL;
    s->recv_thread = NULL;
    s->send_thread = NULL;
    s->send_queue = NewQueue();
    s->halt_flag = false;
    s->lock = NewLock();
    
    return s;
}

void FreeSession(SESSION* s) {
    if (s == NULL) return;
    
    s->halt_flag = true;
    
    if (s->recv_thread) {
        WaitThread(s->recv_thread, INFINITE);
        ReleaseThread(s->recv_thread);
    }
    
    if (s->send_thread) {
        WaitThread(s->send_thread, INFINITE);
        ReleaseThread(s->send_thread);
    }
    
    if (s->send_queue) {
        ReleaseQueue(s->send_queue);
    }
    
    if (s->lock) {
        DeleteLock(s->lock);
    }
    
    free(s);
}
```

### Expected Rust Translation
```rust
pub struct Session {
    socket: TcpStream,
    tls: Option<TlsStream<TcpStream>>,
    send_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
    shutdown: Arc<AtomicBool>,
}

impl Session {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            tls: None,
            send_queue: Arc::new(Mutex::new(VecDeque::new())),
            shutdown: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        // Automatic cleanup via RAII
    }
}
```

## AST Mapping Strategy

### C Struct → Rust Struct
```c
typedef struct FOO {
    int* data;
    size_t size;
} FOO;
```

UIR Representation:
```yaml
node_type: "Struct"
name: "FOO"
fields:
  - name: "data"
    type: "pointer"
    target_type: "int"
    modernization_hint: "Vec<i32>"
  - name: "size"
    type: "size_t"
    modernization_hint: "usize"
```

### Manual Memory Management → RAII
```c
FOO* foo = malloc(sizeof(FOO));
// ... use foo
free(foo);
```

Legacy Pattern Detection:
```yaml
pattern_type: "manual_memory_management"
lifecycle:
  allocation: "malloc(sizeof(FOO))"
  usage: ["foo->data = ...", "foo->size = ..."]
  deallocation: "free(foo)"
modernization:
  rust: "Box<Foo> or owned struct"
  safety_benefit: "Prevents memory leaks and double-free"
```

## Implementation Plan

### Phase 1: Basic C Parsing (3 days)
- [ ] Integrate tree-sitter-c
- [ ] Parse struct definitions
- [ ] Parse function declarations
- [ ] Handle basic types

### Phase 2: Memory Management Detection (4 days)
- [ ] Detect malloc/free patterns
- [ ] Track pointer usage
- [ ] Identify memory lifecycle
- [ ] Generate modernization hints

### Phase 3: SoftEtherVPN Case Study (5 days)
- [ ] Parse actual SoftEtherVPN source
- [ ] Document detected patterns
- [ ] Generate Rust equivalent
- [ ] Performance comparison

### Phase 4: Integration (2 days)
- [ ] CLI support for C files
- [ ] Test suite for C parsing
- [ ] Documentation updates

## Success Criteria
- [ ] Parse 1000+ lines of SoftEtherVPN C code
- [ ] Detect 10+ legacy patterns accurately
- [ ] Generate compilable Rust code
- [ ] Document modernization opportunities
- [ ] 90%+ parsing accuracy

## Test Cases

### Memory Management
```c
Buffer* CreateBuffer(size_t size) {
    Buffer* buf = malloc(sizeof(Buffer));
    if (!buf) return NULL;
    buf->data = malloc(size);
    if (!buf->data) {
        free(buf);
        return NULL;
    }
    buf->size = size;
    return buf;
}
```

### Platform Conditionals
```c
#ifdef _WIN32
    SOCKET sock = socket(AF_INET, SOCK_STREAM, 0);
#else
    int sock = socket(AF_INET, SOCK_STREAM, 0);
#endif
```

### Complex Structs
```c
typedef struct COMPLEX_SESSION {
    SOCKET sock;
    SSL_CTX* ssl_ctx;
    BIO* bio_read;
    BIO* bio_write;
    THREAD* worker_threads[MAX_THREADS];
    volatile bool shutdown_flag;
    CRITICAL_SECTION lock;
} COMPLEX_SESSION;
```

## Dependencies
- Issue #004: Tree-sitter integration
- Issue #002: Test data (C code samples)

## Estimated Effort
**10-14 days**

## Risk Assessment
- **High**: C parsing complexity (macros, preprocessor)
- **Medium**: Memory pattern detection accuracy
- **Low**: Basic struct/function parsing

---
**Created**: July 18, 2025  
**Updated**: July 18, 2025
