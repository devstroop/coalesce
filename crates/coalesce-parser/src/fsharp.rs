use coalesce_core::{UIRNode, NodeType, Metadata, SourceLocation, Language as CoalesceLanguage, 
                   ExpressionType, StatementType, Result, CoalesceError, Parser as CoalesceParser};
use serde_json::Value;
use std::collections::HashMap;
use regex::Regex;

pub struct FSharpParser {
}

impl CoalesceParser for FSharpParser {
    fn language(&self) -> CoalesceLanguage {
        CoalesceLanguage::FSharp
    }
    
    fn parse(&self, source: &str) -> Result<UIRNode> {
        self.parse_fsharp_source(source)
    }
}

impl FSharpParser {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
    
    fn parse_fsharp_source(&self, source: &str) -> Result<UIRNode> {
        let mut root = UIRNode {
            id: "fsharp_program".to_string(),
            node_type: NodeType::Module,
            name: Some("fsharp_program".to_string()),
            children: Vec::new(),
            metadata: Metadata {
                source_language: CoalesceLanguage::FSharp,
                semantic_tags: vec!["source_file".to_string()],
                complexity_score: None,
                dependencies: Vec::new(),
                annotations: HashMap::new(),
                legacy_patterns: Vec::new(),
            },
            source_location: Some(SourceLocation {
                file: String::new(),
                start_line: 1,
                end_line: source.lines().count() as u32,
                start_column: 0,
                end_column: source.len() as u32,
            }),
        };
        
        // Parse different F# constructs
        self.parse_modules(source, &mut root)?;
        self.parse_types(source, &mut root)?;
        self.parse_functions(source, &mut root)?;
        self.parse_let_bindings(source, &mut root)?;
        
        Ok(root)
    }
    
    fn parse_modules(&self, source: &str, root: &mut UIRNode) -> Result<()> {
        let module_regex = Regex::new(r"(?m)^module\s+(\w+(?:\.\w+)*)\s*=?\s*$").unwrap();
        
        for caps in module_regex.captures_iter(source) {
            let module_name = caps.get(1).unwrap().as_str();
            let line_num = source[..caps.get(0).unwrap().start()].lines().count() + 1;
            
            let module_node = UIRNode {
                id: format!("module_{}", module_name),
                node_type: NodeType::Module,
                name: Some(module_name.to_string()),
                children: Vec::new(),
                metadata: Metadata {
                    source_language: CoalesceLanguage::FSharp,
                    semantic_tags: vec!["module".to_string()],
                    complexity_score: None,
                    dependencies: Vec::new(),
                    annotations: {
                        let mut map = HashMap::new();
                        map.insert("original_text".to_string(), Value::String(caps.get(0).unwrap().as_str().to_string()));
                        map
                    },
                    legacy_patterns: Vec::new(),
                },
                source_location: Some(SourceLocation {
                    file: String::new(),
                    start_line: line_num as u32,
                    end_line: line_num as u32,
                    start_column: 0,
                    end_column: caps.get(0).unwrap().len() as u32,
                }),
            };
            
            root.children.push(module_node);
        }
        
        Ok(())
    }
    
    fn parse_types(&self, source: &str, root: &mut UIRNode) -> Result<()> {
        // Parse type definitions
        let type_regex = Regex::new(r"(?m)^type\s+(\w+)(?:\s*=)?").unwrap();
        
        for caps in type_regex.captures_iter(source) {
            let type_name = caps.get(1).unwrap().as_str();
            let line_num = source[..caps.get(0).unwrap().start()].lines().count() + 1;
            
            let type_node = UIRNode {
                id: format!("type_{}", type_name),
                node_type: NodeType::Class,
                name: Some(type_name.to_string()),
                children: Vec::new(),
                metadata: Metadata {
                    source_language: CoalesceLanguage::FSharp,
                    semantic_tags: vec!["type".to_string()],
                    complexity_score: None,
                    dependencies: Vec::new(),
                    annotations: {
                        let mut map = HashMap::new();
                        map.insert("original_text".to_string(), Value::String(caps.get(0).unwrap().as_str().to_string()));
                        map
                    },
                    legacy_patterns: Vec::new(),
                },
                source_location: Some(SourceLocation {
                    file: String::new(),
                    start_line: line_num as u32,
                    end_line: line_num as u32,
                    start_column: 0,
                    end_column: caps.get(0).unwrap().len() as u32,
                }),
            };
            
            root.children.push(type_node);
        }
        
        Ok(())
    }
    
