# Issue #001: Implement Real JavaScript Parser

**Priority**: P0 - Critical  
**Milestone**: Foundation Reality Check  
**Assignee**: TBD  
**Status**: 🔴 Not Started  

## Problem
Current JavaScript parser is a stub that only detects `function` keyword. It doesn't extract:
- Function names
- Parameters  
- Function body
- Return statements
- Variable declarations

## Current Code
```rust
// In coalesce-parser/src/lib.rs
if source.trim().starts_with("function") {
    let func_node = UIRNode::new("js-function".to_string(), NodeType::Function);
    return Ok(root.add_child(func_node));
}
```

## Goal
Parse real JavaScript and populate UIR with:
- Function name in `uir.name`
- Parameters in UIR children
- Function body as statements
- Proper source location tracking

## Acceptance Criteria
- [ ] Parse `function add(a, b) { return a + b; }` correctly
- [ ] Extract function name: "add"
- [ ] Extract parameters: ["a", "b"]  
- [ ] Extract return statement
- [ ] Handle edge cases: arrow functions, anonymous functions
- [ ] Proper error handling for invalid JavaScript

## Technical Approach
1. **Add tree-sitter-javascript dependency**
   ```toml
   tree-sitter-javascript = "0.20"
   ```

2. **Create AST→UIR mapping**
   - Function declarations → Function UIR nodes
   - Parameters → Variable UIR nodes
   - Statements → Statement UIR nodes

3. **Update UIR structure to hold real data**
   ```rust
   // Function node should have:
   uir.name = Some("add".to_string())
   uir.children = [param_a, param_b, return_stmt]
   ```

## Test Cases
```javascript
// Test 1: Simple function
function add(a, b) { return a + b; }

// Test 2: Arrow function  
const multiply = (x, y) => x * y;

// Test 3: Complex function
function calculateTax(income, rate) {
    if (income > 100000) {
        return income * (rate + 0.05);
    }
    return income * rate;
}
```

## Dependencies
- Issue #004: Tree-sitter Integration
- Issue #002: Test Data Projects

## Estimated Effort
**3-5 days**

## Implementation Notes
- Start with function declarations only
- Add arrow functions in follow-up
- Focus on correctness over performance
- Good error messages for debugging

## Related Issues
- #003: Code generation needs real function data
- #013: UIR enhancement for extracted data

---
**Created**: July 18, 2025  
**Updated**: July 18, 2025
