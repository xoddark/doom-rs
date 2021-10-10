use crate::camera::Camera;
use crate::input::Input;
use crate::render::doom_gl::DoomGl;
use crate::sys::content::Content;
use crate::wad::file::WadFile;

use glutin::{
    dpi::{PhysicalSize, Size},
    event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use std::{cell::RefCell, path::Path, rc::Rc};

pub fn run_app() {
    let el = EventLoop::new();
	let mut app = App::new(&el);
	el.run(move |event, _, control_flow| {
		app.run(event, control_flow)
	});
}

pub struct App {
	windowed_context: glutin::WindowedContext<glutin::PossiblyCurrent>,
	content: Content,
	camera: Rc<RefCell<Camera>>,
	input: Input,
	focus: bool,
}

impl App {
	fn new(el: &glutin::event_loop::EventLoop<()>) -> App {
		let size = Size::Physical(PhysicalSize::new(1680, 1050));
		let wb = WindowBuilder::new()
		    .with_inner_size(size)
		    .with_resizable(false)
		    .with_title("DOOM");
		let windowed_context = ContextBuilder::new().build_windowed(wb, el).unwrap();
		let windowed_context = unsafe { windowed_context.make_current().unwrap() };
		DoomGl::init(windowed_context.context());
		let mut input = Input::new();

		let file = WadFile::new(Path::new("/home/xod/Jeux/DOOM/base/DOOM.WAD")).unwrap();

		let content = Content::new(file);

		let camera = Rc::new(RefCell::new(Camera::new()));

		input.listeners.push(camera.clone());

		windowed_context.window().set_cursor_grab(true).unwrap();
		windowed_context.window().set_cursor_visible(false);
		let focus = true;

		App {
			windowed_context,
			content,
			camera,
			input,
			focus,
		}
	}

	pub fn run(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        //println!("{:?}", event);
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Focused(f) => {
                    self.focus = f;
                    self.windowed_context.window().set_cursor_grab(self.focus).unwrap();
                    self.windowed_context.window().set_cursor_visible(!self.focus);
                }
                WindowEvent::Resized(physical_size) => self.windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                    ..
                } => {
                    if self.focus {
                        self.input.register_input_event(virtual_code, state == ElementState::Pressed)
                    }
                }
                _ => (),
            },
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                if self.focus {
                    self.input.register_mouse_move(delta)
                }
            }

            Event::MainEventsCleared => {
                if self.focus {
                    self.camera.try_borrow_mut().unwrap().update();
                }
                self.content.maps[0].render(&self.camera.try_borrow_mut().unwrap());
                self.windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    }
}
               
