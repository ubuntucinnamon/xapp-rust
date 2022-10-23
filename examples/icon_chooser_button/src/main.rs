pub use xapp::gtk::*;
pub use xapp::gtk::prelude::*;
pub use xapp::*;

fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("Hello from Rust!");
    window.set_border_width(10);
    window.set_position(WindowPosition::Center);
    window.set_default_size(300, 300);

    let icon_chooser_button = IconChooserButton::new();

    window.add(&icon_chooser_button);

    window.show_all();
}

fn main() {
    let application =
        Application::new(Some("com.github.Ubuntu-Cinnamon-Remix.xapp.example"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}