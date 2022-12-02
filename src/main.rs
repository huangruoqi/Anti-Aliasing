use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use ferrux_canvas::canvas::Canvas;
use ferrux_canvas::canvas::winit::WinitCanvas;
use ferrux_canvas::color::{Color, palette};
use std::mem;
mod supersampling;

fn main() {
    const GRID_WIDTH    : usize = 100;
    const GRID_HEIGHT   : usize = 100;
    const PIXEL_SIZE    : usize = 10;
    const RATIO         : usize = 2; // change to 1 for Windows
    const DISPLAY_WIDTH : usize = GRID_WIDTH * PIXEL_SIZE / RATIO;
    const DISPLAY_HEIGHT: usize = GRID_HEIGHT * PIXEL_SIZE / RATIO;
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new((DISPLAY_WIDTH) as i32, (DISPLAY_HEIGHT) as i32);
        WindowBuilder::new()
          .with_title("Anti-Aliasing Demo")
          .with_inner_size(size)
          .with_min_inner_size(size)
          .build(&event_loop)
          .unwrap()
    };
    let mut canvas = WinitCanvas::new(&window).unwrap();
    let mut mouse_x: usize = 0;
    let mut mouse_y: usize = 0;
    // let mut is_pressing: bool = false;

    println!("Press number keys to switch between Anti-Aliasing methods:");
    println!("0. None (Bresenham's line drawing algorithm)");
    println!("1. Xiaolin Wu's line algorithm");
    println!("2. Fast Approximate Anti-Aliasing");
    println!("3. Supersampling Anti-Aliasing");

    let mut grid = vec![vec![palette::BLACK; GRID_WIDTH]; GRID_HEIGHT];
    let mut pairs = vec![vec![0usize;4];0];
    let mut pair = vec![0usize;0];

    fn draw_pixel(cvs: &mut WinitCanvas, _vec_grid: &mut Vec<Vec<Color>>, x: usize, y:usize, color: &Color, alpha: u8) {
        let mut c = (*color).clone();
        c.a = alpha;
        let real_size = PIXEL_SIZE;
        for i in 0..real_size {
            for j in 0..real_size {
                cvs.draw_pixel((x*real_size+i) as u32, (y*real_size+j) as u32, c.clone());
            }
        }
    }
    fn draw_point(cvs: &mut WinitCanvas, vec_grid: &mut Vec<Vec<Color>>, x: usize, y:usize, width: usize, color: &Color) {
        for i in 0..width {
            for j in 0..width {
                draw_pixel(cvs, vec_grid, i+x-width/2, j+y-width/2, color, 255 as u8);
            }
        }
    }
    fn draw_line(cvs: &mut WinitCanvas, vec_grid: &mut Vec<Vec<Color>>, x1:usize,y1:usize,x2:usize,y2:usize, _width: usize, color: &Color) {
        let mut x_lo: i32 = x1 as i32;
        let mut x_hi: i32 = x2 as i32;
        let mut y_lo: i32 = y1 as i32;
        let mut y_hi: i32 = y2 as i32;
        let mut dx = x_hi - x_lo;
        let mut dy = y_hi - y_lo;
        let flipped = dx * sign(dx) < dy * sign(dy);
        let bound;
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
        while (cx as usize) < (bound as usize) {
            if flipped{
                draw_pixel(cvs, vec_grid, cy as usize, cx as usize, color, 255 as u8);
            }
            else {
                draw_pixel(cvs, vec_grid, cx as usize, cy as usize, color, 255 as u8);
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
    fn press(cvs: &mut WinitCanvas, vec_grid: &mut Vec<Vec<Color>>, vec_pairs: &mut Vec<Vec<usize>>,vec_pair: &mut Vec<usize>, x: usize, y:usize, color: &Color) {
        draw_point(cvs, vec_grid, x, y, 3 as usize, color);
        vec_pair.push(x);
        vec_pair.push(y);
        if vec_pair.len()>3 {
            draw_line(cvs, vec_grid, vec_pair[0], vec_pair[1], vec_pair[2], vec_pair[3], 10 as usize, color);
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
                // is_pressing = false;
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput {
                    button: winit::event::MouseButton::Left,
                    state: winit::event::ElementState::Pressed,
                    ..
                },
                ..
            } => {
                // is_pressing = true;
                let x = mouse_x / PIXEL_SIZE;
                let y = mouse_y / PIXEL_SIZE;
                press(&mut canvas, &mut grid, &mut pairs, &mut pair, x as usize, y as usize, &palette::WHITE);
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
                mouse_x = x as usize;
                mouse_y = y as usize;
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
                // canvas.reset_frame();
                let ss_grid = supersampling::ssaa(GRID_WIDTH, GRID_HEIGHT ,&mut pairs);
                for i in 0..GRID_WIDTH {
                    for j in 0..GRID_HEIGHT {
                        draw_pixel(&mut canvas, &mut grid, i, j, &ss_grid[i][j], ss_grid[i][j].a)
                    }
                }

                canvas.render().unwrap();
            }
            _ => (),
        }
    });
}
