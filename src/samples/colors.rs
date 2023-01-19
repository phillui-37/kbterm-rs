use cursive::{
    theme::{Color, ColorStyle},
    view::Resizable,
    views::Canvas,
    Cursive, Printer,
};

use crate::traits::Sample;

pub struct Colors;

impl Sample for Colors {
    fn run(&self, siv: &mut cursive::Cursive) {
        siv.add_global_callback('q', Cursive::quit);
        siv.add_layer(Canvas::new(()).with_draw(draw).fixed_size((20, 10)));
    }
}

fn draw(_: &(), p: &Printer) {
    let x_max = p.size.x as u8;
    let y_max = p.size.y as u8;

    for x in 0..x_max {
        for y in 0..y_max {
            let style = ColorStyle::new(
                front_color(x, y, x_max, y_max),
                back_color(x, y, x_max, y_max),
            );
            p.with_color(style, |p| p.print((x, y), "+"));
        }
    }
}

fn front_color(x: u8, y: u8, x_max: u8, y_max: u8) -> Color {
  // parenthesis must be kept else will overflow as max value of u8 is 255
    Color::Rgb(
        x * (255 / x_max),
        y * (255 / y_max),
        (x + 2 * y) / (x_max + 2 * y_max),
    )
}

fn back_color(x: u8, y: u8, x_max: u8, y_max: u8) -> Color {
    Color::Rgb(
        128 + (2 * y_max + x - 2 * y) * (128 / (x_max + 2 * y_max)),
        255 - y * (255 / y_max),
        255 - x * (255 / x_max),
    )
}
