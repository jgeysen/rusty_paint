use nannou::prelude::*;
use nannou_egui::{egui, Egui};

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    egui: Egui,
    history: Vec<(f32,f32,Hsv,f32)>,
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
    let draw = app.draw();
    if model.pressed {
         draw.ellipse()
        .x_y(coord[0], coord[1])
        .radius(model.radius)
        .color(model.color);
        
        if !model.egui.ctx().is_pointer_over_area() {
            model.history.extend([(app.mouse.x, app.mouse.y, model.color, model.radius)]);
        }
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

    for (x,y,color,radius) in &model.history{
         draw.ellipse()
        .x_y(*x, *y)
        .radius(*radius)
        .color(*color);
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