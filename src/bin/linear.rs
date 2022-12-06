use macroquad::prelude::*;
async fn linear() {
    let render_target = render_target(500, 150);
    render_target.texture.set_filter(FilterMode::Linear);
    loop {
        set_camera(&Camera2D {
            zoom: vec2(0.01, 0.01),
            target: vec2(0.0, 0.0),
            render_target: Some(render_target),
            ..Default::default()
        });
        clear_background(BLACK);
        draw_line(-30.0, 30.0, 30.0, -30.0, 15.0, BLUE);
        set_default_camera();
        draw_texture_ex(
            render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        
        gl_use_default_material();
        next_frame().await;
    }
}
#[macroquad::main("BasicShapes")]
async fn main(){
    linear().await;
}