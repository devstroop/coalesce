use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a library pattern that can be detected and transformed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryPattern {
    pub name: String,
    pub library: String,
    pub ecosystem: String,
    pub signature: String,
    pub semantics: PatternSemantics,
    pub parameters: Vec<PatternParameter>,
    pub transformations: HashMap<String, TransformRule>,
}

/// Semantic meaning of a library pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternSemantics {
    pub intent: String,
    pub category: String,
    pub behavior: String,
    pub side_effects: Vec<String>,
    pub requirements: Vec<String>,
    pub mutability: bool,
    pub reactivity: bool,
}

/// Parameter definition for a pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternParameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub description: String,
}

/// Rule for transforming a pattern to another ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformRule {
    pub target_library: String,
    pub target_pattern: String,
    pub template: String,
    pub imports: Vec<String>,
    pub setup_code: Option<String>,
    pub cleanup_code: Option<String>,
    pub parameter_mappings: HashMap<String, String>,
}

/// Built-in library patterns
pub struct PatternLibrary;

impl PatternLibrary {
    /// Get React patterns
    pub fn react_patterns() -> Vec<LibraryPattern> {
        vec![
            LibraryPattern {
                name: "useState".to_string(),
                library: "react".to_string(),
                ecosystem: "javascript".to_string(),
                signature: "const [state, setState] = useState(initialValue)".to_string(),
                semantics: PatternSemantics {
                    intent: "reactive_state_management".to_string(),
                    category: "state".to_string(),
                    behavior: "Creates reactive state that triggers re-renders".to_string(),
                    side_effects: vec!["component_rerender".to_string()],
                    requirements: vec!["react_component_context".to_string()],
                    mutability: true,
                    reactivity: true,
                },
                parameters: vec![
                    PatternParameter {
                        name: "initialValue".to_string(),
                        param_type: "any".to_string(),
                        required: true,
                        default_value: Some("undefined".to_string()),
                        description: "Initial state value".to_string(),
                    },
                ],
                transformations: HashMap::from([
                    ("vue".to_string(), TransformRule {
                        target_library: "vue".to_string(),
                        target_pattern: "ref".to_string(),
                        template: "const {{state}} = ref({{initialValue}})".to_string(),
                        imports: vec!["import { ref } from 'vue'".to_string()],
                        setup_code: None,
                        cleanup_code: None,
                        parameter_mappings: HashMap::from([
                            ("setState".to_string(), "{{state}}.value = ".to_string()),
                        ]),
                    }),
                    ("svelte".to_string(), TransformRule {
                        target_library: "svelte".to_string(),
                        target_pattern: "writable".to_string(),
                        template: "const {{state}} = writable({{initialValue}})".to_string(),
                        imports: vec!["import { writable } from 'svelte/store'".to_string()],
                        setup_code: None,
                        cleanup_code: None,
                        parameter_mappings: HashMap::new(),
                    }),
                ]),
            },
            LibraryPattern {
                name: "useEffect".to_string(),
                library: "react".to_string(),
                ecosystem: "javascript".to_string(),
                signature: "useEffect(callback, dependencies)".to_string(),
                semantics: PatternSemantics {
                    intent: "side_effect_lifecycle".to_string(),
                    category: "lifecycle".to_string(),
                    behavior: "Executes side effects after render".to_string(),
                    side_effects: vec!["dom_mutation", "api_calls", "subscriptions"].into_iter().map(String::from).collect(),
                    requirements: vec!["react_component_context".to_string()],
                    mutability: false,
                    reactivity: true,
                },
                parameters: vec![
                    PatternParameter {
                        name: "callback".to_string(),
                        param_type: "function".to_string(),
                        required: true,
                        default_value: None,
                        description: "Effect callback function".to_string(),
                    },
                    PatternParameter {
                        name: "dependencies".to_string(),
                        param_type: "array".to_string(),
                        required: false,
                        default_value: Some("[]".to_string()),
                        description: "Dependency array".to_string(),
                    },
                ],
                transformations: HashMap::from([
                    ("vue".to_string(), TransformRule {
                        target_library: "vue".to_string(),
                        target_pattern: "watchEffect".to_string(),
                        template: "watchEffect(() => { {{callback}} })".to_string(),
                        imports: vec!["import { watchEffect } from 'vue'".to_string()],
                        setup_code: None,
                        cleanup_code: None,
                        parameter_mappings: HashMap::new(),
                    }),
                ]),
            },
        ]
    }
    
