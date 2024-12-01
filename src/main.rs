use fltk::app::App;
use fltk::prelude::WidgetBase;
use fltk::window::Window;
use crate::frontend::Browser;
use crate::server::{URI};

mod frontend;
mod server;
mod input;

fn main() {
    let url = "https://localhost:8080/".to_string();
    let mut browser = Browser{
        app: App::default(),
        window: Window::new(900, 450, 800, 600, "RustNet"),
    };

    let mut uri = URI {
        host: "".to_string(),
        path: "".to_string(),
        scheme: "".to_string(),
    };

    uri.parse_uri(url);
    uri.init_server();
    browser.init_browser();
}

