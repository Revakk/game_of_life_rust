use game_of_life::{cell, world, GOL, SCREEN_WIDTH};
use nannou::{prelude::*, ui::color::GRAY};

struct Model {
    gol_state: GOL,
}

fn main() {
    nannou::app(setup)
        .event(event)
        .size(
            game_of_life::SCREEN_WIDTH as u32 + 10,
            game_of_life::SCREEN_HEIGHT as u32 + 10,
        )
        .update(update)
        .run();
}

fn update(_app: &App, gol: &mut GOL, _: Update) {
    gol.update();
}

fn setup(app: &App) -> GOL {
    let _window = app
        .new_window()
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    GOL::new()
}

fn key_pressed(_app: &App, model: &mut GOL, key: Key) {
    match key {
        Key::Space => model.switch_mode(),
        _ => (),
    }
}

fn mouse_moved(_app: &App, model: &mut GOL, pos: Vec2) {
    model.set_mouse_pos(pos);
    model.change_cell_state();
}

fn mouse_pressed(_app: &App, model: &mut GOL, button: MouseButton) {
    match button {
        MouseButton::Left => {
            model.active_mouse_button = MouseButton::Left;
            model.is_mb_active = true;
            model.change_cell_state();
        } //model.change_cell_state(cell::CellState::ALIVE),
        MouseButton::Right => {
            model.active_mouse_button = MouseButton::Right;
            model.is_mb_active = true;
            model.change_cell_state();
        } //model.change_cell_state(cell::CellState::DEAD),
        _ => model.is_mb_active = false,
    }
}

fn mouse_released(_app: &App, model: &mut GOL, button: MouseButton) {
    model.is_mb_active = false;
}

fn event(_app: &App, _model: &mut GOL, _event: Event) {}

fn view(app: &App, gol: &GOL, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    gol.draw(&draw);

    //let fps = format!("FPS: {}", app.fps().round());
    //draw.text(fps.as_str()).x_y(0.0, 0.0).color(BLACK);
    //draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(BLUE);

    draw.to_frame(app, &frame).unwrap();
}
