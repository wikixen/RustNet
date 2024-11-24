// Will implement the frontend in the future
use fltk::app;
use fltk::prelude::{GroupExt, WidgetExt, };
use fltk::window::Window;

pub struct Browser {
    pub(crate) app: app::App,
    pub(crate) window: Window,
}

impl Browser {
    pub fn init_browser(&mut self) {
        // init_browser opens the window

        // Window options
        self.window.make_resizable(true);

        // Code to run window; Don't touch
        self.window.end();
        self.window.show();
        self.app.run().unwrap();
    }
}