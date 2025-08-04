# Milestone 1: Foundation Reality Check

## Overview
Transform Coalesce from "impressive documentation with stubs" to "working proof-of-concept with real code translation."

## Success Criteria
- [ ] Parse real JavaScript functions and extract names, parameters, body
- [ ] Generate working Python/Rust code that actually does something
- [ ] End-to-end demo with meaningful output
- [ ] At least 3 real test cases (JS→Python, JS→Rust, simple C→Rust)

## Timeline: 2-3 weeks

## Issues in This Milestone

### Week 1: Real Parsing
- **001** - Implement Real JavaScript Parser
- **002** - Create Test Data Projects  
- **004** - Add Tree-sitter Integration

### Week 2: Real Generation
- **003** - Fix Code Generation Stubs
- **013** - Extract Function Information from UIR
- **014** - Parameter and Return Type Inference

### Week 3: Integration & Testing
- **015** - End-to-End Integration Tests
- **016** - CLI Improvements for Real Usage
- **017** - Documentation Updates

## Dependencies
- Tree-sitter JavaScript grammar
- Sample code projects
- UIR enhancement for real data

## Deliverables
1. **Working JavaScript Parser** - Extract real function info
2. **Meaningful Code Generation** - Functions with correct names/params
3. **Test Suite** - Automated validation
4. **Demo Script** - Showcase real translation

## Risk Assessment
- **Low Risk**: Parsing is well-understood problem
- **Medium Risk**: Code generation complexity
- **High Risk**: Time estimation - may take longer than 3 weeks

## Success Metrics
- Parse 10+ JavaScript functions correctly
- Generate syntactically valid target code
- 90%+ test pass rate
- CLI usable by external developer
