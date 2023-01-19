mod samples;
mod traits;

use std::collections::BTreeMap;

use cursive::traits::*;
use cursive::views::Dialog;
use cursive::{views::SelectView, Cursive};
use samples::*;
use traits::*;

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let select = SelectView::<String>::new()
        .on_submit(|s: &mut Cursive, name: &str| {
            s.clear_global_callbacks('q');
            s.pop_layer();
            if let Some(sample) = get_samples().get(name) {
                sample.run(s);
            } else {
                println!("{} not found!", name);
            }
        })
        .with_name("select")
        .scrollable();

    siv.add_layer(Dialog::around(select));

    get_samples().iter().for_each(|(name, _)| {
        siv.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name.to_string())
        });
    });

    siv.run();
}

fn get_samples() -> BTreeMap<&'static str, Box<dyn Sample>> {
    let mut tmp_list: [(&'static str, Box<dyn Sample>); 11] = [
        ("Sample 1", Box::new(Test1)),
        ("Sample 2", Box::new(Test2)),
        ("Sample 3", Box::new(Test3)),
        ("Lorem", Box::new(Lorem)),
        ("Window Title", Box::new(WindowTitle)),
        ("Advance User Data", Box::new(AdvancedUserData)),
        ("Ansi", Box::new(Ansi)),
        ("Auto Complete", Box::new(AutoComplete)),
        // ("Builder", Box::new(Builder())), //raw_recipe macro not functions normally. Please ignore this first.
        ("Colors", Box::new(Colors)),
        ("Ctrl-C", Box::new(CtrlC)),
        ("Debug Console", Box::new(DebugConsole)),
    ];
    tmp_list.sort_by(|a, b| a.0.cmp(b.0));

    let mut ret = BTreeMap::<&'static str, Box<dyn Sample>>::new();
    for ele in tmp_list {
        ret.insert(ele.0, ele.1);
    }
    ret
}
