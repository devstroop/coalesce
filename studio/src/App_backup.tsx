import { createSignal, Show, createMemo, onMount } from 'solid-js';
import { MonacoEditor } from './components/MonacoEditor';
import { KnowledgeGraph } from './components/KnowledgeGraph';
import { CodeAnalyzer } from './utils/CodeAnalyzer';

interface UIRNode {
  id: string;
  node_type: string;
  name?: string;
  children?: UIRNode[];
  metadata?: any;
}

function App() {
  const [sourceCode, setSourceCode] = createSignal(`// Welcome to Coalesce Studio
// Try editing this JavaScript code
class Calculator {
  constructor() {
    this.history = [];
  }
  
  add(a, b) {
    const result = a + b;
    this.history.push(\`\${a} + \${b} = \${result}\`);
    return result;
  }
  
  multiply(a, b) {
    const result = a * b;
    this.history.push(\`\${a} * \${b} = \${result}\`);
    return result;
  }
  
  getHistory() {
    return this.history;
  }
}

const calc = new Calculator();
calc.add(5, 3);
calc.multiply(4, 2);
console.log(calc.getHistory());`);

  const [targetLanguage, setTargetLanguage] = createSignal('python');
  const [isLoading, setIsLoading] = createSignal(false);
  const [translatedCode, setTranslatedCode] = createSignal('');
  const [selectedNode, setSelectedNode] = createSignal<string>();
  const [isFullScreen, setIsFullScreen] = createSignal(false);
  const [realUIR, setRealUIR] = createSignal<UIRNode | null>(null);

  const analyzer = new CodeAnalyzer();

  // Convert UIR format to our graph format
  const convertUIRToAnalysisFormat = (uir: UIRNode) => {
    const nodes: any[] = [];
    const links: any[] = [];
    const metrics = {
      functions: 0,
      classes: 0,
      variables: 0,
      complexity: 0,
      lines: sourceCode().split('\n').length
    };

    const processNode = (node: UIRNode, parentId: string | null = null) => {
      const nodeData = {
        id: node.id,
        name: node.name || node.id,
        type: mapUIRNodeType(node.node_type),
        x: Math.random() * 400,
        y: Math.random() * 400
      };

      nodes.push(nodeData);

      // Update metrics
      if (node.node_type === 'Function') metrics.functions++;
      if (node.node_type === 'Class') metrics.classes++;
      if (node.node_type === 'Variable') metrics.variables++;

      // Add parent-child links
      if (parentId) {
        links.push({
          source: parentId,
          target: node.id,
          type: 'contains'
        });
      }

      // Process children
      if (node.children) {
        node.children.forEach((child: UIRNode) => processNode(child, node.id));
      }
    };

    processNode(uir);
    metrics.complexity = Math.floor(nodes.length * 1.5);

    return { nodes, links, metrics };
  };

  const mapUIRNodeType = (nodeType: string) => {
    if (typeof nodeType === 'string') return nodeType.toLowerCase();
    if (nodeType === 'Function') return 'function';
    if (nodeType === 'Class') return 'class';
    if (nodeType === 'Variable') return 'variable';
    if (nodeType === 'Module') return 'module';
    return 'unknown';
  };

  // Analyze code reactively with real Coalesce engine
  const analysisResult = createMemo(() => {
    const code = sourceCode();
    const uir = realUIR();
    if (uir) {
      // Use real UIR data from Coalesce engine
      return convertUIRToAnalysisFormat(uir);
    }
    // Fallback to local analyzer
    return analyzer.analyze(code, 'javascript');
  });

  // Call real Coalesce engine
  const parseWithCoalesceEngine = async (code: string, language: string) => {
    try {
      // Call the Rust backend via API
      const response = await fetch('/api/parse', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          source: code,
          language: language
        })
      });

      if (response.ok) {
        const uir = await response.json();
        setRealUIR(uir);
        return uir;
      } else {
        console.warn('Coalesce API not available, using local analyzer');
        return null;
      }
    } catch (error) {
      console.warn('Coalesce API error:', error);
      return null;
    }
  };

  // Parse code when it changes
  onMount(() => {
    parseWithCoalesceEngine(sourceCode(), 'javascript');
  });

  // Re-parse when code changes
  createMemo(() => {
    const code = sourceCode();
    if (code.trim()) {
      parseWithCoalesceEngine(code, 'javascript');
    }
  });

  const handleTranslate = async () => {
    setIsLoading(true);
    
    try {
      // Try real Coalesce translation first
      const response = await fetch('/api/translate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          source: sourceCode(),
          from: 'javascript',
          to: targetLanguage()
        })
      });

      if (response.ok) {
        const result = await response.json();
        setTranslatedCode(result.translated_code);
      } else {
        // Fallback to simulation
        simulateTranslation();
      }
    } catch (error) {
      console.warn('Translation API error:', error);
      simulateTranslation();
    }
    
    setIsLoading(false);
  };

  const simulateTranslation = () => {
    setTimeout(() => {
      const target = targetLanguage();
      let result = '';
      
      if (target === 'python') {
        result = `# Translated to Python
class Calculator:
    def __init__(self):
        self.history = []
    
    def add(self, a, b):
        result = a + b
        self.history.append(f"{a} + {b} = {result}")
        return result
    
    def multiply(self, a, b):
        result = a * b
        self.history.append(f"{a} * {b} = {result}")
        return result
    
    def get_history(self):
        return self.history

calc = Calculator()
calc.add(5, 3)
calc.multiply(4, 2)
print(calc.get_history())`;
      } else if (target === 'rust') {
        result = `// Translated to Rust
struct Calculator {
    history: Vec<String>,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }
    
    fn add(&mut self, a: i32, b: i32) -> i32 {
        let result = a + b;
        self.history.push(format!("{} + {} = {}", a, b, result));
        result
    }
    
    fn multiply(&mut self, a: i32, b: i32) -> i32 {
        let result = a * b;
        self.history.push(format!("{} * {} = {}", a, b, result));
        result
    }
    
    fn get_history(&self) -> &Vec<String> {
        &self.history
    }
}

fn main() {
    let mut calc = Calculator::new();
    calc.add(5, 3);
    calc.multiply(4, 2);
    println!("{:?}", calc.get_history());
}`;
      }
      
      setTranslatedCode(result);
    }, 1000);
  };

  return (
    <div style="height: 100vh; background: #0a0a0b; color: #e4e4e7; display: flex; flex-direction: column; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;">
      {/* Header */}
      <header style="background: linear-gradient(90deg, #1a1a1b 0%, #2d2d30 100%); border-bottom: 1px solid #333; padding: 12px 20px; box-shadow: 0 2px 10px rgba(0,0,0,0.3); z-index: 10;">
        <div style="display: flex; align-items: center; justify-content: space-between;">
          <div style="display: flex; align-items: center; gap: 12px;">
            <div style="font-size: 28px;">🧠</div>
            <div>
              <h1 style="font-size: 22px; font-weight: 700; margin: 0; background: linear-gradient(90deg, #60a5fa, #a78bfa); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">Coalesce Studio</h1>
              <p style="color: #a1a1aa; margin: 2px 0 0 0; font-size: 13px;">Visual Code Translation Platform</p>
            </div>
          </div>
          
          <div style="display: flex; align-items: center; gap: 16px;">
            <button 
              style="background: #374151; border: 1px solid #52525b; padding: 8px 12px; border-radius: 6px; color: #e4e4e7; cursor: pointer; font-size: 12px; transition: all 0.2s;"
              onClick={() => setIsFullScreen(!isFullScreen())}
            >
              {isFullScreen() ? '📱 Split View' : '🖥️ Full Graph'}
            </button>
            
            <div style="display: flex; align-items: center; gap: 8px;">
              <label style="font-size: 14px; color: #a1a1aa;">Target:</label>
              <select 
                style="background: #27272a; border: 1px solid #52525b; border-radius: 6px; padding: 8px 12px; color: #e4e4e7; font-size: 14px; min-width: 120px;"
                value={targetLanguage()}
                onChange={(e) => setTargetLanguage(e.target.value)}
              >
                <option value="python">Python</option>
                <option value="typescript">TypeScript</option>
                <option value="rust">Rust</option>
                <option value="go">Go</option>
                <option value="java">Java</option>
                <option value="csharp">C#</option>
              </select>
            </div>
            
            <button 
              style={`background: linear-gradient(90deg, #2563eb, #7c3aed); border: none; padding: 10px 20px; border-radius: 6px; color: white; cursor: pointer; font-weight: 600; font-size: 14px; transition: all 0.2s; ${isLoading() ? 'opacity: 0.7; cursor: not-allowed;' : 'hover:shadow-lg;'}`}
              onClick={handleTranslate}
              disabled={isLoading()}
            >
              <Show when={isLoading()} fallback="🚀 Translate">
                ⏳ Translating...
              </Show>
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <div style="flex: 1; display: flex; min-height: 0;">
        <Show 
          when={isFullScreen()}
          fallback={
            <>
              {/* Source Code Panel */}
              <div style="width: 33.33%; border-right: 1px solid #333; display: flex; flex-direction: column; background: #111113;">
                <div style="background: #1a1a1b; padding: 12px 16px; border-bottom: 1px solid #333; display: flex; align-items: center; gap: 8px;">
                  <div style="font-size: 16px;">📝</div>
                  <h3 style="font-size: 14px; font-weight: 600; margin: 0; color: #e4e4e7;">Source Code</h3>
                  <div style="background: #059669; color: white; padding: 2px 6px; border-radius: 4px; font-size: 11px; font-weight: 600;">JAVASCRIPT</div>
                  <Show when={realUIR()}>
                    <div style="background: #7c3aed; color: white; padding: 2px 6px; border-radius: 4px; font-size: 11px; font-weight: 600;">LIVE ENGINE</div>
                  </Show>
                </div>
                <div style="flex: 1; padding: 8px;">
                  <MonacoEditor
                    value={sourceCode()}
                    onChange={setSourceCode}
                    language="javascript"
                    height="100%"
                  />
                </div>
              </div>

              {/* Knowledge Graph Panel */}
              <div style="width: 33.33%; border-right: 1px solid #333; display: flex; flex-direction: column; background: #111113;">
                <div style="background: #1a1a1b; padding: 12px 16px; border-bottom: 1px solid #333; display: flex; align-items: center; gap: 8px;">
                  <div style="font-size: 16px;">🕸️</div>
                  <h3 style="font-size: 14px; font-weight: 600; margin: 0; color: #e4e4e7;">Knowledge Graph</h3>
                  <div style="background: #7c3aed; color: white; padding: 2px 6px; border-radius: 4px; font-size: 11px; font-weight: 600;">LIVE</div>
                </div>
                <div style="flex: 1; padding: 8px;">
                  <Show 
                    when={analysisResult().nodes.length > 0}
                    fallback={
                      <div style="height: 100%; display: flex; align-items: center; justify-content: center; text-align: center; color: #71717a;">
                        <div>
                          <div style="font-size: 48px; margin-bottom: 16px; opacity: 0.7;">🧠</div>
                          <div style="font-size: 16px; margin-bottom: 8px; font-weight: 600;">Interactive Knowledge Graph</div>
                          <div style="font-size: 14px; line-height: 1.5;">Add more code to see the<br />visual structure and dependencies</div>
                        </div>
                      </div>
                    }
                  >
                    <KnowledgeGraph
                      data={analysisResult()}
                      selectedNode={selectedNode()}
                      onNodeSelect={setSelectedNode}
                      width={400}
                      height={400}
                    />
                  </Show>
                </div>
              </div>

              {/* Translation Panel */}
              <div style="width: 33.33%; display: flex; flex-direction: column; background: #111113;">
                <div style="background: #1a1a1b; padding: 12px 16px; border-bottom: 1px solid #333; display: flex; align-items: center; gap: 8px;">
                  <div style="font-size: 16px;">🎯</div>
                  <h3 style="font-size: 14px; font-weight: 600; margin: 0; color: #e4e4e7;">Translation</h3>
                  <div style="background: #dc2626; color: white; padding: 2px 6px; border-radius: 4px; font-size: 11px; font-weight: 600;">{targetLanguage().toUpperCase()}</div>
                </div>
                <div style="flex: 1; padding: 8px;">
                  <Show 
                    when={translatedCode()}
                    fallback={
                      <div style="height: 100%; display: flex; align-items: center; justify-content: center; text-align: center; color: #71717a;">
                        <div>
                          <div style="font-size: 32px; margin-bottom: 16px;">⚡</div>
                          <div style="font-size: 14px; margin-bottom: 8px;">Ready to translate</div>
                          <div style="font-size: 12px;">Choose a target language and click Translate</div>
                        </div>
                      </div>
                    }
                  >
                    <MonacoEditor
                      value={translatedCode()}
                      onChange={() => {}} // Read-only
                      language={targetLanguage() === 'csharp' ? 'csharp' : targetLanguage()}
                      height="100%"
                      readOnly={true}
                    />
                  </Show>
                </div>
              </div>
            </>
          }
        >
          {/* Full Screen Knowledge Graph */}
          <div style="width: 100%; display: flex; flex-direction: column; background: #111113; position: relative;">
            <div style="background: #1a1a1b; padding: 16px 20px; border-bottom: 1px solid #333; display: flex; align-items: center; justify-content: space-between;">
              <div style="display: flex; align-items: center; gap: 12px;">
                <div style="font-size: 24px;">🕸️</div>
                <div>
                  <h2 style="font-size: 18px; font-weight: 700; margin: 0; color: #e4e4e7;">Full Screen Knowledge Graph</h2>
                  <p style="color: #a1a1aa; margin: 2px 0 0 0; font-size: 13px;">Interactive visualization of code structure and dependencies</p>
                </div>
              </div>
              
              <div style="display: flex; align-items: center; gap: 16px;">
                <div style="display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: #27272a; border-radius: 6px;">
                  <div style="width: 8px; height: 8px; background: #10b981; border-radius: 50%;"></div>
                  <span style="font-size: 12px; color: #a1a1aa;">
                    {realUIR() ? 'Coalesce Engine' : 'Local Analyzer'}
                  </span>
                </div>
                <div style="font-size: 12px; color: #71717a;">
                  Nodes: {analysisResult().nodes.length} | 
                  Functions: {analysisResult().metrics.functions} | 
                  Classes: {analysisResult().metrics.classes} |
                  Complexity: {analysisResult().metrics.complexity}
                </div>
              </div>
            </div>

            <div style="flex: 1; position: relative;">
              <Show 
                when={analysisResult().nodes.length > 0}
                fallback={
                  <div style="height: 100%; display: flex; align-items: center; justify-content: center; text-align: center; color: #71717a;">
                    <div>
                      <div style="font-size: 96px; margin-bottom: 24px; opacity: 0.7;">🧠</div>
                      <div style="font-size: 24px; margin-bottom: 16px; font-weight: 700;">Full Screen Knowledge Graph</div>
                      <div style="font-size: 16px; line-height: 1.6; max-width: 500px;">
                        Add more code to see the visual structure and dependencies.<br />
                        The graph will show functions, classes, variables, and their relationships.
                      </div>
                    </div>
                  </div>
                }
              >
                <KnowledgeGraph
                  data={analysisResult()}
                  selectedNode={selectedNode()}
                  onNodeSelect={setSelectedNode}
                  width={800}
                  height={600}
                />
              </Show>

              {/* Floating Code Editor */}
              <div style="position: absolute; top: 20px; left: 20px; width: 400px; height: 300px; background: #1a1a1b; border: 1px solid #333; border-radius: 8px; box-shadow: 0 8px 32px rgba(0,0,0,0.5); z-index: 20;">
                <div style="background: #2d2d30; padding: 8px 12px; border-bottom: 1px solid #333; border-radius: 8px 8px 0 0; display: flex; align-items: center; gap: 8px;">
                  <div style="font-size: 14px;">📝</div>
                  <span style="font-size: 12px; font-weight: 600; color: #e4e4e7;">Live Code Editor</span>
                  <div style="flex: 1;"></div>
                  <div style="background: #059669; color: white; padding: 1px 4px; border-radius: 3px; font-size: 10px; font-weight: 600;">JS</div>
                </div>
                <div style="height: 252px; padding: 4px;">
                  <MonacoEditor
                    value={sourceCode()}
                    onChange={setSourceCode}
                    language="javascript"
                    height="100%"
                  />
                </div>
              </div>

              {/* Floating Translation Panel */}
              <Show when={translatedCode()}>
                <div style="position: absolute; top: 20px; right: 20px; width: 400px; height: 300px; background: #1a1a1b; border: 1px solid #333; border-radius: 8px; box-shadow: 0 8px 32px rgba(0,0,0,0.5); z-index: 20;">
                  <div style="background: #2d2d30; padding: 8px 12px; border-bottom: 1px solid #333; border-radius: 8px 8px 0 0; display: flex; align-items: center; gap: 8px;">
                    <div style="font-size: 14px;">🎯</div>
                    <span style="font-size: 12px; font-weight: 600; color: #e4e4e7;">Translation</span>
                    <div style="flex: 1;"></div>
                    <div style="background: #dc2626; color: white; padding: 1px 4px; border-radius: 3px; font-size: 10px; font-weight: 600;">{targetLanguage().toUpperCase()}</div>
                  </div>
                  <div style="height: 252px; padding: 4px;">
                    <MonacoEditor
                      value={translatedCode()}
                      onChange={() => {}}
                      language={targetLanguage() === 'csharp' ? 'csharp' : targetLanguage()}
                      height="100%"
                      readOnly={true}
                    />
                  </div>
                </div>
              </Show>
            </div>
          </div>
        </Show>
      </div>

      {/* Status Bar */}
      <footer style="background: #0a0a0b; border-top: 1px solid #333; padding: 8px 20px; font-size: 12px; z-index: 10;">
        <div style="display: flex; align-items: center; justify-content: space-between;">
          <div style="display: flex; align-items: center; gap: 16px; color: #71717a;">
            <span style="display: flex; align-items: center; gap: 4px;">
              <div style="width: 8px; height: 8px; background: #10b981; border-radius: 50%;"></div>
              Ready
            </span>
            <span>Functions: {analysisResult().metrics.functions}</span>
            <span>Classes: {analysisResult().metrics.classes}</span>
            <span>Complexity: {analysisResult().metrics.complexity}</span>
            <Show when={realUIR()}>
              <span style="color: #7c3aed;">🔗 Coalesce Engine Connected</span>
            </Show>
          </div>
          <div style="display: flex; align-items: center; gap: 16px; color: #71717a;">
            <span>Lines: {analysisResult().metrics.lines}</span>
            <span>Nodes: {analysisResult().nodes.length}</span>
            <span>⚡ Powered by Coalesce</span>
          </div>
        </div>
      </footer>
    </div>
  );
}

