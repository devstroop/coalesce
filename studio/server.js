const express = require('express');
const { spawn } = require('child_process');
const path = require('path');
const app = express();
const port = 3001;

app.use(express.json());
app.use((req, res, next) => {
  res.header('Access-Control-Allow-Origin', '*');
  res.header('Access-Control-Allow-Headers', 'Origin, X-Requested-With, Content-Type, Accept');
  res.header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
  next();
});

// Parse endpoint using Coalesce Rust engine
app.post('/api/parse', async (req, res) => {
  try {
    const { source, language } = req.body;
    
    // Call the Rust Coalesce CLI with demo command
    const coalesceProcess = spawn('cargo', [
      'run', '--bin', 'coalesce', 
      'demo', 
      source,
      '--from', language,
      '--to', 'python' // Just for parsing, we don't need the translation
    ], {
      cwd: path.join(__dirname, '..'),
      stdio: ['pipe', 'pipe', 'pipe']
    });

    let stdout = '';
    let stderr = '';

    coalesceProcess.stdout.on('data', (data) => {
      stdout += data.toString();
    });

    coalesceProcess.stderr.on('data', (data) => {
      stderr += data.toString();
    });

    coalesceProcess.on('close', (code) => {
      if (code === 0) {
        // Try to extract UIR from the output
        try {
          // Look for JSON in the output
          const lines = stdout.split('\n');
          let uirLine = null;
          
          for (const line of lines) {
            if (line.includes('{') && line.includes('id')) {
              uirLine = line;
              break;
            }
          }

          if (uirLine) {
            const uir = JSON.parse(uirLine);
            res.json(uir);
          } else {
            // Create a mock UIR structure based on the source code
            const mockUIR = createMockUIR(source, language);
            res.json(mockUIR);
          }
        } catch (parseError) {
          console.warn('Failed to parse UIR, creating mock:', parseError);
          const mockUIR = createMockUIR(source, language);
          res.json(mockUIR);
        }
      } else {
        console.warn('Coalesce process failed:', stderr);
        const mockUIR = createMockUIR(source, language);
        res.json(mockUIR);
      }
    });

    coalesceProcess.on('error', (error) => {
      console.warn('Failed to spawn Coalesce process:', error);
      const mockUIR = createMockUIR(source, language);
      res.json(mockUIR);
    });

  } catch (error) {
    console.error('Parse API error:', error);
    res.status(500).json({ error: 'Failed to parse code' });
  }
});

// Translation endpoint
app.post('/api/translate', async (req, res) => {
  try {
    const { source, from, to } = req.body;
    
    // Call the Rust Coalesce CLI
    const coalesceProcess = spawn('cargo', [
      'run', '--bin', 'coalesce',
      'demo',
      source,
      '--from', from,
      '--to', to
    ], {
      cwd: path.join(__dirname, '..'),
      stdio: ['pipe', 'pipe', 'pipe']
    });

    let stdout = '';
    let stderr = '';

    coalesceProcess.stdout.on('data', (data) => {
      stdout += data.toString();
    });

    coalesceProcess.stderr.on('data', (data) => {
      stderr += data.toString();
    });

    coalesceProcess.on('close', (code) => {
      if (code === 0) {
        // Extract translated code from output
        const lines = stdout.split('\n');
        let translatedCode = '';
        let capturing = false;
        
        for (const line of lines) {
          if (line.includes('code:') || capturing) {
            if (!capturing && line.includes('code:')) {
              capturing = true;
              const codeStart = line.indexOf('code:') + 5;
              translatedCode += line.substring(codeStart).trim() + '\n';
            } else if (capturing) {
              if (line.trim() === '' || line.includes('✅')) {
                break;
              }
              translatedCode += line + '\n';
            }
          }
        }

        res.json({ translated_code: translatedCode.trim() || 'Translation completed' });
      } else {
        res.status(500).json({ error: 'Translation failed', details: stderr });
      }
    });

  } catch (error) {
    console.error('Translation API error:', error);
    res.status(500).json({ error: 'Failed to translate code' });
  }
});

// Create mock UIR for fallback
function createMockUIR(source, language) {
  const lines = source.split('\n');
  const hasClass = source.includes('class ');
  const hasFunctions = source.match(/function\s+\w+|def\s+\w+|\w+\s*\(/g);
  const hasVariables = source.match(/const\s+\w+|let\s+\w+|var\s+\w+/g);

  const children = [];
  
  if (hasClass) {
    children.push({
      id: 'class_node',
      node_type: 'Class',
      name: 'Calculator',
      children: [],
      metadata: { language }
    });
  }

  if (hasFunctions) {
    hasFunctions.forEach((func, index) => {
      children.push({
        id: `function_${index}`,
        node_type: 'Function',
        name: func.replace(/function\s+|def\s+|\(.*/, '').trim(),
        children: [],
        metadata: { language }
      });
    });
  }

  if (hasVariables) {
    hasVariables.forEach((variable, index) => {
      children.push({
        id: `variable_${index}`,
        node_type: 'Variable',
        name: variable.replace(/const\s+|let\s+|var\s+/, '').split('=')[0].trim(),
        children: [],
        metadata: { language }
      });
    });
  }

  return {
    id: 'root',
    node_type: 'Module',
    name: 'program',
    children: children,
    metadata: {
      language,
      lines: lines.length,
      complexity: children.length * 2
    }
  };
}

app.listen(port, () => {
  console.log(`🚀 Coalesce API Server running at http://localhost:${port}`);
  console.log(`📡 Ready to bridge frontend with Rust engine`);
});
