import { createEffect, onMount } from 'solid-js';
import * as monaco from 'monaco-editor';

interface CodeEditorProps {
  value: string;
  onChange: (value: string) => void;
  language: string;
  placeholder?: string;
  readOnly?: boolean;
}

export function CodeEditor(props: CodeEditorProps) {
  let containerRef: HTMLDivElement | undefined;
  let editor: monaco.editor.IStandaloneCodeEditor;

  onMount(() => {
    // Initialize Monaco Editor
    if (!containerRef) return;
    editor = monaco.editor.create(containerRef, {
      value: props.value,
      language: props.language === 'auto' ? 'javascript' : props.language,
      theme: 'vs-dark',
      automaticLayout: true,
      minimap: { enabled: false },
      fontSize: 14,
      lineNumbers: 'on',
      readOnly: props.readOnly || false,
      wordWrap: 'on',
      scrollBeyondLastLine: false,
      folding: true,
      renderWhitespace: 'selection',
      tabSize: 2,
      insertSpaces: true,
    });

    // Listen for content changes
    editor.onDidChangeModelContent(() => {
      const value = editor.getValue();
      props.onChange(value);
    });

    // Add placeholder text if empty
    if (!props.value && props.placeholder) {
      editor.deltaDecorations([], [
        {
          range: new monaco.Range(1, 1, 1, 1),
          options: {
            afterContentClassName: 'placeholder-text',
            isWholeLine: false,
          },
        },
      ]);
    }
  });

  // Update editor when value prop changes
  createEffect(() => {
    if (editor && editor.getValue() !== props.value) {
      editor.setValue(props.value);
    }
  });

  // Update language when prop changes
  createEffect(() => {
    if (editor) {
      const model = editor.getModel();
      if (model) {
        const language = props.language === 'auto' ? 'javascript' : props.language;
        monaco.editor.setModelLanguage(model, language);
      }
    }
  });

  return (
    <div class="h-full w-full relative">
      <div
        ref={containerRef!}
        class="h-full w-full"
      />
      <style>{`
        .placeholder-text::after {
          content: '${props.placeholder || 'Enter code here...'}';
          color: #666;
          font-style: italic;
          pointer-events: none;
        }
      `}</style>
    </div>
  );
}
