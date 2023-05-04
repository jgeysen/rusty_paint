use nannou::{prelude::*};
use nannou_egui::{egui, Egui};

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Ellipse {
    x: f32,
    y: f32,
    color: Hsv,
    radius: f32
}

struct Triangle {
    x: f32,
    y: f32,
    z: f32,
    color: Hsv
}

struct Rectangle {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    color: Hsv
}

struct Model {
    egui: Egui,
    history: Vec<Vec<Ellipse>>,
    line_history: Vec<Line>,
    line_start: Vec<(f32, f32)>,
    radius: f32,
    color: Hsv,
    pressed: bool,
    background_colour: Hsv
}

struct Line {
    thickness: f32,
    color: Hsv,
    start_point: Vec<(f32, f32)>,
    end_point: Vec<(f32, f32)>
}

fn model(app: &App) -> Model {
    // Create a new window! Store the ID so we can refer to it later.
    let window_id = app
        .new_window()
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
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
        line_history: Vec::new(),
        line_start: Vec::new(),
        radius: 40.0,
        color: hsv(10.0, 0.5, 1.0),
        pressed: false,
        background_colour: hsv(0.0, 0.0, 255.0),
    }
}

fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {
        let draw = _app.draw();
        model.pressed = true;

        if !model.line_start.is_empty() {
            let line = Line {
                thickness: model.radius,
                start_point: vec![(model.line_start[0].0, model.line_start[0].1)],
                color: model.color,
                end_point: vec![(_app.mouse.x, _app.mouse.y)],
            };
            model.line_history.push(line);
            model.line_start.pop();
            model.line_start.pop();

        } else {
            model.line_start = vec![(_app.mouse.x, _app.mouse.y)];
        }
    }
}
fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.pressed = false
}

fn mouse_moved(app: &App, model: &mut Model, coord: Point2) {
    if model.pressed {
        let last_draw = model.history.last();
        match last_draw {
            Some(ld) => {
                if coord[0] == ld.0 && coord[1] == ld.1 { return }
            }
            _ => {}
        }

        if !model.egui.ctx().is_pointer_over_area() {
            let ellipse = Ellipse {
                    x: app.mouse.x,
                    y: app.mouse.y,
                    color: model.color,
                    radius: model.radius
            };
            model.history.extend([vec![ellipse]]);
        }
    }
}

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {
    if _key == Key::Z && _app.keys.mods.logo() {
        _model.history.pop();
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut radius,
        ref mut color,
        ref mut pressed,
        ref mut history,
        ref mut background_colour,
        ref mut line_history,
        ref mut line_start,
    } = *model;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Drawing parameters")
        .default_size(egui::vec2(0.0, 200.0))
        .show(&ctx, |ui| {
            ui.separator();
            ui.label("Tune parameters with ease");
            ui.add(egui::Slider::new(radius, 10.0..=100.0).text("Radius"));
            ui.label("Select the ellipse colour");
            edit_hsv(ui, color);
            ui.label("Select the background colour");
            edit_hsv(ui,background_colour)
        });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    frame.clear(model.background_colour);

    draw.ellipse()
        .x_y(app.mouse.x, app.mouse.y)
        .radius(model.radius)
        .color(model.color);

    for line in &model.line_history {
        draw.line()
            .weight(line.thickness)
            .color(line.color)
            .points(nannou::geom::vec2(line.start_point[0].0, line.start_point[0].1), nannou::geom::vec2(line.end_point[0].0, line.end_point[0].1));
    }

    for ellipse_vec in &model.history{
        for el in ellipse_vec{
             draw.ellipse()
            .x_y(el.x, el.y)
            .radius(el.radius)
            .color(el.color);
        }
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