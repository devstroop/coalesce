use coalesce_core::{types::*, errors::*, traits::Parser};

mod javascript;
mod c;
mod cpp;
mod csharp;
mod fsharp;
mod vb;
mod rust_parser;
mod go;

pub use javascript::JavaScriptParser;
pub use c::CParser;
pub use cpp::CppParser;
pub use csharp::CSharpParser;
pub use fsharp::FSharpParser;
pub use vb::VisualBasicParser;
pub use rust_parser::RustParser;
pub use go::GoParser;

// Language detection
pub fn detect_language(source: &str, filename: Option<&str>) -> Language {
    if let Some(name) = filename {
        if name.ends_with(".js") || name.ends_with(".mjs") || name.ends_with(".jsx") {
            return Language::JavaScript;
        }
        if name.ends_with(".c") || name.ends_with(".h") {
            return Language::C;
        }
        if name.ends_with(".cpp") || name.ends_with(".cxx") || name.ends_with(".cc") || name.ends_with(".hpp") {
            return Language::Cpp;
        }
        if name.ends_with(".rs") {
            return Language::Rust;
        }
        if name.ends_with(".go") {
            return Language::Go;
        }
        if name.ends_with(".cs") {
            return Language::CSharp;
        }
        if name.ends_with(".fs") || name.ends_with(".fsx") {
            return Language::FSharp;
        }
        if name.ends_with(".vb") || name.ends_with(".bas") {
            return Language::VisualBasic;
        }
        if name.ends_with(".py") {
            return Language::Python;
        }
    }
    
    // Fallback to content-based detection (prioritize system languages)
    if source.contains("using System") || source.contains("namespace ") && source.contains("class ") && source.contains("public ") {
        Language::CSharp
    } else if source.contains("let ") && (source.contains("=") || source.contains("->")) && (source.contains("module ") || source.contains("type ")) {
        Language::FSharp
    } else if source.contains("Sub ") || source.contains("Function ") || source.contains("End Sub") || source.contains("End Function") {
        Language::VisualBasic
    } else if source.contains("fn ") && (source.contains("mut ") || source.contains("impl ") || source.contains("struct ")) {
        Language::Rust
    } else if source.contains("func ") && (source.contains("package ") || source.contains("import ")) {
        Language::Go
    } else if source.contains("class ") && (source.contains("public:") || source.contains("private:") || source.contains("namespace ")) {
        Language::Cpp
    } else if source.contains("#include") || source.contains("int main") {
        Language::C
    } else if source.contains("function ") || source.contains("const ") || source.contains("let ") {
        Language::JavaScript
    } else if source.contains("def ") || source.contains("import ") {
        Language::Python
    } else {
        Language::JavaScript // Default fallback
    }
}

// Factory function for creating parsers
pub fn create_parser(language: Language) -> Result<Box<dyn Parser>> {
    match language {
        Language::JavaScript => Ok(Box::new(JavaScriptParser::new()?)),
        Language::C => Ok(Box::new(CParser::new()?)),
        Language::Cpp => Ok(Box::new(CppParser::new()?)),
        Language::CSharp => Ok(Box::new(CSharpParser::new()?)),
        Language::FSharp => Ok(Box::new(FSharpParser::new()?)),
        Language::VisualBasic => Ok(Box::new(VisualBasicParser::new()?)),
        Language::Rust => Ok(Box::new(RustParser::new()?)),
        Language::Go => Ok(Box::new(GoParser::new()?)),
        Language::Python => Err(CoalesceError::ParseError {
            message: "Python parser not yet implemented".to_string(),
            line: 0,
            column: 0,
        }),
        Language::Cobol => Err(CoalesceError::ParseError {
            message: "COBOL parser not yet implemented".to_string(),
            line: 0,
            column: 0,
        }),
        _ => Err(CoalesceError::ParseError {
            message: "Unsupported language".to_string(),
            line: 0,
            column: 0,
        }),
    }
}

// Legacy stub functions for backward compatibility
pub fn parse_javascript(source: &str) -> Result<UIRNode> {
    let parser = JavaScriptParser::new()?;
    parser.parse(source)
}

pub fn parse_c(source: &str) -> Result<UIRNode> {
    let parser = CParser::new()?;
    parser.parse(source)
}

pub fn parse_cpp(source: &str) -> Result<UIRNode> {
    let parser = CppParser::new()?;
    parser.parse(source)
}

pub fn parse_rust(source: &str) -> Result<UIRNode> {
    let parser = RustParser::new()?;
    parser.parse(source)
}

pub fn parse_go(source: &str) -> Result<UIRNode> {
    let parser = GoParser::new()?;
    parser.parse(source)
}

pub fn parse_csharp(source: &str) -> Result<UIRNode> {
    let parser = CSharpParser::new()?;
    parser.parse(source)
}

pub fn parse_fsharp(source: &str) -> Result<UIRNode> {
    let parser = FSharpParser::new()?;
    parser.parse(source)
}

pub fn parse_vb(source: &str) -> Result<UIRNode> {
    let parser = VisualBasicParser::new()?;
    parser.parse(source)
}

pub fn parse_python(source: &str) -> Result<UIRNode> {
    // Legacy stub - will be replaced with real parser
    if source.contains("def ") {
        Ok(UIRNode {
            id: "python_func".to_string(),
            node_type: NodeType::Function,
            name: Some("extracted_function".to_string()),
            children: vec![],
            metadata: Metadata::default(),
            source_location: None,
        })
    } else {
        Err(CoalesceError::ParseError {
            message: "No Python functions found".to_string(),
            line: 0,
            column: 0,
        })
    }
}
