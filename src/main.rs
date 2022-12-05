use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use ferrux_canvas::canvas::Canvas;
use ferrux_canvas::canvas::winit::WinitCanvas;
use ferrux_canvas::color::{Color, palette};
use std::mem;
mod supersampling;
mod fastapproximate;

fn main() {
    const GRID_WIDTH    : usize = 200;
    const GRID_HEIGHT   : usize = 200;
    const PIXEL_SIZE    : usize = 8;
    const RATIO         : usize = 2; // change to 1 for Windows
    const DISPLAY_WIDTH : usize = GRID_WIDTH * PIXEL_SIZE / RATIO;
    const DISPLAY_HEIGHT: usize = GRID_HEIGHT * PIXEL_SIZE / RATIO;
    const POINT_WIDTH   : usize = 5;
    const LINE_WIDTH    : usize = 3;
    const COLOR         : Color = palette::WHITE;
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

    println!("Press number keys to apply Anti-Aliasing methods:");
    println!("1. None (Bresenham's line drawing algorithm)");
    println!("2. Xiaolin Wu's line algorithm");
    println!("3. Fast Approximate Anti-Aliasing");
    println!("4. Supersampling Anti-Aliasing");
    println!("Press <C> to clear");

    let mut grid = vec![vec![palette::BLACK; GRID_WIDTH]; GRID_HEIGHT];
    let mut pairs = vec![vec![0usize;4];0];
    let mut points = vec![vec![0usize;2];0];
    let mut pair = vec![0usize;0];

    fn draw_and_save_pixel(cvs: &mut WinitCanvas, vec_grid: &mut Vec<Vec<Color>>, x: usize, y:usize, color: &Color) {
        let c = (*color).clone();
        let real_size = PIXEL_SIZE;
        for i in 0..real_size {
            for j in 0..real_size {
                cvs.draw_pixel((x*real_size+i) as u32, (y*real_size+j) as u32, c.clone());
            }
        }
        vec_grid[x][y] = c;
    }

    fn draw_pixel(cvs: &mut WinitCanvas, x: usize, y:usize, color: &Color) {
        let c = (*color).clone();
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
                draw_and_save_pixel(cvs, vec_grid, i+x-width/2, j+y-width/2, color);
            }
        }
    }
    fn draw_line(cvs: &mut WinitCanvas, vec_grid: &mut Vec<Vec<Color>>, x1:usize,y1:usize,x2:usize,y2:usize, width: usize, color: &Color) {
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
        while (cx as usize) <= (bound as usize) {
            if flipped{
                draw_point(cvs, vec_grid, cy as usize, cx as usize, width, color);
            }
            else {
                draw_point(cvs, vec_grid, cx as usize, cy as usize, width, color);
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
    fn press(cvs: &mut WinitCanvas, vec_grid: &mut Vec<Vec<Color>>,vec_points:&mut Vec<Vec<usize>>, vec_pairs: &mut Vec<Vec<usize>>,vec_pair: &mut Vec<usize>, x: usize, y:usize, color: &Color) {
        draw_point(cvs, vec_grid, x, y, POINT_WIDTH, color);
        vec_points.push(vec![x,y]);
        vec_pair.push(x);
        vec_pair.push(y);
        if vec_pair.len()>3 {
            draw_line(cvs, vec_grid, vec_pair[0], vec_pair[1], vec_pair[2], vec_pair[3], LINE_WIDTH, color);
            vec_pairs.push(vec![vec_pair[0], vec_pair[1], vec_pair[2], vec_pair[3]]);
            vec_pair.clear();
        }
    }


    event_loop.run(move |e, _, control_flow| {
        match e {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: winit::event::KeyboardInput {
                        virtual_keycode,
                        state: winit::event::ElementState::Released,
                        ..
                    },
                    ..
                },
                ..
            } => {
                match virtual_keycode.unwrap() {
                    winit::event::VirtualKeyCode::Key1 => {
                        canvas.reset_frame();
                        for i in 0..GRID_WIDTH {
                            for j in 0..GRID_HEIGHT {
                                draw_pixel(&mut canvas, i, j, &grid[i][j]);
                            }
                        }
                    }
                    winit::event::VirtualKeyCode::Key2 => {}
                    winit::event::VirtualKeyCode::Key3 => {
                        canvas.reset_frame();
                        let fx_grid = fastapproximate::fxaa(GRID_WIDTH, GRID_HEIGHT , &grid);
                        for i in 0..GRID_WIDTH {
                            for j in 0..GRID_HEIGHT {
                                draw_pixel(&mut canvas, i, j, &fx_grid[i][j]);
                            }
                        }
                    }
                    winit::event::VirtualKeyCode::Key4 => {
                        canvas.reset_frame();
                        let ss_grid = supersampling::ssaa(GRID_WIDTH, GRID_HEIGHT ,&mut points, &mut pairs, POINT_WIDTH, LINE_WIDTH);
                        for i in 0..GRID_WIDTH {
                            for j in 0..GRID_HEIGHT {
                                draw_pixel(&mut canvas, i, j, &ss_grid[i][j]);
                            }
                        }
                    }
                    winit::event::VirtualKeyCode::C => {
                        canvas.reset_frame();
                        points.clear();
                        pairs.clear();
                        pair.clear();
                        for i in 0..GRID_WIDTH {
                            for j in 0..GRID_HEIGHT {
                                grid[i][j] = palette::BLACK;
                            }
                        }
                    }
                    _ => { println!("not found"); }
                }
                window.request_redraw();
            }
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
                press(&mut canvas, &mut grid, &mut points, &mut pairs, &mut pair, x as usize, y as usize, &COLOR);
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
                canvas.render().unwrap();
            }
            _ => (),
        }
    });
}
