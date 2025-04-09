use nannou::prelude::*;

struct Model {}

// fn model(app: &App) -> Model {
//     Model {}
// }
//
// fn update(_app: &App, _model: &mut Model, _update: Update) {}
//
fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(NAVAJOWHITE);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::sketch(view).run();
}
