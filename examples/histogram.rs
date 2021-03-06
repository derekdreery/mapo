use fake::{faker::name::raw::FirstName, locales::EN};
use mapo::histogram::histogram;
use piet::{
    kurbo::{Affine, Point, Rect, Size, Vec2},
    Color,
};
use piet_common::{Device, Piet, RenderContext};
use rand_distr::{Distribution, Normal};

const WIDTH: usize = 1000;
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
    /* todo move into seaparate example
    let mut axis = Axis::new(
        Direction::Left,
        LabelPosition::Before,
        WIDTH as f64 * 0.6,
        Categorical::new(vec![
            "first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        ]),
    );
    axis.layout(rc).unwrap();
    axis.draw((WIDTH as f64 * 0.2, 100.), rc);
    let mut axis = Axis::new(
        Direction::Down,
        LabelPosition::After,
        HEIGHT as f64 * 0.6,
        Categorical::new(vec!["first", "second", "third", "fourth", "eggs"]).space_around(),
    );
    axis.layout(rc).unwrap();
    axis.draw((100., HEIGHT as f64 * 0.2), rc);
    */

    let hist_size = Size::new(WIDTH as f64 * 0.95, HEIGHT as f64 * 0.95);
    let hist_tl = Vec2::new(WIDTH as f64 * 0.025, HEIGHT as f64 * 0.025);

    let normal = Normal::new(10., 8.).unwrap();
    let rng = &mut rand::thread_rng();
    let names = fake::vec![String as FirstName(EN); 10];
    let values: Vec<_> = (0..names.len())
        .map(|_| {
            let n: f64 = normal.sample(rng);
            n.round().abs()
        })
        .collect();
    let mut histogram = histogram(names, values);

    rc.with_save(|rc| {
        rc.transform(Affine::translate(hist_tl));
        histogram.layout(hist_size, rc).unwrap();
        histogram.draw(rc);
        Ok(())
    })
    .unwrap();
}
