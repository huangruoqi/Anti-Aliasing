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

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                if !(1..100).contains(&x) {
                    incrementing = !incrementing;
                }
                x += if incrementing { 1 } else { -1 };

                let a = 200 - x as u32;

                canvas.fill_triangle((100, 100 + x as u32), (100 + x as u32, 100),
                                     (200 + x as u32, 200 + x as u32),
                ColorBuilder::new().with_red(125).with_blue(150).with_green(50).build());

                canvas.draw_triangle((100, 100 - x as u32), (100 - x as u32, 100),
                                     (a, a), palette::WHITE);

                canvas.draw_line((a, 100), (a, a), palette::RED);
                canvas.draw_line((a, a), (100, a), palette::BLUE);
                canvas.draw_line((100, a), (a, 100), palette::GREEN);

                canvas.render().unwrap();
                canvas.reset_frame();
            }
            _ => (),
        }
    });
}