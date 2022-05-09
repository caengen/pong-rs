use macroquad::prelude::*;

type Point = (f32, f32);

const UNITS: i16 = 32;

struct Player {
    length: f32,
    width: f32,
    vel: f32,
    pos: (f32, f32),
}

impl Player {
    fn center_pos(&self, scale: f32) -> Point {
        let p: Point = (
            (self.pos.0 + self.width * scale) / 2.,
            (self.pos.1 + self.length * scale) / 2.,
        );
        p
    }
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

    fn center_pos(&self, scale: f32) -> Point {
        let p: Point = (
            (self.pos.0 + self.size * scale) / 2.,
            (self.pos.1 + self.size * scale) / 2.,
        );
        p
    }

    fn is_out_of_bounds(&self, bounds: (f32, f32, f32, f32), scale: f32) -> i8 {
        let (x, y, w, h) = bounds;
        if self.pos.0 + self.size * scale < x {
            return -1;
        } else if self.pos.0 + self.size * scale > x + w {
            return 1;
        }

        0
    }

    fn move_self(
        &mut self,
        bounds: (f32, f32, f32, f32),
        player: &Player,
        computer: &Player,
        scale: f32,
    ) {
        let (x, y, w, h) = bounds;
        let mut new_pos = (self.pos.0 + self.vel.0, self.pos.1 + self.vel.1);
        if new_pos.1 < y {
            new_pos.1 = y;
            self.vel.1 *= -1.0;
        } else if new_pos.1 + self.size * scale > h {
            new_pos.1 = h - self.size * scale;
            self.vel.1 *= -1.0;
        }

        let rect1 = (new_pos.0, new_pos.1, self.size * scale, self.size * scale);
        if intersects(
            rect1,
            (
                player.pos.0,
                player.pos.1,
                player.width * scale,
                player.length * scale,
            ),
        ) {
            new_pos.0 = player.pos.0 + player.width * scale;
            self.vel.0 *= -1.05;
        } else if intersects(
            rect1,
            (
                computer.pos.0,
                computer.pos.1,
                computer.width * scale,
                computer.length * scale,
            ),
        ) {
            new_pos.0 = computer.pos.0 - self.size * scale;
            self.vel.0 *= -1.05;
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
const START_HEIGHT: f32 = 300.0;
const START_WIDTH: f32 = 600.0;
const BG_FIELD_THICKNESS: f32 = 10.0;

#[macroquad::main("Pong")]
async fn main() {
    let font_size = 30.;
    let mut player = Player {
        length: 6.0,
        width: 1.0,
        vel: 10.,
        pos: (10.0, START_HEIGHT / 2. - 6.0),
    };

    let mut computer = Player {
        length: 6.0,
        width: 1.0,
        vel: 10.,
        pos: (START_WIDTH - 11., START_HEIGHT / 2. - 6.0),
    };

    let ball_start = (START_WIDTH / 2., START_HEIGHT / 2.);
    let mut ball = Ball::new(ball_start);

    let mut player_score = 0;
    let mut comp_score = 0;

    let mut last_update = get_time();
    let mut run_state = RunState::Start;

    request_new_screen_size(START_WIDTH, START_HEIGHT);

    loop {
        clear_background(WHITE);

        let width = screen_width();
        let height = screen_height();
        let offset_x = 0.;
        let offset_y = 0.;
        let height_unit = (height - offset_y * 2.) / UNITS as f32;
        computer.pos.0 = width - computer.width * height_unit - 10.;

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

                if comp_score == 10 || player_score == 10 {
                    run_state = RunState::GameOver;
                    continue;
                }

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

                let ball_placement =
                    ball.is_out_of_bounds((offset_x, offset_y, width, height), height_unit);
                if ball_placement == 0 {
                    ball.move_self(
                        (offset_x, offset_y, width, height),
                        &player,
                        &computer,
                        height_unit,
                    );
                } else {
                    if ball_placement < 0 {
                        comp_score += 1;
                    } else {
                        player_score += 1;
                    }
                    ball = Ball::new(ball_start);
                }

                // move computer

                let disp = get_frame_time() * computer.vel * height_unit;
                if ball.vel.0 < 0.0 {
                    if computer.center_pos(height_unit).1 < height / 2.0 {
                        computer.pos.1 = clamp(
                            computer.pos.1 + disp,
                            offset_y,
                            height / 2.0 - (computer.length * height_unit / 2.0),
                        );
                    } else if computer.center_pos(height_unit).1 > height / 2.0 {
                        computer.pos.1 = clamp(
                            computer.pos.1 - disp,
                            height / 2.0 - (computer.length * height_unit / 2.0),
                            height - (computer.length * height_unit / 2.0),
                        );
                    }
                } else {
                    let ballCY = ball.center_pos(height_unit).1 + ball.vel.1 * height_unit;
                    let compCY = computer.center_pos(height_unit).1;
                    if compCY - ballCY > 0. && compCY - ballCY > 10.0 {
                        computer.pos.1 = clamp(
                            computer.pos.1 - disp,
                            offset_y,
                            height - computer.length * height_unit,
                        );
                    } else if ballCY - compCY > 0. && ballCY - compCY > 10.0 {
                        computer.pos.1 = clamp(
                            computer.pos.1 + disp,
                            offset_y,
                            height - computer.length * height_unit,
                        );
                    }
                }

                //draw background
                //top line
                draw_line(
                    offset_x,
                    offset_y,
                    width,
                    offset_y,
                    BG_FIELD_THICKNESS,
                    BLACK,
                );
                //bottom line
                draw_line(offset_x, height, width, height, BG_FIELD_THICKNESS, BLACK);

                //center separator
                draw_line(
                    width / 2.0 - BG_FIELD_THICKNESS / 2.0,
                    offset_y,
                    width / 2.0 - BG_FIELD_THICKNESS / 2.0,
                    height,
                    BG_FIELD_THICKNESS,
                    BLACK,
                );
                for i in 1..=10 {
                    draw_line(
                        offset_x,
                        offset_y + (height / 10.0 * i as f32),
                        width,
                        offset_y + (height / 10.0 * i as f32),
                        BG_FIELD_THICKNESS,
                        WHITE,
                    );
                }

                // draw scores
                let score_size = 40.0;
                let player_score_text = format!("{}", player_score).to_string();
                let player_text_size = measure_text(&player_score_text, None, score_size as _, 1.0);

                draw_text(
                    &player_score_text,
                    width / 2.0 - 27. - player_text_size.width,
                    offset_y + 40.,
                    score_size,
                    BLACK,
                );

                draw_text(
                    format!("{}", comp_score).as_str(),
                    width / 2.0 + 20.,
                    offset_y + 40.,
                    score_size,
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
                    offset_x + computer.pos.0,
                    offset_y + computer.pos.1,
                    computer.width * height_unit,
                    computer.length * height_unit,
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
                let text = if comp_score == 10 {
                    "You are a loser. Press [enter] to try again."
                } else {
                    "You win. Congratulations."
                };
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
