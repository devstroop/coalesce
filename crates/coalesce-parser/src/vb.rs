use coalesce_core::{UIRNode, NodeType, Metadata, SourceLocation, Language as CoalesceLanguage, 
                   ExpressionType, StatementType, Result, CoalesceError, Parser as CoalesceParser};
use serde_json::Value;
use std::collections::HashMap;
use regex::Regex;

pub struct VisualBasicParser {
}

impl CoalesceParser for VisualBasicParser {
    fn language(&self) -> CoalesceLanguage {
        CoalesceLanguage::VisualBasic
    }
    
    fn parse(&self, source: &str) -> Result<UIRNode> {
        self.parse_vb_source(source)
    }
}

impl VisualBasicParser {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
    
    fn parse_vb_source(&self, source: &str) -> Result<UIRNode> {
        let mut root = UIRNode {
            id: "vb_program".to_string(),
            node_type: NodeType::Module,
            name: Some("vb_program".to_string()),
            children: Vec::new(),
            metadata: Metadata {
                source_language: CoalesceLanguage::VisualBasic,
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
        
        // Parse different VB constructs
        self.parse_namespaces(source, &mut root)?;
        self.parse_modules(source, &mut root)?;
        self.parse_classes(source, &mut root)?;
        self.parse_functions(source, &mut root)?;
        self.parse_subs(source, &mut root)?;
        self.parse_properties(source, &mut root)?;
        
        Ok(root)
    }
    
    fn parse_namespaces(&self, source: &str, root: &mut UIRNode) -> Result<()> {
        let namespace_regex = Regex::new(r"(?mi)^Namespace\s+(\w+(?:\.\w+)*)\s*$").unwrap();
        
        for caps in namespace_regex.captures_iter(source) {
            let namespace_name = caps.get(1).unwrap().as_str();
            let line_num = source[..caps.get(0).unwrap().start()].lines().count() + 1;
            
            let namespace_node = UIRNode {
                id: format!("namespace_{}", namespace_name),
                node_type: NodeType::Module,
                name: Some(namespace_name.to_string()),
                children: Vec::new(),
                metadata: Metadata {
                    source_language: CoalesceLanguage::VisualBasic,
                    semantic_tags: vec!["namespace".to_string()],
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
            
            root.children.push(namespace_node);
        }
        
        Ok(())
    }
    
    fn parse_modules(&self, source: &str, root: &mut UIRNode) -> Result<()> {
        let module_regex = Regex::new(r"(?mi)^(?:Public\s+|Private\s+)?Module\s+(\w+)\s*$").unwrap();
        
        for caps in module_regex.captures_iter(source) {
            let module_name = caps.get(1).unwrap().as_str();
            let line_num = source[..caps.get(0).unwrap().start()].lines().count() + 1;
            
            let module_node = UIRNode {
                id: format!("module_{}", module_name),
                node_type: NodeType::Module,
                name: Some(module_name.to_string()),
                children: Vec::new(),
                metadata: Metadata {
                    source_language: CoalesceLanguage::VisualBasic,
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
    
    fn parse_classes(&self, source: &str, root: &mut UIRNode) -> Result<()> {
        let class_regex = Regex::new(r"(?mi)^(?:Public\s+|Private\s+)?Class\s+(\w+)\s*$").unwrap();
        
        for caps in class_regex.captures_iter(source) {
            let class_name = caps.get(1).unwrap().as_str();
            let line_num = source[..caps.get(0).unwrap().start()].lines().count() + 1;
            
            let class_node = UIRNode {
                id: format!("class_{}", class_name),
                node_type: NodeType::Class,
                name: Some(class_name.to_string()),
                children: Vec::new(),
                metadata: Metadata {
                    source_language: CoalesceLanguage::VisualBasic,
                    semantic_tags: vec!["class".to_string()],
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
            
            root.children.push(class_node);
        }
        
        Ok(())
    }
    
    fn parse_functions(&self, source: &str, root: &mut UIRNode) -> Result<()> {
        let func_regex = Regex::new(r"(?mi)^(?:Public\s+|Private\s+|Protected\s+)?Function\s+(\w+)\s*\(([^)]*)\)(?:\s+As\s+\w+)?\s*$").unwrap();
        
        for caps in func_regex.captures_iter(source) {
            let func_name = caps.get(1).unwrap().as_str();
            let params_str = caps.get(2).map_or("", |m| m.as_str()).trim();
            let line_num = source[..caps.get(0).unwrap().start()].lines().count() + 1;
            
            let mut func_node = UIRNode {
                id: format!("func_{}", func_name),
                node_type: NodeType::Function,
                name: Some(func_name.to_string()),
                children: Vec::new(),
                metadata: Metadata {
                    source_language: CoalesceLanguage::VisualBasic,
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
            if !params_str.is_empty() {
                for param in params_str.split(',') {
                    let param = param.trim();
                    if let Some(param_name) = param.split_whitespace().next() {
                        if param_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                            let param_node = UIRNode {
                                id: format!("param_{}", param_name),
                                node_type: NodeType::Variable,
                                name: Some(param_name.to_string()),
                                children: Vec::new(),
                                metadata: Metadata {
                                    source_language: CoalesceLanguage::VisualBasic,
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
                                    end_column: param_name.len() as u32,
                                }),
                            };
                            func_node.children.push(param_node);
                        }
                    }
                }
            }
            
            root.children.push(func_node);
        }
        
        Ok(())
    }
    
    fn parse_subs(&self, source: &str, root: &mut UIRNode) -> Result<()> {
        let sub_regex = Regex::new(r"(?mi)^(?:Public\s+|Private\s+|Protected\s+)?Sub\s+(\w+)\s*\(([^)]*)\)\s*$").unwrap();
        
        for caps in sub_regex.captures_iter(source) {
            let sub_name = caps.get(1).unwrap().as_str();
            let params_str = caps.get(2).map_or("", |m| m.as_str()).trim();
            let line_num = source[..caps.get(0).unwrap().start()].lines().count() + 1;
            
            let mut sub_node = UIRNode {
                id: format!("sub_{}", sub_name),
                node_type: NodeType::Function,
                name: Some(sub_name.to_string()),
                children: Vec::new(),
                metadata: Metadata {
                    source_language: CoalesceLanguage::VisualBasic,
                    semantic_tags: vec!["sub".to_string()],
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
            if !params_str.is_empty() {
                for param in params_str.split(',') {
                    let param = param.trim();
                    if let Some(param_name) = param.split_whitespace().next() {
                        if param_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                            let param_node = UIRNode {
                                id: format!("param_{}", param_name),
                                node_type: NodeType::Variable,
                                name: Some(param_name.to_string()),
                                children: Vec::new(),
                                metadata: Metadata {
                                    source_language: CoalesceLanguage::VisualBasic,
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
                                    end_column: param_name.len() as u32,
                                }),
                            };
                            sub_node.children.push(param_node);
                        }
                    }
                }
            }
            
            root.children.push(sub_node);
        }
        
        Ok(())
    }
    
    fn parse_properties(&self, source: &str, root: &mut UIRNode) -> Result<()> {
        let prop_regex = Regex::new(r"(?mi)^(?:Public\s+|Private\s+|Protected\s+)?Property\s+(\w+)\s*(?:\([^)]*\))?\s*As\s+\w+\s*$").unwrap();
        
        for caps in prop_regex.captures_iter(source) {
            let prop_name = caps.get(1).unwrap().as_str();
            let line_num = source[..caps.get(0).unwrap().start()].lines().count() + 1;
            
            let prop_node = UIRNode {
                id: format!("prop_{}", prop_name),
                node_type: NodeType::Variable,
                name: Some(prop_name.to_string()),
                children: Vec::new(),
                metadata: Metadata {
                    source_language: CoalesceLanguage::VisualBasic,
                    semantic_tags: vec!["property".to_string()],
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
            
            root.children.push(prop_node);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_vb_function() {
        let parser = VisualBasicParser::new().unwrap();
        let source = r#"
Function Add(a As Integer, b As Integer) As Integer
    Return a + b
End Function
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
        
        let uir = result.unwrap();
        assert_eq!(uir.node_type, NodeType::Module);
        assert!(!uir.children.is_empty());
    }
    
    #[test]
    fn test_vb_class() {
        let parser = VisualBasicParser::new().unwrap();
        let source = r#"
Public Class Calculator
    Public Function Add(a As Integer, b As Integer) As Integer
        Return a + b
    End Function
End Class
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_vb_module() {
        let parser = VisualBasicParser::new().unwrap();
        let source = r#"
Module MathModule
    Sub Main()
        Console.WriteLine("Hello World!")
    End Sub
End Module
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
}
