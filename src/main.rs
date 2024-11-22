use fltk::app;
use fltk::window::Window;
use crate::frontend::Browser;
use crate::server::init_server;

mod frontend;
mod server;

fn main() {
    init_server();
    let mut browser = Browser{
        app: app::App::default(),
        window: Window::default(),
    };

    // browser.init_browser();
}

