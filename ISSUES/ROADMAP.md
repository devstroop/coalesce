# Coalesce Development Roadmap & Next Steps

## Current State Analysis ✅

**What's Working:**
- Solid documentation and vision
- Basic Rust workspace architecture  
- CLI framework with demo functionality
- End-to-end pipeline (stub level)
- Comprehensive issue tracking system

**Critical Gaps:**
- All parsers are stubs (only detects "function" keyword)
- Code generation produces empty functions
- No test data to validate against
- ML pipeline completely missing
- No real-world validation

## Immediate Action Plan (Next 2 weeks)

### 🔥 **Week 1: Make It Real**
**Priority: Fix the stubs to do actual work**

**Day 1-2**: Issue #002 - Create Test Data Projects
- Banking COBOL project (INTEREST.COB, ACCOUNT.COB)
- Network C project (session.c, protocol.c) - SoftEtherVPN style
- Modern JavaScript project (user-service.js, auth.js)

**Day 3-4**: Issue #004 - Tree-sitter Integration  
- Add tree-sitter-javascript dependency
- Basic AST → UIR conversion
- Extract function names and parameters

**Day 5-7**: Issue #001 - Real JavaScript Parser
- Parse function declarations properly
- Extract parameters and return statements
- Handle basic expressions and variables

### 🎯 **Week 2: Generate Useful Code**
**Priority: Make output actually useful**

**Day 8-10**: Issue #003 - Fix Code Generation
- Generate functions with correct names and parameters
- Handle return statements
- Produce compilable Python/Rust code

**Day 11-12**: Integration Testing
- End-to-end tests with real code samples
- CLI improvements for better user experience
- Documentation updates

**Day 13-14**: SoftEtherVPN Case Study Setup
- Issue #005 - Start C parser for session management
- Parse one real C function from networking code
- Generate Rust equivalent

## Success Metrics (2 weeks)

### Functional Metrics
- [ ] Parse 10+ real JavaScript functions correctly
- [ ] Generate syntactically valid Python/Rust code  
- [ ] 90%+ test pass rate for basic function translation
- [ ] CLI usable by external developer

### Demo Capabilities
```bash
# Should work by end of 2 weeks:
cargo run --bin coalesce demo "function calculateTax(income, rate) { 
    if (income > 100000) return income * (rate + 0.05);
    return income * rate; 
}" --to python

# Expected output:
def calculate_tax(income, rate):
    if income > 100000:
        return income * (rate + 0.05)
    return income * rate
```

### Quality Gates
- All generated code must compile in target language
- Function names and parameters preserved accurately
- Basic control flow (if statements) handled
- No more "TODO" comments in critical paths

## Medium-term Roadmap (Month 2-3)

### 🧠 **Month 2: Intelligence Layer**
- Issue #005: Complete C parser for SoftEtherVPN
- Issue #007: SurrealDB integration
- Issue #008: Basic Candle ML pipeline
- Issue #009: Legacy pattern recognition

### 🏢 **Month 3: Real-world Validation**  
- SoftEtherVPN session management translation
- Banking COBOL → Python migration
- Performance benchmarking
- User feedback collection

## Technology Stack Priorities

### Immediate (Next 2 weeks)
1. **Tree-sitter** - Robust parsing foundation
2. **Test data** - Validation and development
3. **Basic UIR** - Function-level translation

### Near-term (Month 2)  
1. **SurrealDB** - Persistent storage and relationships
2. **Candle** - ML model development
3. **ONNX Runtime** - Production inference

### Future (Month 3+)
1. **SolidJS Frontend** - Visual interface
2. **GraphQL API** - Integration capabilities
3. **Advanced ML** - Pattern learning and optimization

## Risk Mitigation

### Technical Risks
- **Tree-sitter complexity**: Start with simple cases, expand gradually
- **C parsing difficulty**: Focus on specific SoftEtherVPN patterns first
- **ML integration**: Use simple models initially, enhance over time

### Project Risks  
- **Scope creep**: Stick to milestones, resist feature additions
- **Over-engineering**: Focus on working code over perfect architecture
- **Time estimation**: Add 50% buffer to all estimates

## Call to Action

**Start immediately with Issue #002** - creating test data. Everything else depends on having real code to work with.

The project has excellent vision and documentation. Now it needs **execution focus** to turn stubs into reality.

**Next command to run:**
```bash
mkdir -p test-projects/banking-cobol/src
mkdir -p test-projects/network-c/src  
mkdir -p test-projects/modern-js/src
# Then populate with real code samples
```
