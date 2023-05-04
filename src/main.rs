use nannou::prelude::*;
use nannou_egui::{egui, Egui};
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Enum { Square, Rectangle, Line, Ellipse, Triangle }


const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    egui: Egui,
    history: Vec<(f32,f32,Hsv,f32)>,
    radius: f32,
    color: Hsv,
    pressed: bool,
    background_colour: Hsv,
    tool: Enum
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
        pressed: false,
        background_colour: hsv(0.0, 0.0, 255.0),
        tool: Enum::Ellipse,
    }
}

fn mouse_pressed(_app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {
        model.pressed = true
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

impl fmt::Display for Enum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Enum::Square => write!(f, "Square"),
           Enum::Rectangle => write!(f, "Rectangle"),
           Enum::Line => write!(f, "Line"),
           Enum::Ellipse => write!(f, "Ellipse"),
           Enum::Triangle => write!(f, "Triangle"),
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
        ref mut tool
    } = *model;


    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Drawing parameters")
        .default_size(egui::vec2(0.0, 200.0))
        .show(&ctx, |ui| {
            ui.separator();
            ui.label("Tune parameters with ease");
            ui.add(egui::Slider::new(radius, 10.0..=100.0).text("Radius"));
            for option in [Enum::Square, Enum::Rectangle, Enum::Line, Enum::Ellipse, Enum::Triangle] {
                // SelectableLabel is a similar widget; it works like a button that can be checked. Here it serves the 
                // purpose of a radio button, with a single option being selected at any time
                if ui
                    .add(egui::SelectableLabel::new(
                        model.tool == option,
                        option.to_string(),
                    ))
                    .clicked()
                {
                    model.tool = option;
                }
            }

            ui.label("Select the ellipse colour");
            edit_hsv(ui, color);
            ui.label("Select the background colour");
            edit_hsv(ui,background_colour);
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