# Coalesce Issue Tracker

## Current Status Assessment (July 18, 2025)

### ✅ What's Working
- [x] Basic Rust workspace setup
- [x] Core UIR type definitions
- [x] CLI framework with demo commands
- [x] Stub parsers for JavaScript/Python
- [x] Stub generators for Python/Rust
- [x] End-to-end pipeline (basic level)
- [x] Comprehensive documentation structure

### ❌ What's Missing (Critical Gaps)
- [ ] **Real parsing**: All parsers are just stubs
- [ ] **Meaningful code generation**: Only generates empty functions
- [ ] **Test data**: No sample projects exist (src dirs deleted)
- [ ] **ML pipeline**: No Candle/ONNX integration
- [ ] **SurrealDB integration**: No database layer
- [ ] **Tree-sitter integration**: Promised but not implemented
- [ ] **Legacy pattern recognition**: Core feature missing
- [ ] **SoftEtherVPN case study**: Inspiration not yet utilized

## Milestone Structure

### 🏗️ [Milestone 1: Foundation Reality Check](./milestone-1-foundation/)
**Goal**: Make basic parsing and generation actually work  
**Timeline**: 2-3 weeks  
**Status**: 🔴 Not Started

### 🧠 [Milestone 2: Intelligence Layer](./milestone-2-intelligence/)
**Goal**: Add ML capabilities and pattern recognition  
**Timeline**: 4-6 weeks  
**Status**: 🔴 Not Started

### 🏢 [Milestone 3: Real-World Validation](./milestone-3-validation/)
**Goal**: SoftEtherVPN case study and production readiness  
**Timeline**: 6-8 weeks  
**Status**: 🔴 Not Started

### 🚀 [Milestone 4: Platform & Scale](./milestone-4-platform/)
**Goal**: Full platform with UI, API, and deployment  
**Timeline**: 8-12 weeks  
**Status**: 🔴 Not Started

## Priority Issues

### 🔥 P0 - Critical (Blocking Progress)
- [001 - Implement Real JavaScript Parser](./P0/001-real-js-parser.md)
- [002 - Create Test Data Projects](./P0/002-test-data.md)
- [003 - Fix Code Generation Stubs](./P0/003-code-generation.md)

### ⚡ P1 - High (Core Features)
- [004 - Add Tree-sitter Integration](./P1/004-tree-sitter.md)
- [005 - C Language Parser (SoftEtherVPN)](./P1/005-c-parser.md)
- [006 - COBOL Parser (Banking Demo)](./P1/006-cobol-parser.md)

### 📈 P2 - Medium (Enhancement)
- [007 - SurrealDB Integration](./P2/007-surrealdb.md)
- [008 - Candle ML Pipeline](./P2/008-candle-ml.md)
- [009 - Legacy Pattern Recognition](./P2/009-legacy-patterns.md)

### 🔮 P3 - Low (Future)
- [010 - GraphQL API](./P3/010-graphql-api.md)
- [011 - SolidJS Frontend](./P3/011-solidjs-frontend.md)
- [012 - ONNX Runtime Integration](./P3/012-onnx-runtime.md)

## How to Use This Tracker

1. **Start with P0 issues** - These are blocking all progress
2. **Work within milestones** - Follow the logical progression
3. **Update status** - Mark issues as In Progress/Done
4. **Create sample data** - Each parser needs test cases
5. **Document decisions** - Record architectural choices

## Current Blockers

The project currently has impressive documentation and architecture, but the actual implementation is mostly stubs. The biggest blockers are:

1. **No real parsing** - Tree-sitter integration is missing
2. **No test data** - Can't validate anything without sample code
3. **Stub generation** - Output is not useful
4. **No ML integration** - Core differentiator missing

**Recommendation**: Focus on Milestone 1 to get basic functionality working before adding more features.
