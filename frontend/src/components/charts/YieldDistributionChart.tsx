import { useEffect, useRef } from 'react';
import * as d3 from 'd3';
import { useQuery } from '@tanstack/react-query';
import { api } from '../../services/api';

interface YieldDistributionChartProps {
  period: string;
}

export const YieldDistributionChart = ({ period }: YieldDistributionChartProps) => {
  const svgRef = useRef<SVGSVGElement>(null);

  const { data } = useQuery({
    queryKey: ['yield-distribution', period],
    queryFn: () => api.getYieldDistribution(period),
  });

  useEffect(() => {
    if (!data || !svgRef.current) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll('*').remove();

    const margin = { top: 40, right: 120, bottom: 40, left: 60 };
    const width = svgRef.current.clientWidth - margin.left - margin.right;
    const height = 400 - margin.top - margin.bottom;

    const g = svg
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);

    // Stack data
    const keys = Object.keys(data[0]).filter((k) => k !== 'date');
    const stack = d3.stack().keys(keys);
    const series = stack(data);

    // Scales
    const x = d3
      .scaleTime()
      .domain(d3.extent(data, (d: any) => new Date(d.date)) as [Date, Date])
      .range([0, width]);

    const y = d3
      .scaleLinear()
      .domain([0, d3.max(series, (d) => d3.max(d, (d) => d[1])) as number])
      .nice()
      .range([height, 0]);

    const color = d3
      .scaleOrdinal()
      .domain(keys)
      .range(['#0ea5e9', '#d946ef', '#10b981', '#f59e0b']);

    // Area generator
    const area = d3
      .area<any>()
      .x((d) => x(new Date(d.data.date)))
      .y0((d) => y(d[0]))
      .y1((d) => y(d[1]))
      .curve(d3.curveMonotoneX);

    // Draw areas
    g.selectAll('.layer')
      .data(series)
      .join('path')
      .attr('class', 'layer')
      .attr('d', area)
      .attr('fill', (d) => color(d.key) as string)
      .attr('opacity', 0.8)
      .on('mouseover', function () {
        d3.select(this).attr('opacity', 1);
      })
      .on('mouseout', function () {
        d3.select(this).attr('opacity', 0.8);
      });

    // Axes
    g.append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(x).ticks(6))
      .call((g) => g.select('.domain').remove())
      .call((g) => g.selectAll('.tick text').attr('fill', '#6b7280'));

    g.append('g')
      .call(d3.axisLeft(y).ticks(5).tickFormat((d) => `${(d as number / 1000000).toFixed(1)}M`))
      .call((g) => g.select('.domain').remove())
      .call((g) =>
        g
          .selectAll('.tick line')
          .attr('stroke', '#e5e7eb')
          .attr('stroke-dasharray', '2,2')
          .attr('x2', width)
      )
      .call((g) => g.selectAll('.tick text').attr('fill', '#6b7280'));

    // Legend
    const legend = svg
      .append('g')
      .attr('transform', `translate(${width + margin.left + 20}, ${margin.top})`);

    keys.forEach((key, i) => {
      const legendRow = legend
        .append('g')
        .attr('transform', `translate(0, ${i * 25})`);

      legendRow
        .append('rect')
        .attr('width', 15)
        .attr('height', 15)
        .attr('fill', color(key) as string)
        .attr('rx', 3);

      legendRow
        .append('text')
        .attr('x', 20)
        .attr('y', 12)
        .attr('fill', '#374151')
        .attr('font-size', '13px')
        .text(key);
    });
  }, [data]);

  if (!data) {
    return (
      <div className="w-full h-[400px] flex items-center justify-center">
        <div className="text-gray-400">Loading...</div>
      </div>
    );
  }

  return (
    <div className="w-full h-[400px]">
      <svg ref={svgRef} className="w-full h-full" />
    </div>
  );
};
