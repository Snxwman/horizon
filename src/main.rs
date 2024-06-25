#![allow(unused_variables)]
#![allow(dead_code)]

mod state;
mod widgets;
mod x;

mod horizon;
mod util;

use std::rc::Rc;

use gdk_x11::gdk::Display;
use glib::clone;
use gtk::prelude::*;
use gtk::{Application, CssProvider, glib};

use x::x::{XSessionContext, XWindowContext};

const APP_ID: &str = "dev.snxwman.horizon";

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("../styles/main.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."), 
        &provider, 
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}

fn app_main(app: &Application) {
    let x_session_context = Rc::new(XSessionContext::new());
    let horizon_windows = horizon::get_windows(app, x_session_context.clone());

    for window in horizon_windows {
        window.window.present();

        let x_window_context = XWindowContext::new(x_session_context.clone(), &window); 
        x_window_context.set_strut_partial_hint(x_session_context.clone());
        x_window_context.set_window_type_hint(x_session_context.clone());

        window.window.connect_unrealize(clone!(@strong x_session_context => move |_| {
            println!("unrealizing window");
            x_window_context.reset_strut_partial_hint(x_session_context.clone());
        })); 
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(app_main);

    app.run()
}
