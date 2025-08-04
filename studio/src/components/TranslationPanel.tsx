import { Show, createSignal } from 'solid-js';
import { CodeEditor } from './CodeEditor';
import { UIRNode } from '../types/uir';
import { Copy, Download, RefreshCw, ThumbsUp, ThumbsDown } from 'lucide-solid';

interface TranslationPanelProps {
  translatedCode: string;
  targetLanguage: string;
  isTranslating: boolean;
  selectedNode?: string | null;
  uir?: UIRNode | null;
}

export function TranslationPanel(props: TranslationPanelProps) {
  const [feedback, setFeedback] = createSignal<'positive' | 'negative' | null>(null);
  const [showComparison, setShowComparison] = createSignal(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(props.translatedCode);
      // Could add a toast notification here
    } catch (error) {
      console.error('Failed to copy:', error);
    }
  };

  const handleDownload = () => {
    const fileExtension = getFileExtension(props.targetLanguage);
    const blob = new Blob([props.translatedCode], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `translated.${fileExtension}`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const handleFeedback = (type: 'positive' | 'negative') => {
    setFeedback(type);
    // Send feedback to backend for learning
    fetch('/api/feedback', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        feedback: type,
        uir: props.uir,
        translation: props.translatedCode,
        target_language: props.targetLanguage,
        selected_node: props.selectedNode,
      }),
    }).catch(console.error);
  };

  const getFileExtension = (language: string): string => {
    const extensions: Record<string, string> = {
      typescript: 'ts',
      javascript: 'js',
      python: 'py',
      rust: 'rs',
      go: 'go',
      java: 'java',
      csharp: 'cs',
      cpp: 'cpp',
    };
    return extensions[language] || 'txt';
  };

  return (
    <div class="h-full flex flex-col">
      {/* Header */}
      <div class="px-4 py-2 border-b border-border bg-muted/50 flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <h3 class="text-sm font-medium">Translation</h3>
          <Show when={props.isTranslating}>
            <div class="flex items-center space-x-1 text-green-500">
              <RefreshCw class="w-3 h-3 animate-spin" />
              <span class="text-xs">Translating...</span>
            </div>
          </Show>
        </div>
        
        <div class="flex items-center space-x-1">
          <button
            class="p-1 hover:bg-accent rounded text-muted-foreground hover:text-foreground"
            onClick={handleCopy}
            title="Copy to clipboard"
            disabled={!props.translatedCode}
          >
            <Copy class="w-4 h-4" />
          </button>
          
          <button
            class="p-1 hover:bg-accent rounded text-muted-foreground hover:text-foreground"
            onClick={handleDownload}
            title="Download file"
            disabled={!props.translatedCode}
          >
            <Download class="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Translation Content */}
      <div class="flex-1 flex flex-col">
        <Show 
          when={props.translatedCode}
          fallback={
            <div class="flex-1 flex items-center justify-center text-muted-foreground">
              <div class="text-center">
                <div class="text-4xl mb-2">🚀</div>
                <div class="text-sm">Translation will appear here</div>
                <div class="text-xs mt-1">Enter source code to begin</div>
              </div>
            </div>
          }
        >
          <div class="flex-1">
            <CodeEditor
              value={props.translatedCode}
              onChange={() => {}} // Read-only
              language={props.targetLanguage}
              readOnly={true}
            />
          </div>
        </Show>
      </div>

      {/* Feedback Section */}
      <Show when={props.translatedCode}>
        <div class="border-t border-border p-3">
          <div class="flex items-center justify-between">
            <span class="text-sm text-muted-foreground">How's this translation?</span>
            
            <div class="flex items-center space-x-2">
              <button
                class={`p-2 rounded-lg transition-colors ${
                  feedback() === 'positive' 
                    ? 'bg-green-100 text-green-700' 
                    : 'hover:bg-accent text-muted-foreground hover:text-foreground'
                }`}
                onClick={() => handleFeedback('positive')}
                title="Good translation"
              >
                <ThumbsUp class="w-4 h-4" />
              </button>
              
              <button
                class={`p-2 rounded-lg transition-colors ${
                  feedback() === 'negative' 
                    ? 'bg-red-100 text-red-700' 
                    : 'hover:bg-accent text-muted-foreground hover:text-foreground'
                }`}
                onClick={() => handleFeedback('negative')}
                title="Needs improvement"
              >
                <ThumbsDown class="w-4 h-4" />
              </button>
            </div>
          </div>

          <Show when={feedback() === 'negative'}>
            <div class="mt-3 p-3 bg-muted rounded-lg">
              <textarea
                class="w-full bg-transparent text-sm resize-none"
                placeholder="What could be improved? Your feedback helps Coalesce learn."
                rows="2"
              />
              <div class="flex justify-end mt-2">
                <button class="px-3 py-1 text-xs bg-primary text-primary-foreground rounded">
                  Send Feedback
                </button>
              </div>
            </div>
          </Show>
        </div>
      </Show>

      {/* Node-specific information */}
      <Show when={props.selectedNode && props.uir}>
        <div class="border-t border-border p-3 bg-muted/50">
          <h4 class="text-sm font-medium mb-2">Selected Node</h4>
          <div class="text-xs text-muted-foreground space-y-1">
            <div>ID: {props.selectedNode}</div>
            {/* Add more node-specific details here */}
          </div>
        </div>
      </Show>
    </div>
  );
}
