use std::sync::RwLock;

use glib::clone;
use gtk::Label;

use crate::state::*;

pub struct Clock {
    gtk_widget: Label,
    data: &'static RwLock<HorizonDateTime>,
}

impl Clock {
    pub fn new() -> Self {
        let label = Label::builder()
            .label(Clock::formatted_time())
            .build();

        let tick = clone!(@strong label => move || {
            label.set_label(&Clock::formatted_time());
            glib::ControlFlow::Continue
        });

        glib::timeout_add_seconds_local(1, tick);

        Self {
            gtk_widget: label,
            data: &DATETIME,
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
