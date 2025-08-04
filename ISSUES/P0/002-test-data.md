# Issue #002: Create Test Data Projects

**Priority**: P0 - Critical  
**Milestone**: Foundation Reality Check  
**Assignee**: TBD  
**Status**: 🔴 Not Started  

## Problem
All test project directories were deleted. We have no sample code to:
- Test parsers against
- Validate translations
- Demonstrate real-world usage
- Benchmark performance

## Current Situation
```bash
$ ls -la banking-cobol-project/src/
# ERROR: no such file or directory

$ ls -la network-c-project/src/  
# ERROR: no such file or directory
```

## Goal
Create comprehensive test data covering:
1. **Banking COBOL Project** - Financial calculations
2. **Network C Project** - SoftEtherVPN-style code  
3. **Modern JavaScript Project** - React/Node.js patterns
4. **Legacy FORTRAN Project** - Scientific computing

## Acceptance Criteria

### Banking COBOL Project
- [ ] `INTEREST.COB` - Interest calculation routines
- [ ] `ACCOUNT.COB` - Account management 
- [ ] `VALIDATE.COB` - Data validation
- [ ] Representative of real banking systems
- [ ] 200-500 lines total

### Network C Project  
- [ ] `session.h` - Session management header
- [ ] `session.c` - Session implementation
- [ ] `protocol.c` - Protocol handling
- [ ] `buffer.c` - Memory management
- [ ] Similar complexity to SoftEtherVPN
- [ ] 500-1000 lines total

### JavaScript Modern Project
- [ ] `user-service.js` - REST API
- [ ] `auth.js` - Authentication logic
- [ ] `database.js` - Data access layer
- [ ] Modern patterns (async/await, classes)
- [ ] 300-600 lines total

### FORTRAN Legacy Project
- [ ] `MATRIX.FOR` - Matrix operations
- [ ] `STATS.FOR` - Statistical functions  
- [ ] `SOLVER.FOR` - Equation solving
- [ ] Representative scientific code
- [ ] 400-800 lines total

## File Structure
```
test-projects/
├── banking-cobol/
│   ├── src/
│   │   ├── INTEREST.COB
│   │   ├── ACCOUNT.COB
│   │   └── VALIDATE.COB
│   ├── expected-output/
│   │   ├── python/
│   │   └── rust/
│   └── README.md
├── network-c/
│   ├── src/
│   │   ├── session.h
│   │   ├── session.c
│   │   ├── protocol.c
│   │   └── buffer.c
│   ├── expected-output/
│   │   └── rust/
│   └── README.md
├── modern-js/
│   ├── src/
│   │   ├── user-service.js
│   │   ├── auth.js
│   │   └── database.js
│   ├── expected-output/
│   │   ├── python/
│   │   └── rust/
│   └── README.md
└── legacy-fortran/
    ├── src/
    │   ├── MATRIX.FOR
    │   ├── STATS.FOR
    │   └── SOLVER.FOR
    ├── expected-output/
    │   └── python/
    └── README.md
```

## Quality Guidelines
1. **Realistic Complexity** - Not toy examples
2. **Representative Patterns** - Show real legacy pain points
3. **Well-Commented** - Explain business logic
4. **Edge Cases** - Include tricky constructs
5. **Documentation** - README for each project

## Sample Code Requirements

### COBOL Example
```cobol
IDENTIFICATION DIVISION.
PROGRAM-ID. CALCULATE-COMPOUND-INTEREST.

DATA DIVISION.
WORKING-STORAGE SECTION.
01 WS-PRINCIPAL       PIC 9(7)V99.
01 WS-RATE           PIC 99V99.
01 WS-TIME           PIC 99.
01 WS-COMPOUND-AMT   PIC 9(8)V99.

PROCEDURE DIVISION.
    DISPLAY "Enter Principal Amount: ".
    ACCEPT WS-PRINCIPAL.
    
    PERFORM VARYING WS-TIME FROM 1 BY 1 UNTIL WS-TIME > 10
        COMPUTE WS-COMPOUND-AMT = WS-PRINCIPAL * 
            ((1 + WS-RATE / 100) ** WS-TIME)
        DISPLAY "Year " WS-TIME ": $" WS-COMPOUND-AMT
    END-PERFORM.
```

### C Network Example  
```c
// Similar to SoftEtherVPN session management
typedef struct SESSION {
    int socket_fd;
    SSL* ssl_context;
    BUFFER* send_buffer;
    BUFFER* recv_buffer;
    bool is_authenticated;
    char client_ip[16];
    time_t last_activity;
} SESSION;

SESSION* session_create(int socket_fd) {
    SESSION* session = malloc(sizeof(SESSION));
    if (!session) return NULL;
    
    session->socket_fd = socket_fd;
    session->send_buffer = buffer_create(8192);
    session->recv_buffer = buffer_create(8192);
    session->is_authenticated = false;
    session->last_activity = time(NULL);
    
    return session;
}
```

## Dependencies
- None (but blocks all other parser work)

## Estimated Effort
**2-3 days** to create all test projects

## Implementation Strategy
1. Start with banking-cobol (simplest)
2. Add network-c (core use case)  
3. Create modern-js (validation)
4. Add legacy-fortran (stretch goal)

## Success Metrics
- Each project builds/runs in original language
- Covers 80% of language constructs we want to support
- Provides clear translation targets
- Demonstrates real-world complexity

---
**Created**: July 18, 2025  
**Updated**: July 18, 2025
