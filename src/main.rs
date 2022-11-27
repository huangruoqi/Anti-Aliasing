use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use ferrux_canvas::canvas::Canvas;
use ferrux_canvas::canvas::winit::WinitCanvas;
use ferrux_canvas::color::{ColorBuilder, palette};
use std::mem;

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
    fn draw_pixel(vec_grid: &mut Vec<Vec<u8>>, x: usize, y:usize) {
        vec_grid[y][x] = 1 as u8
    }
    fn draw_point(vec_grid: &mut Vec<Vec<u8>>, x: usize, y:usize, width: usize) {
        for i in 0..width{
            for j in 0..width {
                draw_pixel(vec_grid, i+x-width/2, j+y-width/2);
            }
        }
    }
    fn draw_line(vec_grid: &mut Vec<Vec<u8>>, x1:usize,y1:usize,x2:usize,y2:usize, width: usize) {
        let mut x_lo: i32 = x1 as i32;
        let mut x_hi: i32 = x2 as i32;
        let mut y_lo: i32 = y1 as i32;
        let mut y_hi: i32 = y2 as i32;
        let mut dx = x_hi - x_lo;
        let mut dy = y_hi - y_lo;
        let flipped = dx * sign(dx) < dy * sign(dy);
        let mut bound;
        let mut cx;
        let mut cy;
        if flipped {
            if y_lo > y_hi {
                mem::swap(&mut x_lo, &mut x_hi);
                mem::swap(&mut y_lo, &mut y_hi);
            }
            dx = y_hi - y_lo;
            dy = x_hi - x_lo;
            cx = y_lo;
            cy = x_lo;
            bound = y_hi;
        }
        else {
            if x_lo > x_hi {
                mem::swap(&mut x_lo, &mut x_hi);
                mem::swap(&mut y_lo, &mut y_hi);
            }
            dx = x_hi - x_lo;
            dy = y_hi - y_lo;
            cx = x_lo;
            cy = y_lo;
            bound = x_hi;
        }
        let mut p: i32 = 2 * dy - dx;
        print!("{} {}", cx, bound);
        while (cx as usize) < (bound as usize) {
            if flipped{
                draw_point(vec_grid, cy as usize, cx as usize, 10 as usize);
            }
            else {
                draw_point(vec_grid, cx as usize, cy as usize, 10 as usize);
            }
            cx+=sign(dx);
            if p < 0 {
                p = p + 2 * dy * sign(dy);
            }
            else {
                p = p + 2 * dy * sign(dy) - 2 * dx * sign(dx);
                cy+=sign(dy);
            }
        }
    }

    fn sign(n: i32) -> i32 {
        if n < 0 {
            return -1;
        }
        else {
            return 1;
        }
    }
    fn press(vec_grid: &mut Vec<Vec<u8>>, vec_pairs: &mut Vec<Vec<usize>>,vec_pair: &mut Vec<usize>, x: usize, y:usize) {
        draw_point(vec_grid, x, y, 10 as usize);
        vec_pair.push(x);
        vec_pair.push(y);
        if vec_pair.len()>3 {
            draw_line(vec_grid, vec_pair[0], vec_pair[1], vec_pair[2], vec_pair[3], 10 as usize);
            vec_pairs.push(vec![vec_pair[0], vec_pair[1], vec_pair[2], vec_pair[3]]);
            vec_pair.clear();
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
                // TODO: 
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
