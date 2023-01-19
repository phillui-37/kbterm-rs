use crate::traits::Sample;
use cursive::{
    align::HAlign,
    event::{EventResult, Key},
    view::{Scrollable, scroll::Scroller},
    views::{Dialog, OnEventView, Panel, TextView},
    Cursive, With,
};

pub struct Lorem;

impl Sample for Lorem {
    fn run(&self, siv: &mut Cursive) {
        let content = include_str!("assets/lorem.txt");
        siv.add_global_callback('q', |s| s.quit());
        siv.add_fullscreen_layer(
            Dialog::around(Panel::new(
                TextView::new(content)
                    .scrollable()
                    .wrap_with(OnEventView::new)
                    .on_pre_event_inner(Key::PageUp, |v, _| {
                        let scroller = v.get_scroller_mut();
                        if scroller.can_scroll_up() {
                            scroller.scroll_up(scroller.last_outer_size().y.saturating_sub(1));
                        }
                        Some(EventResult::Consumed(None))
                      })
                    .on_pre_event_inner(Key::PageDown, |v, _| {
                        let scroller = v.get_scroller_mut();
                        if scroller.can_scroll_down() {
                            scroller.scroll_down(scroller.last_outer_size().y.saturating_sub(1));
                        }
                        Some(EventResult::Consumed(None))
                      }),
            ))
            .title("Unicode and wide-char support")
            .h_align(HAlign::Center)
            .button("Quit", |s| s.quit()),
        );
        siv.add_layer(Dialog::info("Try resizing terminal\n(Press 'q' to quit)"));
    }
}
