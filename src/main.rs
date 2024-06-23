#![allow(unused_variables)]

mod state;
mod widgets;

mod config;
mod ewmh;
mod horizon;
mod util;
mod x;

use std::rc::Rc;

use gtk::prelude::*;
use gtk::glib;
use gtk::{Application, ApplicationWindow};
use glib::clone;
use horizon::HorizonXSurfaceContext;

use ewmh::StrutPartialDef;
use util::Side;

const APP_ID: &str = "dev.snxwman.horizon";

fn build_ui(app: &Application) {

    let app_window = ApplicationWindow::builder()
        .application(app)
        .default_width(5120)
        .default_height(20)
        .resizable(false)
        .build();

    app_window.present();

    let strut = StrutPartialDef::builder()
        .full_length(Side::Top, 20)
        .build();

    let x_context = Rc::new(HorizonXSurfaceContext::new(&app_window, Some(strut)));

    x_context.set_strut_partial();
    x_context.set_window_type();

    app_window.connect_unrealize(clone!(@strong x_context => move |_| {
        x_context.reset_strut_partial();
    })); 
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run()
}
