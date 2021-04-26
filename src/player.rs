use crate::{NUM_COLS, NUM_ROWS};
use crate::frame::{Drawable, Frame};
use crate::shot::Shot;
use std::time::Duration;
use crate::invaders::Invaders;

// This structure contains operations of player
pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>
}

impl Player {

    // creates new player
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS -1,
            shots: Vec::new()
        }
    }

    // move player to left
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    // move player to right
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS -1 {
            self.x += 1;
        }
    }

    // shoot enemies
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 1 {
            self.shots.push(Shot::new(self.x, self.y -1));
            true
        } else {
            false
        }
    }

    // update player's activity
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }

    // detect bullet hits enemy or not
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    hit_something = true;
                }
            }
        }
        hit_something
    }
}

// create player's image
impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
