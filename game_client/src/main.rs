use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut camera_position = vec2(0.,0.);
    loop {
        if is_key_pressed(KeyCode::Right) {
            camera_position.x +=  1.;
        }
        if is_key_pressed(KeyCode::Left) {
            camera_position.x -=  1.;
        }
        if is_key_pressed(KeyCode::Up) {
            camera_position.y -=  1.;
        }
        if is_key_pressed(KeyCode::Down) {
            camera_position.y +=  1.;
        }

        clear_background(RED);

        set_camera(&Camera2D {
           zoom: vec2(1. / screen_width() * 2., -1. / screen_height() * 2.),
            
            target: camera_position,
            ..Default::default()
        });

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(0., 0., 1.0, 1.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}