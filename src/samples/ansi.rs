use cursive::{Cursive, views::TextView, utils::markup::ansi};

use crate::traits::Sample;

pub struct Ansi;

impl Sample for Ansi {
  fn run(&self, siv: &mut Cursive) {
    let content = include_str!("assets/text_with_ansi_codes.txt");
    let styled = ansi::parse(content);
    let tv = TextView::new(styled);
    siv.add_layer(tv);
  }
}