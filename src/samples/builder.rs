use cursive::{
    builder::Context,
    raw_recipe,
    views::{Button, EditView, TextView}, Cursive,
};

use crate::traits::Sample;

raw_recipe!(LabeledField from {serde_yaml::from_str(include_str!("assets/label-view.yaml")).unwrap()});
raw_recipe!(VSpace from {serde_yaml::from_str(include_str!("assets/vspace.yaml")).unwrap()});

raw_recipe!(Titled, |config, context| {
    use cursive::views::Panel;
    let title: String = context.resolve(&config["title"])?;
    let child = context.build(&config["child"])?;
    Ok(Panel::new(child).title(title))
});

pub struct Builder;

impl Sample for Builder {
    fn run(&self, siv: &mut Cursive) {
        cursive::logger::init();

        let mut ctx = Context::new();
        ctx.store("title", String::from("Config-driven layout example"));
        ctx.store("on_edit", EditView::on_edit_cb(Builder::on_edit_cb));
        ctx.store(
            "randomize",
            Button::new_cb(|s| {
                let cb = s
                    .call_on_name("edit", |e: &mut EditView| e.set_content("Not so random!"))
                    .unwrap();
                cb(s);
            }),
        );
        const CONFIG: &str = include_str!("assets/builder.yaml");
        let cnf = serde_yaml::from_str(CONFIG).unwrap();
        let view = ctx.build(&cnf).unwrap();
        siv.add_global_callback('~', Cursive::toggle_debug_console);
        siv.screen_mut().add_transparent_layer(view);
    }
}

impl Builder {
  fn on_edit_cb(siv: &mut Cursive, text: &str, cursor: usize) {
    siv.call_on_name("status", |v: &mut TextView| {
      let spaces: String = std::iter::repeat(" ")
      .take(format!("{cursor}You wrote `").len())
      .collect();
      v.set_content(format!("You wrote `{text}`\n{spaces}^"));
    })
    .unwrap();
  }
}