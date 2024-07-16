use std::sync::RwLock;

use glib::{clone, ControlFlow};
use gtk::prelude::*;
use gtk::{Button, Label};
use tokio::sync::watch::Receiver;

use crate::state::*;

pub struct Clock {
    gtk_widget: Label,
    receiver: Receiver<ChannelMessage>,
}

impl Clock {
    pub fn new() -> Self {
        let receiver = DATETIME.read().unwrap().sender.subscribe();

        let label = Label::builder()
            .label(Clock::formatted_time())
            .build();

        let mut rx = receiver.clone();
        glib::spawn_future_local(clone!(@strong label => async move {
            while rx.changed().await.is_ok() {
                match *rx.borrow_and_update() {
                    ChannelMessage::Init => {},
                    ChannelMessage::Updated => {
                        label.set_label(&Clock::formatted_time());
                    },
                }
            }
        }));

        Self {
            gtk_widget: label,
            receiver,
        }
    }

    pub fn widget(self) -> Label {
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
