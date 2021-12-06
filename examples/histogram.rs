use mapo::{
    axis::{Axis, Direction, LabelPosition},
    histogram::Histogram,
    prelude::*,
    Categorical,
};
use piet::{
    kurbo::{Point, Rect, Size},
    Color,
};
use piet_common::{Device, Piet, RenderContext};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(WIDTH * 2, HEIGHT * 2, 2.0).unwrap();
    let mut rc = bitmap.render_context();

    rc.fill(
        Rect::from_origin_size(Point::ZERO, Size::new(WIDTH as f64, HEIGHT as f64)),
        &Color::WHITE,
    );
    draw(&mut rc);

    rc.finish().unwrap();
    std::mem::drop(rc);

    bitmap
        .save_to_file("temp-image.png")
        .expect("file save error");
}

fn draw(rc: &mut Piet) {
    let mut axis = Axis::new(
        Direction::Horizontal,
        LabelPosition::Before,
        WIDTH as f64 * 0.6,
        Categorical::new(vec![
            "first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        ]),
    );
    axis.layout(rc).unwrap();
    axis.draw((WIDTH as f64 * 0.2, 100.), rc);
    let mut axis = Axis::new(
        Direction::Vertical,
        LabelPosition::After,
        HEIGHT as f64 * 0.6,
        Categorical::new(vec!["first", "second", "third", "fourth", "eggs"]).space_around(),
    );
    axis.layout(rc).unwrap();
    axis.draw((100., HEIGHT as f64 * 0.2), rc);

    let hist_size = Size::new(WIDTH as f64 * 0.6, HEIGHT as f64 * 0.6);
    let mut histogram = Histogram::new(hist_size, ["first", "second", "third"], vec![12., 14., 2.]);

    histogram.layout(rc);
    histogram.draw(rc);
}