    /// Get Django patterns
    pub fn django_patterns() -> Vec<LibraryPattern> {
        vec![
            LibraryPattern {
                name: "Model".to_string(),
                library: "django".to_string(),
                ecosystem: "python".to_string(),
                signature: "class MyModel(models.Model)".to_string(),
                semantics: PatternSemantics {
                    intent: "orm_model".to_string(),
                    category: "database".to_string(),
                    behavior: "Defines a database table structure".to_string(),
                    side_effects: vec!["database_table_creation".to_string()],
                    requirements: vec!["django_orm".to_string()],
                    mutability: true,
                    reactivity: false,
                },
                parameters: vec![],
                transformations: HashMap::from([
                    ("sqlalchemy".to_string(), TransformRule {
                        target_library: "sqlalchemy".to_string(),
                        target_pattern: "declarative_base".to_string(),
                        template: "class {{name}}(Base):\n    __tablename__ = '{{table_name}}'".to_string(),
                        imports: vec![
                            "from sqlalchemy.ext.declarative import declarative_base".to_string(),
                            "Base = declarative_base()".to_string(),
                        ],
                        setup_code: None,
                        cleanup_code: None,
                        parameter_mappings: HashMap::new(),
                    }),
                ]),
            },
            LibraryPattern {
                name: "CharField".to_string(),
                library: "django".to_string(),
                ecosystem: "python".to_string(),
                signature: "field = models.CharField(max_length=100)".to_string(),
                semantics: PatternSemantics {
                    intent: "text_field".to_string(),
                    category: "database_field".to_string(),
                    behavior: "Defines a text field in database".to_string(),
                    side_effects: vec!["database_column_creation".to_string()],
                    requirements: vec!["django_model".to_string()],
                    mutability: true,
                    reactivity: false,
                },
                parameters: vec![
                    PatternParameter {
                        name: "max_length".to_string(),
                        param_type: "integer".to_string(),
                        required: true,
                        default_value: None,
                        description: "Maximum character length".to_string(),
                    },
                ],
                transformations: HashMap::from([
                    ("sqlalchemy".to_string(), TransformRule {
                        target_library: "sqlalchemy".to_string(),
                        target_pattern: "String".to_string(),
                        template: "{{field_name}} = Column(String({{max_length}}))".to_string(),
                        imports: vec!["from sqlalchemy import Column, String".to_string()],
                        setup_code: None,
                        cleanup_code: None,
                        parameter_mappings: HashMap::new(),
                    }),
                ]),
            },
        ]
    }
    
    /// Get networking patterns (cross-platform)
    pub fn networking_patterns() -> Vec<LibraryPattern> {
        vec![
            LibraryPattern {
                name: "tcp_socket".to_string(),
                library: "socket".to_string(),
                ecosystem: "c".to_string(),
                signature: "int sock = socket(AF_INET, SOCK_STREAM, 0)".to_string(),
                semantics: PatternSemantics {
                    intent: "tcp_socket_creation".to_string(),
                    category: "networking".to_string(),
                    behavior: "Creates a TCP socket for network communication".to_string(),
                    side_effects: vec!["system_resource_allocation".to_string()],
                    requirements: vec!["socket_library".to_string()],
                    mutability: false,
                    reactivity: false,
                },
                parameters: vec![],
                transformations: HashMap::from([
                    ("rust".to_string(), TransformRule {
                        target_library: "std".to_string(),
                        target_pattern: "TcpStream".to_string(),
                        template: "let stream = TcpStream::connect(\"{{address}}:{{port}}\")".to_string(),
                        imports: vec!["use std::net::TcpStream".to_string()],
                        setup_code: None,
                        cleanup_code: None,
                        parameter_mappings: HashMap::new(),
                    }),
                    ("go".to_string(), TransformRule {
                        target_library: "net".to_string(),
                        target_pattern: "Dial".to_string(),
                        template: "conn, err := net.Dial(\"tcp\", \"{{address}}:{{port}}\")".to_string(),
                        imports: vec!["import \"net\"".to_string()],
                        setup_code: None,
                        cleanup_code: None,
                        parameter_mappings: HashMap::new(),
                    }),
                    ("python".to_string(), TransformRule {
                        target_library: "socket".to_string(),
                        target_pattern: "socket".to_string(),
                        template: "sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)".to_string(),
                        imports: vec!["import socket".to_string()],
                        setup_code: None,
                        cleanup_code: None,
                        parameter_mappings: HashMap::new(),
                    }),
                ]),
            },
        ]
    }
}
