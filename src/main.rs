use macroquad::prelude::*;

type Point = (i16, i16);

const UNITS: i16 = 32;

struct Player {
    length: f32,
    width: f32,
    vel: f32,
    pos: (f32, f32),
}

struct Ball {
    vel: (f32, f32),
    pos: (f32, f32),
    size: f32,
}

impl Ball {
    fn new(pos: (f32, f32)) -> Ball {
        let x = if rand::gen_range(0., 1.) > 0.5 {
            -5.
        } else {
            5.
        };
        let mut y = 5. * rand::gen_range(0., 1.);
        y *= if rand::gen_range(0., 1.) > 0.5 {
            1.
        } else {
            -1.
        };
        Ball {
            vel: (x, y),
            pos,
            size: 1.0,
        }
    }

    fn is_inside(&self, bounds: (f32, f32, f32, f32), scale: f32) -> bool {
        let (x, y, w, h) = bounds;
        if self.pos.0 + self.size * scale < x {
            return false;
        } else if self.pos.0 + self.size * scale > x + w {
            return false;
        }

        true
    }

    fn move_self(&mut self, bounds: (f32, f32, f32, f32), scale: f32) {
        let (x, y, w, h) = bounds;
        let mut newPos = (self.pos.0 + self.vel.0, self.pos.1 + self.vel.1);
        if newPos.1 < y {
            newPos.1 = y;
            self.vel.1 *= -1.0;
        } else if newPos.1 + self.size * scale > h {
            newPos.1 = h - self.size * scale;
            self.vel.1 *= -1.0;
        }

        self.pos = newPos;
    }
}
enum RunState {
    Start,
    Running,
    GameOver,
}

#[macroquad::main("Pong")]
async fn main() {
    let font_size = 30.;
    let mut player = Player {
        length: 6.0,
        width: 1.0,
        vel: 10.,
        pos: (10.0, screen_height() / 2. - 6.0),
    };

    let ball_start = (screen_width() / 2., screen_height() / 2.);
    let mut ball = Ball::new(ball_start);

    let mut player_score = 0;
    let mut comp_score = 0;

    let mut last_update = get_time();
    let mut run_state = RunState::Start;

    let up = (0, -1);
    let down = (0, 1);

    request_new_screen_size(600.0, 300.0);

    loop {
        clear_background(WHITE);
        let width = f32::max(screen_width(), 600.0);
        let height = f32::max(screen_height(), 300.0);
        let offset_x = 0.;
        let offset_y = 0.;
        let height_unit = (height - offset_y * 2.) / UNITS as f32;

        match run_state {
            RunState::Start => {
                let text = "Press [enter] to start Pong.";
                let text_size = measure_text(text, None, font_size as _, 1.0);
                draw_text(
                    text,
                    width / 2. - text_size.width / 2.,
                    height / 2. - text_size.height / 2.,
                    font_size,
                    BLACK,
                );
                if is_key_down(KeyCode::Enter) {
                    run_state = RunState::Running;
                }
            }
            RunState::Running => {
                //handle inputs
                last_update = get_time();

                if is_key_down(KeyCode::Up) {
                    player.pos.1 = clamp(
                        player.pos.1 - player.vel,
                        offset_y,
                        height - player.length * height_unit,
                    );
                }
                if is_key_down(KeyCode::Down) {
                    player.pos.1 = clamp(
                        player.pos.1 + player.vel,
                        offset_y,
                        height - player.length * height_unit,
                    );
                }

                if ball.is_inside((offset_x, offset_y, width, height), height_unit) {
                    ball.move_self((offset_x, offset_y, width, height), height_unit);
                } else {
                    ball = Ball::new(ball_start);
                }

                // draw scores
                draw_text(
                    format!("{}", player_score).as_str(),
                    offset_x + 10.,
                    offset_y + 20.,
                    40.,
                    BLACK,
                );

                // draw players
                draw_rectangle(
                    offset_x + player.pos.0,
                    offset_y + player.pos.1,
                    player.width * height_unit,
                    player.length * height_unit,
                    BLACK,
                );

                draw_rectangle(
                    ball.pos.0,
                    ball.pos.1,
                    ball.size as f32 * height_unit,
                    ball.size as f32 * height_unit,
                    BLACK,
                )
            }
            RunState::GameOver => {
                clear_background(WHITE);
                let text = "You are a loser. Press [enter] to try again.";
                let text_size = measure_text(text, None, font_size as _, 1.0);

                draw_text(
                    text,
                    width / 2. - text_size.width / 2.,
                    height / 2. - text_size.height / 2.,
                    font_size,
                    BLACK,
                );
                if is_key_down(KeyCode::Enter) {
                    player_score = 0;
                    comp_score = 0;
                    run_state = RunState::Running;
                    last_update = get_time();
                }
            }
        }

        next_frame().await
    }
}
