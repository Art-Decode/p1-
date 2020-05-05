use nannou::prelude::*;

mod palette;

use palette::Palette;

struct Model { }

fn update(_app: &App, _model: &mut Model, _update: Update) { }

fn view(app: &App, _model: &Model, frame: Frame) {
    let palette = Palette::new([0x142850, 0x27496d, 0x0c7b93, 0x00a8cc].to_vec());

    let draw = app.draw();

    draw.line()
        .start(pt2(app.mouse.y, app.mouse.x))
        .end(pt2(random_range::<f32>(-300.0, 300.0), random_range::<f32>(-300.0, 300.0)))
        .weight(random_range::<f32>(1.0, 4.0))
        .color(palette.colors[random_range::<usize>(0, 4)]);

    for _ in 0..10 {
        draw.ellipse()
            .x(app.mouse.x)
            .y(app.mouse.y)
            .color(palette.colors[random_range::<usize>(0, 4)])
            .radius(random_range::<f32>(1.0, 30.0));
    }

    draw.to_frame(app, &frame).unwrap();
}

fn model(app: &App) -> Model {
    app.new_window().size(600, 600).view(view).build().unwrap();
    Model { }
}

fn main() {
    nannou::app(model).update(update).run();
}
