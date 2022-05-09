use macroquad::prelude::*;

type Point = (f32, f32);

const UNITS: i16 = 32;

const FONT_SIZE: f32 = 30.;
const START_HEIGHT: f32 = 300.0;
const START_WIDTH: f32 = 600.0;

const BG_FIELD_THICKNESS: f32 = 10.0;

const GAME_END_SCORE: i8 = 10;

struct Player {
    length: f32,
    width: f32,
    vel: f32,
    pos: (f32, f32),
}

impl Player {
    fn center_pos(&self, scale: f32) -> Point {
        let p: Point = (
            self.pos.0 + (self.width * scale / 2.),
            self.pos.1 + (self.length * scale / 2.),
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
            self.pos.0 + (self.size * scale / 2.),
            self.pos.1 + (self.size * scale / 2.),
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

struct GameState {
    player: Player,
    computer: Player,
    ball: Ball,
    player_score: i8,
    comp_score: i8,
    last_update: f64,
    run_state: RunState,
    width: f32,
    height: f32,
    offset_x: f32,
    offset_y: f32,
    scale: f32,
}

fn handle_input(gs: &mut GameState) {
    match gs.run_state {
        RunState::Start => {
            //start game
            if is_key_down(KeyCode::Enter) {
                gs.run_state = RunState::Running;
            }
        }
        RunState::Running => {
            //movement
            if is_key_down(KeyCode::Up) {
                gs.player.pos.1 = clamp(
                    gs.player.pos.1 - gs.player.vel,
                    gs.offset_y,
                    gs.height - gs.player.length * gs.scale,
                );
            }
            if is_key_down(KeyCode::Down) {
                gs.player.pos.1 = clamp(
                    gs.player.pos.1 + gs.player.vel,
                    gs.offset_y,
                    gs.height - gs.player.length * gs.scale,
                );
            }
        }
        RunState::GameOver => {
            //restart game
            if is_key_down(KeyCode::Enter) {
                gs.player_score = 0;
                gs.comp_score = 0;
                gs.run_state = RunState::Running;
                gs.last_update = get_time();
            }
        }
    }
}

fn update(gs: &mut GameState) {
    let ball_start = (screen_width() / 2., screen_height() / 2.);
    match gs.run_state {
        RunState::Running => {
            gs.last_update = get_time();
            gs.computer.pos.0 = gs.width - gs.computer.width * gs.scale - 10.;

            if gs.comp_score == GAME_END_SCORE || gs.player_score == GAME_END_SCORE {
                gs.run_state = RunState::GameOver;
                return;
            }

            let ball_placement = gs
                .ball
                .is_out_of_bounds((gs.offset_x, gs.offset_y, gs.width, gs.height), gs.scale);
            if ball_placement == 0 {
                gs.ball.move_self(
                    (gs.offset_x, gs.offset_y, gs.width, gs.height),
                    &gs.player,
                    &gs.computer,
                    gs.scale,
                );
            } else {
                if ball_placement < 0 {
                    gs.comp_score += 1;
                } else {
                    gs.player_score += 1;
                }
                gs.ball = Ball::new(ball_start);
            }

            // move computer
            let disp = get_frame_time() * gs.computer.vel * gs.scale;
            // if ball is moving left the computer resets to center position
            let comp_center_y = gs.computer.center_pos(gs.scale).1;
            if gs.ball.vel.0 < 0.0 {
                if comp_center_y < (gs.height / 2.0) {
                    gs.computer.pos.1 = clamp(
                        gs.computer.pos.1 + disp,
                        gs.offset_y,
                        gs.height / 2.0 - (gs.computer.length * gs.scale / 2.0),
                    );
                } else if comp_center_y > gs.height / 2.0 {
                    gs.computer.pos.1 = clamp(
                        gs.computer.pos.1 - disp,
                        gs.offset_y,
                        gs.height - (gs.computer.length * gs.scale / 2.0),
                    );
                }
            // if ball is moving towards computer attempt to intercept
            } else {
                let ball_cy = gs.ball.center_pos(gs.scale).1;
                if comp_center_y > ball_cy {
                    gs.computer.pos.1 = clamp(gs.computer.pos.1 - disp, gs.offset_y, gs.height);
                } else if ball_cy > comp_center_y {
                    gs.computer.pos.1 = clamp(
                        gs.computer.pos.1 + disp,
                        gs.offset_y,
                        gs.height - (gs.computer.length * gs.scale / 2.0),
                    );
                }
            }
        }
        _ => {}
    }
}

fn draw(gs: &GameState) {
    clear_background(WHITE);

    match gs.run_state {
        RunState::Start => {
            let text = "Press [enter] to start Pong.";
            let text_size = measure_text(text, None, FONT_SIZE as _, 1.0);
            draw_text(
                text,
                gs.width / 2. - text_size.width / 2.,
                gs.height / 2. - text_size.height / 2.,
                FONT_SIZE,
                BLACK,
            );
        }
        RunState::Running => {
            //draw background
            //top line
            draw_line(
                gs.offset_x,
                gs.offset_y,
                gs.width,
                gs.offset_y,
                BG_FIELD_THICKNESS,
                BLACK,
            );
            //bottom line
            draw_line(
                gs.offset_x,
                gs.height,
                gs.width,
                gs.height,
                BG_FIELD_THICKNESS,
                BLACK,
            );

            //center separator
            draw_line(
                gs.width / 2.0 - BG_FIELD_THICKNESS / 2.0,
                gs.offset_y,
                gs.width / 2.0 - BG_FIELD_THICKNESS / 2.0,
                gs.height,
                BG_FIELD_THICKNESS,
                BLACK,
            );
            for i in 1..=10 {
                draw_line(
                    gs.offset_x,
                    gs.offset_y + (gs.height / 10.0 * i as f32),
                    gs.width,
                    gs.offset_y + (gs.height / 10.0 * i as f32),
                    BG_FIELD_THICKNESS,
                    WHITE,
                );
            }

            // draw scores
            let score_size = 40.0;
            let player_score_text = format!("{}", gs.player_score).to_string();
            let player_text_size = measure_text(&player_score_text, None, score_size as _, 1.0);

            draw_text(
                &player_score_text,
                gs.width / 2.0 - 27. - player_text_size.width,
                gs.offset_y + 40.,
                score_size,
                BLACK,
            );

            draw_text(
                format!("{}", gs.comp_score).as_str(),
                gs.width / 2.0 + 20.,
                gs.offset_y + 40.,
                score_size,
                BLACK,
            );

            // draw players
            draw_rectangle(
                gs.offset_x + gs.player.pos.0,
                gs.offset_y + gs.player.pos.1,
                gs.player.width * gs.scale,
                gs.player.length * gs.scale,
                BLACK,
            );
            draw_rectangle(
                gs.offset_x + gs.computer.pos.0,
                gs.offset_y + gs.computer.pos.1,
                gs.computer.width * gs.scale,
                gs.computer.length * gs.scale,
                BLACK,
            );

            draw_rectangle(
                gs.ball.pos.0,
                gs.ball.pos.1,
                gs.ball.size as f32 * gs.scale,
                gs.ball.size as f32 * gs.scale,
                BLACK,
            );
        }
        RunState::GameOver => {
            clear_background(WHITE);
            let text = if gs.comp_score == GAME_END_SCORE {
                "You are a loser. Press [enter] to try again."
            } else {
                "You win. Congratulations."
            };
            let text_size = measure_text(text, None, FONT_SIZE as _, 1.0);

            draw_text(
                text,
                gs.width / 2. - text_size.width / 2.,
                gs.height / 2. - text_size.height / 2.,
                FONT_SIZE,
                BLACK,
            );
        }
    }
}

#[macroquad::main("Pong")]
async fn main() {
    let mut gs = GameState {
        player: Player {
            length: 6.0,
            width: 1.0,
            vel: 10.,
            pos: (10.0, START_HEIGHT / 2. - 6.0),
        },
        computer: Player {
            length: 6.0,
            width: 1.0,
            vel: 10.,
            pos: (START_WIDTH - 11., START_HEIGHT / 2. - 6.0),
        },
        ball: Ball::new((START_WIDTH / 2.0, START_HEIGHT / 2.0)),
        player_score: 0,
        comp_score: 0,
        last_update: get_time(),
        run_state: RunState::Start,
        width: screen_width(),
        height: screen_height(),
        offset_x: 0.0,
        offset_y: 0.0,
        scale: screen_height() / UNITS as f32,
    };

    request_new_screen_size(START_WIDTH, START_HEIGHT);

    loop {
        gs.width = screen_width();
        gs.height = screen_height();
        gs.scale = (screen_height() - gs.offset_y * 2.) / UNITS as f32;

        handle_input(&mut gs);
        update(&mut gs);
        draw(&gs);

        next_frame().await
    }
}
