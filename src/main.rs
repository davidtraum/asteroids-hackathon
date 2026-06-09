use macroquad::prelude::*;

mod structs;
mod traits;

use structs::astroid::Astroid;
use structs::spaceship::Spaceship;
use traits::drawable::Drawable;
use traits::updatable::Updatable;

fn window_conf() -> Conf {
    Conf {
        window_title: "Astroid Rust - Macroquad Hello".to_string(),
        window_width: 960,
        window_height: 540,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut astroids = vec![];
    let mut spaceship = Spaceship::new();

    let mut pos = vec2(screen_width() * 0.5, screen_height() * 0.5);
    let mut velocity = vec2(160.0, 110.0);
    let radius = 32.0;

    for _ in 0..5 {
        astroids.push(Astroid::new_at_random_position())
    }

    loop {
        let dt = get_frame_time();

        // update
        spaceship.update();

        for astroid in &mut astroids {
            astroid.update();
        }

        // draw
        clear_background(Color::from_rgba(16, 20, 28, 255));
        spaceship.draw();
        for astroid in &mut astroids {
            astroid.draw();
        }

        // hud
        let fps = format!("FPS: {}", get_fps());
        draw_text(&fps, 32.0, screen_height() - 32.0, 24.0, GRAY);

        // ...
        next_frame().await;
    }
}
