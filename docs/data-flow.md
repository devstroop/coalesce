# Coalesce Data Flow

## Overview

This document describes how data flows through Coalesce from source code input to translated output.

## High-Level Data Flow

```mermaid
graph TB
    subgraph "Input Stage"
        SC[Source Code] --> ZIP[ZIP/Folder]
        GIT[Git Repo] --> CLONE[Clone]
        API[Direct API] --> UPLOAD[Upload]
    end
    
    subgraph "Processing Pipeline"
        ZIP --> EXTRACT[Extract Files]
        CLONE --> EXTRACT
        UPLOAD --> EXTRACT
        
        EXTRACT --> PARSE[Parse to AST]
        PARSE --> ANALYZE[Semantic Analysis]
        ANALYZE --> UIR[Build UIR]
        UIR --> EMBED[Generate Embeddings]
        EMBED --> STORE[(Store in SurrealDB)]
    end
    
    subgraph "Translation Stage"
        STORE --> FETCH[Fetch UIR]
        FETCH --> ML[ML Processing]
        ML --> GEN[Code Generation]
        GEN --> OPT[Optimization]
        OPT --> OUTPUT[Target Code]
    end
    
    subgraph "Learning Loop"
        OUTPUT --> REVIEW[Human Review]
        REVIEW --> FEEDBACK[Feedback]
        FEEDBACK --> TRAIN[Update Models]
        TRAIN --> ML
    end
```

## Detailed Processing Pipeline

### 1. Input Processing

```mermaid
sequenceDiagram
    participant User
    participant API
    participant Storage
    participant Queue
    
    User->>API: Upload source code
    API->>API: Validate input
    API->>Storage: Store raw files
    Storage-->>API: File IDs
    API->>Queue: Queue parsing job
    Queue-->>API: Job ID
    API-->>User: Processing started
```

### 2. Parsing Pipeline

```mermaid
graph LR
    subgraph "File Processing"
        A[Source File] --> B[Detect Language]
        B --> C[Select Parser]
        C --> D[Tokenize]
        D --> E[Parse AST]
    end
    
    subgraph "Semantic Analysis"
        E --> F[Type Inference]
        F --> G[Scope Analysis]
        G --> H[Dependency Graph]
        H --> I[Control Flow]
    end
    
    subgraph "UIR Generation"
        I --> J[Map to UIR]
        J --> K[Normalize]
        K --> L[Validate]
        L --> M[Optimize]
    end
```

### 3. ML Pipeline Data Flow

```mermaid
graph TD
    subgraph "Embedding Generation"
        UIR1[UIR Node] --> TOK[Tokenization]
        TOK --> EMB[Candle Embedder]
        EMB --> VEC[Vector Output]
    end
    
    subgraph "Pattern Matching"
        VEC --> SIM[Similarity Search]
        SIM --> CAND[Candidates]
        CAND --> RANK[Ranking]
    end
    
    subgraph "Translation Model"
        UIR1 --> FEAT[Feature Extraction]
        FEAT --> ONNX[ONNX Runtime]
        ONNX --> PRED[Predictions]
        RANK --> MERGE[Merge Results]
        PRED --> MERGE
    end
```

## Data Structures

### UIR Node Structure

```mermaid
classDiagram
    class UIRNode {
        +ID: String
        +Type: NodeType
        +Children: List~UIRNode~
        +Metadata: Metadata
        +Embeddings: Float32Array
        +SourceRef: SourceLocation
    }
    
    class Metadata {
        +Language: Language
        +Version: String
        +Annotations: Map~String,Any~
        +Complexity: Float
        +Dependencies: List~ID~
    }
    
    class SourceLocation {
        +File: String
        +StartLine: Int
        +EndLine: Int
        +StartCol: Int
        +EndCol: Int
    }
    
    UIRNode --> Metadata
    UIRNode --> SourceLocation
```

### Database Schema

```mermaid
erDiagram
    PROJECTS ||--o{ CODEBASES : contains
    CODEBASES ||--o{ FILES : includes
    FILES ||--o{ UIR_NODES : parsed_to
    UIR_NODES ||--o{ EMBEDDINGS : has
    UIR_NODES ||--o{ TRANSLATIONS : translated_to
    TRANSLATIONS ||--o{ FEEDBACK : receives
    
    UIR_NODES {
        string id PK
        string type
        jsonb metadata
        string parent_id FK
        timestamp created_at
    }
    
    EMBEDDINGS {
        string id PK
        string node_id FK
        vector embedding
        string model_version
        float confidence
    }
    
    TRANSLATIONS {
        string id PK
        string source_node_id FK
        string target_language
        text generated_code
        float confidence_score
        string status
    }
```

## Stream Processing

### Real-time Translation Flow

