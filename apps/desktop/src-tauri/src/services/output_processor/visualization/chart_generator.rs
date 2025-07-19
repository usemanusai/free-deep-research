use async_trait::async_trait;
use std::collections::HashMap;
use tracing::{info, debug, error};
use chrono::Utc;
use uuid::Uuid;

use crate::error::{AppResult, ResearchError};
use super::chart_types::{
    ChartType, ChartConfig, ChartData, ChartResult, Dataset, DataPoint, DataValue,
    ChartMetadata, ColorScheme
};
use super::ChartOutputFormat;

/// Trait for chart generators
#[async_trait]
pub trait ChartGenerator: Send + Sync {
    /// Generate a chart from data and configuration
    async fn generate_chart(
        &self,
        data: ChartData,
        config: &ChartConfig,
    ) -> AppResult<ChartResult>;

    /// Get supported chart types for this generator
    fn supported_chart_types(&self) -> Vec<ChartType>;

    /// Get output format for this generator
    fn output_format(&self) -> ChartOutputFormat;
}

/// SVG chart generator
pub struct SVGChartGenerator;

impl SVGChartGenerator {
    pub fn new() -> Self {
        Self
    }

    fn generate_bar_chart_svg(&self, data: &ChartData, config: &ChartConfig) -> String {
        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
            <style>
                .chart-title {{ font-family: {}; font-size: {}px; text-anchor: middle; }}
                .axis-label {{ font-family: {}; font-size: {}px; }}
                .bar {{ stroke: {}; stroke-width: 1; }}
                .grid {{ stroke: {}; stroke-width: 0.5; opacity: 0.3; }}
            </style>
            <rect width="100%" height="100%" fill="{}"/>
            <text x="{}" y="30" class="chart-title">{}</text>"#,
            config.width,
            config.height,
            config.styling.font_family,
            config.styling.font_size + 4,
            config.styling.font_family,
            config.styling.font_size,
            config.styling.border_color,
            config.axes.grid_color,
            config.styling.background_color,
            config.width / 2,
            config.title
        );

        // Chart area dimensions
        let chart_left = 80;
        let chart_top = 60;
        let chart_width = config.width - 120;
        let chart_height = config.height - 120;

