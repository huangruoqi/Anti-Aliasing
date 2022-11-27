use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use ferrux_canvas::canvas::Canvas;
use ferrux_canvas::canvas::winit::WinitCanvas;
use ferrux_canvas::color::{ColorBuilder, palette};
extern crate std;

fn main() {
    const WIDTH_DISPLAY : usize = 500;
    const HEIGHT_DISPLAY: usize = 500;
    const RATIO         : usize = 2; // change to 1 for Windows
    const WIDTH         : usize = WIDTH_DISPLAY * RATIO;
    const HEIGHT        : usize = HEIGHT_DISPLAY * RATIO;
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new((WIDTH_DISPLAY) as i32, (HEIGHT_DISPLAY) as i32);
        WindowBuilder::new()
          .with_title("Anti-Aliasing Demo")
          .with_inner_size(size)
          .with_min_inner_size(size)
          .build(&event_loop)
          .unwrap()
    };
    let mut canvas = WinitCanvas::new(&window).unwrap();
    let mut mouse_x: u32 = 0;
    let mut mouse_y: u32 = 0;
    let mut is_pressing: bool = false;

    println!("Press number keys to switch between Anti-Aliasing methods:");
    println!("0. None (Bresenham's line drawing algorithm)");
    println!("1. Xiaolin Wu's line algorithm");
    println!("2. Fast Approximate Anti-Aliasing");
    println!("3. Supersampling Anti-Aliasing");

    let mut grid = vec![vec![0u8; WIDTH]; HEIGHT];
    let mut pairs = vec![vec![0usize;4];0];
    let mut pair = vec![0usize;0];
    fn draw_pixel(vec_grid: &mut Vec<Vec<u8>>, i: usize, j:usize) {
        vec_grid[i][j] = 1 as u8
    }
    fn draw_point(vec_grid: &mut Vec<Vec<u8>>, x: usize, y:usize, width: usize) {
        for i in 0..width{
            for j in 0..width {
                draw_pixel(vec_grid, j+y-width/2, i+x-width/2);
            }
        }
    }
    fn draw_line(vec_grid: &mut Vec<Vec<u8>>, vec_pairs: &mut Vec<Vec<usize>>, arr: &mut Vec<usize>, width: usize) {
        
    }
    fn press(vec_grid: &mut Vec<Vec<u8>>, vec_pairs: &mut Vec<Vec<usize>>,arr: &mut Vec<usize>, x: usize, y:usize) {
        draw_point(vec_grid, x, y, 10 as usize);
        arr.push(x);
        arr.push(y);
        if arr.len()>3 {
            vec_pairs.push(vec![arr[0], arr[1]]);
            *arr = vec![0usize;0];
            draw_line(vec_grid, vec_pairs,arr, 10 as usize);
        }
    }


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
                is_pressing = false;
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
                is_pressing = true;
                press(&mut grid, &mut pairs, &mut pair, mouse_x as usize, mouse_y as usize);
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
                canvas.reset_frame();
                for row in 0..HEIGHT {
                    for col in 0..WIDTH {
                        if grid[row as usize][col as usize] == 1u8 {
                            canvas.draw_pixel(col as u32, row as u32, palette::WHITE);
                        }
                    }
                }
                canvas.render().unwrap();
                // canvas.reset_frame();
            }
            _ => (),
        }
    });
}
