use tree_sitter::{Language, Node, Parser};
use coalesce_core::{UIRNode, NodeType, Metadata, SourceLocation, Language as CoalesceLanguage, 
                   ExpressionType, StatementType, Result, CoalesceError, Parser as CoalesceParser};
use serde_json::Value;
use std::collections::HashMap;

pub struct RustParser {
    parser: Parser,
}

impl CoalesceParser for RustParser {
    fn language(&self) -> CoalesceLanguage {
        CoalesceLanguage::Rust
    }
    
    fn parse(&self, source: &str) -> Result<UIRNode> {
        // Create a new parser for this parse operation
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_rust::language())
            .map_err(|e| CoalesceError::ParseError {
                message: format!("Failed to set Rust language: {}", e),
                line: 0,
                column: 0,
            })?;
            
        let tree = parser.parse(source, None)
            .ok_or_else(|| CoalesceError::ParseError {
                message: "Failed to parse Rust source".to_string(),
                line: 0,
                column: 0,
            })?;
        
        let root_node = tree.root_node();
        self.convert_to_uir(source, root_node, 0)
    }
}

impl RustParser {
    pub fn new() -> Result<Self> {
        // We don't need to store the parser, we'll create it per-parse
        Ok(Self { parser: tree_sitter::Parser::new() })
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
            source_language: CoalesceLanguage::Rust,
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
            "source_file" => (NodeType::Module, Some("rust_program".to_string())),
            "function_item" => {
                let func_name = self.extract_function_name(source, node);
                (NodeType::Function, func_name)
            }
            "impl_item" => {
                let impl_name = self.extract_impl_name(source, node);
                (NodeType::Class, impl_name)
            }
            "struct_item" => {
                let struct_name = self.extract_struct_name(source, node);
                (NodeType::Class, struct_name)
            }
            "enum_item" => {
                let enum_name = self.extract_enum_name(source, node);
                (NodeType::Class, enum_name)
            }
            "trait_item" => {
                let trait_name = self.extract_trait_name(source, node);
                (NodeType::Interface, trait_name)
            }
            "parameter" => {
                let param_name = self.extract_parameter_name(source, node);
                (NodeType::Variable, param_name)
            }
            "identifier" => {
                let var_name = Some(original_text.clone());
                (NodeType::Expression(ExpressionType::Variable), var_name)
            }
            "integer_literal" | "float_literal" => {
                (NodeType::Expression(ExpressionType::Literal), None)
            }
            "string_literal" | "raw_string_literal" | "char_literal" => {
                (NodeType::Expression(ExpressionType::Literal), None)
            }
            "boolean_literal" => {
                (NodeType::Expression(ExpressionType::Literal), None)
            }
            "return_expression" => {
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
            "if_expression" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Conditional), None)
            }
            "for_expression" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Loop(coalesce_core::LoopType::For)), None)
            }
            "while_expression" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Loop(coalesce_core::LoopType::While)), None)
            }
            "loop_expression" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Loop(coalesce_core::LoopType::While)), None)
            }
            "match_expression" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Switch), None)
            }
            "mod_item" => {
                let mod_name = self.extract_mod_name(source, node);
                (NodeType::Module, mod_name)
            }
            _ => {
                // For other node types, try to categorize them generically
                if node_type.contains("statement") || node_type.contains("expression_statement") {
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
    
    fn extract_struct_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "type_identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(name.to_string());
                }
            }
        }
        None
    }
    
    fn extract_enum_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "type_identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(name.to_string());
                }
            }
        }
        None
    }
    
    fn extract_trait_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "type_identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(name.to_string());
                }
            }
        }
        None
    }
    
    fn extract_impl_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "type_identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(format!("impl_{}", name));
                }
            }
        }
        Some("anonymous_impl".to_string())
    }
    
    fn extract_mod_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(format!("mod_{}", name));
                }
            }
        }
        Some("anonymous_mod".to_string())
    }
}

extern "C" {
    fn tree_sitter_rust() -> Language;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_rust_function() {
        let mut parser = RustParser::new().unwrap();
        let source = "fn add(a: i32, b: i32) -> i32 { a + b }";
        
        let result = parser.parse(source);
        assert!(result.is_ok());
        
        let uir = result.unwrap();
        assert_eq!(uir.node_type, NodeType::Module);
        assert!(!uir.children.is_empty());
    }
    
    #[test]
    fn test_rust_struct() {
        let mut parser = RustParser::new().unwrap();
        let source = r#"
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_rust_enum() {
        let mut parser = RustParser::new().unwrap();
        let source = r#"
enum Result<T, E> {
    Ok(T),
    Err(E),
}
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
}
