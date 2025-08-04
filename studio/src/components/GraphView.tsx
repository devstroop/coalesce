import { createEffect, onMount, createSignal } from 'solid-js';
import cytoscape from 'cytoscape';
import { UIRNode, UIRNodeType } from '../types/uir';

interface GraphViewProps {
  uir: UIRNode;
  selectedNode?: string | null;
  onNodeSelect: (nodeId: string) => void;
}

export function GraphView(props: GraphViewProps) {
  let containerRef: HTMLDivElement | undefined;
  let cy: cytoscape.Core;
  const [isReady, setIsReady] = createSignal(false);

  // Convert UIR to Cytoscape format
  const convertUIRToGraph = (uir: UIRNode) => {
    const nodes: any[] = [];
    const edges: any[] = [];

    const addNode = (node: UIRNode, parent?: string) => {
      // Add the node
      nodes.push({
        data: {
          id: node.id,
          label: node.name || node.type,
          type: node.type,
          parent: parent,
          metadata: node.metadata,
        },
        position: node.position || { x: Math.random() * 500, y: Math.random() * 500 },
        classes: getNodeClass(node.type),
      });

      // Add children recursively
      node.children?.forEach((child) => {
        addNode(child, node.id);
        
        // Add edge from parent to child
        edges.push({
          data: {
            id: `${node.id}-${child.id}`,
            source: node.id,
            target: child.id,
            type: 'contains',
          },
          classes: 'contains-edge',
        });
      });

      // Add library dependency edges
      node.library_dependencies?.forEach((dep, index) => {
        const depId = `${node.id}-dep-${index}`;
        nodes.push({
          data: {
            id: depId,
            label: `${dep.library}.${dep.pattern}`,
            type: 'library',
            metadata: { dependency: dep },
          },
          position: { x: Math.random() * 500, y: Math.random() * 500 },
          classes: 'library-node',
        });

        edges.push({
          data: {
            id: `${node.id}-${depId}`,
            source: node.id,
            target: depId,
            type: 'uses',
          },
          classes: 'uses-edge',
        });
      });
    };

    addNode(uir);
    return { nodes, edges };
  };

  const getNodeClass = (type: UIRNodeType): string => {
    switch (type) {
      case UIRNodeType.Module:
        return 'module-node';
      case UIRNodeType.Function:
        return 'function-node';
      case UIRNodeType.Class:
        return 'class-node';
      case UIRNodeType.Variable:
        return 'variable-node';
      case UIRNodeType.Import:
      case UIRNodeType.Export:
        return 'import-node';
      default:
        return 'default-node';
    }
  };

  const getNodeColor = (type: UIRNodeType): string => {
    switch (type) {
      case UIRNodeType.Module:
        return '#3B82F6'; // blue
      case UIRNodeType.Function:
        return '#10B981'; // green
      case UIRNodeType.Class:
        return '#8B5CF6'; // purple
      case UIRNodeType.Variable:
        return '#F59E0B'; // yellow
      case UIRNodeType.Import:
      case UIRNodeType.Export:
        return '#EF4444'; // red
      default:
        return '#6B7280'; // gray
    }
  };

  onMount(() => {
    // Initialize Cytoscape
    if (!containerRef) return;
    cy = cytoscape({
      container: containerRef,
      style: [
        {
          selector: 'node',
          style: {
            'background-color': (ele: any) => getNodeColor(ele.data('type')),
            'label': 'data(label)',
            'color': '#ffffff',
            'text-valign': 'center',
            'text-halign': 'center',
            'font-size': '12px',
            'font-weight': 'bold',
            'width': '60px',
            'height': '60px',
            'border-width': '2px',
            'border-color': '#ffffff',
            'text-wrap': 'wrap',
            'text-max-width': '80px',
          }
        },
        {
          selector: 'node.selected',
          style: {
            'border-color': '#FFD700',
            'border-width': '4px',
            'overlay-color': '#FFD700',
            'overlay-opacity': 0.3,
          }
        },
        {
          selector: 'node.module-node',
          style: {
            'shape': 'round-rectangle',
            'width': '80px',
            'height': '40px',
          }
        },
        {
          selector: 'node.function-node',
          style: {
            'shape': 'ellipse',
          }
        },
        {
          selector: 'node.class-node',
          style: {
            'shape': 'round-rectangle',
            'width': '70px',
            'height': '50px',
          }
        },
        {
          selector: 'node.library-node',
          style: {
            'shape': 'diamond',
            'background-color': '#F97316', // orange
            'width': '50px',
            'height': '50px',
          }
        },
        {
          selector: 'edge',
          style: {
            'width': 2,
            'line-color': '#64748B',
            'target-arrow-color': '#64748B',
            'target-arrow-shape': 'triangle',
            'curve-style': 'bezier',
          }
        },
        {
          selector: 'edge.contains-edge',
          style: {
            'line-color': '#3B82F6',
            'target-arrow-color': '#3B82F6',
            'line-style': 'solid',
          }
        },
        {
          selector: 'edge.uses-edge',
          style: {
            'line-color': '#F97316',
            'target-arrow-color': '#F97316',
            'line-style': 'dashed',
          }
        },
      ],
      layout: {
        name: 'cose',
        animate: true,
        animationDuration: 500,
        nodeRepulsion: 8000,
        idealEdgeLength: 100,
        edgeElasticity: 100,
        nestingFactor: 1.2,
      },
      zoom: 1,
      pan: { x: 0, y: 0 },
      minZoom: 0.1,
      maxZoom: 3,
      wheelSensitivity: 0.2,
    });

    // Handle node selection
    cy.on('tap', 'node', (event) => {
      const node = event.target;
      const nodeId = node.data('id');
      
      // Remove previous selection
      cy.nodes().removeClass('selected');
      
      // Add selection to clicked node
      node.addClass('selected');
      
      // Notify parent component
      props.onNodeSelect(nodeId);
    });

    // Handle background click to deselect
    cy.on('tap', (event) => {
      if (event.target === cy) {
        cy.nodes().removeClass('selected');
        props.onNodeSelect('');
      }
    });

    setIsReady(true);
  });

  // Update graph when UIR changes
  createEffect(() => {
    if (!cy || !isReady()) return;

    const { nodes, edges } = convertUIRToGraph(props.uir);
    
    cy.elements().remove();
    cy.add([...nodes, ...edges]);
    
    // Re-run layout
    cy.layout({
      name: 'cose',
      animate: true,
      animationDuration: 1000,
      nodeRepulsion: 8000,
      idealEdgeLength: 100,
      edgeElasticity: 100,
      nestingFactor: 1.2,
    }).run();
  });

  // Update selection when selectedNode prop changes
  createEffect(() => {
    if (!cy || !isReady()) return;

    cy.nodes().removeClass('selected');
    
    if (props.selectedNode) {
      const node = cy.getElementById(props.selectedNode);
      if (node.length > 0) {
        node.addClass('selected');
        // Center the view on the selected node
        cy.center(node);
      }
    }
  });

  return (
    <div class="h-full w-full relative">
      <div
        ref={containerRef!}
        class="h-full w-full graph-container"
      />
      
      {/* Graph Controls */}
      <div class="absolute top-4 left-4 flex flex-col space-y-2">
        <button
          class="p-2 bg-background border border-border rounded-lg shadow-sm hover:bg-accent"
          onClick={() => cy?.fit()}
          title="Fit to view"
        >
          🔍
        </button>
        <button
          class="p-2 bg-background border border-border rounded-lg shadow-sm hover:bg-accent"
          onClick={() => cy?.center()}
          title="Center view"
        >
          🎯
        </button>
        <button
          class="p-2 bg-background border border-border rounded-lg shadow-sm hover:bg-accent"
          onClick={() => cy?.zoom(1)}
          title="Reset zoom"
        >
          ↻
        </button>
      </div>

      {/* Legend */}
      <div class="absolute top-4 right-4 bg-background border border-border rounded-lg p-3 shadow-sm">
        <h4 class="text-sm font-medium mb-2">Legend</h4>
        <div class="space-y-1 text-xs">
          <div class="flex items-center space-x-2">
            <div class="w-3 h-3 rounded bg-blue-500"></div>
            <span>Module</span>
          </div>
          <div class="flex items-center space-x-2">
            <div class="w-3 h-3 rounded-full bg-green-500"></div>
            <span>Function</span>
          </div>
          <div class="flex items-center space-x-2">
            <div class="w-3 h-3 rounded bg-purple-500"></div>
            <span>Class</span>
          </div>
          <div class="flex items-center space-x-2">
            <div class="w-3 h-3 rounded bg-yellow-500"></div>
            <span>Variable</span>
          </div>
          <div class="flex items-center space-x-2">
            <div class="w-3 h-3 rotate-45 bg-orange-500"></div>
            <span>Library</span>
          </div>
        </div>
      </div>
    </div>
  );
}
