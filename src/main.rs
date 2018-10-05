extern crate grapher;

use grapher::{
    context::Context,
    graphable::GraphableFn,
    primitive::ColorRgb,
};

struct IdealModel;

impl GraphableFn for IdealModel {
    fn evaluate(&self, x: f32) -> (f32, ColorRgb) {
        (x * x * (-x).exp(), (255, 127, 200).into())
    }
}

/*
struct Series;

impl GraphableFn for Series {
    fn evaluate(&self, x: f32) -> (f32, ColorRgb) {

    }
}
*/

fn main() {
    let mut ctx = Context::new((800, 600), "grapher", 8, true);
    ctx.add_figure(Box::new(IdealModel));

    while ctx.tick() {

    }
}
