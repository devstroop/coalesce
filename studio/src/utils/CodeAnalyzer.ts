interface AnalysisResult {
  nodes: Array<{
    id: string;
    name: string;
    type: 'function' | 'class' | 'variable' | 'import' | 'export';
    group: number;
  }>;
  links: Array<{
    source: string;
    target: string;
    type: 'calls' | 'imports' | 'extends' | 'uses';
    value: number;
  }>;
  metrics: {
    functions: number;
    classes: number;
    variables: number;
    imports: number;
    lines: number;
    complexity: number;
  };
}

export class CodeAnalyzer {
  analyzeJavaScript(code: string): AnalysisResult {
    const lines = code.split('\n');
    const nodes: AnalysisResult['nodes'] = [];
    const links: AnalysisResult['links'] = [];
    let nodeId = 0;

    // Function detection
    const functionRegex = /(?:function\s+(\w+)|const\s+(\w+)\s*=\s*(?:\([^)]*\)\s*=>|\([^)]*\)\s*\{|function))/g;
    let match;
    const functions: string[] = [];
    
    while ((match = functionRegex.exec(code)) !== null) {
      const funcName = match[1] || match[2];
      if (funcName) {
        functions.push(funcName);
        nodes.push({
          id: `func_${nodeId++}`,
          name: funcName,
          type: 'function',
          group: 1
        });
      }
    }

    // Class detection
    const classRegex = /class\s+(\w+)/g;
    const classes: string[] = [];
    
    while ((match = classRegex.exec(code)) !== null) {
      const className = match[1];
      classes.push(className);
      nodes.push({
        id: `class_${nodeId++}`,
        name: className,
        type: 'class',
        group: 2
      });

      // Find methods in class
      const classStart = match.index;
      const classEnd = this.findClassEnd(code, classStart);
      const classBody = code.slice(classStart, classEnd);
      
      const methodRegex = /(\w+)\s*\([^)]*\)\s*\{/g;
      let methodMatch;
      
      while ((methodMatch = methodRegex.exec(classBody)) !== null) {
        const methodName = methodMatch[1];
        if (methodName !== 'constructor' && methodName !== className) {
          const methodId = `method_${nodeId++}`;
          nodes.push({
            id: methodId,
            name: methodName,
            type: 'function',
            group: 1
          });
          
          // Link method to class
          links.push({
            source: `class_${classes.length - 1}`,
            target: methodId,
            type: 'uses',
            value: 1
          });
        }
      }
    }

    // Variable detection
    const variableRegex = /(?:const|let|var)\s+(\w+)/g;
    const variables: string[] = [];
    
    while ((match = variableRegex.exec(code)) !== null) {
      const varName = match[1];
      if (!functions.includes(varName) && !classes.includes(varName)) {
        variables.push(varName);
        nodes.push({
          id: `var_${nodeId++}`,
          name: varName,
          type: 'variable',
          group: 3
        });
      }
    }