```mermaid
graph LR
    subgraph "Input Stream"
        WS[WebSocket] --> BUFFER[Buffer]
        BUFFER --> CHUNK[Chunking]
    end
    
    subgraph "Processing Stream"
        CHUNK --> PSTREAM[Parser Stream]
        PSTREAM --> USTREAM[UIR Stream]
        USTREAM --> TSTREAM[Translation Stream]
    end
    
    subgraph "Output Stream"
        TSTREAM --> FORMAT[Formatter]
        FORMAT --> STREAM[Output Stream]
        STREAM --> CLIENT[Client]
    end
```

### Batch Processing

```mermaid
sequenceDiagram
    participant Scheduler
    participant Worker
    participant DB
    participant ML
    participant Cache
    
    Scheduler->>Worker: Assign batch
    Worker->>DB: Fetch UIR nodes
    DB-->>Worker: Node data
    Worker->>ML: Process batch
    ML-->>Worker: Predictions
    Worker->>Cache: Store results
    Worker->>DB: Update translations
    Worker-->>Scheduler: Batch complete
```

## Caching Strategy

```mermaid
graph TD
    subgraph "Cache Layers"
        L1[L1: In-Memory]
        L2[L2: Redis]
        L3[L3: SurrealDB]
        L4[L4: S3 Archive]
    end
    
    subgraph "Cache Keys"
        UIR[UIR Nodes]
        EMB[Embeddings]
        TRANS[Translations]
        MODEL[Model Outputs]
    end
    
    subgraph "Eviction Policy"
        LRU[LRU for L1/L2]
        TTL[TTL for translations]
        PERSIST[Permanent for UIR]
    end
    
    UIR --> L2
    EMB --> L2
    TRANS --> L1
    MODEL --> L1
    
    L1 --> L2
    L2 --> L3
    L3 --> L4
```

## Error Handling Flow

```mermaid
graph TD
    A[Error Occurs] --> B{Error Type}
    
    B -->|Parse Error| C[Log Details]
    C --> D[Partial UIR]
    D --> E[Mark Failed Nodes]
    
    B -->|ML Error| F[Fallback Rules]
    F --> G[Basic Translation]
    
    B -->|System Error| H[Retry Logic]
    H --> I{Retry Count}
    I -->|< 3| J[Retry]
    I -->|>= 3| K[Dead Letter Queue]
    
    E --> L[User Notification]
    G --> L
    K --> L
```

## Performance Optimization

### Parallel Processing

```mermaid
graph LR
    subgraph "File Level"
        F1[File 1] --> P1[Parser 1]
        F2[File 2] --> P2[Parser 2]
        F3[File 3] --> P3[Parser 3]
    end
    
    subgraph "Node Level"
        UIR1[UIR Tree] --> N1[Node 1]
        UIR1 --> N2[Node 2]
        UIR1 --> N3[Node 3]
    end
    
    subgraph "Translation Level"
        N1 --> T1[Translate 1]
        N2 --> T2[Translate 2]
        N3 --> T3[Translate 3]
    end
```

### Data Compression

```mermaid
graph TD
    A[Raw UIR] --> B[Compression Analysis]
    B --> C{Size > Threshold}
    C -->|Yes| D[Compress]
    C -->|No| E[Store Raw]
    D --> F[Store Compressed]
    
    G[Retrieval] --> H{Is Compressed?}
    H -->|Yes| I[Decompress]
    H -->|No| J[Use Direct]
    I --> K[Process]
    J --> K
```

## Monitoring Points

```mermaid
graph TB
    subgraph "Metrics Collection"
        A[Parse Time]
        B[UIR Size]
        C[Translation Time]
        D[Cache Hit Rate]
        E[Model Accuracy]
        F[Queue Depth]
    end
    
    subgraph "Aggregation"
        A --> AGG[Prometheus]
        B --> AGG
        C --> AGG
        D --> AGG
        E --> AGG
        F --> AGG
    end
    
    subgraph "Visualization"
        AGG --> GRAF[Grafana]
        AGG --> ALERT[Alerting]
    end
```

## Security Considerations

### Data Isolation

```mermaid
graph TD
    subgraph "Multi-tenancy"
        T1[Tenant 1] --> ISO1[Isolated Namespace]
        T2[Tenant 2] --> ISO2[Isolated Namespace]
        T3[Tenant 3] --> ISO3[Isolated Namespace]
    end
    
    subgraph "Access Control"
        ISO1 --> RBAC[Role-Based Access]
        ISO2 --> RBAC
        ISO3 --> RBAC
        RBAC --> AUDIT[Audit Log]
    end
    
    subgraph "Encryption"
        ISO1 --> ENC[At-Rest Encryption]
        ISO2 --> ENC
        ISO3 --> ENC
    end
```

This data flow architecture ensures efficient, secure, and scalable processing of code translations while maintaining the ability to learn and improve from user feedback.
