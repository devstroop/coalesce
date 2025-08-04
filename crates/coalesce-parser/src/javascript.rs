use coalesce_core::{types::*, errors::*, traits::Parser};
use tree_sitter::{Parser as TSParser, Node};

/// JavaScript parser using tree-sitter
pub struct JavaScriptParser {
    parser: TSParser,
}

impl Parser for JavaScriptParser {
    fn language(&self) -> coalesce_core::types::Language {
        coalesce_core::types::Language::JavaScript
    }
    
    fn parse(&self, source: &str) -> Result<UIRNode> {
        let mut parser_clone = self.clone();
        parser_clone.parse_source(source)
    }
}

impl Clone for JavaScriptParser {
    fn clone(&self) -> Self {
        JavaScriptParser::new().unwrap()
    }
}

impl JavaScriptParser {
    pub fn new() -> Result<Self> {
        let mut parser = TSParser::new();
        parser.set_language(tree_sitter_javascript::language())
            .map_err(|e| CoalesceError::ParseError {
                message: format!("Failed to set JavaScript language: {}", e),
                line: 0,
                column: 0,
            })?;
        
        Ok(JavaScriptParser { parser })
    }
    
    fn parse_source(&mut self, source: &str) -> Result<UIRNode> {
        let tree = self.parser.parse(source, None);
        
        match tree {
            Some(tree) => {
                if tree.root_node().has_error() {
                    self.handle_parse_error(source, tree.root_node())
                } else {
                    self.ast_to_uir(tree.root_node(), source)
                }
            }
            None => Err(CoalesceError::ParseError {
                message: "Failed to parse source code".to_string(),
                line: 0,
                column: 0,
            })
        }
    }
    
    fn ast_to_uir(&self, node: Node, source: &str) -> Result<UIRNode> {
        match node.kind() {
            "program" => self.convert_program(node, source),
            "function_declaration" => self.convert_function_declaration(node, source),
            "arrow_function" => self.convert_arrow_function(node, source),
            "class_declaration" => self.convert_class_declaration(node, source),
            "method_definition" => self.convert_method(node, source),
            "variable_declaration" => self.convert_variable_declaration(node, source),
            "return_statement" => self.convert_return_statement(node, source),
            "if_statement" => self.convert_if_statement(node, source),
            "call_expression" => self.convert_call_expression(node, source),
            "binary_expression" => self.convert_binary_expression(node, source),
            "identifier" => self.convert_identifier(node, source),
            "number" | "string" | "true" | "false" => self.convert_literal(node, source),
            _ => self.convert_generic(node, source),
        }
    }
    
