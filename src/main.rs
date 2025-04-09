use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

const N_THINGS: usize = 1000;

struct Model {
    things: Vec<Thing>,
    noise: Perlin,
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
    let noise = Perlin::new();

    for i in 0..N_THINGS {
        let thing = Thing::new(Vec2::new(
            (random::<f32>() - 0.5) * 1024.0,
            (random::<f32>() - 0.5) * 1024.0,
        ));
        things.push(thing);
    }

    Model { things, noise }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let scale_factor = 0.02;
    for thing in model.things.iter_mut() {
        thing.pos += Vec2::new(
            model.noise.get([
                scale_factor * thing.pos.x as f64,
                scale_factor * thing.pos.y as f64,
                0.0,
            ]) as f32,
            model.noise.get([
                scale_factor * thing.pos.x as f64,
                scale_factor * thing.pos.y as f64,
                1.0,
            ]) as f32,
        );
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