    // Import detection
    const importRegex = /import\s+(?:\{[^}]+\}|\w+)\s+from\s+['"]([^'"]+)['"]/g;
    const imports: string[] = [];
    
    while ((match = importRegex.exec(code)) !== null) {
      const importPath = match[1];
      const importName = this.getModuleName(importPath);
      imports.push(importName);
      nodes.push({
        id: `import_${nodeId++}`,
        name: importName,
        type: 'import',
        group: 4
      });
    }

    // Function call detection and linking
    const callRegex = /(\w+)\s*\(/g;
    
    while ((match = callRegex.exec(code)) !== null) {
      const calledFunction = match[1];
      const callerIndex = this.findCallerFunction(code, match.index);
      
      if (callerIndex !== -1) {
        const caller = functions[callerIndex];
        const callerNode = nodes.find(n => n.name === caller && n.type === 'function');
        const calleeNode = nodes.find(n => n.name === calledFunction && n.type === 'function');
        
        if (callerNode && calleeNode && callerNode.id !== calleeNode.id) {
          const existingLink = links.find(l => 
            l.source === callerNode.id && l.target === calleeNode.id
          );
          
          if (existingLink) {
            existingLink.value++;
          } else {
            links.push({
              source: callerNode.id,
              target: calleeNode.id,
              type: 'calls',
              value: 1
            });
          }
        }
      }
    }

    // Calculate complexity
    const complexity = this.calculateComplexity(code);

    return {
      nodes,
      links,
      metrics: {
        functions: functions.length,
        classes: classes.length,
        variables: variables.length,
        imports: imports.length,
        lines: lines.length,
        complexity
      }
    };
  }

  private findClassEnd(code: string, start: number): number {
    let braceCount = 0;
    let inClass = false;
    
    for (let i = start; i < code.length; i++) {
      const char = code[i];
      
      if (char === '{') {
        braceCount++;
        inClass = true;
      } else if (char === '}') {
        braceCount--;
        if (inClass && braceCount === 0) {
          return i + 1;
        }
      }
    }
    
    return code.length;
  }

  private getModuleName(path: string): string {
    const parts = path.split('/');
    const filename = parts[parts.length - 1];
    return filename.replace(/\.(js|ts|jsx|tsx)$/, '');
  }

  private findCallerFunction(code: string, position: number): number {
    const beforePosition = code.slice(0, position);
    const functionRegex = /function\s+(\w+)|const\s+(\w+)\s*=\s*(?:\([^)]*\)\s*=>|\([^)]*\)\s*\{|function)/g;
    
    let lastMatch;
    let match;
    
    while ((match = functionRegex.exec(beforePosition)) !== null) {
      lastMatch = match;
    }
    
    return lastMatch ? 0 : -1; // Simplified for demo
  }

  private calculateComplexity(code: string): number {
    // Simplified cyclomatic complexity
    const conditions = (code.match(/if\s*\(|while\s*\(|for\s*\(|switch\s*\(|\?\s*\w+\s*:/g) || []).length;
    const functions = (code.match(/function\s+\w+|=>\s*\{/g) || []).length;
    
    return Math.max(1, conditions + functions);
  }

  analyzePython(code: string): AnalysisResult {
    const lines = code.split('\n');
    const nodes: AnalysisResult['nodes'] = [];
    const links: AnalysisResult['links'] = [];
    let nodeId = 0;

    // Function detection
    const functionRegex = /def\s+(\w+)\s*\(/g;
    let match;
    const functions: string[] = [];
    
    while ((match = functionRegex.exec(code)) !== null) {
      const funcName = match[1];
      functions.push(funcName);
      nodes.push({
        id: `func_${nodeId++}`,
        name: funcName,
        type: 'function',
        group: 1
      });
    }

    // Class detection
    const classRegex = /class\s+(\w+)/g;
    const classes: string[] = [];
    
    while ((match = classRegex.exec(code)) !== null) {
      const className = match[1];
      classes.push(className);
      nodes.push({
        id: `class_${nodeId++}`,
        name: className,
        type: 'class',
        group: 2
      });
    }

    // Import detection
    const importRegex = /(?:from\s+(\w+)\s+import|import\s+(\w+))/g;
    
    while ((match = importRegex.exec(code)) !== null) {
      const importName = match[1] || match[2];
      nodes.push({
        id: `import_${nodeId++}`,
        name: importName,
        type: 'import',
        group: 4
      });
    }

    return {
      nodes,
      links,
      metrics: {
        functions: functions.length,
        classes: classes.length,
        variables: (code.match(/^\s*\w+\s*=/gm) || []).length,
        imports: (code.match(/import\s+|from\s+\w+\s+import/g) || []).length,
        lines: lines.length,
        complexity: this.calculateComplexity(code)
      }
    };
  }

  analyze(code: string, language: string): AnalysisResult {
    switch (language.toLowerCase()) {
      case 'javascript':
      case 'js':
      case 'typescript':
      case 'ts':
        return this.analyzeJavaScript(code);
      case 'python':
      case 'py':
        return this.analyzePython(code);
      default:
        return this.analyzeJavaScript(code); // Fallback
    }
  }
}
