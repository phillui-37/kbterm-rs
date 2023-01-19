use crate::traits::Sample;
use cursive::views::TextView;
use cursive::Cursive;

pub struct Test1;

impl Sample for Test1 {
    fn run(&self, siv: &mut Cursive) {
        siv.add_global_callback('q', |s| s.quit());
        siv.add_layer(TextView::new("Hello cursive! Press <q> to quit."));
    }
}
