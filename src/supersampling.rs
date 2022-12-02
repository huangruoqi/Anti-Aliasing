use ferrux_canvas::color::{Color, palette};
use std::mem;

const FACTOR: usize = 2 as usize;

pub fn ssaa(width: usize, height: usize, pairs: &mut Vec<Vec<usize>>) -> Vec<Vec<Color>>{
    let mut s_grid = vec![vec![palette::BLACK; width * FACTOR]; height * FACTOR];
    for i in pairs{
        draw_line(&mut s_grid, i[0]*FACTOR, i[1]*FACTOR, i[2]*FACTOR, i[3]*FACTOR, 1 as usize, &palette::WHITE);
    }
    return downsample(width, height, s_grid);
}

fn downsample(width: usize, height: usize, vec_grid: Vec<Vec<Color>>) -> Vec<Vec<Color>> {
    let mut r_grid = vec![vec![palette::BLACK; width]; height];
    for i in 0..width {
        for j in 0..height {
            let base_x = i*FACTOR;
            let base_y = j*FACTOR;
            let mut c = vec_grid[base_x][base_y].clone();
            let mut alpha: i32 = 0;
            for x in 0..FACTOR {
                for y in 0..FACTOR {
                    alpha += (vec_grid[base_x+x][base_y+y].a) as i32;
                }
            }
            c.a = (alpha as usize / FACTOR / FACTOR) as u8;
            r_grid[i][j] = c;
        }
    }
    return r_grid;
}

fn draw_pixel(vec_grid: &mut Vec<Vec<Color>>, x: usize, y:usize, color: &Color, alpha: u8) {
    let mut c = (*color).clone();
    c.a = alpha;
    vec_grid[x][y] = c;
}

fn draw_line(vec_grid: &mut Vec<Vec<Color>>, x1:usize,y1:usize,x2:usize,y2:usize, _width: usize, color: &Color) {
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
            draw_pixel(vec_grid, cy as usize, cx as usize, color, 255 as u8);
        }
        else {
            draw_pixel(vec_grid, cx as usize, cy as usize, color, 255 as u8);
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