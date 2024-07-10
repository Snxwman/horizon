#![allow(unused_variables)]
#![allow(dead_code)]

mod state;
mod widgets;
mod x;

mod event;
mod horizon;
mod util;

use std::rc::Rc;
use std::time::Duration;

use gdk_x11::gdk::Display;
use glib::clone;
use gtk::prelude::*;
use gtk::{Application, CssProvider, glib};
use tokio::task;
use tokio::time;

use event::Event;
use x::x::{XSessionContext, XWindowContext};

use event::EVENT_MANAGER;
use state::time::DATETIME;

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

async fn tokio_main() {
    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;
            DATETIME.write().unwrap().update();
            // EVENT_MANAGER.notify_listeners(Event::HorizonDateTimeUpdated);
            // print!("{:#?}", DATETIME.read().unwrap());
        }
    }).await;
}

fn gtk_main(app: &Application) {
    // Get the xsession and config for the whole application.
    // These should be constant for the whole application duration.
    let x_session_context = Rc::new(XSessionContext::new());
    let horizon_windows = horizon::get_windows(app, x_session_context.clone());

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    std::thread::spawn(move || {
        rt.block_on(tokio_main());
    });

    // Build up the window definitions from the user config.
    for window in horizon_windows {
        window.window.present();

        let x_window_context = XWindowContext::new(x_session_context.clone(), &window);
        x_window_context.set_window_type_hint(x_session_context.clone());
        x_window_context.set_strut_partial_hint(x_session_context.clone());

        window.window.connect_unrealize(clone!(@strong x_session_context => move |_| {
            x_window_context.reset_strut_partial_hint(x_session_context.clone());
        }));
    }
}

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(gtk_main);

    app.run();
}

