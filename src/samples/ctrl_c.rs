use cursive::{event::Event, views::Dialog, Cursive};

use crate::traits::Sample;

pub struct CtrlC;

impl Sample for CtrlC {
    fn run(&self, siv: &mut Cursive) {
        siv.clear_global_callbacks(Event::CtrlChar('c'));
        siv.set_on_pre_event(Event::CtrlChar('c'), |s| {
            s.add_layer(
                Dialog::text("Do you want to quit?")
                    .button("Yes", Cursive::quit)
                    .button("No", |s| {
                        s.pop_layer();
                    }),
            )
        });
        siv.add_layer(Dialog::text("Try pressing Ctrl-C!"));
    }
}
