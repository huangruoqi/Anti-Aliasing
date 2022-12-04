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
            let mut input = vec![
                vec_grid[i][j].clone(),       // middle
                vec_grid[i-1][j+1].clone(),   // top left
                vec_grid[i+1][j+1].clone(),   // top right
                vec_grid[i-1][j-1].clone(),   // bottom left
                vec_grid[i+1][j-1].clone(),   // bottom right
            ];
            r_grid[i][j] = fxaa_helper(input);
        }
    }
    return r_grid;
}
fn fxaa_helper(input: Vec<Color>) -> Color {
    return palette::BLACK;
}
