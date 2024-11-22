use fltk::app;
use fltk::prelude::{GroupExt, WidgetExt};
use fltk::window::Window;

pub struct Browser {
    pub(crate) app: app::App,
    pub(crate) window: Window,
}

impl Browser {
    pub fn init_browser(&mut self) {
        // init_browser runs the window
        self.window.end();
        self.window.show();
        self.app.run().unwrap();
    }
}