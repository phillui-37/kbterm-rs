use cursive::{
    views::{Dialog, EditView},
    Cursive,
};

use crate::traits::Sample;

pub struct WindowTitle;

impl Sample for WindowTitle {
    fn run(&self, siv: &mut Cursive) {
        siv.add_layer(
            Dialog::new().title("Write yourself a new title!").content(
                EditView::new()
                    .on_edit(|s, content, _| {
                        s.set_window_title(content);
                    }))
                    .button("Quit", |s| s.quit()),
        );
    }
}
