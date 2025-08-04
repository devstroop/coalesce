# Issue #007: SurrealDB Integration

**Priority**: P2 - Medium  
**Milestone**: Intelligence Layer  
**Assignee**: TBD  
**Status**: 🔴 Not Started  

## Problem
UIR data is currently only in-memory. Need persistent storage for:
- Project codebases
- Translation history
- ML training data
- Pattern recognition results
- User feedback/corrections

## Goal
Integrate SurrealDB as the primary data store with:
- Graph-based UIR storage
- Efficient querying across codebases
- Version tracking
- Relationship modeling between code elements

## Technical Requirements

### 1. Database Schema Design
```sql
-- Projects and codebases
DEFINE TABLE projects SCHEMAFULL;
DEFINE FIELD name ON projects TYPE string;
DEFINE FIELD created_at ON projects TYPE datetime;
DEFINE FIELD settings ON projects TYPE object;

DEFINE TABLE codebases SCHEMAFULL;
DEFINE FIELD project ON codebases TYPE record<projects>;
DEFINE FIELD language ON codebases TYPE string;
DEFINE FIELD version ON codebases TYPE string;

-- UIR nodes as graph structure
DEFINE TABLE uir_nodes SCHEMAFULL;
DEFINE FIELD node_type ON uir_nodes TYPE string;
DEFINE FIELD name ON uir_nodes TYPE option<string>;
DEFINE FIELD metadata ON uir_nodes TYPE object;
DEFINE FIELD source_location ON uir_nodes TYPE option<object>;

-- Relationships between nodes
DEFINE TABLE parent_child TYPE RELATION;
DEFINE TABLE depends_on TYPE RELATION;
DEFINE TABLE calls TYPE RELATION;

-- Translations and feedback
DEFINE TABLE translations SCHEMAFULL;
DEFINE FIELD source_node ON translations TYPE record<uir_nodes>;
DEFINE FIELD target_language ON translations TYPE string;
DEFINE FIELD generated_code ON translations TYPE string;
DEFINE FIELD confidence ON translations TYPE float;

DEFINE TABLE feedback SCHEMAFULL;
DEFINE FIELD translation ON feedback TYPE record<translations>;
DEFINE FIELD correction ON feedback TYPE string;
DEFINE FIELD user_notes ON feedback TYPE string;
```

### 2. Storage Interface
```rust
use surrealdb::{Surreal, engine::remote::ws::Ws};

pub struct CoalesceDB {
    db: Surreal<surrealdb::engine::remote::ws::Client>,
}

impl CoalesceDB {
    pub async fn new(url: &str) -> Result<Self> {
        let db = Surreal::new::<Ws>(url).await?;
        db.use_ns("coalesce").use_db("main").await?;
        Ok(Self { db })
    }
    
    pub async fn store_uir(&self, uir: &UIRNode, codebase_id: &str) -> Result<String> {
        let record = self.db
            .create::<Option<UIRRecord>>("uir_nodes")
            .content(UIRRecord::from_uir(uir, codebase_id))
            .await?;
            
        Ok(record.unwrap().id)
    }
    
    pub async fn get_uir(&self, id: &str) -> Result<UIRNode> {
        let record: Option<UIRRecord> = self.db.select(("uir_nodes", id)).await?;
        record.map(|r| r.to_uir()).ok_or_else(|| anyhow!("UIR not found"))
    }
}
```

### 3. Graph Queries
```rust
impl CoalesceDB {
    // Find all functions that call a specific function
    pub async fn find_callers(&self, function_id: &str) -> Result<Vec<UIRNode>> {
        let query = "
            SELECT source.* FROM calls 
            WHERE out = $function_id
            FETCH source
        ";
        
        let results: Vec<UIRRecord> = self.db.query(query)
            .bind(("function_id", function_id))
            .await?
            .take(0)?;
            
        Ok(results.into_iter().map(|r| r.to_uir()).collect())
    }
    
    // Find similar code patterns using embeddings
    pub async fn find_similar_patterns(&self, embeddings: &[f32], threshold: f64) -> Result<Vec<UIRNode>> {
        let query = "
            SELECT * FROM uir_nodes 
            WHERE vector::similarity::cosine(metadata.embeddings, $embeddings) > $threshold
            ORDER BY vector::similarity::cosine(metadata.embeddings, $embeddings) DESC
            LIMIT 10
        ";
        
        let results: Vec<UIRRecord> = self.db.query(query)
            .bind(("embeddings", embeddings))
            .bind(("threshold", threshold))
            .await?
            .take(0)?;
            
        Ok(results.into_iter().map(|r| r.to_uir()).collect())
    }
}
```

