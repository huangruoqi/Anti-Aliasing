use ferrux_canvas::color::{Color, palette};
use std::mem;

const FACTOR: usize = 2 as usize;

pub fn SSAA(width: usize, height: usize, pairs: &mut Vec<Vec<usize>>) -> Vec<Vec<Color>>{
    let mut s_grid = vec![vec![palette::BLACK; width * FACTOR]; height * FACTOR];
    for i in pairs{
        draw_line(&mut s_grid, i[0], i[1], i[2], i[3], 1 as usize, &palette::WHITE);
    }
    let mut r_grid = vec![vec![palette::BLACK; width]; height];
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