use std::sync::RwLock;

use glib::{clone, ControlFlow};
use gtk::prelude::*;
use gtk::{Button, GestureClick};
use tokio::sync::watch::Receiver;

use crate::state::*;

pub struct Clock {
    gtk_widget: Button,
    receiver: Receiver<ChannelMessage>,
    left_click_action: Box<dyn Fn() -> ()>,
    middle_click_action: Box<dyn Fn() -> ()>,
    right_click_action: Box<dyn Fn() -> ()>,
}

impl Clock {
    pub fn new() -> Self {
        let receiver = DATETIME.read().unwrap().sender.subscribe();

        let button = Button::builder()
            .label(Clock::formatted_time())
            .build();

        let mut rx = receiver.clone();
        glib::spawn_future_local(clone!(@strong button => async move {
            while rx.changed().await.is_ok() {
                match *rx.borrow_and_update() {
                    ChannelMessage::Init => {},
                    ChannelMessage::Updated => {
                        button.set_label(&Clock::formatted_time());
                    },
                }
            }
        }));

        let click_gesture = GestureClick::new();
        click_gesture.set_button(0);  // Listen for all buttons

        click_gesture.connect_pressed(clone!(@weak click_gesture => move |_, _, _, _| {
            match click_gesture.current_button() {
                gtk::gdk::BUTTON_PRIMARY => println!("Left click"),   // Left click
                gtk::gdk::BUTTON_MIDDLE => println!("Middle click"), // Middle click
                gtk::gdk::BUTTON_SECONDARY => println!("Right click"),  // Right click
                _ => (),
            }
        }));

        button.add_controller(click_gesture);

        Self {
            gtk_widget: button,
            receiver,
            left_click_action: Box::new(|| println!("Left click")),
            middle_click_action: Box::new(|| println!("Middle click")),
            right_click_action: Box::new(|| println!("Right click")),
        }
    }

    pub fn set_left_click_action<F: Fn() -> () + 'static>(&mut self, action: F) {
        self.left_click_action = Box::new(action);
    }

    pub fn set_middle_click_action<F: Fn() -> () + 'static>(&mut self, action: F) {
        self.middle_click_action = Box::new(action);
    }

    pub fn set_right_click_action<F: Fn() -> () + 'static>(&mut self, action: F) {
        self.right_click_action = Box::new(action);
    }

    pub fn widget(self) -> Button {
        self.gtk_widget
    }

    fn formatted_time() -> String {
        let datetime = DATETIME.read().unwrap();
        format!(
            "{}, {} {} | {}:{:02}:{:02}",
            datetime.date.day.short_name,
            datetime.date.month.short_name,
            datetime.date.day.day_of_month,
            datetime.time.hour,
            datetime.time.minute,
            datetime.time.second,
        )
    }
}

// impl UsesHorizonData for Clock {
//     fn update(&self) {
//         self.gtk_widget.set_label(&self.data.read().unwrap().to_string());
//     }
// }