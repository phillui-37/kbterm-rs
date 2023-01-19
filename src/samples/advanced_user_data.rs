use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, ListView, RadioGroup, SliderView},
    With,
};

use crate::traits::Sample;

#[derive(Clone, Debug, Default)]
struct UserData {
    boolean: bool,
    string: String,
    number: usize,
}

pub struct AdvancedUserData;

impl Sample for AdvancedUserData {
    fn run(&self, siv: &mut cursive::Cursive) {
        siv.set_user_data(UserData::default());

        let dialog = Dialog::text("Some stuff happens here.")
            .title("Main")
            .button("UserData", |s| {
                let mut bool_gp: RadioGroup<bool> = RadioGroup::new();
                let current_data = s
                    .with_user_data(|user_data: &mut UserData| user_data.clone())
                    .unwrap();

                let str_view = EditView::new()
                    .content(current_data.string.clone())
                    .with_name("string")
                    .fixed_width(18);
                let num_view = SliderView::horizontal(18)
                    .value(current_data.number)
                    .with_name("number");
                let bool_view = LinearLayout::horizontal()
                    .child(bool_gp.button(false, "False").fixed_width(10))
                    .child(
                        bool_gp
                            .button(true, "True")
                            .with_if(current_data.boolean, |b| {
                                b.select();
                            })
                            .fixed_width(10),
                    );
                let ls_view = ListView::new()
                    .child("String: ", str_view)
                    .child("Number: ", num_view)
                    .child("Boolean: ", bool_view);

                s.add_layer(Dialog::new().title("UserData").content(ls_view).button(
                    "Done",
                    move |s| {
                        let string = s
                            .call_on_name("string", |view: &mut EditView| view.get_content())
                            .unwrap();
                        let number = s
                            .call_on_name("number", |view: &mut SliderView| view.get_value())
                            .unwrap();
                        s.with_user_data(|user_data: &mut UserData| {
                            user_data.boolean = *bool_gp.selection();
                            user_data.string = string.to_string();
                            user_data.number = number;
                        })
                        .unwrap();
                        s.pop_layer();
                    },
                ))
            });
        siv.add_layer(dialog);
    }
}
