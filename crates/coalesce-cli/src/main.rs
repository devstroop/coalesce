use clap::{Arg, Command};
use coalesce_core::{UIRNode, NodeType, Language, Metadata, Parser, Generator};
use coalesce_parser::{JavaScriptParser, CParser, CppParser, CSharpParser, FSharpParser, VisualBasicParser, RustParser, GoParser, detect_language, create_parser};
use coalesce_gen::{PythonGenerator, RustGenerator, CGenerator, GoGenerator};
use coalesce_lal::LibraryAbstractionLayer;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let matches = Command::new("coalesce")
        .version("0.1.0")
        .about("Universal code translation platform")
        .subcommand(
            Command::new("demo")
                .about("Run a demo translation")
                .arg(
                    Arg::new("input")
                        .help("Input code snippet")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("from")
                        .long("from")
                        .help("Source language (javascript, c, cpp, csharp, fsharp, vb, rust, go)")
                        .default_value("javascript")
                )
                .arg(
                    Arg::new("to")
                        .long("to")
                        .help("Target language (python, rust, c, go)")
                        .default_value("python")
                )
        )
        .subcommand(
            Command::new("analyze-libs")
                .about("Analyze library dependencies in code")
                .arg(
                    Arg::new("input")
                        .help("Input code snippet or file path")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("language")
                        .long("language")
                        .help("Source language")
                        .default_value("javascript")
                )
        )
        .subcommand(
            Command::new("init")
                .about("Initialize a new Coalesce project")
                .arg(
                    Arg::new("directory")
                        .help("Project directory")
                        .required(true)
                        .index(1)
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("demo", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").unwrap();
            let from = sub_matches.get_one::<String>("from").unwrap();
            let to = sub_matches.get_one::<String>("to").unwrap();
            
            println!("🚀 Coalesce Demo");
            println!("📝 Input: {}", input);
            println!("🔄 Translating from {} to {}", from, to);
            
            // Parse the source language
            let source_language = match from.as_str() {
                "javascript" | "js" => Language::JavaScript,
                "c" => Language::C,
                "cpp" | "c++" => Language::Cpp,
                "csharp" | "cs" | "c#" => Language::CSharp,
                "fsharp" | "fs" | "f#" => Language::FSharp,
                "vb" | "visualbasic" | "visual-basic" => Language::VisualBasic,
                "rust" | "rs" => Language::Rust,
                "go" => Language::Go,
                _ => {
                    println!("❌ Unsupported source language: {}", from);
                    return Ok(());
                }
            };
            
            // Create parser and parse the input
            let parser = create_parser(source_language.clone())?;
            let mut uir = parser.parse(input)?;
            
            // Initialize Library Abstraction Layer
            let lal = LibraryAbstractionLayer::new()?;
            
            // Analyze library dependencies
            let dependencies = lal.analyze_dependencies(input, source_language.clone())?;
            
            if !dependencies.is_empty() {
                println!("🔍 Detected library dependencies:");
                for dep in &dependencies {
                    println!("  📦 {} ({})", dep.name, dep.ecosystem);
                    for usage in &dep.usage_patterns {
                        println!("    🔧 {}: {}", usage.pattern_name, usage.semantic_intent);
                    }
                }
                println!();
            }
            
            // Enhance UIR with library metadata
            lal.enhance_uir(&mut uir, &dependencies)?;
            
            // Transform library patterns for target language
            let target_lang_enum = match to.as_str() {
                "python" | "py" => Language::Python,
                "rust" | "rs" => Language::Rust,
                "c" => Language::C,
                "go" => Language::Go,
                _ => source_language, // Fallback
            };
            
            let enhanced_uir = lal.transform_library_calls(&uir, target_lang_enum, None)?;
            
            println!("🔧 Generated UIR:");
            println!("{}", serde_json::to_string_pretty(&enhanced_uir)?);
            
            // Generate target code
            let generated_code = match to.as_str() {
                "python" | "py" => {
                    let generator = PythonGenerator;
                    generator.generate(&enhanced_uir)?
                }
                "rust" | "rs" => {
                    let generator = RustGenerator;
                    generator.generate(&enhanced_uir)?
                }
                "c" => {
                    let generator = CGenerator;
                    generator.generate(&enhanced_uir)?
                }
                "go" => {
                    let generator = GoGenerator;
                    generator.generate(&enhanced_uir)?
                }
                _ => format!("# Target language '{}' not yet supported\n", to)
            };
            
            println!("\n🎯 Generated {} code:", to);
            println!("{}", generated_code);
            
            println!("✅ Demo complete! This is just the beginning...");
        }
        Some(("analyze-libs", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").unwrap();
            let language_str = sub_matches.get_one::<String>("language").unwrap();
            
            // Parse the source language
            let source_language = match language_str.as_str() {
                "javascript" | "js" => Language::JavaScript,
                "c" => Language::C,
                "cpp" | "c++" => Language::Cpp,
                "csharp" | "cs" | "c#" => Language::CSharp,
                "python" | "py" => Language::Python,
                _ => {
                    println!("❌ Unsupported language: {}", language_str);
                    return Ok(());
                }
            };
            
            println!("🔍 Analyzing library dependencies in {} code...", language_str);
            
            // Read input (could be file or direct code)
            let code = if std::path::Path::new(input).exists() {
                println!("📂 Reading from file: {}", input);
                fs::read_to_string(input)?
            } else {
                input.clone()
            };
            
            // Initialize LAL and analyze
            let lal = LibraryAbstractionLayer::new()?;
            let dependencies = lal.analyze_dependencies(&code, source_language)?;
            
            if dependencies.is_empty() {
                println!("✅ No library dependencies detected.");
            } else {
                println!("📦 Detected {} library dependencies:", dependencies.len());
                for dep in &dependencies {
                    println!("\n🔧 Library: {} ({})", dep.name, dep.ecosystem);
                    if !dep.usage_patterns.is_empty() {
                        println!("   Patterns found:");
                        for usage in &dep.usage_patterns {
                            println!("     • {}: {} ({})", 
                                usage.pattern_name, 
                                usage.semantic_intent,
                                usage.method_name
                            );
                            if !usage.parameters.is_empty() {
                                println!("       Parameters: {:?}", usage.parameters);
                            }
                        }
                    }
                    
                    // Show available target ecosystems
                    let targets = lal.get_target_ecosystems(&dep.name);
                    if !targets.is_empty() {
                        println!("   🎯 Can translate to: {}", targets.join(", "));
                    }
                }
            }
        }
        Some(("init", sub_matches)) => {
            let directory = sub_matches.get_one::<String>("directory").unwrap();
            
            println!("🔨 Initializing Coalesce project in: {}", directory);
            
            // Create project structure
            fs::create_dir_all(format!("{}/src", directory))?;
            fs::create_dir_all(format!("{}/.coalesce", directory))?;
            
            let config = r#"{
  "version": "0.1.0",
  "project_name": "my-coalesce-project",
  "source_languages": ["javascript"],
  "target_languages": ["python", "rust"],
  "preserve_legacy_patterns": true,
  "ml_enhancement": true
}"#;
            
            fs::write(format!("{}/.coalesce/config.json", directory), config)?;
            
            println!("✅ Project initialized!");
            println!("📁 Created: {}/src", directory);
            println!("⚙️  Created: {}/.coalesce/config.json", directory);
            println!("\n🚀 Next steps:");
            println!("   cd {}", directory);
            println!("   coalesce analyze ./src");
        }
        _ => {
            println!("🌟 Welcome to Coalesce!");
            println!("💡 Try: coalesce demo \"function add(a, b) {{ return a + b; }}\" --to rust");
            println!("🔧 Or:  coalesce demo \"int add(int a, int b) {{ return a + b; }}\" --from c --to go");
            println!("🔧 Or:  coalesce demo \"public int Add(int a, int b) {{ return a + b; }}\" --from csharp --to python");
            println!("🔧 Or:  coalesce demo \"let add x y = x + y\" --from fsharp --to rust");
            println!("🔧 Or:  coalesce demo \"Function Add(a As Integer, b As Integer) As Integer\" --from vb --to go");
            println!("🚀 Or:  coalesce demo \"func add(a, b int) int {{ return a + b }}\" --from go --to python");
            println!("� Or:  coalesce analyze-libs \"import React, {{ useState }} from 'react'\" --language javascript");
            println!("�📦 Or:  coalesce init ./my-project");
            println!("\n🔧 Supported languages:");
            println!("   📥 Source: javascript, c, cpp, csharp, fsharp, vb, rust, go");
            println!("   📤 Target: python, rust, c, go");
        }
    }

    Ok(())
}
