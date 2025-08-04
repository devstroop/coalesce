import { onMount, onCleanup, createEffect } from 'solid-js';
import loader from '@monaco-editor/loader';
import type { editor } from 'monaco-editor';

interface MonacoEditorProps {
  value: string;
  onChange: (value: string) => void;
  language: string;
  theme?: string;
  readOnly?: boolean;
  height?: string;
}

export function MonacoEditor(props: MonacoEditorProps) {
  let editorRef: HTMLDivElement | undefined;
  let editor: editor.IStandaloneCodeEditor | undefined;

  onMount(async () => {
    if (!editorRef) return;

    try {
      const monaco = await loader.init();
      
      // Configure Monaco themes
      monaco.editor.defineTheme('coalesce-dark', {
        base: 'vs-dark',
        inherit: true,
        rules: [
          { token: 'comment', foreground: '71717a', fontStyle: 'italic' },
          { token: 'keyword', foreground: '8b5cf6' },
          { token: 'string', foreground: '10b981' },
          { token: 'number', foreground: 'f59e0b' },
          { token: 'function', foreground: '06b6d4' },
        ],
        colors: {
          'editor.background': '#0f0f10',
          'editor.foreground': '#e4e4e7',
          'editor.lineHighlightBackground': '#18181b',
          'editor.selectionBackground': '#374151',
          'editorCursor.foreground': '#8b5cf6',
          'editorLineNumber.foreground': '#52525b',
          'editorLineNumber.activeForeground': '#a1a1aa',
        },
      });

      editor = monaco.editor.create(editorRef, {
        value: props.value,
        language: props.language,
        theme: props.theme || 'coalesce-dark',
        readOnly: props.readOnly || false,
        automaticLayout: true,
        minimap: { enabled: false },
        scrollBeyondLastLine: false,
        fontSize: 14,
        fontFamily: 'SF Mono, Monaco, Inconsolata, Roboto Mono, monospace',
        lineHeight: 1.6,
        tabSize: 2,
        insertSpaces: true,
        wordWrap: 'on',
        bracketPairColorization: { enabled: true },
        suggest: { preview: true },
        quickSuggestions: true,
        parameterHints: { enabled: true },
      });

      // Handle value changes
      editor.onDidChangeModelContent(() => {
        const value = editor?.getValue() || '';
        props.onChange(value);
      });

      // Add keybindings
      editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
        console.log('Save shortcut pressed');
      });

    } catch (error) {
      console.error('Failed to initialize Monaco Editor:', error);
    }
  });

  // Update editor value when prop changes
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
        loader.init().then(monaco => {
          monaco.editor.setModelLanguage(model, props.language);
        });
      }
    }
  });

  onCleanup(() => {
    editor?.dispose();
  });

  return (
    <div 
      ref={editorRef} 
      style={{ 
        height: props.height || '100%', 
        width: '100%',
        border: '1px solid #27272a',
        'border-radius': '8px',
        overflow: 'hidden'
      }} 
    />
  );
}
