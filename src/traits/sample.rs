use cursive::Cursive;

pub trait Sample {
  fn run(&self, siv: &mut Cursive);
}