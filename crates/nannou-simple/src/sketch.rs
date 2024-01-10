use nannou::prelude::*;
use nannou_egui::{egui, Egui};
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use std::cell::RefCell;


pub struct Model {
    settings: Settings,
    egui: Egui,
}

struct Settings {
    sigma: f32,
}

pub fn Model(app: &App) -> Model {
	let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let settings = Settings {
        sigma: 128.0,
    };


    Model { settings, egui}
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    if let nannou::winit::event::WindowEvent::MouseWheel { delta, .. } = event {
        let cursor_over_egui = model.egui.ctx().wants_pointer_input();
        // if !cursor_over_egui {
        //     match delta {
        //         nannou::winit::event::MouseScrollDelta::LineDelta(_, y) => {
        //             model.scale *= 1.0 + *y * 0.05;
        //             model.scale = model.scale.max(0.1).min(10.0);
        //         }
        //         _ => (),
        //     }
        // }
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}

pub async fn run_app() {
	let model = Model();
	// Since ModelFn is not a closure we need this workaround to pass the calculated model
	thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

	MODEL.with(|m| m.borrow_mut().replace(model));

	let output = app::Builder::new_async(|app| {
		Box::new(async move {
			create_window(app).await;
			MODEL.with(|m| m.borrow_mut().take().unwrap())
		})
	})
	.backends(Backends::PRIMARY | Backends::GL)
	.update(update)
	.run_async()
	.await;

	output
}

async fn create_window(app: &App) {
	let device_desc = DeviceDescriptor {
		limits: Limits {
			max_texture_dimension_2d: 8192,
			..Limits::downlevel_webgl2_defaults()
		},
		..Default::default()
	};

	app.new_window()
		.device_descriptor(device_desc)
		.title("Nannou Simple App")
		.size(512, 512)
		.raw_event(raw_window_event)
		// .key_pressed(key_pressed)
		// .key_released(key_released)
		// .mouse_pressed(mouse_pressed)
		// .mouse_moved(mouse_moved)
		// .mouse_released(mouse_released)
		// .mouse_wheel(mouse_wheel)
		// .touch(touch)
		.view(view)
		.build_async()
		.await
		.unwrap();
}

