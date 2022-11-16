use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use ferrux_canvas::canvas::Canvas;
use ferrux_canvas::canvas::winit::WinitCanvas;
use ferrux_canvas::color::{ColorBuilder, palette};

fn main() {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(960, 480);
        WindowBuilder::new()
          .with_title("FerruX Canvas")
          .with_inner_size(size)
          .with_min_inner_size(size)
          .build(&event_loop)
          .unwrap()
    };
    let mut canvas = WinitCanvas::new(&window).unwrap();
    let mut x: i32 = 1;
    let mut incrementing = true;
    let mut mouse_x: u32 = 0;
    let mut mouse_y: u32 = 0;

    event_loop.run(move |e, _, control_flow| {
        match e {
            Event::WindowEvent {
                event: WindowEvent::MouseInput {
                    button: winit::event::MouseButton::Left,
                    state: winit::event::ElementState::Released,
                    ..
                },
                ..
            } => {
                println!("Released!!");
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput {
                    button: winit::event::MouseButton::Left,
                    state: winit::event::ElementState::Pressed,
                    ..
                },
                ..
            } => {
                println!("Pressed!!");
                for i in 0..10 {
                    for j in 0..10 {
                        canvas.draw_pixel(mouse_x+i, mouse_y+j, palette::WHITE);
                    }
                }
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved {
                    position: winit::dpi::PhysicalPosition {
                        x, y
                    },
                    ..
                },
                ..
            } => {
                mouse_x = x as u32;
                mouse_y = y as u32;
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                // window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                canvas.render().unwrap();
                // canvas.reset_frame();
            }
            _ => (),
        }
    });
}