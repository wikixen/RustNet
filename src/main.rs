use fltk::app::App;
use fltk::prelude::WidgetBase;
use fltk::window::Window;
use crate::frontend::Browser;
use crate::server::init_server;

mod frontend;
mod server;
mod dom_tree;
mod parse;

fn main() {

    let mut browser = Browser{
        app: App::default(),
        window: Window::new(900, 450, 800, 600, "RustNet"),
    };

    browser.init_browser();
    init_server();
}

