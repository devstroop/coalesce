import { createSignal, For } from 'solid-js';
import { ChevronDown, Play, Download, Upload, Settings, Zap } from 'lucide-solid';

interface ToolbarProps {
  targetLanguage: string;
  onTargetLanguageChange: (language: string) => void;
  isAnalyzing: boolean;
  isTranslating: boolean;
}

const SUPPORTED_LANGUAGES = [
  { id: 'typescript', name: 'TypeScript', icon: '📘' },
  { id: 'javascript', name: 'JavaScript', icon: '🟨' },
  { id: 'python', name: 'Python', icon: '🐍' },
  { id: 'rust', name: 'Rust', icon: '🦀' },
  { id: 'go', name: 'Go', icon: '🔵' },
  { id: 'java', name: 'Java', icon: '☕' },
  { id: 'csharp', name: 'C#', icon: '🟣' },
  { id: 'cpp', name: 'C++', icon: '⚡' },
];

export function Toolbar(props: ToolbarProps) {
  const [showLanguageDropdown, setShowLanguageDropdown] = createSignal(false);

  const selectedLanguage = () => 
    SUPPORTED_LANGUAGES.find(lang => lang.id === props.targetLanguage) || SUPPORTED_LANGUAGES[0];

  return (
    <div class="h-16 border-b border-border bg-card px-4 flex items-center justify-between">
      {/* Left side - Logo and title */}
      <div class="flex items-center space-x-4">
        <div class="flex items-center space-x-2">
          <div class="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
            <span class="text-white font-bold">C</span>
          </div>
          <h1 class="text-xl font-semibold">Coalesce Studio</h1>
        </div>
        
        {props.isAnalyzing && (
          <div class="flex items-center space-x-2 text-blue-500">
            <div class="animate-spin w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full"></div>
            <span class="text-sm">Analyzing...</span>
          </div>
        )}
        
        {props.isTranslating && (
          <div class="flex items-center space-x-2 text-green-500">
            <Zap class="w-4 h-4 animate-pulse" />
            <span class="text-sm">Translating...</span>
          </div>
        )}
      </div>

      {/* Center - Quick actions */}
      <div class="flex items-center space-x-2">
        <button class="px-3 py-2 text-sm border border-border rounded-lg hover:bg-accent flex items-center space-x-2">
          <Upload class="w-4 h-4" />
          <span>Load File</span>
        </button>
        
        <button class="px-3 py-2 text-sm border border-border rounded-lg hover:bg-accent flex items-center space-x-2">
          <Download class="w-4 h-4" />
          <span>Export</span>
        </button>
      </div>

      {/* Right side - Language selector and settings */}
      <div class="flex items-center space-x-4">
        {/* Target Language Selector */}
        <div class="relative">
          <button
            class="px-4 py-2 border border-border rounded-lg bg-background hover:bg-accent flex items-center space-x-2 min-w-32"
            onClick={() => setShowLanguageDropdown(!showLanguageDropdown())}
          >
            <span class="text-lg">{selectedLanguage().icon}</span>
            <span class="text-sm font-medium">{selectedLanguage().name}</span>
            <ChevronDown class="w-4 h-4" />
          </button>

          {showLanguageDropdown() && (
            <div class="absolute top-full right-0 mt-1 w-48 bg-popover border border-border rounded-lg shadow-lg z-50">
              <div class="p-1">
                <For each={SUPPORTED_LANGUAGES}>
                  {(language) => (
                    <button
                      class="w-full px-3 py-2 text-sm text-left hover:bg-accent rounded flex items-center space-x-2"
                      onClick={() => {
                        props.onTargetLanguageChange(language.id);
                        setShowLanguageDropdown(false);
                      }}
                    >
                      <span class="text-lg">{language.icon}</span>
                      <span>{language.name}</span>
                      {language.id === props.targetLanguage && (
                        <span class="ml-auto text-green-500">✓</span>
                      )}
                    </button>
                  )}
                </For>
              </div>
            </div>
          )}
        </div>

        <button class="p-2 hover:bg-accent rounded-lg">
          <Settings class="w-5 h-5" />
        </button>
      </div>
    </div>
  );
}
