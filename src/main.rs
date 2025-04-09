use nannou::prelude::*;

struct Model {
    things: Vec<Thing>,
}

struct Thing {
    pos: Vec2,
}

impl Thing {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();
    let mut things = Vec::new();
    let thing = Thing::new(Vec2::new(0.0, 0.0));
    things.push(thing);
    Model { things }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let time = app.elapsed_frames() as f32 / 60.0;
    draw.background().color(NAVAJOWHITE);

    for (idx, thing) in model.things.iter().enumerate() {
        let angle = idx as f32 * 0.1 * TAU + time;
        draw.ellipse()
            .x_y(thing.pos.x * angle.cos(), thing.pos.y * angle.sin())
            .color(GRAY);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
