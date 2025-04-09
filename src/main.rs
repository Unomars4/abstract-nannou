use nannou::{
    lyon::geom::euclid::vec2,
    noise::{NoiseFn, Perlin},
    prelude::*,
};

const N_THINGS: usize = 2000;

struct Model {
    things: Vec<Thing>,
    noise: Perlin,
}

struct Thing {
    positions: Vec<Vec2>,
}

impl Thing {
    pub fn new(pos: Vec2) -> Self {
        let mut positions = Vec::new();
        positions.push(pos);
        Self { positions }
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

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 120.0;
    let scale_factor = 0.01 + time.cos() as f64 * 0.005;
    for thing in model.things.iter_mut() {
        thing.positions.clear();
        thing.positions.push(Vec2::new(
            (random::<f32>() - 0.5) * 1024.0,
            (random::<f32>() - 0.5) * 1024.0,
        ));
        for i in 0..50 {
            let last = thing.positions[0];
            let new = last
                + Vec2::new(
                    model.noise.get([
                        scale_factor * last.x as f64,
                        scale_factor * last.y as f64,
                        0.0,
                    ]) as f32,
                    model.noise.get([
                        scale_factor * last.x as f64,
                        scale_factor * last.y as f64,
                        1.0,
                    ]) as f32,
                );
            thing.positions.insert(0, new);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let time = app.elapsed_frames() as f32 / 20.0;

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    draw.rect()
        .w_h(1024.0, 1024.0)
        .color(srgba(0.0, 0.0, 0.0, 0.1));

    for thing in model.things.iter() {
        draw.polyline()
            .points(thing.positions.iter().cloned())
            .color(WHITE);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
