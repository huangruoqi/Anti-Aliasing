use ferrux_canvas::color::{Color, palette};

pub fn fxaa(width: usize, height: usize, vec_grid: &Vec<Vec<Color>>) -> Vec<Vec<Color>>{
    let mut r_grid = vec![vec![palette::BLACK; width]; height];
    for i in 0..width {
        r_grid[i][0] = vec_grid[i][0].clone();
        r_grid[i][height - 1] = vec_grid[i][height - 1].clone();
    }
    for i in 0..height {
        r_grid[0][i] = vec_grid[0][i].clone();
        r_grid[width - 1][i] = vec_grid[width - 1][i].clone();
    }
    for i in 1..width-1 {
        for j in 1..height-1 {
            r_grid[i][j] = fxaa_helper(i,j,&vec_grid, width, height);
        }
    }
    return r_grid;
}

const FXAA_SPAN_MAX  : f64 = 8.0; 
const FXAA_REDUCE_MIN: f64 = 1.0/128.0;
const FXAA_REDUCE_MUL: f64 = 1.0/8.0;
fn fxaa_helper(i:usize, j:usize, vec_grid: &Vec<Vec<Color>>, width:usize, height:usize) -> Color {
    let lumi_md = get_luminosity(&vec_grid[i][j]);       // middle
    let lumi_tl = get_luminosity(&vec_grid[i-1][j-1]);   // top left
    let lumi_tr = get_luminosity(&vec_grid[i+1][j-1]);   // top right
    let lumi_bl = get_luminosity(&vec_grid[i-1][j+1]);   // bottom left
    let lumi_br = get_luminosity(&vec_grid[i+1][j+1]);   // bottom right
    let mut direction_x = -((lumi_tl + lumi_tr) - (lumi_bl + lumi_br));
    let mut direction_y = (lumi_tl + lumi_bl) - (lumi_tr + lumi_br);
    let mut direction_reduce = (lumi_tl + lumi_tr + lumi_bl + lumi_br) * (FXAA_REDUCE_MUL * 0.25);
    if direction_reduce < FXAA_REDUCE_MIN { direction_reduce = FXAA_REDUCE_MIN; }
    let mut ratio = 1.0/(abs(direction_x)+direction_reduce);
    if abs(direction_y) < abs(direction_x) { ratio = 1.0/(abs(direction_y)+direction_reduce); }
    direction_x *= ratio;
    direction_y *= ratio;
    direction_x = clamp(direction_x, -FXAA_SPAN_MAX, FXAA_SPAN_MAX);
    direction_y = clamp(direction_y, -FXAA_SPAN_MAX, FXAA_SPAN_MAX);

    let r_1_x_1 = clamp((i as f64 + direction_x * (1.0/3.0 - 0.5)), 0.0, (width-1)  as f64) as usize;
    let r_1_y_1 = clamp((j as f64 + direction_y * (1.0/3.0 - 0.5)), 0.0, (height-1) as f64) as usize;
    let r_1_x_2 = clamp((i as f64 + direction_x * (2.0/3.0 - 0.5)), 0.0, (width-1)  as f64) as usize;
    let r_1_y_2 = clamp((j as f64 + direction_y * (2.0/3.0 - 0.5)), 0.0, (height-1) as f64) as usize;
    let r_1_c_1 = vec_grid[r_1_x_1][r_1_y_1].clone();
    let r_1_c_2 = vec_grid[r_1_x_2][r_1_y_2].clone();
    let mut result_1 = palette::BLACK;
    result_1.r = ((r_1_c_1.r as i32 + r_1_c_2.r as i32)/ 2) as u8;
    result_1.g = ((r_1_c_1.g as i32 + r_1_c_2.g as i32)/ 2) as u8;
    result_1.b = ((r_1_c_1.b as i32 + r_1_c_2.b as i32)/ 2) as u8;

    let r_2_x_1 = clamp((i as f64 + direction_x * (0.0/3.0 - 0.5)), 0.0, (width-1)  as f64) as usize;
    let r_2_y_1 = clamp((j as f64 + direction_y * (0.0/3.0 - 0.5)), 0.0, (height-1) as f64) as usize;
    let r_2_x_2 = clamp((i as f64 + direction_x * (3.0/3.0 - 0.5)), 0.0, (width-1)  as f64) as usize;
    let r_2_y_2 = clamp((j as f64 + direction_y * (3.0/3.0 - 0.5)), 0.0, (height-1) as f64) as usize;
    let r_2_c_1 = vec_grid[r_2_x_1][r_2_y_1].clone();
    let r_2_c_2 = vec_grid[r_2_x_2][r_2_y_2].clone();
    let mut result_2 = palette::BLACK;
    result_2.r = ((r_2_c_1.r as i32 + r_2_c_2.r as i32)/ 2) as u8;
    result_2.g = ((r_2_c_1.g as i32 + r_2_c_2.g as i32)/ 2) as u8;
    result_2.b = ((r_2_c_1.b as i32 + r_2_c_2.b as i32)/ 2) as u8;
    let lumi_r2 = get_luminosity(&result_2);

    let lumi_min = min(lumi_md, min(min(lumi_tl, lumi_tr),min(lumi_bl, lumi_br)));
    let lumi_max = max(lumi_md, max(max(lumi_tl, lumi_tr),max(lumi_bl, lumi_br)));
    if lumi_min > lumi_r2 || lumi_max < lumi_r2 {
        return result_1;
    }
    return result_2;
}

fn abs(n: f64) -> f64 {
    if n < 0.0 { return -n; }
    return n;
}
fn clamp(n:f64, low:f64, high:f64) ->f64 {
    if n < low  { return low;  }
    if n > high { return high; }
    return n;
}
fn min(a:f64, b:f64) -> f64 {
    if a < b { return a; }
    return b;
}
fn max(a:f64, b:f64) -> f64 {
    if a > b { return a; }
    return b;
}

const LUMI_R: f64 = 0.299;
const LUMI_G: f64 = 0.587;
const LUMI_B: f64 = 0.114;
fn get_luminosity(c: &Color) -> f64{
    return (c.r as f64)/255.0*LUMI_R+ 
           (c.g as f64)/255.0*LUMI_G+ 
           (c.b as f64)/255.0*LUMI_B;
}
