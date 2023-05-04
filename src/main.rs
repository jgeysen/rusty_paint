use nannou::{prelude::*};
use nannou_egui::{egui, Egui};
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Enum { Square, Rectangle, Line, Ellipse, Triangle }


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

struct Model {
    egui: Egui,
    history: Vec<Vec<Ellipse>>,
    line_start: Vec<(f32, f32)>,
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
        line_start: Vec::new(),
        radius: 40.0,
        color: hsv(10.0, 0.5, 1.0),
        pressed: false,
        background_colour: hsv(0.0, 0.0, 255.0),
        tool: Enum::Ellipse,
    }
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {
        let draw = app.draw();
        model.pressed = true;

        if model.egui.ctx().is_pointer_over_area() {
            model.line_start.pop();
        }

        if model.tool == Enum::Line {
            if !model.line_start.is_empty() {
                let ellipse_start = Ellipse {
                    x: model.line_start[0].0,
                    y: model.line_start[0].1,
                    color: model.color,
                    radius: model.radius
                };
                let ellipse_end = Ellipse {
                    x: app.mouse.x,
                    y: app.mouse.y,
                    color: model.color,
                    radius: model.radius
                };
                model.history.extend([vec![ellipse_start, ellipse_end]]);
                model.line_start.pop();
            } else {
                if !model.egui.ctx().is_pointer_over_area() {
                    model.line_start = vec![(app.mouse.x, app.mouse.y)];
                }
            }
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
                if ld.is_empty() { return }
                let last_draw_el = &ld[ld.len() - 1];
                if coord[0] == last_draw_el.x && coord[1] == last_draw_el.y { return }
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
        ref mut line_start,
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
                    model.line_start.pop();
                }
            };

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

    for ellipse_vec in &model.history{
        if ellipse_vec.len() == 2 {
            draw.line()
            .weight(2.0*ellipse_vec[1].radius)
            .color(ellipse_vec[1].color)
            .points(nannou::geom::vec2(ellipse_vec[0].x, ellipse_vec[0].y), nannou::geom::vec2(ellipse_vec[1].x, ellipse_vec[1].y));
        } else {
            for el in ellipse_vec{
                draw.ellipse()
                .x_y(el.x, el.y)
                .radius(el.radius)
                .color(el.color);
            }
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