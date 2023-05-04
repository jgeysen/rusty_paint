use nannou::prelude::*;
use nannou_egui::{egui, Egui};

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    egui: Egui,
    history: Vec<(f32,f32)>,
    radius: f32,
    color: Hsv,
    pressed: bool,
}

fn model(app: &App) -> Model {
    // Create a new window! Store the ID so we can refer to it later.
    let window_id = app
        .new_window()
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_moved(mouse_moved)
        .title("Nannou + Egui")
        .size(WIDTH as u32, HEIGHT as u32)
        .raw_event(raw_window_event) // This is where we forward all raw events for egui to process them
        .view(view) // The function that will be called for presenting graphics to a frame.
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();

    Model {
        egui: Egui::from_window(&window),
        history: Vec::new(),
        radius: 40.0,
        color: hsv(10.0, 0.5, 1.0),
        pressed: false
    }
}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {
    if _button == MouseButton::Left {
        _model.pressed = true
    }
    
}
fn mouse_released(_app: &App, _model: &mut Model, _button: MouseButton) {
    _model.pressed = false
}

fn mouse_moved(_app: &App, _model: &mut Model, _coord: Point2) {
    let draw = _app.draw();
    if _model.pressed {
         dbg!(_coord);
         draw.ellipse()
        .x_y(_coord[0], _coord[1])
        .radius(_model.radius)
        .color(_model.color);
        _model.history.extend([(_app.mouse.x, _app.mouse.y)]);
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut radius,
        ref mut color,
        ref mut pressed,
        ref mut history,
    } = *model;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("EGUI window")
        .default_size(egui::vec2(0.0, 200.0))
        .show(&ctx, |ui| {
            ui.separator();
            ui.label("Tune parameters with ease");
            ui.add(egui::Slider::new(radius, 10.0..=100.0).text("Radius"));
            edit_hsv(ui, color);
        });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    frame.clear(BLACK);

    draw.ellipse()
        .x_y(app.mouse.x, app.mouse.y)
        .radius(model.radius)
        .color(model.color);

    for (x,y) in &model.history{
         draw.ellipse()
        .x_y(*x, *y)
        .radius(model.radius)
        .color(model.color);
    }

    draw.to_frame(app, &frame).unwrap();

    // Do this as the last operation on your frame.
    model.egui.draw_to_frame(&frame).unwrap();
}

fn edit_hsv(ui: &mut egui::Ui, color: &mut Hsv) {
    let mut egui_hsv = egui::color::Hsva::new(
        color.hue.to_positive_radians() as f32 / (std::f32::consts::PI * 2.0),
        color.saturation,
        color.value,
        1.0,
    );

    if egui::color_picker::color_edit_button_hsva(
        ui,
        &mut egui_hsv,
        egui::color_picker::Alpha::Opaque,
    )
    .changed()
    {
        *color = nannou::color::hsv(egui_hsv.h, egui_hsv.s, egui_hsv.v);
    }
}