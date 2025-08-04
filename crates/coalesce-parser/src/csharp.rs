use tree_sitter::{Language, Node, Parser};
use coalesce_core::{UIRNode, NodeType, Metadata, SourceLocation, Language as CoalesceLanguage, 
                   ExpressionType, StatementType, Result, CoalesceError, Parser as CoalesceParser};
use serde_json::Value;
use std::collections::HashMap;

pub struct CSharpParser {
    parser: Parser,
}

impl CoalesceParser for CSharpParser {
    fn language(&self) -> CoalesceLanguage {
        CoalesceLanguage::CSharp
    }
    
    fn parse(&self, source: &str) -> Result<UIRNode> {
        // Create a new parser for this parse operation
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(tree_sitter_c_sharp::language())
            .map_err(|e| CoalesceError::ParseError {
                message: format!("Failed to set C# language: {}", e),
                line: 0,
                column: 0,
            })?;
            
        let tree = parser.parse(source, None)
            .ok_or_else(|| CoalesceError::ParseError {
                message: "Failed to parse C# source".to_string(),
                line: 0,
                column: 0,
            })?;
        
        let root_node = tree.root_node();
        self.convert_to_uir(source, root_node, 0)
    }
}

impl CSharpParser {
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
            source_language: CoalesceLanguage::CSharp,
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
            "compilation_unit" => (NodeType::Module, Some("csharp_program".to_string())),
            "method_declaration" => {
                let method_name = self.extract_method_name(source, node);
                (NodeType::Function, method_name)
            }
            "constructor_declaration" => {
                let ctor_name = self.extract_constructor_name(source, node);
                (NodeType::Function, ctor_name)
            }
            "class_declaration" => {
                let class_name = self.extract_class_name(source, node);
                (NodeType::Class, class_name)
            }
            "interface_declaration" => {
                let interface_name = self.extract_interface_name(source, node);
                (NodeType::Interface, interface_name)
            }
            "struct_declaration" => {
                let struct_name = self.extract_struct_name(source, node);
                (NodeType::Class, struct_name)
            }
            "enum_declaration" => {
                let enum_name = self.extract_enum_name(source, node);
                (NodeType::Class, enum_name)
            }
            "parameter" => {
                let param_name = self.extract_parameter_name(source, node);
                (NodeType::Variable, param_name)
            }
            "identifier" => {
                let var_name = Some(original_text.clone());
                (NodeType::Expression(ExpressionType::Variable), var_name)
            }
            "integer_literal" | "real_literal" => {
                (NodeType::Expression(ExpressionType::Literal), None)
            }
            "string_literal" | "character_literal" => {
                (NodeType::Expression(ExpressionType::Literal), None)
            }
            "boolean_literal" => {
                (NodeType::Expression(ExpressionType::Literal), None)
            }
            "null_literal" => {
                (NodeType::Expression(ExpressionType::Literal), None)
            }
            "return_statement" => {
                (NodeType::Statement(StatementType::Return), None)
            }
            "binary_expression" => {
                (NodeType::Expression(ExpressionType::Arithmetic), None)
            }
            "invocation_expression" => {
                (NodeType::Expression(ExpressionType::FunctionCall), None)
            }
            "assignment_expression" => {
                (NodeType::Expression(ExpressionType::Assignment), None)
            }
            "if_statement" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Conditional), None)
            }
            "for_statement" | "foreach_statement" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Loop(coalesce_core::LoopType::For)), None)
            }
            "while_statement" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Loop(coalesce_core::LoopType::While)), None)
            }
            "do_statement" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Loop(coalesce_core::LoopType::DoWhile)), None)
            }
            "switch_statement" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Switch), None)
            }
            "try_statement" => {
                (NodeType::ControlFlow(coalesce_core::ControlFlowType::Try), None)
            }
            "namespace_declaration" => {
                let namespace_name = self.extract_namespace_name(source, node);
                (NodeType::Module, namespace_name)
            }
            "using_directive" => {
                let using_name = self.extract_using_name(source, node);
                (NodeType::Module, using_name)
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
    
    fn extract_method_name(&self, source: &str, node: Node) -> Option<String> {
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
    
    fn extract_constructor_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(format!("ctor_{}", name));
                }
            }
        }
        Some("constructor".to_string())
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
            if child.kind() == "identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(name.to_string());
                }
            }
        }
        None
    }
    
    fn extract_interface_name(&self, source: &str, node: Node) -> Option<String> {
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
            if child.kind() == "identifier" {
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
            if child.kind() == "identifier" {
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
            if child.kind() == "qualified_name" || child.kind() == "identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(format!("namespace_{}", name));
                }
            }
        }
        Some("global_namespace".to_string())
    }
    
    fn extract_using_name(&self, source: &str, node: Node) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "qualified_name" || child.kind() == "identifier" {
                if let Ok(name) = child.utf8_text(source.as_bytes()) {
                    return Some(format!("using_{}", name.replace(".", "_")));
                }
            }
        }
        Some("unknown_using".to_string())
    }
}

extern "C" {
    fn tree_sitter_c_sharp() -> Language;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_csharp_method() {
        let parser = CSharpParser::new().unwrap();
        let source = "public int Add(int a, int b) { return a + b; }";
        
        let result = parser.parse(source);
        assert!(result.is_ok());
        
        let uir = result.unwrap();
        assert_eq!(uir.node_type, NodeType::Module);
        assert!(!uir.children.is_empty());
    }
    
    #[test]
    fn test_csharp_class() {
        let parser = CSharpParser::new().unwrap();
        let source = r#"
public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
}
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_csharp_namespace() {
        let parser = CSharpParser::new().unwrap();
        let source = r#"
namespace MathLibrary {
    public class Calculator {
        public static int Add(int a, int b) {
            return a + b;
        }
    }
}
"#;
        
        let result = parser.parse(source);
        assert!(result.is_ok());
    }
}
