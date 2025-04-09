use nannou::prelude::*;

const N_THINGS: usize = 2000;

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

    for i in 0..N_THINGS {
        let thing = Thing::new(Vec2::new(
            (random::<f32>() - 0.5) * 1024.0,
            (random::<f32>() - 0.5) * 1024.0,
        ));
        things.push(thing);
    }

    Model { things }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for thing in model.things.iter_mut() {
        thing.pos += Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let time = app.elapsed_frames() as f32 / 60.0;
    draw.background().color(NAVAJOWHITE);

    for (idx, thing) in model.things.iter().enumerate() {
        let angle = idx as f32 * 0.1 * TAU + time;
        draw.ellipse().xy(thing.pos).radius(5.0).color(GRAY);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