### 4. Data Models
```rust
#[derive(Serialize, Deserialize)]
struct UIRRecord {
    id: Option<Thing>,
    node_type: String,
    name: Option<String>,
    metadata: serde_json::Value,
    source_location: Option<serde_json::Value>,
    codebase: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl UIRRecord {
    fn from_uir(uir: &UIRNode, codebase_id: &str) -> Self {
        Self {
            id: None,
            node_type: format!("{:?}", uir.node_type),
            name: uir.name.clone(),
            metadata: serde_json::to_value(&uir.metadata).unwrap(),
            source_location: uir.source_location.as_ref()
                .map(|loc| serde_json::to_value(loc).unwrap()),
            codebase: codebase_id.to_string(),
            created_at: chrono::Utc::now(),
        }
    }
    
    fn to_uir(&self) -> UIRNode {
        // Convert back to UIRNode
        // Handle deserialization
    }
}
```

## Implementation Plan

### Phase 1: Basic Storage (4 days)
- [ ] Add SurrealDB dependency
- [ ] Create connection management
- [ ] Define basic schema
- [ ] Store/retrieve UIR nodes

### Phase 2: Graph Relationships (3 days)
- [ ] Model parent-child relationships
- [ ] Add dependency tracking
- [ ] Function call graphs
- [ ] Cross-reference resolution

### Phase 3: Query Interface (3 days)
- [ ] Complex graph queries
- [ ] Pattern search capabilities
- [ ] Performance optimization
- [ ] Indexing strategy

### Phase 4: Integration (2 days)
- [ ] Update CLI to use database
- [ ] Migration from in-memory storage
- [ ] Error handling and recovery
- [ ] Testing and validation

## Use Cases

### 1. Project Management
```rust
// Store entire codebase
let project = db.create_project("SoftEtherVPN Migration").await?;
let codebase = db.create_codebase(&project.id, "C", "1.0").await?;

for file in c_files {
    let uir = parser.parse_file(&file)?;
    db.store_uir(&uir, &codebase.id).await?;
}
```

### 2. Cross-Reference Analysis
```rust
// Find all usages of a function
let function_id = "session_create_func";
let callers = db.find_callers(function_id).await?;
let dependencies = db.find_dependencies(function_id).await?;

println!("Function {} is called by {} functions", function_id, callers.len());
```

### 3. Pattern Recognition
```rust
// Find similar code patterns
let patterns = db.find_similar_patterns(&embeddings, 0.8).await?;
for pattern in patterns {
    println!("Similar pattern found: {} (confidence: {})", 
             pattern.name.unwrap_or("anonymous".to_string()),
             pattern.metadata.complexity_score.unwrap_or(0.0));
}
```

### 4. Translation History
```rust
// Track translation attempts and improvements
let translation = db.store_translation(&uir_node, "rust", &generated_code, 0.85).await?;

// User provides correction
db.store_feedback(&translation.id, &corrected_code, "Fixed memory management").await?;

// Learn from feedback
let feedback_data = db.get_feedback_for_pattern(&pattern_id).await?;
ml_pipeline.train_from_feedback(feedback_data).await?;
```

## Success Criteria
- [ ] Store 10,000+ UIR nodes efficiently
- [ ] Query response time < 100ms for common operations
- [ ] Graph traversal across 1000+ node relationships
- [ ] Automatic relationship inference
- [ ] Data persistence across application restarts

## Performance Considerations
- Index frequently queried fields
- Batch operations for bulk data
- Connection pooling
- Query optimization
- Memory usage for large graphs

## Dependencies
- Working UIR structure (Issue #001, #003)
- SurrealML for vector operations

## Estimated Effort
**10-12 days**

---
**Created**: July 18, 2025  
**Updated**: July 18, 2025
