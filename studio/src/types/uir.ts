// Universal Intermediate Representation types
export interface UIRNode {
  id: string;
  type: UIRNodeType;
  name: string;
  value?: any;
  metadata: Record<string, any>;
  children: UIRNode[];
  position?: { x: number; y: number };
  library_dependencies?: LibraryDependency[];
}

export enum UIRNodeType {
  Module = "module",
  Function = "function",
  Class = "class",
  Variable = "variable",
  Expression = "expression",
  Statement = "statement",
  Import = "import",
  Export = "export",
  Type = "type",
  Interface = "interface",
  Enum = "enum",
  Literal = "literal",
  Operator = "operator",
  Call = "call",
  Block = "block",
  Loop = "loop",
  Conditional = "conditional",
  Return = "return",
  Throw = "throw",
  Try = "try",
  Catch = "catch",
  Finally = "finally",
}

export interface LibraryDependency {
  library: string;
  pattern: string;
  version?: string;
  import_path?: string;
  usage_type: string;
  parameters?: Record<string, any>;
}

export interface GraphNode {
  data: {
    id: string;
    label: string;
    type: UIRNodeType;
    metadata: Record<string, any>;
  };
  position: { x: number; y: number };
  selected?: boolean;
}

export interface GraphEdge {
  data: {
    id: string;
    source: string;
    target: string;
    label?: string;
    type: 'dependency' | 'call' | 'inheritance' | 'composition';
  };
}

export interface TranslationResult {
  code: string;
  confidence: number;
  warnings: string[];
  metadata: Record<string, any>;
}

export interface AnalysisResult {
  uir: UIRNode;
  dependencies: LibraryDependency[];
  complexity: number;
  suggestions: string[];
}
