use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Supported chart types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Scatter,
    Timeline,
    Network,
    Heatmap,
    Histogram,
    Area,
    Donut,
    Radar,
    Treemap,
}

impl std::fmt::Display for ChartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartType::Bar => write!(f, "bar"),
            ChartType::Line => write!(f, "line"),
            ChartType::Pie => write!(f, "pie"),
            ChartType::Scatter => write!(f, "scatter"),
            ChartType::Timeline => write!(f, "timeline"),
            ChartType::Network => write!(f, "network"),
            ChartType::Heatmap => write!(f, "heatmap"),
            ChartType::Histogram => write!(f, "histogram"),
            ChartType::Area => write!(f, "area"),
            ChartType::Donut => write!(f, "donut"),
            ChartType::Radar => write!(f, "radar"),
            ChartType::Treemap => write!(f, "treemap"),
        }
    }
}

/// Chart configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub styling: ChartStyling,
    pub axes: AxesConfig,
    pub legend: LegendConfig,
    pub animation: AnimationConfig,
    pub interactive: bool,
    pub responsive: bool,
}

impl ChartConfig {
    /// Create default configuration for a chart type
    pub fn default_for_type(chart_type: ChartType) -> Self {
        let title = match chart_type {
            ChartType::Bar => "Step Duration Analysis",
            ChartType::Line => "Progress Over Time",
            ChartType::Pie => "Status Distribution",
            ChartType::Scatter => "Data Correlation",
            ChartType::Timeline => "Workflow Timeline",
            ChartType::Network => "Source Relationships",
            ChartType::Heatmap => "Activity Heatmap",
            ChartType::Histogram => "Data Distribution",
            ChartType::Area => "Cumulative Progress",
            ChartType::Donut => "Category Breakdown",
            ChartType::Radar => "Multi-dimensional Analysis",
            ChartType::Treemap => "Hierarchical Data",
        };

        Self {
            title: title.to_string(),
            width: 800,
            height: 600,
            styling: ChartStyling::default(),
            axes: AxesConfig::default(),
            legend: LegendConfig::default(),
            animation: AnimationConfig::default(),
            interactive: true,
            responsive: true,
        }
    }
}

/// Chart styling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartStyling {
    pub color_scheme: ColorScheme,
    pub background_color: String,
    pub border_color: String,
    pub border_width: u32,
    pub font_family: String,
    pub font_size: u32,
    pub custom_colors: Vec<String>,
    pub theme: String,
}

impl Default for ChartStyling {
    fn default() -> Self {
        Self {
            color_scheme: ColorScheme::Professional,
            background_color: "#ffffff".to_string(),
            border_color: "#e0e0e0".to_string(),
            border_width: 1,
            font_family: "Arial, sans-serif".to_string(),
            font_size: 12,
            custom_colors: vec![
                "#3498db".to_string(), "#e74c3c".to_string(), "#2ecc71".to_string(),
                "#f39c12".to_string(), "#9b59b6".to_string(), "#1abc9c".to_string(),
            ],
            theme: "light".to_string(),
        }
    }
}

/// Color schemes for charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorScheme {
    Professional,
    Vibrant,
    Pastel,
    Monochrome,
    Custom,
}

/// Axes configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxesConfig {
    pub x_axis: AxisConfig,
    pub y_axis: AxisConfig,
    pub show_grid: bool,
    pub grid_color: String,
}

impl Default for AxesConfig {
    fn default() -> Self {
        Self {
            x_axis: AxisConfig::default(),
            y_axis: AxisConfig::default(),
            show_grid: true,
            grid_color: "#f0f0f0".to_string(),
        }
    }
}

/// Individual axis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxisConfig {
    pub label: String,
    pub show_labels: bool,
    pub show_ticks: bool,
    pub tick_count: Option<u32>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub scale_type: ScaleType,
}

impl Default for AxisConfig {
    fn default() -> Self {
        Self {
            label: String::new(),
            show_labels: true,
            show_ticks: true,
            tick_count: None,
            min_value: None,
            max_value: None,
            scale_type: ScaleType::Linear,
        }
    }
}

/// Scale types for axes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScaleType {
    Linear,
    Logarithmic,
    Time,
    Category,
}

/// Legend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegendConfig {
    pub show: bool,
    pub position: LegendPosition,
    pub orientation: LegendOrientation,
    pub font_size: u32,
}

impl Default for LegendConfig {
    fn default() -> Self {
        Self {
            show: true,
            position: LegendPosition::TopRight,
            orientation: LegendOrientation::Vertical,
            font_size: 10,
        }
    }
}

/// Legend positions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}

/// Legend orientations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendOrientation {
    Horizontal,
    Vertical,
}

/// Animation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    pub enabled: bool,
    pub duration_ms: u32,
    pub easing: EasingType,
    pub delay_ms: u32,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            duration_ms: 1000,
            easing: EasingType::EaseInOut,
            delay_ms: 0,
        }
    }
}

/// Animation easing types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EasingType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
}

/// Chart data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<Dataset>,
    pub metadata: ChartMetadata,
}

/// Dataset for chart data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    pub label: String,
    pub data: Vec<DataPoint>,
    pub color: String,
    pub border_color: Option<String>,
    pub fill: bool,
    pub line_style: LineStyle,
}

/// Data point for charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub x: DataValue,
    pub y: DataValue,
    pub label: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

/// Data value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataValue {
    Number(f64),
    String(String),
    DateTime(DateTime<Utc>),
    Boolean(bool),
}

/// Line styles for line charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineStyle {
    Solid,
    Dashed,
    Dotted,
    DashDot,
}

/// Chart metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartMetadata {
    pub workflow_id: Uuid,
    pub chart_type: ChartType,
    pub generated_at: DateTime<Utc>,
    pub data_source: String,
    pub total_data_points: usize,
    pub custom_fields: HashMap<String, String>,
}

/// Chart generation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartResult {
    pub id: Uuid,
    pub chart_type: ChartType,
    pub content: String,
    pub format: super::ChartOutputFormat,
    pub metadata: ChartMetadata,
    pub file_size_bytes: u64,
    pub generation_time_ms: u64,
    pub created_at: DateTime<Utc>,
}

/// Chart template for reusable configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartTemplate {
    pub id: String,
    pub name: String,
    pub chart_type: ChartType,
    pub config: ChartConfig,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Chart export options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartExportOptions {
    pub format: super::ChartOutputFormat,
    pub quality: u32, // 1-100 for raster formats
    pub dpi: u32,     // For print formats
    pub transparent_background: bool,
    pub include_metadata: bool,
}
