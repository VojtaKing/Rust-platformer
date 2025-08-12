use macroquad::{
    camera,
    prelude::{scene::camera_pos, *},
};

fn collide_and_fix(
    x1: f32,
    y1: f32,
    w1: f32,
    h1: f32,
    x2: f32,
    y2: f32,
    w2: f32,
    h2: f32,
    velocity_x: f32,
    velocity_y: f32,
) -> Option<(f32, f32)> {
    if !(x1 + w1 < x2 || x1 > x2 + w2 || y1 + h1 < y2 || y1 > y2 + h2) {
        let overlap_left = (x1 + w1) - x2;
        let overlap_right = (x2 + w2) - x1;
        let overlap_top = (y1 + h1) - y2;
        let overlap_bottom = (y2 + h2) - y1;

        let min_overlap = overlap_left
            .min(overlap_right)
            .min(overlap_top)
            .min(overlap_bottom);

        if min_overlap == overlap_left {
            Some((x1 - overlap_left, y1))
        } else if min_overlap == overlap_right {
            Some((x1 + overlap_right, y1))
        } else if min_overlap == overlap_top {
            Some((x1, y1 - overlap_top))
        } else {
            Some((x1, y1 + overlap_bottom))
        }
    } else {
        None
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut x = 100.0;
    let mut y = 100.0;
    let size = 25.0;
    let jump_strength = -300.0;
    let ground = 600.0 - size;
    let mut velocity_y = 0.0;
    let mut velocity_x = 0.0;
    let gravity = 500.0;
    let speed = 200.0;

    let x2 = 250.0;
    let y2 = 550.0;
    let x3 = 450.0;
    let y3 = 500.0;
    let platform_w = 100.0;
    let platform_h = 20.0;
    let dt = get_frame_time();
    let mut camera = Camera2D {
        target: vec2(x, y),
        zoom: vec2(0.7 / screen_width() * 2.0, 0.7 / screen_height() * 2.0),
        ..Default::default()
    };

    loop {
        set_default_camera();
        if is_key_pressed(KeyCode::Space) && velocity_y == 0.0 {
            velocity_y = jump_strength;
        }

        if is_key_down(KeyCode::D) {
            velocity_x = speed;
        } else if is_key_down(KeyCode::A) {
            velocity_x = -speed;
        } else {
            velocity_x = 0.0;
        }

        velocity_y += gravity * dt;
        y += velocity_y * dt;
        x += velocity_x * dt;
        camera.target = vec2(x, y);

        if let Some((new_x, new_y)) = collide_and_fix(
            x, y, size, size, x2, y2, platform_w, platform_h, velocity_x, velocity_y,
        ) {
            x = new_x;
            y = new_y;
            velocity_x = 0.0;
            velocity_y = 0.0;
        }
        if let Some((new_x, new_y)) = collide_and_fix(
            x, y, size, size, x3, y3, platform_w, platform_h, velocity_x, velocity_y,
        ) {
            x = new_x;
            y = new_y;
            velocity_x = 0.0;
            velocity_y = 0.0;
        }

        if y >= ground {
            y = ground;
            velocity_y = 0.0;
        }

        clear_background(BLACK);
        set_camera(&camera);
        draw_rectangle(x2, y2, platform_w, platform_h, GREEN);
        draw_rectangle(x3, y3, platform_w, platform_h, GREEN);
        draw_rectangle(x, y, size, size, RED);
        draw_line(-10000.0, 600.0, 10000000.0, 600.0, 5.0, GRAY);
        next_frame().await;
    }
}
