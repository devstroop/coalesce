use clap::{Arg, Command};
use coalesce_core::{UIRNode, NodeType, Language, Metadata, Parser, Generator};
use coalesce_parser::{Java        _ => {
            println!("🌟 Welcome to Coalesce!");
            println!("💡 Try: coalesce demo \"function add(a, b) {{ return a + b; }}\" --to rust");
            println!("🔧 Or:  coalesce demo \"int add(int a, int b) {{ return a + b; }}\" --from c --to go");
            println!("🔧 Or:  coalesce demo \"public int Add(int a, int b) {{ return a + b; }}\" --from csharp --to python");
            println!("🔧 Or:  coalesce demo \"let add x y = x + y\" --from fsharp --to rust");
            println!("🔧 Or:  coalesce demo \"Function Add(a As Integer, b As Integer) As Integer\" --from vb --to go");
            println!("🚀 Or:  coalesce demo \"func add(a, b int) int {{ return a + b }}\" --from go --to python");
            println!("📦 Or:  coalesce init ./my-project");
            println!("\n🔧 Supported languages:");
            println!("   📥 Source: javascript, c, cpp, csharp, fsharp, vb, rust, go");
            println!("   📤 Target: python, rust, c, go");
        }r, CParser, CppParser, CSharpParser, FSharpParser, VisualBasicParser, RustParser, GoParser, detect_language, create_parser};
use coalesce_gen::{PythonGenerator, RustGenerator, CGenerator, GoGenerator};
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
            let parser = create_parser(source_language)?;
            let uir = parser.parse(input)?;
            
            println!("🔧 Generated UIR:");
            println!("{}", serde_json::to_string_pretty(&uir)?);
            
            // Generate target code
            let generated_code = match to.as_str() {
                "python" | "py" => {
                    let generator = PythonGenerator;
                    generator.generate(&uir)?
                }
                "rust" | "rs" => {
                    let generator = RustGenerator;
                    generator.generate(&uir)?
                }
                "c" => {
                    let generator = CGenerator;
                    generator.generate(&uir)?
                }
                "go" => {
                    let generator = GoGenerator;
                    generator.generate(&uir)?
                }
                _ => format!("# Target language '{}' not yet supported\n", to)
            };
            
            println!("\n🎯 Generated {} code:", to);
            println!("{}", generated_code);
            
            println!("✅ Demo complete! This is just the beginning...");
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
            println!("� Or:  coalesce demo \"public int Add(int a, int b) {{ return a + b; }}\" --from csharp --to python");
            println!("�🚀 Or:  coalesce demo \"func add(a, b int) int {{ return a + b }}\" --from go --to python");
            println!("📦 Or:  coalesce init ./my-project");
            println!("\n🔧 Supported languages:");
            println!("   📥 Source: javascript, c, cpp, csharp, fsharp, vb, rust, go");
            println!("   📤 Target: python, rust, c, go");
        }
    }

    Ok(())
}