        // Draw grid lines if enabled
        if config.axes.show_grid {
            for i in 0..=10 {
                let y = chart_top + (chart_height * i / 10);
                svg.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" class="grid"/>"#,
                    chart_left, y, chart_left + chart_width, y
                ));
            }
        }

        // Draw bars
        if let Some(dataset) = data.datasets.first() {
            let bar_width = chart_width / dataset.data.len() as u32;
            let max_value = dataset.data.iter()
                .filter_map(|p| if let DataValue::Number(n) = p.y { Some(n) } else { None })
                .fold(0.0, |acc, x| acc.max(x));

            for (i, point) in dataset.data.iter().enumerate() {
                if let DataValue::Number(value) = point.y {
                    let bar_height = (value / max_value * chart_height as f64) as u32;
                    let x = chart_left + (i as u32 * bar_width) + bar_width / 4;
                    let y = chart_top + chart_height - bar_height;

                    svg.push_str(&format!(
                        r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" class="bar"/>"#,
                        x, y, bar_width / 2, bar_height, dataset.color
                    ));

                    // Add value label
                    svg.push_str(&format!(
                        r#"<text x="{}" y="{}" class="axis-label" text-anchor="middle">{:.1}</text>"#,
                        x + bar_width / 4, y - 5, value
                    ));
                }
            }

            // Add x-axis labels
            for (i, label) in data.labels.iter().enumerate() {
                let x = chart_left + (i as u32 * bar_width) + bar_width / 2;
                let y = chart_top + chart_height + 20;
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" class="axis-label" text-anchor="middle">{}</text>"#,
                    x, y, label
                ));
            }
        }

        // Draw axes
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2"/>"#,
            chart_left, chart_top + chart_height, chart_left + chart_width, chart_top + chart_height
        ));
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2"/>"#,
            chart_left, chart_top, chart_left, chart_top + chart_height
        ));

        svg.push_str("</svg>");
        svg
    }

    fn generate_pie_chart_svg(&self, data: &ChartData, config: &ChartConfig) -> String {
        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
            <style>
                .chart-title {{ font-family: {}; font-size: {}px; text-anchor: middle; }}
                .slice-label {{ font-family: {}; font-size: {}px; text-anchor: middle; }}
            </style>
            <rect width="100%" height="100%" fill="{}"/>
            <text x="{}" y="30" class="chart-title">{}</text>"#,
            config.width,
            config.height,
            config.styling.font_family,
            config.styling.font_size + 4,
            config.styling.font_family,
            config.styling.font_size,
            config.styling.background_color,
            config.width / 2,
            config.title
        );

        let center_x = config.width / 2;
        let center_y = config.height / 2;
        let radius = (config.width.min(config.height) / 3) as f64;

        if let Some(dataset) = data.datasets.first() {
            let total: f64 = dataset.data.iter()
                .filter_map(|p| if let DataValue::Number(n) = p.y { Some(n) } else { None })
                .sum();

            let mut current_angle = 0.0;
            let colors = &config.styling.custom_colors;

            for (i, point) in dataset.data.iter().enumerate() {
                if let DataValue::Number(value) = point.y {
                    let slice_angle = (value / total) * 2.0 * std::f64::consts::PI;
                    let end_angle = current_angle + slice_angle;

                    let x1 = center_x as f64 + radius * current_angle.cos();
                    let y1 = center_y as f64 + radius * current_angle.sin();
                    let x2 = center_x as f64 + radius * end_angle.cos();
                    let y2 = center_y as f64 + radius * end_angle.sin();

                    let large_arc = if slice_angle > std::f64::consts::PI { 1 } else { 0 };
                    let color = colors.get(i % colors.len()).unwrap_or(&"#3498db".to_string());

                    svg.push_str(&format!(
                        r#"<path d="M {} {} L {} {} A {} {} 0 {} 1 {} {} Z" fill="{}" stroke="white" stroke-width="2"/>"#,
                        center_x, center_y, x1, y1, radius, radius, large_arc, x2, y2, color
                    ));

                    // Add percentage label
                    let label_angle = current_angle + slice_angle / 2.0;
                    let label_x = center_x as f64 + (radius * 0.7) * label_angle.cos();
                    let label_y = center_y as f64 + (radius * 0.7) * label_angle.sin();
                    let percentage = (value / total * 100.0).round();

                    svg.push_str(&format!(
                        r#"<text x="{:.1}" y="{:.1}" class="slice-label" fill="white">{:.0}%</text>"#,
                        label_x, label_y, percentage
                    ));

                    current_angle = end_angle;
                }
            }
        }

        svg.push_str("</svg>");
        svg
    }

    fn generate_timeline_svg(&self, data: &ChartData, config: &ChartConfig) -> String {
        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
            <style>
                .chart-title {{ font-family: {}; font-size: {}px; text-anchor: middle; }}
                .timeline-item {{ font-family: {}; font-size: {}px; }}
                .timeline-line {{ stroke: #3498db; stroke-width: 3; }}
                .timeline-point {{ fill: #e74c3c; stroke: white; stroke-width: 2; }}
            </style>
            <rect width="100%" height="100%" fill="{}"/>
            <text x="{}" y="30" class="chart-title">{}</text>"#,
            config.width,
            config.height,
            config.styling.font_family,
            config.styling.font_size + 4,
            config.styling.font_family,
            config.styling.font_size,
            config.styling.background_color,
            config.width / 2,
            config.title
        );

        let timeline_left = 100;
        let timeline_width = config.width - 200;
        let item_height = 60;
        let start_y = 80;

        // Draw timeline line
        let line_x = timeline_left + 20;
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" class="timeline-line"/>"#,
            line_x, start_y, line_x, start_y + (data.labels.len() as u32 * item_height)
        ));

        // Draw timeline items
        for (i, label) in data.labels.iter().enumerate() {
            let y = start_y + (i as u32 * item_height);
            
            // Timeline point
            svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="8" class="timeline-point"/>"#,
                line_x, y
            ));

            // Item label
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" class="timeline-item">{}</text>"#,
                line_x + 30, y + 5, label
            ));

            // Add status or duration if available
            if let Some(dataset) = data.datasets.first() {
                if let Some(point) = dataset.data.get(i) {
                    if let DataValue::Number(duration) = point.y {
                        svg.push_str(&format!(
                            r#"<text x="{}" y="{}" class="timeline-item" fill="#666">{:.1}min</text>"#,
                            line_x + 30, y + 20, duration
                        ));
                    }
                }
            }
        }

        svg.push_str("</svg>");
        svg
    }
}

#[async_trait]
impl ChartGenerator for SVGChartGenerator {
    async fn generate_chart(
        &self,
        data: ChartData,
        config: &ChartConfig,
    ) -> AppResult<ChartResult> {
        debug!("Generating SVG chart: {:?}", data.metadata.chart_type);

        let start_time = std::time::Instant::now();

        let svg_content = match data.metadata.chart_type {
            ChartType::Bar => self.generate_bar_chart_svg(&data, config),
            ChartType::Pie => self.generate_pie_chart_svg(&data, config),
            ChartType::Timeline => self.generate_timeline_svg(&data, config),
            _ => {
                return Err(ResearchError::invalid_request(
                    format!("Chart type {:?} not yet implemented for SVG", data.metadata.chart_type)
                ).into());
            }
        };

        let generation_time = start_time.elapsed();

        Ok(ChartResult {
            id: Uuid::new_v4(),
            chart_type: data.metadata.chart_type,
            content: svg_content.clone(),
            format: ChartOutputFormat::SVG,
            metadata: data.metadata,
            file_size_bytes: svg_content.len() as u64,
            generation_time_ms: generation_time.as_millis() as u64,
            created_at: Utc::now(),
        })
    }

