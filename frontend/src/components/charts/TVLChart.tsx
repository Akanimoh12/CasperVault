import { useEffect, useRef } from 'react';
import * as d3 from 'd3';
import { useQuery } from '@tanstack/react-query';
import { api } from '@/services/api';

interface TVLChartProps {
  period?: string;
}

export const TVLChart = ({ period = '30d' }: TVLChartProps) => {
  const svgRef = useRef<SVGSVGElement>(null);
  
  const { data: history } = useQuery({
    queryKey: ['tvl-history', period],
    queryFn: () => api.getTVLHistory(period),
  });
  
  useEffect(() => {
    if (!history || !svgRef.current) return;
    
    const svg = d3.select(svgRef.current);
    svg.selectAll('*').remove(); // Clear previous render
    
    const margin = { top: 20, right: 30, bottom: 30, left: 60 };
    const width = svgRef.current.clientWidth - margin.left - margin.right;
    const height = 300 - margin.top - margin.bottom;
    
    const g = svg
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);
    
    // Parse data
    const data = (history as any[]).map((d: any) => ({
      date: new Date(d.timestamp),
      value: parseFloat(d.total_assets),
    }));
    
    // Scales
    const x = d3
      .scaleTime()
      .domain(d3.extent(data, (d) => d.date) as [Date, Date])
      .range([0, width]);
    
    const y = d3
      .scaleLinear()
      .domain([0, d3.max(data, (d) => d.value) as number])
      .nice()
      .range([height, 0]);
    
    // Area generator
    const area = d3
      .area<any>()
      .x((d) => x(d.date))
      .y0(height)
      .y1((d) => y(d.value))
      .curve(d3.curveMonotoneX);
    
    // Line generator
    const line = d3
      .line<any>()
      .x((d) => x(d.date))
      .y((d) => y(d.value))
      .curve(d3.curveMonotoneX);
    
    // Gradient
    const gradient = svg
      .append('defs')
      .append('linearGradient')
      .attr('id', 'tvl-gradient')
      .attr('gradientUnits', 'userSpaceOnUse')
      .attr('x1', 0)
      .attr('y1', 0)
      .attr('x2', 0)
      .attr('y2', height);
    
    gradient
      .append('stop')
      .attr('offset', '0%')
      .attr('stop-color', '#0ea5e9')
      .attr('stop-opacity', 0.3);
    
    gradient
      .append('stop')
      .attr('offset', '100%')
      .attr('stop-color', '#0ea5e9')
      .attr('stop-opacity', 0);
    
    // Draw area
    g.append('path')
      .datum(data)
      .attr('fill', 'url(#tvl-gradient)')
      .attr('d', area);
    
    // Draw line
    g.append('path')
      .datum(data)
      .attr('fill', 'none')
      .attr('stroke', '#0ea5e9')
      .attr('stroke-width', 2)
      .attr('d', line);
    
    // Axes
    g.append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(x).ticks(6))
      .call((g) => g.select('.domain').remove())
      .call((g) => g.selectAll('.tick line').remove())
      .call((g) => g.selectAll('.tick text').attr('fill', '#6b7280'));
    
    g.append('g')
      .call(
        d3
          .axisLeft(y)
          .ticks(5)
          .tickFormat((d) => `${d3.format('.2s')(d as any)} CSPR`)
      )
      .call((g) => g.select('.domain').remove())
      .call((g) =>
        g
          .selectAll('.tick line')
          .attr('stroke', '#e5e7eb')
          .attr('stroke-dasharray', '2,2')
          .attr('x2', width)
      )
      .call((g) => g.selectAll('.tick text').attr('fill', '#6b7280'));
    
  }, [history]);
  
  return (
    <div className="w-full h-[300px]">
      <svg ref={svgRef} className="w-full h-full" />
    </div>
  );
};
