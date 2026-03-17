//! GUI module - egui/Tauri bindings

/// GUI configuration
#[derive(Debug, Clone)]
pub struct GuiConfig {
    /// Window title
    pub title: String,
    /// Window width
    pub width: f32,
    /// Window height
    pub height: f32,
    /// Enable Chinese font
    pub chinese_font: bool,
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            title: "rsklls Application".to_string(),
            width: 800.0,
            height: 600.0,
            chinese_font: true,
        }
    }
}

impl GuiConfig {
    /// Set window title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    
    /// Set window size
    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    
    /// Enable/disable Chinese font
    pub fn with_chinese_font(mut self, enabled: bool) -> Self {
        self.chinese_font = enabled;
        self
    }
}
