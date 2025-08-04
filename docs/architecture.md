# Coalesce Architecture

## Overview

Coalesce is built as a modular, extensible system for universal code translation using a modern Rust-based stack with machine learning capabilities.

## Technology Stack

### Core Engine
- **Language**: Rust
- **ML Framework**: Candle (Rust-native deep learning)
- **Database**: SurrealDB with SurrealML
- **Inference**: Embedded ONNX Runtime
- **Frontend**: SolidJS
- **API**: GraphQL over WebSocket

### Architecture Diagram

```mermaid
graph TB
    subgraph "Frontend Layer"
        UI[SolidJS Web UI]
        CLI[CLI Tool]
    end
    
    subgraph "API Gateway"
        GQL[GraphQL API]
        WS[WebSocket Server]
    end
    
    subgraph "Core Engine"
        Parser[Language Parsers]
        Analyzer[Code Analyzer]
        IR[Universal IR]
        Generator[Code Generators]
        
        subgraph "ML Pipeline"
            Candle[Candle Models]
            ONNX[ONNX Runtime]
            SML[SurrealML]
        end
    end
    
    subgraph "Data Layer"
        SDB[(SurrealDB)]
        Cache[Redis Cache]
        S3[Object Storage]
    end
    
    UI --> GQL
    CLI --> GQL
    GQL --> WS
    WS --> Parser
    Parser --> Analyzer
    Analyzer --> IR
    IR --> Generator
    
    Analyzer --> Candle
    Candle --> SML
    SML --> SDB
    ONNX --> Generator
    
    IR --> SDB
    SDB --> Cache
    S3 --> Parser
```

## Component Details

### 1. Language Parsers (Rust)
Each language has a dedicated parser module that:
- Tokenizes source code
- Builds Abstract Syntax Trees (AST)
- Extracts semantic information
- Handles language-specific idioms

```mermaid
graph LR
    Code[Source Code] --> Lexer[Lexer]
    Lexer --> Tokens[Token Stream]
    Tokens --> Parser[Parser]
    Parser --> AST[AST]
    AST --> Semantic[Semantic Analyzer]
    Semantic --> IR[Universal IR]
```

### 2. Universal Intermediate Representation (UIR)
The heart of Coalesce - a language-agnostic representation:

```mermaid
classDiagram
    class UIRNode {
        +id: String
        +node_type: NodeType
        +metadata: Metadata
        +children: Vec~UIRNode~
    }
    
    class NodeType {
        <<enumeration>>
        Function
        Class
        Variable
        Control
        Expression
    }
    
    class Metadata {
        +source_lang: Language
        +line_info: LineInfo
        +semantic_tags: Vec~String~
        +ml_embeddings: Vec~f32~
    }
    
    UIRNode --> NodeType
    UIRNode --> Metadata
```

### 3. Machine Learning Pipeline

```mermaid
graph TD
    subgraph "Training Pipeline"
        Data[Code Pairs] --> Prep[Preprocessing]
        Prep --> Embed[Embedding Generation]
        Embed --> Train[Candle Training]
        Train --> Model[Trained Models]
        Model --> ONNX[ONNX Export]
    end
    
    subgraph "Inference Pipeline"
        Input[UIR] --> Features[Feature Extraction]
        Features --> Runtime[ONNX Runtime]
        Runtime --> Predict[Predictions]
        Predict --> Generate[Code Generation]
    end
    
    subgraph "Learning Loop"
        Generate --> Review[Human Review]
        Review --> Feedback[Feedback]
        Feedback --> SML[SurrealML]
        SML --> Retrain[Model Update]
        Retrain --> Model
    end
```

### 4. Data Model (SurrealDB)

```mermaid
erDiagram
    PROJECT ||--o{ CODEBASE : contains
    CODEBASE ||--o{ MODULE : has
    MODULE ||--o{ UIR_NODE : contains
    UIR_NODE ||--o{ TRANSLATION : has
    TRANSLATION ||--o{ FEEDBACK : receives
    
    PROJECT {
        id string
        name string
        created datetime
        settings json
    }
    
    CODEBASE {
        id string
        project_id string
        language string
        version string
    }
    
    UIR_NODE {
        id string
        node_type enum
        embeddings vector
        metadata json
    }
    
    TRANSLATION {
        id string
        source_node string
        target_lang string
        generated_code text
        confidence float
    }
    
    FEEDBACK {
        id string
        translation_id string
        correction text
        ml_features vector
    }
```

## System Flow

```mermaid
sequenceDiagram
    participant User
    participant UI as SolidJS UI
    participant API as GraphQL API
    participant Parser
    participant ML as ML Pipeline
    participant DB as SurrealDB
    participant Gen as Generator
    
    User->>UI: Upload Code
    UI->>API: Parse Request
    API->>Parser: Parse Source
    Parser->>ML: Extract Features
    ML->>DB: Store UIR + Embeddings
    
    User->>UI: Select Target Language
    UI->>API: Translation Request
    API->>DB: Fetch UIR
    DB->>ML: Get Embeddings
    ML->>Gen: Generate Code
    Gen->>API: Return Translation
    API->>UI: Display Result
    
    User->>UI: Provide Feedback
    UI->>API: Store Correction
    API->>DB: Update Training Data
    DB->>ML: Trigger Learning
```

## Deployment Architecture

```mermaid
graph TB
    subgraph "Client"
        Browser[Web Browser]
        CLIClient[CLI Client]
    end
    
    subgraph "Edge Layer"
        CDN[CDN - SolidJS Assets]
        LB[Load Balancer]
    end
    
    subgraph "Application Layer"
        API1[API Server 1]
        API2[API Server 2]
        APIn[API Server N]
    end
    
    subgraph "Processing Layer"
        Parser1[Parser Service]
        Parser2[Parser Service]
        ML1[ML Service]
        ML2[ML Service]
    end
    
    subgraph "Data Layer"
        SDB1[(SurrealDB Primary)]
        SDB2[(SurrealDB Replica)]
        S3[(S3 Storage)]
        Redis[(Redis Cache)]
    end
    
    Browser --> CDN
    CLIClient --> LB
    CDN --> LB
    LB --> API1
    LB --> API2
    LB --> APIn
    
    API1 --> Parser1
    API2 --> Parser2
    API1 --> ML1
    API2 --> ML2
    
    Parser1 --> SDB1
    ML1 --> SDB1
    SDB1 --> SDB2
    API1 --> Redis
    Parser1 --> S3
```

## Security Architecture

- **Authentication**: JWT with refresh tokens
- **Authorization**: RBAC with SurrealDB's native permissions
- **Code Isolation**: Sandboxed parser execution
- **API Security**: Rate limiting, CORS, CSP headers
- **Data Encryption**: At-rest (SurrealDB) and in-transit (TLS)

## Performance Considerations

1. **Caching Strategy**
   - UIR nodes cached in Redis
   - ML inference results cached
   - Generated code cached with fingerprinting

2. **Parallel Processing**
   - Rust's async runtime for concurrent parsing
   - Batch processing for ML inference
   - Horizontal scaling for API servers

3. **Optimization**
   - ONNX Runtime for optimized inference
   - SurrealDB's graph queries for relationship traversal
   - Lazy loading of large codebases
