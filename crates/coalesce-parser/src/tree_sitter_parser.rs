use coalesce_core::{types::*, errors::*};
use tree_sitter::{Language, Parser as TSParser, Tree, Node};
use std::collections::HashMap;

/// Base trait for all tree-sitter based parsers
pub trait TreeSitterParser {
    /// Get the tree-sitter language for this parser
    fn language() -> Language;
    
    /// Create a new parser instance
    fn new() -> Result<Self> 
    where 
        Self: Sized;
    
    /// Parse source code into UIR
    fn parse(&mut self, source: &str) -> Result<UIRNode>;
    
    /// Convert a tree-sitter AST node to UIR
    fn ast_to_uir(&self, node: Node, source: &str) -> Result<UIRNode>;
    
    /// Get the tree-sitter parser instance
    fn parser(&mut self) -> &mut TSParser;
}

/// Helper functions for tree-sitter operations
pub struct TreeSitterHelpers;

impl TreeSitterHelpers {
    /// Extract text content from a node
    pub fn node_text<'a>(node: Node, source: &'a str) -> &'a str {
        &source[node.byte_range()]
    }
    
    /// Get all children of a specific kind
    pub fn children_by_kind<'a>(node: Node<'a>, kind: &str) -> Vec<Node<'a>> {
        let mut cursor = node.walk();
        let mut children = Vec::new();
        
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == kind {
                    children.push(child);
                }
                
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        
        children
    }
    
    /// Find first child of a specific kind
    pub fn find_child_by_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
        let mut cursor = node.walk();
        
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == kind {
                    return Some(child);
                }
                
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        
        None
    }
    
    /// Create UIR metadata from node position
    pub fn create_metadata(node: Node) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        
        metadata.insert("start_line".to_string(), node.start_position().row.to_string());
        metadata.insert("start_column".to_string(), node.start_position().column.to_string());
        metadata.insert("end_line".to_string(), node.end_position().row.to_string());
        metadata.insert("end_column".to_string(), node.end_position().column.to_string());
        metadata.insert("byte_start".to_string(), node.start_byte().to_string());
        metadata.insert("byte_end".to_string(), node.end_byte().to_string());
        metadata.insert("node_kind".to_string(), node.kind().to_string());
        
        metadata
    }
    
    /// Generate unique ID for UIR node
    pub fn generate_node_id(node: Node, source: &str) -> String {
        let text = Self::node_text(node, source);
        let kind = node.kind();
        let line = node.start_position().row;
        let col = node.start_position().column;
        
        // Create a hash-like ID based on content and position
        format!("{}_{}_{}_{}", kind, line, col, 
                text.chars().take(20).collect::<String>()
                    .replace(|c: char| !c.is_alphanumeric(), "_"))
    }
    
    /// Handle tree-sitter errors gracefully
    pub fn handle_parse_error(source: &str, tree: Option<Tree>) -> Result<UIRNode> {
        match tree {
            Some(tree) => {
                let root = tree.root_node();
                if root.has_error() {
                    // Find error nodes and provide detailed information
                    let errors = Self::collect_error_nodes(root);
                    let error_msg = format!(
                        "Parse errors found: {} error nodes. First error at line {}",
                        errors.len(),
                        errors.first().map(|n| n.start_position().row + 1).unwrap_or(0)
                    );
                    
                    // Still try to create partial UIR
                    Ok(UIRNode {
                        id: "error_recovery".to_string(),
                        node_type: NodeType::Program,
                        name: Some("partial_parse".to_string()),
                        value: Some(error_msg),
                        children: vec![],
                        metadata: HashMap::new(),
                    })
                } else {
                    // Tree parsed successfully but might be empty
                    Ok(UIRNode {
                        id: "empty_program".to_string(),
                        node_type: NodeType::Program,
                        name: None,
                        value: None,
                        children: vec![],
                        metadata: HashMap::new(),
                    })
                }
            }
            None => Err(CoalesceError::ParseError("Failed to parse source code".to_string()))
        }
    }
    
    /// Recursively collect all error nodes in the tree
    fn collect_error_nodes(node: Node) -> Vec<Node> {
        let mut errors = Vec::new();
        
        if node.is_error() {
            errors.push(node);
        }
        
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                errors.extend(Self::collect_error_nodes(cursor.node()));
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        
        errors
    }
    
    /// Convert tree-sitter node kind to UIR NodeType
    pub fn map_node_type(kind: &str) -> NodeType {
        match kind {
            "program" | "source_file" => NodeType::Program,
            "function_declaration" | "function_definition" => NodeType::Function,
            "variable_declaration" | "variable_declarator" => NodeType::Variable,
            "if_statement" | "while_statement" | "for_statement" => NodeType::Statement(StatementType::Control),
            "return_statement" => NodeType::Statement(StatementType::Return),
            "expression_statement" => NodeType::Statement(StatementType::Expression),
            "assignment_expression" => NodeType::Statement(StatementType::Assignment),
            "binary_expression" | "unary_expression" => NodeType::Expression(ExpressionType::Binary),
            "call_expression" => NodeType::Expression(ExpressionType::Call),
            "identifier" => NodeType::Expression(ExpressionType::Identifier),
            "number" | "string" | "boolean" => NodeType::Expression(ExpressionType::Literal),
            "comment" => NodeType::Comment,
            "class_declaration" => NodeType::Class,
            _ => NodeType::Generic,
        }
    }
}
