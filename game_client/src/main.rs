use macroquad::prelude::*;

struct Snek {
    rect: Rect,
    velocity: Vec2,
}

struct Barrier {
    rect: Rect,
}

#[macroquad::main("SwimmySnek")]
async fn main() {
    let mut camera_position = vec2(0.,0.);
    let mut barriers = Vec::new();
    barriers.push(Barrier{rect: Rect{x: 0., y:-10., w: 200., h: 10.}});

    let mut snek = Snek{rect: Rect{x: 0., y: 0., w: 1., h: 1.}, velocity: vec2(0., 0.)};
    loop {
        // update logic
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
        if is_key_pressed(KeyCode::Space) {
            snek.velocity.y = 10.;
        }

        // apply gravity to the snek
        snek.velocity.y -= 9.8 * get_frame_time();

        // move snek based on its velocity, with gravity accelerating down.
        snek.rect.x = snek.rect.x + snek.velocity.x * get_frame_time();
        snek.rect.y = snek.rect.y + snek.velocity.y * get_frame_time();

        // did the snek hit the barrier?
        for b in &barriers {
            // for some reason macroquad bottom and top are flipped?!?
            if b.rect.bottom() >= snek.rect.top() {
                snek.rect.y += b.rect.bottom() - snek.rect.top();
                snek.velocity.y *= -0.5;
            }
        }

        // begin rendering
        clear_background(LIGHTGRAY);

        // draw all world based things
        set_camera(&Camera2D {
           zoom: vec2(1. / screen_width() * 50., 1. / screen_height() * 50.),
            
            target: camera_position,
            ..Default::default()
        });

        // draw the snek
        draw_rectangle(snek.rect.x, snek.rect.y, snek.rect.w, snek.rect.h, GREEN);

        // draw barriers
        for b in &barriers {
            draw_rectangle(b.rect.x, b.rect.y, b.rect.w, b.rect.h, BROWN);
        }

        next_frame().await
    }
}