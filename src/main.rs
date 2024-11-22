use crate::server::init_server;

mod server;

fn main() {
    init_server();
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}