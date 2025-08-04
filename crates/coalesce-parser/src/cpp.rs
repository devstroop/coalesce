use tree_sitter::{Language, Node, Parser};
use coalesce_core::{UIRNode, NodeType, Metadata, SourceLocation, Language as CoalesceLanguage, 
                   ExpressionType, StatementType, Result, CoalesceError, Parser as CoalesceParser};
use serde_json::Value;
use std::collections::HashMap;

pub struct CppParser {
    parser: Parser,
}

impl CoalesceParser for CppParser {
    fn language(&self) -> CoalesceLanguage {
        CoalesceLanguage::Cpp
    }
    
    fn parse(&self, source: &str) -> Result<UIRNode> {
        // Create a new parser for this parse operation
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_cpp::language())
            .map_err(|e| CoalesceError::ParseError {
                message: format!("Failed to set C++ language: {}", e),
                line: 0,
                column: 0,
            })?;
            
        let tree = parser.parse(source, None)
            .ok_or_else(|| CoalesceError::ParseError {
                message: "Failed to parse C++ source".to_string(),
                line: 0,
                column: 0,
            })?;
        
        let root_node = tree.root_node();
        self.convert_to_uir(source, root_node, 0)
    }
}

impl CppParser {
    pub fn new() -> Result<Self> {
        // We don't need to store the parser, we'll create it per-parse
        Ok(Self { parser: tree_sitter::Parser::new() })
    }
    
    pub fn new_parser(&mut self) -> Result<UIRNode> {
        // This method will be removed, keeping for now to avoid compilation issues
        Ok(UIRNode::new("temp".to_string(), NodeType::Module))
    }
    
    fn convert_to_uir(&self, source: &str, node: Node, depth: usize) -> Result<UIRNode> {
        let node_type = node.kind();
        let start_position = node.start_position();
        let end_position = node.end_position();
        
        let source_location = SourceLocation {
            file: String::new(),
            start_line: start_position.row as u32 + 1,
            end_line: end_position.row as u32 + 1,
            start_column: start_position.column as u32,
            end_column: end_position.column as u32,
        };
        
        let original_text = node.utf8_text(source.as_bytes())
            .unwrap_or("").to_string();
        
        let mut annotations = HashMap::new();
        annotations.insert("original_text".to_string(), Value::String(original_text.clone()));
        
        let metadata = Metadata {
            source_language: CoalesceLanguage::Cpp,
            semantic_tags: vec![node_type.to_string()],
            complexity_score: None,
            dependencies: Vec::new(),
            annotations,
            legacy_patterns: Vec::new(),
        };
        
        // Generate unique ID
        let id = format!("{}_{}_{}_{}", 
            node_type.replace(" ", "_"), 
            start_position.row, 
            start_position.column,
            original_text.chars().take(15).collect::<String>().replace(" ", "_")
        );
        
        let (uir_node_type, name) = match node_type {
            "translation_unit" => (NodeType::Module, Some("cpp_program".to_string())),
            "function_definition" => {
                let func_name = self.extract_function_name(source, node);
                (NodeType::Function, func_name)
            }
            "function_declarator" => {
                let func_name = self.extract_function_name(source, node);
                (NodeType::Function, func_name)
            }
            "parameter_declaration" => {
                let param_name = self.extract_parameter_name(source, node);
                (NodeType::Variable, param_name)
            }
            "class_specifier" => {
                let class_name = self.extract_class_name(source, node);
                (NodeType::Class, class_name)
            }
            "method_definition" => {
                let method_name = self.extract_function_name(source, node);
                (NodeType::Function, method_name)
            }
            "identifier" => {
                let var_name = Some(original_text.clone());
                (NodeType::Expression(ExpressionType::Variable), var_name)
            }
            "number_literal" => {
                (NodeType::Expression(ExpressionType::Literal), None)
            }
            "string_literal" => {
                (NodeType::Expression(ExpressionType::Literal), None)
            }
            "char_literal" => {
                (NodeType::Expression(ExpressionType::Literal), None)
            }
            "true" | "false" => {
                (NodeType::Expression(ExpressionType::Literal), None)
            }
            "return_statement" => {
                (NodeType::Statement(StatementType::Return), None)
            }
            "binary_expression" => {
                (NodeType::Expression(ExpressionType::Arithmetic), None)
            }
            "call_expression" => {
                (NodeType::Expression(ExpressionType::FunctionCall), None)
            }
            "assignment_expression" => {
                (NodeType::Expression(ExpressionType::Assignment), None)
            }
            "if_statement" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Conditional), None)
            }
            "for_statement" | "for_range_loop" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Loop(coalesce_core::LoopType::For)), None)
            }
            "while_statement" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Loop(coalesce_core::LoopType::While)), None)
            }
            "try_statement" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Try), None)
            }
            "namespace_definition" => {
                let namespace_name = self.extract_namespace_name(source, node);
                (NodeType::Module, namespace_name)
            }
            _ => {
                // For other node types, try to categorize them generically
                if node_type.contains("statement") {
                    (NodeType::Statement(StatementType::Expression), None)
                } else if node_type.contains("expression") {
                    (NodeType::Expression(ExpressionType::Variable), None)
                } else {
                    (NodeType::Expression(ExpressionType::Literal), None)
                }
            }
        };
        
        let mut uir_node = UIRNode {
            id,
            node_type: uir_node_type,
            name,
            children: Vec::new(),
            metadata,
            source_location: Some(source_location),
        };
        
        // Process children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if !child.is_error() {
                let child_uir = self.convert_to_uir(source, child, depth + 1)?;
                uir_node.children.push(child_uir);
            }
        }
        
        Ok(uir_node)
    }
    
    fn extract_function_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(name.to_string());
                }
            } else if child.kind() == "function_declarator" {
                // Recursive search in function_declarator
                return self.extract_function_name(source, child);
            }
        }
        None
    }
    
    fn extract_parameter_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(name.to_string());
                }
            }
        }
        None
    }
    
    fn extract_class_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "type_identifier" || child.kind() == "identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(name.to_string());
                }
            }
        }
        None
    }
    
    fn extract_namespace_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(format!("namespace_{}", name));
                }
            }
        }
        Some("anonymous_namespace".to_string())
    }
}

extern "C" {
    fn tree_sitter_cpp() -> Language;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_cpp_function() {
        let mut parser = CppParser::new().unwrap();
        let source = "int add(int a, int b) { return a + b; }";
        
        let result = parser.parse(source);
        assert!(result.is_ok());
        
        let uir = result.unwrap();
        assert_eq!(uir.node_type, NodeType::Module);
        assert!(!uir.children.is_empty());
    }
    
    #[test]
    fn test_cpp_class() {
        let mut parser = CppParser::new().unwrap();
        let source = r#"
class Calculator {
public:
    int add(int a, int b) {
        return a + b;
    }
};
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_cpp_namespace() {
        let mut parser = CppParser::new().unwrap();
        let source = r#"
namespace math {
    int add(int a, int b) {
        return a + b;
    }
}
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
}
