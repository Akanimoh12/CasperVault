import { useEffect, useRef } from 'react';
import * as d3 from 'd3';
import { useQuery } from '@tanstack/react-query';
import { api } from '../../services/api';

interface UserGrowthChartProps {
  period: string;
}

export const UserGrowthChart = ({ period }: UserGrowthChartProps) => {
  const svgRef = useRef<SVGSVGElement>(null);

  const { data } = useQuery({
    queryKey: ['user-growth', period],
    queryFn: () => api.getUserGrowth(period),
  });

  useEffect(() => {
    if (!data || !svgRef.current) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll('*').remove();

    const margin = { top: 20, right: 30, bottom: 40, left: 60 };
    const width = svgRef.current.clientWidth - margin.left - margin.right;
    const height = 300 - margin.top - margin.bottom;

    const g = svg
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);

    // Parse data
    const parsedData = data.map((d: any) => ({
      date: new Date(d.date),
      users: d.users,
    }));

    // Scales
    const x = d3
      .scaleTime()
      .domain(d3.extent(parsedData, (d: any) => d.date) as [Date, Date])
      .range([0, width]);

    const y = d3
      .scaleLinear()
      .domain([0, d3.max(parsedData, (d: any) => d.users) as number])
      .nice()
      .range([height, 0]);

    // Bar generator
    const barWidth = width / parsedData.length - 2;

    g.selectAll('.bar')
      .data(parsedData)
      .join('rect')
      .attr('class', 'bar')
      .attr('x', (d: any) => x(d.date) - barWidth / 2)
      .attr('y', height)
      .attr('width', barWidth)
      .attr('height', 0)
      .attr('fill', '#0ea5e9')
      .attr('rx', 4)
      .transition()
      .duration(800)
      .attr('y', (d: any) => y(d.users))
      .attr('height', (d: any) => height - y(d.users));

    // Axes
    g.append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(x).ticks(6))
      .call((g) => g.select('.domain').remove())
      .call((g) => g.selectAll('.tick text').attr('fill', '#6b7280'));

    g.append('g')
      .call(d3.axisLeft(y).ticks(5))
      .call((g) => g.select('.domain').remove())
      .call((g) =>
        g
          .selectAll('.tick line')
          .attr('stroke', '#e5e7eb')
          .attr('stroke-dasharray', '2,2')
          .attr('x2', width)
      )
      .call((g) => g.selectAll('.tick text').attr('fill', '#6b7280'));
  }, [data]);

  if (!data) {
    return (
      <div className="w-full h-[300px] flex items-center justify-center">
        <div className="text-gray-400">Loading...</div>
      </div>
    );
  }

  return (
    <div className="w-full h-[300px]">
      <svg ref={svgRef} className="w-full h-full" />
    </div>
  );
};
