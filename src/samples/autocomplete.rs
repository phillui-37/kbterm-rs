use cursive::{
    align::HAlign,
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, EditView, LinearLayout, SelectView, TextView},
    Cursive,
};
use lazy_static::lazy_static;

use crate::traits::Sample;

lazy_static! {
    static ref CITIES: &'static str = include_str!("assets/cities.txt");
}

pub struct AutoComplete;

impl Sample for AutoComplete {
    fn run(&self, siv: &mut Cursive) {
        let ev = EditView::new()
            .on_edit(AutoComplete::on_edit)
            .on_submit(AutoComplete::on_submit)
            .with_name("query");
        let sv = SelectView::new()
            .with_all_str(CITIES.lines())
            .on_submit(AutoComplete::show_next_window)
            .h_align(HAlign::Center)
            .with_name("matches")
            .scrollable();
        let dialog = Dialog::around(
            LinearLayout::vertical()
                .child(ev)
                .child(sv)
                .fixed_height(10),
        )
        .button("Quit", Cursive::quit)
        .title("Where are you from?");
        siv.add_layer(dialog);
    }
}

impl AutoComplete {
    fn on_edit(siv: &mut Cursive, query: &str, _cursor: usize) {
        let matches = AutoComplete::search_fn(CITIES.lines(), query);
        siv.call_on_name("matches", |v: &mut SelectView| {
            v.clear();
            v.add_all_str(matches);
        });
    }

    fn search_fn<'a, 'b, T: IntoIterator<Item = &'a str>>(
        items: T,
        query: &'b str,
    ) -> Vec<&'a str> {
        items
            .into_iter()
            .filter(|&item| {
                let item = item.to_lowercase();
                let query = query.to_lowercase();
                item.contains(&query)
            })
            .collect()
    }

    fn on_submit(siv: &mut Cursive, query: &str) {
        let matches = siv.find_name::<SelectView>("matches").unwrap();
        if matches.is_empty() {
            AutoComplete::show_next_window(siv, query);
        } else {
            let city = &*matches.selection().unwrap();
            AutoComplete::show_next_window(siv, city);
        }
    }

    fn show_next_window(siv: &mut Cursive, city: &str) {
        siv.pop_layer();
        let text = format!("{city} is a great city!");
        siv.add_layer(Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()));
    }
}
