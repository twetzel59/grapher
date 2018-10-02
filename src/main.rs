extern crate grapher;

use grapher::context::Context;

fn main() {
    let mut ctx = Context::new((800, 600), "grapher", 8, true);

    while ctx.tick() {

    }
}