    fn supported_chart_types(&self) -> Vec<ChartType> {
        vec![ChartType::Bar, ChartType::Pie, ChartType::Timeline]
    }

    fn output_format(&self) -> ChartOutputFormat {
        ChartOutputFormat::SVG
    }
}

/// HTML chart generator (using Chart.js or similar)
pub struct HTMLChartGenerator;

impl HTMLChartGenerator {
    pub fn new() -> Self {
        Self
    }

    fn generate_html_chart(&self, data: &ChartData, config: &ChartConfig) -> String {
        let chart_id = format!("chart_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>{}</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {{ font-family: {}; margin: 20px; background-color: {}; }}
        .chart-container {{ width: {}px; height: {}px; margin: 0 auto; }}
        h1 {{ text-align: center; color: #333; }}
    </style>
</head>
<body>
    <h1>{}</h1>
    <div class="chart-container">
        <canvas id="{}"></canvas>
    </div>
    <script>
        const ctx = document.getElementById('{}').getContext('2d');
        const chart = new Chart(ctx, {{
            type: '{}',
            data: {{
                labels: {},
                datasets: [{}]
            }},
            options: {{
                responsive: {},
                plugins: {{
                    legend: {{
                        display: {},
                        position: 'top'
                    }}
                }},
                scales: {{
                    y: {{
                        beginAtZero: true
                    }}
                }}
            }}
        }});
    </script>
</body>
</html>"#,
            config.title,
            config.styling.font_family,
            config.styling.background_color,
            config.width,
            config.height,
            config.title,
            chart_id,
            chart_id,
            self.chart_type_to_chartjs(data.metadata.chart_type),
            serde_json::to_string(&data.labels).unwrap_or_else(|_| "[]".to_string()),
            self.datasets_to_chartjs(&data.datasets),
            config.responsive,
            config.legend.show
        )
    }

    fn chart_type_to_chartjs(&self, chart_type: ChartType) -> &'static str {
        match chart_type {
            ChartType::Bar => "bar",
            ChartType::Line => "line",
            ChartType::Pie => "pie",
            ChartType::Scatter => "scatter",
            ChartType::Area => "line",
            ChartType::Donut => "doughnut",
            _ => "bar", // Default fallback
        }
    }

    fn datasets_to_chartjs(&self, datasets: &[Dataset]) -> String {
        let chartjs_datasets: Vec<String> = datasets.iter().map(|dataset| {
            let data_values: Vec<f64> = dataset.data.iter()
                .filter_map(|p| if let DataValue::Number(n) = p.y { Some(n) } else { None })
                .collect();

            format!(
                r#"{{
                    label: '{}',
                    data: {},
                    backgroundColor: '{}',
                    borderColor: '{}',
                    borderWidth: 1
                }}"#,
                dataset.label,
                serde_json::to_string(&data_values).unwrap_or_else(|_| "[]".to_string()),
                dataset.color,
                dataset.border_color.as_deref().unwrap_or(&dataset.color)
            )
        }).collect();

        chartjs_datasets.join(",")
    }
}

#[async_trait]
impl ChartGenerator for HTMLChartGenerator {
    async fn generate_chart(
        &self,
        data: ChartData,
        config: &ChartConfig,
    ) -> AppResult<ChartResult> {
        debug!("Generating HTML chart: {:?}", data.metadata.chart_type);

        let start_time = std::time::Instant::now();
        let html_content = self.generate_html_chart(&data, config);
        let generation_time = start_time.elapsed();

        Ok(ChartResult {
            id: Uuid::new_v4(),
            chart_type: data.metadata.chart_type,
            content: html_content.clone(),
            format: ChartOutputFormat::HTML,
            metadata: data.metadata,
            file_size_bytes: html_content.len() as u64,
            generation_time_ms: generation_time.as_millis() as u64,
            created_at: Utc::now(),
        })
    }

    fn supported_chart_types(&self) -> Vec<ChartType> {
        vec![
            ChartType::Bar, ChartType::Line, ChartType::Pie, 
            ChartType::Scatter, ChartType::Area, ChartType::Donut
        ]
    }

    fn output_format(&self) -> ChartOutputFormat {
        ChartOutputFormat::HTML
    }
}
