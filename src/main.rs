use nannou::prelude::*;

struct Model {}

fn model(app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    frame.clear(NAVAJOWHITE);
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
