use cursive::{views::Dialog, Cursive};

use crate::traits::Sample;

pub struct Test2;

impl Sample for Test2 {
    fn run(&self, siv: &mut cursive::Cursive) {
        siv.add_layer(
            Dialog::text("This is a survey!\nPress <Next> when you're ready.")
                .title("Important survey")
                .button("Next", Test2::show_next),
        );
    }
}

impl Test2 {
    fn show_next(s: &mut Cursive) {
        s.pop_layer();
        s.add_layer(
            Dialog::text("Did you do the thing?")
                .title("Question 1")
                .button("Yes!", |s| Test2::show_answer(s, "I knew it! Well done!"))
                .button("No!", |s| {
                    Test2::show_answer(s, "I knew you couldn't be trusted!")
                })
                .button("Uh?", |s| s.add_layer(Dialog::info("Try again!"))),
        );
    }

    fn show_answer(s: &mut Cursive, msg: &str) {
        s.pop_layer();
        s.add_layer(
            Dialog::text(msg)
                .title("Results")
                .button("Finish", |s| s.quit()),
        );
    }
}
