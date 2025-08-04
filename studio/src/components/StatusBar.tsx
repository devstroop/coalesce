import { Show } from 'solid-js';
import { UIRNode } from '../types/uir';

interface StatusBarProps {
  uirGraph?: UIRNode | null;
  selectedNode?: string | null;
  isAnalyzing: boolean;
  isTranslating: boolean;
}

export function StatusBar(props: StatusBarProps) {
  const getNodeCount = (node: UIRNode): number => {
    let count = 1;
    node.children?.forEach(child => {
      count += getNodeCount(child);
    });
    return count;
  };

  const getLibraryCount = (node: UIRNode): number => {
    let count = node.library_dependencies?.length || 0;
    node.children?.forEach(child => {
      count += getLibraryCount(child);
    });
    return count;
  };

  return (
    <div class="h-6 bg-muted border-t border-border px-4 flex items-center justify-between text-xs text-muted-foreground">
      {/* Left side - Graph stats */}
      <div class="flex items-center space-x-4">
        <Show when={props.uirGraph}>
          <span>
            Nodes: {getNodeCount(props.uirGraph!)}
          </span>
          <span>
            Libraries: {getLibraryCount(props.uirGraph!)}
          </span>
        </Show>
        
        <Show when={props.selectedNode}>
          <span class="text-blue-500">
            Selected: {props.selectedNode}
          </span>
        </Show>
      </div>

      {/* Center - Status */}
      <div class="flex items-center space-x-4">
        <Show when={props.isAnalyzing}>
          <span class="text-blue-500">Analyzing code structure...</span>
        </Show>
        
        <Show when={props.isTranslating}>
          <span class="text-green-500">Generating translation...</span>
        </Show>
        
        <Show when={!props.isAnalyzing && !props.isTranslating && props.uirGraph}>
          <span class="text-green-500">Ready</span>
        </Show>
      </div>

      {/* Right side - Version */}
      <div>
        Coalesce Studio v0.1.0
      </div>
    </div>
  );
}
