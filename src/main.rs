extern crate grapher;

use grapher::{
    context::Context,
    graphable::GraphableFn,
};

struct IdealModel;

impl GraphableFn for IdealModel {
    fn evaluate(&self, x: f32) -> f32 {
        x * x * (-x).exp()
    }
}

fn main() {
    let mut ctx = Context::new((800, 600), "grapher", 8, true);
    ctx.add_figure(Box::new(IdealModel));

    while ctx.tick() {

    }
}
