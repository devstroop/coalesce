use tree_sitter::{Language, Node, Parser};
use coalesce_core::{UIRNode, NodeType, Metadata, SourceLocation, Language as CoalesceLanguage, 
                   ExpressionType, StatementType, Result, CoalesceError, Parser as CoalesceParser};
use serde_json::Value;
use std::collections::HashMap;

pub struct GoParser {
    parser: Parser,
}

impl CoalesceParser for GoParser {
    fn language(&self) -> CoalesceLanguage {
        CoalesceLanguage::Go
    }
    
    fn parse(&self, source: &str) -> Result<UIRNode> {
        // Create a new parser for this parse operation
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_go::language())
            .map_err(|e| CoalesceError::ParseError {
                message: format!("Failed to set Go language: {}", e),
                line: 0,
                column: 0,
            })?;
            
        let tree = parser.parse(source, None)
            .ok_or_else(|| CoalesceError::ParseError {
                message: "Failed to parse Go source".to_string(),
                line: 0,
                column: 0,
            })?;
        
        let root_node = tree.root_node();
        self.convert_to_uir(source, root_node, 0)
    }
}

impl GoParser {
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
            source_language: CoalesceLanguage::Go,
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
            "source_file" => (NodeType::Module, Some("go_program".to_string())),
            "function_declaration" | "method_declaration" => {
                let func_name = self.extract_function_name(source, node);
                (NodeType::Function, func_name)
            }
            "type_declaration" => {
                let type_name = self.extract_type_name(source, node);
                (NodeType::Class, type_name)
            }
            "struct_type" => {
                let struct_name = self.extract_struct_name(source, node);
                (NodeType::Class, struct_name)
            }
            "interface_type" => {
                let interface_name = self.extract_interface_name(source, node);
                (NodeType::Interface, interface_name)
            }
            "parameter_declaration" => {
                let param_name = self.extract_parameter_name(source, node);
                (NodeType::Variable, param_name)
            }
            "identifier" => {
                let var_name = Some(original_text.clone());
                (NodeType::Expression(ExpressionType::Variable), var_name)
            }
            "int_literal" | "float_literal" => {
                (NodeType::Expression(ExpressionType::Literal), None)
            }
            "interpreted_string_literal" | "raw_string_literal" | "rune_literal" => {
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
            "assignment_statement" => {
                (NodeType::Expression(ExpressionType::Assignment), None)
            }
            "if_statement" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Conditional), None)
            }
            "for_statement" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Loop(coalesce_core::LoopType::For)), None)
            }
            "range_clause" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Loop(coalesce_core::LoopType::ForEach)), None)
            }
            "switch_statement" | "type_switch_statement" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Switch), None)
            }
            "package_clause" => {
                let package_name = self.extract_package_name(source, node);
                (NodeType::Module, package_name)
            }
            "import_declaration" => {
                let import_name = self.extract_import_name(source, node);
                (NodeType::Module, import_name)
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
    
    fn extract_type_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "type_spec" {
                return self.extract_type_spec_name(source, child);
            }
        }
        None
    }
    
    fn extract_type_spec_name(&self, source: &str, node: Node) -> Option<String> {
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
    
    fn extract_struct_name(&self, source: &str, node: Node) -> Option<String> {
        // For anonymous structs
        Some("anonymous_struct".to_string())
    }
    
    fn extract_interface_name(&self, source: &str, node: Node) -> Option<String> {
        // For anonymous interfaces
        Some("anonymous_interface".to_string())
    }
    
    fn extract_package_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "package_identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(format!("package_{}", name));
                }
            }
        }
        Some("main_package".to_string())
    }
    
    fn extract_import_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "import_spec" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(format!("import_{}", name.replace("\"", "").replace("/", "_")));
                }
            }
        }
        Some("unknown_import".to_string())
    }
}

extern "C" {
    fn tree_sitter_go() -> Language;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_go_function() {
        let mut parser = GoParser::new().unwrap();
        let source = "func add(a, b int) int { return a + b }";
        
        let result = parser.parse(source);
        assert!(result.is_ok());
        
        let uir = result.unwrap();
        assert_eq!(uir.node_type, NodeType::Module);
        assert!(!uir.children.is_empty());
    }
    
    #[test]
    fn test_go_struct() {
        let mut parser = GoParser::new().unwrap();
        let source = r#"
type Point struct {
    X, Y float64
}

func (p Point) Distance() float64 {
    return math.Sqrt(p.X*p.X + p.Y*p.Y)
}
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_go_interface() {
        let mut parser = GoParser::new().unwrap();
        let source = r#"
type Writer interface {
    Write([]byte) (int, error)
}
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_go_package() {
        let mut parser = GoParser::new().unwrap();
        let source = r#"
package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
}