export default App;

  return (
    <div style="height: 100vh; background: #0a0a0b; color: #e4e4e7; display: flex; flex-direction: column; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;">
      {/* Header */}
      <header style="background: linear-gradient(90deg, #1a1a1b 0%, #2d2d30 100%); border-bottom: 1px solid #333; padding: 12px 20px; box-shadow: 0 2px 10px rgba(0,0,0,0.3);">
        <div style="display: flex; align-items: center; justify-content: space-between;">
          <div style="display: flex; align-items: center; gap: 12px;">
            <div style="font-size: 28px;">🧠</div>
            <div>
              <h1 style="font-size: 22px; font-weight: 700; margin: 0; background: linear-gradient(90deg, #60a5fa, #a78bfa); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">Coalesce Studio</h1>
              <p style="color: #a1a1aa; margin: 2px 0 0 0; font-size: 13px;">Visual Code Translation Platform</p>
            </div>
          </div>
          
          <div style="display: flex; align-items: center; gap: 16px;">
            <div style="display: flex; align-items: center; gap: 8px;">
              <label style="font-size: 14px; color: #a1a1aa;">Target:</label>
              <select 
                style="background: #27272a; border: 1px solid #52525b; border-radius: 6px; padding: 8px 12px; color: #e4e4e7; font-size: 14px; min-width: 120px;"
                value={targetLanguage()}
                onChange={(e) => setTargetLanguage(e.target.value)}
              >
                <option value="python">Python</option>
                <option value="typescript">TypeScript</option>
                <option value="rust">Rust</option>
                <option value="go">Go</option>
                <option value="java">Java</option>
                <option value="csharp">C#</option>
              </select>
            </div>
            
            <button 
              style={`background: linear-gradient(90deg, #2563eb, #7c3aed); border: none; padding: 10px 20px; border-radius: 6px; color: white; cursor: pointer; font-weight: 600; font-size: 14px; transition: all 0.2s; ${isLoading() ? 'opacity: 0.7; cursor: not-allowed;' : 'hover:shadow-lg;'}`}
              onClick={handleTranslate}
              disabled={isLoading()}
            >
              <Show when={isLoading()} fallback="🚀 Translate">
                ⏳ Translating...
              </Show>
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <div style="flex: 1; display: flex; min-height: 0;">
        {/* Source Code Panel */}
        <div style="width: 33.33%; border-right: 1px solid #333; display: flex; flex-direction: column; background: #111113;">
          <div style="background: #1a1a1b; padding: 12px 16px; border-bottom: 1px solid #333; display: flex; align-items: center; gap: 8px;">
            <div style="font-size: 16px;">📝</div>
            <h3 style="font-size: 14px; font-weight: 600; margin: 0; color: #e4e4e7;">Source Code</h3>
            <div style="background: #059669; color: white; padding: 2px 6px; border-radius: 4px; font-size: 11px; font-weight: 600;">JAVASCRIPT</div>
          </div>
          <div style="flex: 1; padding: 8px;">
            <MonacoEditor
              value={sourceCode()}
              onChange={setSourceCode}
              language="javascript"
              height="100%"
            />
          </div>
        </div>

        {/* Knowledge Graph Panel */}
        <div style="width: 33.33%; border-right: 1px solid #333; display: flex; flex-direction: column; background: #111113;">
          <div style="background: #1a1a1b; padding: 12px 16px; border-bottom: 1px solid #333; display: flex; align-items: center; gap: 8px;">
            <div style="font-size: 16px;">🕸️</div>
            <h3 style="font-size: 14px; font-weight: 600; margin: 0; color: #e4e4e7;">Knowledge Graph</h3>
            <div style="background: #7c3aed; color: white; padding: 2px 6px; border-radius: 4px; font-size: 11px; font-weight: 600;">LIVE</div>
          </div>
          <div style="flex: 1; padding: 8px;">
            <Show 
              when={analysisResult().nodes.length > 0}
              fallback={
                <div style="height: 100%; display: flex; align-items: center; justify-content: center; text-align: center; color: #71717a;">
                  <div>
                    <div style="font-size: 48px; margin-bottom: 16px; opacity: 0.7;">🧠</div>
                    <div style="font-size: 16px; margin-bottom: 8px; font-weight: 600;">Interactive Knowledge Graph</div>
                    <div style="font-size: 14px; line-height: 1.5;">Add more code to see the<br />visual structure and dependencies</div>
                  </div>
                </div>
              }
            >
              <KnowledgeGraph
                data={analysisResult()}
                selectedNode={selectedNode()}
                onNodeSelect={setSelectedNode}
                width={400}
                height={400}
              />
            </Show>
          </div>
        </div>

        {/* Translation Panel */}
        <div style="width: 33.33%; display: flex; flex-direction: column; background: #111113;">
          <div style="background: #1a1a1b; padding: 12px 16px; border-bottom: 1px solid #333; display: flex; align-items: center; gap: 8px;">
            <div style="font-size: 16px;">🎯</div>
            <h3 style="font-size: 14px; font-weight: 600; margin: 0; color: #e4e4e7;">Translation</h3>
            <div style="background: #dc2626; color: white; padding: 2px 6px; border-radius: 4px; font-size: 11px; font-weight: 600;">{targetLanguage().toUpperCase()}</div>
          </div>
          <div style="flex: 1; padding: 8px;">
            <Show 
              when={translatedCode()}
              fallback={
                <div style="height: 100%; display: flex; align-items: center; justify-content: center; text-align: center; color: #71717a;">
                  <div>
                    <div style="font-size: 32px; margin-bottom: 16px;">⚡</div>
                    <div style="font-size: 14px; margin-bottom: 8px;">Ready to translate</div>
                    <div style="font-size: 12px;">Choose a target language and click Translate</div>
                  </div>
                </div>
              }
            >
              <MonacoEditor
                value={translatedCode()}
                onChange={() => {}} // Read-only
                language={targetLanguage() === 'csharp' ? 'csharp' : targetLanguage()}
                height="100%"
                readOnly={true}
              />
            </Show>
          </div>
        </div>
      </div>

      {/* Status Bar */}
      <footer style="background: #0a0a0b; border-top: 1px solid #333; padding: 8px 20px; font-size: 12px;">
        <div style="display: flex; align-items: center; justify-content: space-between;">
          <div style="display: flex; align-items: center; gap: 16px; color: #71717a;">
            <span style="display: flex; align-items: center; gap: 4px;">
              <div style="width: 8px; height: 8px; background: #10b981; border-radius: 50%;"></div>
              Ready
            </span>
            <span>Functions: {analysisResult().metrics.functions}</span>
            <span>Classes: {analysisResult().metrics.classes}</span>
            <span>Complexity: {analysisResult().metrics.complexity}</span>
          </div>
          <div style="display: flex; align-items: center; gap: 16px; color: #71717a;">
            <span>Lines: {analysisResult().metrics.lines}</span>
            <span>Nodes: {analysisResult().nodes.length}</span>
            <span>⚡ Powered by Coalesce</span>
          </div>
        </div>
      </footer>
    </div>
  );
}

export default App;
