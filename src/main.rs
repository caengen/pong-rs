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

    fn move_self(&mut self, bounds: (f32, f32, f32, f32), player: &Player, scale: f32) {
        let (x, y, w, h) = bounds;
        let mut new_pos = (self.pos.0 + self.vel.0, self.pos.1 + self.vel.1);
        if new_pos.1 < y {
            new_pos.1 = y;
            self.vel.1 *= -1.0;
        } else if new_pos.1 + self.size * scale > h {
            new_pos.1 = h - self.size * scale;
            self.vel.1 *= -1.0;
        }

        if intersects(
            (new_pos.0, new_pos.1, self.size * scale, self.size * scale),
            (
                player.pos.0,
                player.pos.1,
                player.width * scale,
                player.length * scale,
            ),
        ) {
            new_pos.0 = player.pos.0 + player.width * scale;
            self.vel.0 *= -1.;
        }

        self.pos = new_pos;
    }
}
enum RunState {
    Start,
    Running,
    GameOver,
}

fn intersects(r1: (f32, f32, f32, f32), r2: (f32, f32, f32, f32)) -> bool {
    let leftX = f32::max(r1.0, r2.0);
    let rightX = f32::min(r1.0 + r1.2, r2.0 + r2.2);
    let topY = f32::max(r1.1, r2.1);
    let bottomY = f32::min(r1.1 + r1.3, r2.1 + r2.3);
    if leftX < rightX && topY < bottomY {
        return true;
    }

    false
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
                    ball.move_self((offset_x, offset_y, width, height), &player, height_unit);
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
