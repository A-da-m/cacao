//! This example showcases setting up a basic application and window delegate.
//! Window Delegate's give you lifecycle methods that you can respond to.

use cacao::appkit::menu::{Menu, MenuItem};
use cacao::appkit::window::{Window, WindowConfig, WindowDelegate};
use cacao::appkit::{App, AppDelegate};

struct BasicApp {
    window: Window<MyWindow>,
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        App::set_menu(vec![
            Menu::new(
                "",
                vec![
                    MenuItem::Services,
                    MenuItem::Separator,
                    MenuItem::Hide,
                    MenuItem::HideOthers,
                    MenuItem::ShowAll,
                    MenuItem::Separator,
                    MenuItem::Quit,
                ],
            ),
            Menu::new("File", vec![MenuItem::CloseWindow]),
            Menu::new("View", vec![MenuItem::EnterFullScreen]),
            Menu::new(
                "Window",
                vec![
                    MenuItem::Minimize,
                    MenuItem::Zoom,
                    MenuItem::Separator,
                    MenuItem::new("Bring All to Front"),
                ],
            ),
        ]);

        App::activate();

        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

#[derive(Default)]
struct MyWindow;

impl WindowDelegate for MyWindow {
    const NAME: &'static str = "MyWindow";

    fn did_load(&mut self, window: Window) {
        window.set_minimum_content_size(400., 400.);
        window.set_title("A Basic Window!?");
    }

    fn will_close(&self) {
        println!("Closing now!");
    }

    fn will_move(&self) {
        println!("Will move...");
    }

    fn did_move(&self) {
        println!("Did move...");
    }

    fn will_resize(&self, width: f64, height: f64) -> (f64, f64) {
        println!("Resizing to: {} {}", width, height);
        (width, height)
    }
}

fn main() {
    App::new(
        "com.test.window-delegate",
        BasicApp {
            window: Window::with(WindowConfig::default(), MyWindow::default()),
        },
    )
    .run();
}