    fn convert_program(&self, node: Node, source: &str) -> Result<UIRNode> {
        let mut children = Vec::new();
        
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if !child.is_extra() {
                    if let Ok(child_uir) = self.ast_to_uir(child, source) {
                        children.push(child_uir);
                    }
                }
                
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: NodeType::Module,
            name: Some("javascript_program".to_string()),
            children,
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    fn convert_function_declaration(&self, node: Node, source: &str) -> Result<UIRNode> {
        let name_node = self.find_child_by_kind(node, "identifier")
            .ok_or_else(|| CoalesceError::ParseError {
                message: "Function missing name".to_string(),
                line: node.start_position().row as u32 + 1,
                column: node.start_position().column as u32,
            })?;
        
        let function_name = self.node_text(name_node, source);
        
        // Get parameters
        let mut parameters = Vec::new();
        if let Some(params_node) = self.find_child_by_kind(node, "formal_parameters") {
            parameters = self.extract_parameters(params_node, source)?;
        }
        
        // Get body
        let mut body_children = Vec::new();
        if let Some(body_node) = self.find_child_by_kind(node, "statement_block") {
            body_children = self.extract_function_body(body_node, source)?;
        }
        
        // Combine parameters and body as children
        let mut children = parameters;
        children.extend(body_children);
        
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: NodeType::Function,
            name: Some(function_name.to_string()),
            children,
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    fn convert_arrow_function(&self, node: Node, source: &str) -> Result<UIRNode> {
        let mut parameters = Vec::new();
        
        if let Some(params_node) = self.find_child_by_kind(node, "formal_parameters") {
            parameters = self.extract_parameters(params_node, source)?;
        } else if let Some(param_node) = self.find_child_by_kind(node, "identifier") {
            parameters.push(UIRNode {
                id: self.generate_node_id(param_node, source),
                node_type: NodeType::Variable,
                name: Some(self.node_text(param_node, source).to_string()),
                children: vec![],
                metadata: self.create_metadata(param_node, source),
                source_location: self.create_source_location(param_node, ""),
            });
        }
        
        let mut body_children = Vec::new();
        if let Some(body_node) = self.find_child_by_kind(node, "statement_block") {
            body_children = self.extract_function_body(body_node, source)?;
        } else {
            let mut cursor = node.walk();
            if cursor.goto_first_child() {
                while cursor.goto_next_sibling() {
                    let child = cursor.node();
                    if child.kind() != "=>" && child.kind() != "formal_parameters" && child.kind() != "identifier" {
                        if let Ok(expr_uir) = self.ast_to_uir(child, source) {
                            body_children.push(expr_uir);
                        }
                        break;
                    }
                }
            }
        }
        
        let mut children = parameters;
        children.extend(body_children);
        
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: NodeType::Function,
            name: Some("arrow_function".to_string()),
            children,
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    fn convert_class_declaration(&self, node: Node, source: &str) -> Result<UIRNode> {
        let name_node = self.find_child_by_kind(node, "identifier")
            .ok_or_else(|| CoalesceError::ParseError {
                message: "Class missing name".to_string(),
                line: node.start_position().row as u32 + 1,
                column: node.start_position().column as u32,
            })?;
        
        let class_name = self.node_text(name_node, source);
        
        let mut children = Vec::new();
        if let Some(body_node) = self.find_child_by_kind(node, "class_body") {
            let mut cursor = body_node.walk();
            if cursor.goto_first_child() {
                loop {
                    let child = cursor.node();
                    if !child.is_extra() {
                        if let Ok(child_uir) = self.ast_to_uir(child, source) {
                            children.push(child_uir);
                        }
                    }
                    
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
        }
        
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: NodeType::Class,
            name: Some(class_name.to_string()),
            children,
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    fn convert_method(&self, node: Node, source: &str) -> Result<UIRNode> {
        let name_node = self.find_child_by_kind(node, "property_identifier")
            .or_else(|| self.find_child_by_kind(node, "identifier"))
            .ok_or_else(|| CoalesceError::ParseError {
                message: "Method missing name".to_string(),
                line: node.start_position().row as u32 + 1,
                column: node.start_position().column as u32,
            })?;
        
        let method_name = self.node_text(name_node, source);
        
        let mut parameters = Vec::new();
        if let Some(params_node) = self.find_child_by_kind(node, "formal_parameters") {
            parameters = self.extract_parameters(params_node, source)?;
        }
        
        let mut body_children = Vec::new();
        if let Some(body_node) = self.find_child_by_kind(node, "statement_block") {
            body_children = self.extract_function_body(body_node, source)?;
        }
        
        let mut children = parameters;
        children.extend(body_children);
        
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: NodeType::Function,
            name: Some(method_name.to_string()),
            children,
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    fn convert_variable_declaration(&self, node: Node, source: &str) -> Result<UIRNode> {
        let mut children = Vec::new();
        
        let declarators = self.children_by_kind(node, "variable_declarator");
        for declarator in declarators {
            if let Some(name_node) = self.find_child_by_kind(declarator, "identifier") {
                let var_name = self.node_text(name_node, source);
                
                let mut value_child = None;
                let mut cursor = declarator.walk();
                if cursor.goto_first_child() {
                    while cursor.goto_next_sibling() {
                        let child = cursor.node();
                        if child.kind() != "identifier" && child.kind() != "=" {
                            if let Ok(init_uir) = self.ast_to_uir(child, source) {
                                value_child = Some(Box::new(init_uir));
                            }
                            break;
                        }
                    }
                }
                
                let var_children = if let Some(val) = value_child {
                    vec![*val]
                } else {
                    vec![]
                };
                
                children.push(UIRNode {
                    id: self.generate_node_id(declarator, source),
                    node_type: NodeType::Variable,
                    name: Some(var_name.to_string()),
                    children: var_children,
                    metadata: self.create_metadata(declarator, source),
                    source_location: self.create_source_location(declarator, ""),
                });
            }
        }
        
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: NodeType::Statement(StatementType::Expression),
            name: Some("variable_declaration".to_string()),
            children,
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    fn convert_return_statement(&self, node: Node, source: &str) -> Result<UIRNode> {
        let mut children = Vec::new();
        
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            while cursor.goto_next_sibling() {
                let child = cursor.node();
                if child.kind() != ";" {
                    if let Ok(expr_uir) = self.ast_to_uir(child, source) {
                        children.push(expr_uir);
                    }
                }
            }
        }
        
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: NodeType::Statement(StatementType::Return),
            name: None,
            children,
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    fn convert_if_statement(&self, node: Node, source: &str) -> Result<UIRNode> {
        let mut children = Vec::new();
        
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                match child.kind() {
                    "parenthesized_expression" => {
                        if let Some(condition) = self.find_child_by_kind(child, "binary_expression")
                            .or_else(|| self.find_child_by_kind(child, "identifier"))
                            .or_else(|| self.find_child_by_kind(child, "call_expression")) {
                            if let Ok(cond_uir) = self.ast_to_uir(condition, source) {
                                children.push(cond_uir);
                            }
                        }
                    }
                    "statement_block" | "expression_statement" => {
                        if let Ok(body_uir) = self.ast_to_uir(child, source) {
                            children.push(body_uir);
                        }
                    }
                    _ => {}
                }
                
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: NodeType::ControlFlow(ControlFlowType::Conditional),
            name: Some("if_statement".to_string()),
            children,
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    fn convert_call_expression(&self, node: Node, source: &str) -> Result<UIRNode> {
        let mut children = Vec::new();
        
        if let Some(func_node) = self.find_child_by_kind(node, "identifier")
            .or_else(|| self.find_child_by_kind(node, "member_expression")) {
            if let Ok(func_uir) = self.ast_to_uir(func_node, source) {
                children.push(func_uir);
            }
        }
        
        if let Some(args_node) = self.find_child_by_kind(node, "arguments") {
            let mut cursor = args_node.walk();
            if cursor.goto_first_child() {
                loop {
                    let child = cursor.node();
                    if child.kind() != "(" && child.kind() != ")" && child.kind() != "," {
                        if let Ok(arg_uir) = self.ast_to_uir(child, source) {
                            children.push(arg_uir);
                        }
                    }
                    
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
        }
        
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: NodeType::Expression(ExpressionType::FunctionCall),
            name: None,
            children,
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    fn convert_binary_expression(&self, node: Node, source: &str) -> Result<UIRNode> {
        let mut children = Vec::new();
        
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                match child.kind() {
                    "+" | "-" | "*" | "/" | "%" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&" | "||" => {
                        // Skip operators - they're implicit in the binary expression type
                    }
                    _ => {
                        if let Ok(operand_uir) = self.ast_to_uir(child, source) {
                            children.push(operand_uir);
                        }
                    }
                }
                
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: NodeType::Expression(ExpressionType::Arithmetic),
            name: None,
            children,
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    fn convert_identifier(&self, node: Node, source: &str) -> Result<UIRNode> {
        let name = self.node_text(node, source);
        
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: NodeType::Expression(ExpressionType::Variable),
            name: Some(name.to_string()),
            children: vec![],
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    fn convert_literal(&self, node: Node, source: &str) -> Result<UIRNode> {
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: NodeType::Expression(ExpressionType::Literal),
            name: None,
            children: vec![],
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    fn convert_generic(&self, node: Node, source: &str) -> Result<UIRNode> {
        let mut children = Vec::new();
        
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if !child.is_extra() {
                    if let Ok(child_uir) = self.ast_to_uir(child, source) {
                        children.push(child_uir);
                    }
                }
                
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        
        Ok(UIRNode {
            id: self.generate_node_id(node, source),
            node_type: self.map_node_type(node.kind()),
            name: Some(node.kind().to_string()),
            children,
            metadata: self.create_metadata(node, source),
            source_location: self.create_source_location(node, ""),
        })
    }
    
    // Helper methods
    fn extract_parameters(&self, params_node: Node, source: &str) -> Result<Vec<UIRNode>> {
        let mut parameters = Vec::new();
        
        let mut cursor = params_node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if child.kind() == "identifier" {
                    let param_name = self.node_text(child, source);
                    parameters.push(UIRNode {
                        id: self.generate_node_id(child, source),
                        node_type: NodeType::Variable,
                        name: Some(param_name.to_string()),
                        children: vec![],
                        metadata: self.create_metadata(child, source),
                        source_location: self.create_source_location(child, ""),
                    });
                }
                
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        
        Ok(parameters)
    }
    
    fn extract_function_body(&self, body_node: Node, source: &str) -> Result<Vec<UIRNode>> {
        let mut statements = Vec::new();
        
        let mut cursor = body_node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                if !child.is_extra() && child.kind() != "{" && child.kind() != "}" {
                    if let Ok(stmt_uir) = self.ast_to_uir(child, source) {
                        statements.push(stmt_uir);
                    }
                }
                
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        
        Ok(statements)
    }
    
    fn node_text<'a>(&self, node: Node, source: &'a str) -> &'a str {
        &source[node.byte_range()]
    }
    
    fn children_by_kind<'a>(&self, node: Node<'a>, kind: &str) -> Vec<Node<'a>> {
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
    
    fn find_child_by_kind<'a>(&self, node: Node<'a>, kind: &str) -> Option<Node<'a>> {
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
    
    fn create_metadata(&self, node: Node, source: &str) -> Metadata {
        let mut metadata = Metadata::default();
        metadata.source_language = coalesce_core::types::Language::JavaScript;
        metadata.semantic_tags.push(node.kind().to_string());
        
        // Add text content as annotation for debugging
        let text = self.node_text(node, source);
        if text.len() < 100 {
            metadata.annotations.insert(
                "original_text".to_string(), 
                serde_json::Value::String(text.to_string())
            );
        }
        
        metadata
    }
    
    fn create_source_location(&self, node: Node, file: &str) -> Option<SourceLocation> {
        Some(SourceLocation {
            file: file.to_string(),
            start_line: node.start_position().row as u32 + 1,
            end_line: node.end_position().row as u32 + 1,
            start_column: node.start_position().column as u32,
            end_column: node.end_position().column as u32,
        })
    }
    
    fn generate_node_id(&self, node: Node, source: &str) -> String {
        let text = self.node_text(node, source);
        let kind = node.kind();
        let line = node.start_position().row;
        let col = node.start_position().column;
        
        format!("{}_{}_{}_{}", kind, line, col, 
                text.chars().take(20).collect::<String>()
                    .replace(|c: char| !c.is_alphanumeric(), "_"))
    }
    
    fn map_node_type(&self, kind: &str) -> NodeType {
        match kind {
            "program" | "source_file" => NodeType::Module,
            "function_declaration" | "function_definition" => NodeType::Function,
            "variable_declaration" | "variable_declarator" => NodeType::Variable,
            "if_statement" | "while_statement" | "for_statement" => NodeType::ControlFlow(ControlFlowType::Conditional),
            "return_statement" => NodeType::Statement(StatementType::Return),
            "expression_statement" => NodeType::Statement(StatementType::Expression),
            "assignment_expression" => NodeType::Expression(ExpressionType::Assignment),
            "binary_expression" | "unary_expression" => NodeType::Expression(ExpressionType::Arithmetic),
            "call_expression" => NodeType::Expression(ExpressionType::FunctionCall),
            "identifier" => NodeType::Expression(ExpressionType::Variable),
            "number" | "string" | "boolean" => NodeType::Expression(ExpressionType::Literal),
            "class_declaration" => NodeType::Class,
            _ => NodeType::Expression(ExpressionType::Literal), // Generic fallback
        }
    }
    
    fn handle_parse_error(&self, source: &str, root: Node) -> Result<UIRNode> {
        let errors = self.collect_error_nodes(root);
        let error_msg = format!(
            "Parse errors found: {} error nodes. First error at line {}",
            errors.len(),
            errors.first().map(|n| n.start_position().row + 1).unwrap_or(0)
        );
        
        Ok(UIRNode {
            id: "error_recovery".to_string(),
            node_type: NodeType::Module,
            name: Some("partial_parse".to_string()),
            children: vec![],
            metadata: {
                let mut metadata = Metadata::default();
                metadata.annotations.insert(
                    "parse_error".to_string(), 
                    serde_json::Value::String(error_msg)
                );
                metadata
            },
            source_location: None,
        })
    }
    
    fn collect_error_nodes<'a>(&self, node: Node<'a>) -> Vec<Node<'a>> {
        let mut errors = Vec::new();
        
        if node.is_error() {
            errors.push(node);
        }
        
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                errors.extend(self.collect_error_nodes(cursor.node()));
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
        
        errors
    }
}
