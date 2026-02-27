use log::info;

#[derive(Debug, Clone)]
pub enum DisplayServer { Wayland, X11, Unknown }

#[derive(Debug, Clone)]
pub enum AppContext { Banking, Browser, Terminal, Game, VideoEditor, TextEditor, Other(String) }

pub struct ContextDetector {
    pub display_server: DisplayServer,
    pub app_context: AppContext,
    pub multiplier: f64,
}

impl ContextDetector {
    pub fn detect() -> Self {
        let display_server = if std::env::var("WAYLAND_DISPLAY").is_ok() {
            DisplayServer::Wayland
        } else if std::env::var("DISPLAY").is_ok() {
            DisplayServer::X11
        } else {
            DisplayServer::Unknown
        };
        let app_context = Self::detect_context();
        let multiplier = Self::context_multiplier(&app_context);
        let ds_str = match &display_server {
            DisplayServer::Wayland => "Wayland",
            DisplayServer::X11 => "X11",
            DisplayServer::Unknown => "Unknown",
        };
        let ctx_str = match &app_context {
            AppContext::Browser => "Browser",
            AppContext::Terminal => "Terminal",
            AppContext::Game => "Game",
            _ => "Other",
        };
        info!("Display: {} | Context: {} | Multiplier: {}x", ds_str, ctx_str, multiplier);
        Self { display_server, app_context, multiplier }
    }

    fn detect_context() -> AppContext {
        if let Ok(term) = std::env::var("TERM") {
            if !term.is_empty() { return AppContext::Terminal; }
        }
        AppContext::Browser
    }

    fn context_multiplier(ctx: &AppContext) -> f64 {
        match ctx {
            AppContext::Banking => 0.3,
            AppContext::Game => 1.4,
            AppContext::Terminal => 0.8,
            AppContext::Browser => 1.0,
            _ => 1.0,
        }
    }

    pub fn multiplier(&self) -> f64 { self.multiplier }
    pub fn display_server(&self) -> &DisplayServer { &self.display_server }
}