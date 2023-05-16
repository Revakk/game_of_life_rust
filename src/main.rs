use game_of_life::{world, GOL, SCREEN_WIDTH};
use nannou::{prelude::*, ui::color::GRAY};

struct Model {}

fn main() {
    nannou::app(setup)
        .event(event)
        .size(
            game_of_life::SCREEN_WIDTH as u32,
            game_of_life::SCREEN_HEIGHT as u32,
        )
        .update(update)
        .run();
}

fn update(_app: &App, gol: &mut GOL, _: Update) {
    gol.update();
}

fn setup(app: &App) -> GOL {
    let window = app
        .new_window()
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    GOL::new()
}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn event(_app: &App, _model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple: _ } => {}
        _ => {}
    }
}

fn view(app: &App, gol: &GOL, frame: Frame) {
    let draw = app.draw();

    gol.draw(&draw);
    draw.background().color(WHITE);
    //draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(BLUE);

    draw.to_frame(app, &frame).unwrap();
}
