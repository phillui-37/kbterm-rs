mod samples;
mod traits;

use std::collections::{BTreeMap};

use cursive::traits::*;
use cursive::views::{Dialog};
use cursive::{views::SelectView, Cursive};
use samples::*;
use traits::*;

fn main() {
    let mut siv = cursive::default();

    let select = SelectView::<String>::new()
        .on_submit(|s: &mut Cursive, name: &str| {
            s.pop_layer();
            if let Some(sample) = get_samples().get(name) {
                sample.run(s);
            }
        })
        .with_name("select");

    siv.add_layer(
        Dialog::around(select)
    );

    get_samples().iter().for_each(|(name, _)| {
        siv.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name.to_string())
        });
    });

    siv.run();
}

fn get_samples() -> BTreeMap<&'static str, Box<dyn Sample>> {
    let mut ret = BTreeMap::<&'static str, Box<dyn Sample>>::new();
    ret.insert("Sample 1", Box::new(Test1()));
    ret.insert("Sample 2", Box::new(Test2()));
    ret.insert("Sample 3", Box::new(Test3()));
    ret.insert("Sample 3_1", Box::new(Test3()));
    ret.insert("Sample 3_2", Box::new(Test3()));
    ret.insert("Sample 3_3", Box::new(Test3()));
    ret.insert("Sample 3_4", Box::new(Test3()));
    ret.insert("Sample 3_5", Box::new(Test3()));
    ret.insert("Sample 3_6", Box::new(Test3()));
    ret.insert("Sample 3_7", Box::new(Test3()));
    ret.insert("Sample 3_8", Box::new(Test3()));
    ret.insert("Sample 3_9", Box::new(Test3()));
    ret
}
