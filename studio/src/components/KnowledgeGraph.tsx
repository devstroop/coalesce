import { onMount, onCleanup, createEffect } from 'solid-js';
import * as d3 from 'd3';

interface GraphNode {
  id: string;
  name: string;
  type: 'function' | 'class' | 'variable' | 'import' | 'export';
  group: number;
  x?: number;
  y?: number;
  fx?: number;
  fy?: number;
}

interface GraphLink {
  source: string | GraphNode;
  target: string | GraphNode;
  type: 'calls' | 'imports' | 'extends' | 'uses';
  value: number;
}

interface GraphData {
  nodes: GraphNode[];
  links: GraphLink[];
}

interface KnowledgeGraphProps {
  data: GraphData;
  selectedNode?: string;
  onNodeSelect?: (nodeId: string) => void;
  width?: number;
  height?: number;
}

export function KnowledgeGraph(props: KnowledgeGraphProps) {
  let svgRef: SVGSVGElement | undefined;
  let simulation: d3.Simulation<GraphNode, GraphLink> | undefined;

  const colors = {
    function: '#06b6d4',
    class: '#8b5cf6',
    variable: '#10b981',
    import: '#f59e0b',
    export: '#ef4444',
  };

  onMount(() => {
    if (!svgRef) return;
    initializeGraph();
  });

  createEffect(() => {
    if (props.data && svgRef) {
      updateGraph(props.data);
    }
  });

  const initializeGraph = () => {
    if (!svgRef) return;

    const width = props.width || 400;
    const height = props.height || 300;

    const svg = d3.select(svgRef)
      .attr('width', width)
      .attr('height', height)
      .attr('viewBox', `0 0 ${width} ${height}`)
      .style('background', '#0a0a0b')
      .style('border-radius', '8px');

    // Add zoom behavior
    const zoom = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .on('zoom', (event) => {
        svg.select('g').attr('transform', event.transform);
      });

    svg.call(zoom);

    // Create main group
    const g = svg.append('g');

    // Add arrow markers for directed edges
    svg.append('defs').selectAll('marker')
      .data(['calls', 'imports', 'extends', 'uses'])
      .enter().append('marker')
      .attr('id', d => `arrow-${d}`)
      .attr('viewBox', '0 -5 10 10')
      .attr('refX', 15)
      .attr('refY', 0)
      .attr('markerWidth', 6)
      .attr('markerHeight', 6)
      .attr('orient', 'auto')
      .append('path')
      .attr('d', 'M0,-5L10,0L0,5')
      .attr('fill', '#52525b');

    // Initialize simulation
    simulation = d3.forceSimulation<GraphNode>()
      .force('link', d3.forceLink<GraphNode, GraphLink>().id(d => d.id).distance(80))
      .force('charge', d3.forceManyBody().strength(-300))
      .force('center', d3.forceCenter(width / 2, height / 2))
      .force('collision', d3.forceCollide().radius(30));
  };

  const updateGraph = (data: GraphData) => {
    if (!svgRef || !simulation) return;

    const svg = d3.select(svgRef);
    const g = svg.select('g');

    // Update links
    const link = g.selectAll<SVGLineElement, GraphLink>('.link')
      .data(data.links);

    link.exit().remove();

    const linkEnter = link.enter().append('line')
      .attr('class', 'link')
      .attr('stroke', '#52525b')
      .attr('stroke-width', d => Math.sqrt(d.value))
      .attr('marker-end', d => `url(#arrow-${d.type})`);

    const linkUpdate = linkEnter.merge(link);

    // Update nodes
    const node = g.selectAll<SVGGElement, GraphNode>('.node')
      .data(data.nodes);

    node.exit().remove();

    const nodeEnter = node.enter().append('g')
      .attr('class', 'node')
      .style('cursor', 'pointer')
      .call(d3.drag<SVGGElement, GraphNode>()
        .on('start', dragstarted)
        .on('drag', dragged)
        .on('end', dragended));

    // Add circles
    nodeEnter.append('circle')
      .attr('r', 12)
      .attr('fill', d => colors[d.type])
      .attr('stroke', '#1f2937')
      .attr('stroke-width', 2);

    // Add labels
    nodeEnter.append('text')
      .attr('dy', -16)
      .attr('text-anchor', 'middle')
      .attr('fill', '#e4e4e7')
      .attr('font-size', '12px')
      .attr('font-family', 'system-ui, sans-serif')
      .text(d => d.name);

    // Add type indicators
    nodeEnter.append('text')
      .attr('dy', 4)
      .attr('text-anchor', 'middle')
      .attr('fill', '#a1a1aa')
      .attr('font-size', '8px')
      .attr('font-family', 'system-ui, sans-serif')
      .text(d => d.type.charAt(0).toUpperCase());

    const nodeUpdate = nodeEnter.merge(node);

    // Handle selection
    nodeUpdate.select('circle')
      .attr('stroke', d => d.id === props.selectedNode ? colors[d.type] : '#1f2937')
      .attr('stroke-width', d => d.id === props.selectedNode ? 3 : 2);

    // Add click handler
    nodeUpdate.on('click', (event, d) => {
      event.stopPropagation();
      props.onNodeSelect?.(d.id);
    });

    // Update simulation
    simulation
      .nodes(data.nodes)
      .on('tick', () => {
        linkUpdate
          .attr('x1', d => (d.source as GraphNode).x!)
          .attr('y1', d => (d.source as GraphNode).y!)
          .attr('x2', d => (d.target as GraphNode).x!)
          .attr('y2', d => (d.target as GraphNode).y!);

        nodeUpdate
          .attr('transform', d => `translate(${d.x},${d.y})`);
      });

    (simulation.force('link') as d3.ForceLink<GraphNode, GraphLink>)
      .links(data.links);

    simulation.alpha(1).restart();
  };

  const dragstarted = (event: d3.D3DragEvent<SVGGElement, GraphNode, GraphNode>, d: GraphNode) => {
    if (!event.active && simulation) simulation.alphaTarget(0.3).restart();
    d.fx = d.x;
    d.fy = d.y;
  };

  const dragged = (event: d3.D3DragEvent<SVGGElement, GraphNode, GraphNode>, d: GraphNode) => {
    d.fx = event.x;
    d.fy = event.y;
  };

  const dragended = (event: d3.D3DragEvent<SVGGElement, GraphNode, GraphNode>, d: GraphNode) => {
    if (!event.active && simulation) simulation.alphaTarget(0);
    d.fx = undefined;
    d.fy = undefined;
  };

  onCleanup(() => {
    simulation?.stop();
  });

  return (
    <div style="width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
      <svg ref={svgRef} style="max-width: 100%; max-height: 100%;" />
    </div>
  );
}
