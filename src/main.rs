use nannou::prelude::*;

mod palette;

use palette::Palette;

const PALETTE: [u32; 4] = [0x142850, 0x27496d, 0x0c7b93, 0x00a8cc];
const W: u32 = 600;
const H: u32 = 600;

struct Model {
    ellipse_offset: f32
}

fn random_xy(w: f32, h: f32) -> Point2 {
    let x = if random::<bool>() { -random::<f32>() } else { random::<f32>() };
    let y = if random::<bool>() { -random::<f32>() } else { random::<f32>() };
    pt2(x * w * 0.5, y * h * 0.5)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.ellipse_offset += 1.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let palette = Palette::new(PALETTE.to_vec());

    let draw = app.draw();

    draw.line()
        .start(random_xy(W as f32, H as f32))
        .end(random_xy(W as f32 / 3.0, H as f32 / 3.0))
        .weight(0.1)
        .color(palette.colors[random_range::<usize>(0, 4)]);

    if model.ellipse_offset % 10.0 == 0.0 {
        draw.ellipse()
            .x(0.0)
            .y(0.0)
            .color(rgba(0.0, 0.0, 0.0, 0.0))
            .stroke_color(palette.colors[random_range::<usize>(0, 4)])
            .stroke_weight(0.1)
            .radius(model.ellipse_offset);
    }

    draw.ellipse()
        .radius(random_range::<f32>(0.1, 1.0))
        .xy(random_xy(W as f32, H as f32))
        .color(palette.colors[random_range::<usize>(0, 4)]);

    draw.to_frame(app, &frame).unwrap();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::NTimes { number_of_updates: 1000 });
    app.new_window().size(W, H).view(view).build().unwrap();
    Model {
        ellipse_offset: 1.0
    }
}

fn main() {
    nannou::app(model).update(update).run();
}