    fn parse_functions(&self, source: &str, root: &mut UIRNode) -> Result<()> {
        // Parse function definitions with explicit parameters
        let func_regex = Regex::new(r"(?m)^let\s+(\w+)\s+([^=]+?)\s*=").unwrap();
        
        for caps in func_regex.captures_iter(source) {
            let func_name = caps.get(1).unwrap().as_str();
            let params_str = caps.get(2).unwrap().as_str().trim();
            let line_num = source[..caps.get(0).unwrap().start()].lines().count() + 1;
            
            // Only treat as function if it has parameters
            if !params_str.is_empty() && params_str.chars().any(|c| c.is_alphabetic()) {
                let mut func_node = UIRNode {
                    id: format!("func_{}", func_name),
                    node_type: NodeType::Function,
                    name: Some(func_name.to_string()),
                    children: Vec::new(),
                    metadata: Metadata {
                        source_language: CoalesceLanguage::FSharp,
                        semantic_tags: vec!["function".to_string()],
                        complexity_score: None,
                        dependencies: Vec::new(),
                        annotations: {
                            let mut map = HashMap::new();
                            map.insert("original_text".to_string(), Value::String(caps.get(0).unwrap().as_str().to_string()));
                            map
                        },
                        legacy_patterns: Vec::new(),
                    },
                    source_location: Some(SourceLocation {
                        file: String::new(),
                        start_line: line_num as u32,
                        end_line: line_num as u32,
                        start_column: 0,
                        end_column: caps.get(0).unwrap().len() as u32,
                    }),
                };
                
                // Parse parameters
                for param in params_str.split_whitespace() {
                    if param.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        let param_node = UIRNode {
                            id: format!("param_{}", param),
                            node_type: NodeType::Variable,
                            name: Some(param.to_string()),
                            children: Vec::new(),
                            metadata: Metadata {
                                source_language: CoalesceLanguage::FSharp,
                                semantic_tags: vec!["parameter".to_string()],
                                complexity_score: None,
                                dependencies: Vec::new(),
                                annotations: HashMap::new(),
                                legacy_patterns: Vec::new(),
                            },
                            source_location: Some(SourceLocation {
                                file: String::new(),
                                start_line: line_num as u32,
                                end_line: line_num as u32,
                                start_column: 0,
                                end_column: param.len() as u32,
                            }),
                        };
                        func_node.children.push(param_node);
                    }
                }
                
                root.children.push(func_node);
            }
        }
        
        Ok(())
    }
    
    fn parse_let_bindings(&self, source: &str, root: &mut UIRNode) -> Result<()> {
        // Parse simple let bindings (variables)
        let let_regex = Regex::new(r"(?m)^let\s+(\w+)\s*=\s*([^=\r\n]+)").unwrap();
        
        for caps in let_regex.captures_iter(source) {
            let var_name = caps.get(1).unwrap().as_str();
            let value = caps.get(2).unwrap().as_str().trim();
            let line_num = source[..caps.get(0).unwrap().start()].lines().count() + 1;
            
            // Skip if this looks like a function (has parameters before =)
            let full_match = caps.get(0).unwrap().as_str();
            if full_match.matches(char::is_whitespace).count() <= 3 {
                let var_node = UIRNode {
                    id: format!("var_{}", var_name),
                    node_type: NodeType::Variable,
                    name: Some(var_name.to_string()),
                    children: Vec::new(),
                    metadata: Metadata {
                        source_language: CoalesceLanguage::FSharp,
                        semantic_tags: vec!["variable".to_string()],
                        complexity_score: None,
                        dependencies: Vec::new(),
                        annotations: {
                            let mut map = HashMap::new();
                            map.insert("original_text".to_string(), Value::String(caps.get(0).unwrap().as_str().to_string()));
                            map.insert("value".to_string(), Value::String(value.to_string()));
                            map
                        },
                        legacy_patterns: Vec::new(),
                    },
                    source_location: Some(SourceLocation {
                        file: String::new(),
                        start_line: line_num as u32,
                        end_line: line_num as u32,
                        start_column: 0,
                        end_column: caps.get(0).unwrap().len() as u32,
                    }),
                };
                
                root.children.push(var_node);
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_fsharp_function() {
        let parser = FSharpParser::new().unwrap();
        let source = "let add x y = x + y";
        
        let result = parser.parse(source);
        assert!(result.is_ok());
        
        let uir = result.unwrap();
        assert_eq!(uir.node_type, NodeType::Module);
        assert!(!uir.children.is_empty());
    }
    
    #[test]
    fn test_fsharp_type() {
        let parser = FSharpParser::new().unwrap();
        let source = r#"
type Person = {
    Name: string
    Age: int
}
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_fsharp_module() {
        let parser = FSharpParser::new().unwrap();
        let source = r#"
module Math =
    let add x y = x + y
    let multiply x y = x * y
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
}
