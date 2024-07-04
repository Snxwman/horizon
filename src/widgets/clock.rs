use std::sync::RwLock;

use glib::clone;
use gtk::Label;

use crate::state::time::LocalDateTime;
use crate::state::time::DATETIME;


pub struct Clock {
    gtk_widget: Label,
    data: &'static RwLock<LocalDateTime>,
}

impl Clock {
    pub fn new() -> Clock {
        let time = "test";
        let label = Label::builder()
            .label(time)
            .build();

        let tick = clone!(@strong label => move || {
            let time = DATETIME.read().unwrap().0;
            label.set_label(&time.to_string());
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
}

// impl UsesHorizonData for Clock {
//     fn update(&self) {
//         self.gtk_widget.set_label(&self.data.read().unwrap().to_string());
//     }
// }
