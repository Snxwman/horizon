use gtk::{Button, Widget};

pub struct Icon {
    gtk_widget: Widget,
}

impl Icon {
    pub fn new(icon: &str) -> Self {
        let icon = Button::builder()
            .label(icon.to_owned())
            .build();

        Self {
            gtk_widget: icon.into(),
        }
    }

    pub fn widget(self) -> Widget {
        self.gtk_widget
    }

    // pub fn as_button(self) -> Option<Button> {
    //     self.gtk_widget.downcast::<Button>()
    // }
}
