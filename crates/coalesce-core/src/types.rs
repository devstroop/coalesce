use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal Intermediate Representation Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIRNode {
    pub id: String,
    pub node_type: NodeType,
    pub name: Option<String>,
    pub children: Vec<UIRNode>,
    pub metadata: Metadata,
    pub source_location: Option<SourceLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Module,
    Function,
    Class,
    Interface,
    Variable,
    Constant,
    ControlFlow(ControlFlowType),
    Expression(ExpressionType),
    Statement(StatementType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlFlowType {
    Conditional,
    Loop(LoopType),
    Switch,
    Try,
    Goto, // For legacy pattern preservation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoopType {
    For,
    While,
    DoWhile,
    ForEach,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionType {
    Literal,
    Variable,
    FunctionCall,
    Arithmetic,
    Comparison,
    Logical,
    Assignment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatementType {
    Expression,
    Return,
    Break,
    Continue,
    Throw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub source_language: Language,
    pub semantic_tags: Vec<String>,
    pub complexity_score: Option<f32>,
    pub dependencies: Vec<String>,
    pub annotations: HashMap<String, serde_json::Value>,
    pub legacy_patterns: Vec<LegacyPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyPattern {
    pub pattern_type: String,
    pub original_construct: String,
    pub modernization_hint: Option<String>,
    pub preserve_exactly: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: String,
    pub start_line: u32,
    pub end_line: u32,
    pub start_column: u32,
    pub end_column: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    JavaScript,
    TypeScript,
    Python,
    Rust,
    Go,
    Java,
    CSharp,
    FSharp,
    VisualBasic,
    Cobol,
    Fortran,
    C,
    Cpp,
    // SoftEtherVPN is primarily C, so this is crucial
}

impl UIRNode {
    pub fn new(id: String, node_type: NodeType) -> Self {
        Self {
            id,
            node_type,
            name: None,
            children: Vec::new(),
            metadata: Metadata::default(),
            source_location: None,
        }
    }
    
    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }
    
    pub fn add_child(mut self, child: UIRNode) -> Self {
        self.children.push(child);
        self
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            source_language: Language::JavaScript, // Default
            semantic_tags: Vec::new(),
            complexity_score: None,
            dependencies: Vec::new(),
            annotations: HashMap::new(),
            legacy_patterns: Vec::new(),
        }
    }
}
