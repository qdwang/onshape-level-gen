//! This file contains implementations for Wall module.
//!
use super::*;
use rand::rngs::ThreadRng;

impl Wall {
    pub fn new(
        note: &Note,
        rng: &mut ThreadRng,
        time2next: f32,
        acc_coin_time: &mut f32,
        prev_wall: &mut Option<Wall>,
    ) -> Self {
        const TIME2NEXT_SPAN : f32 = 0.8f32;
        const MAX_COIN_TIME : f32 = 3f32;
        const TIME2HIT : f32 = 2f32;

        let select: i32 = if time2next < TIME2NEXT_SPAN && *acc_coin_time < MAX_COIN_TIME {
            *acc_coin_time += time2next;
            3
        } else {
            *acc_coin_time = 0f32;
            if time2next > TIME2HIT {
                1
            } else {
                rng.gen_range(0..=2)
            }
        };

        let wall_type = match select {
            0 => WallType::Shape {
                hands_over: rand::random(),
                position: rand::random(),
                lean: rand::random(),
                standing: rand::random(),
                leg_lunge: rand::random(),
            },
            1 => WallType::Hit {
                position: rand::random(),
                standing: rand::random(),
                hands: rand::random(),
            },
            2 => {
                let d = (time2next * 100.) as u16;
                WallType::Dodge {
                    t: rand::random(),
                    duration: d.saturating_sub(10),
                }
            }
            _ => {
                let (range_x, range_y) = if let Some(Wall {
                    time: _,
                    t: WallType::Coin { x, y },
                }) = prev_wall
                {
                    ((*x - 1..=*x + 1), (*y - 1..=*y + 1))
                } else {
                    ((-5..=5), (3..=10))
                };
                WallType::Coin {
                    x: rng.gen_range(range_x),
                    y: rng.gen_range(range_y),
                }
            }
        };

        let result = Wall {
            time: note.time,
            t: wall_type,
        };
        *prev_wall = Some(result.clone());
        result
    }

    pub fn to_code(&self) -> String {
        match self.t {
            WallType::Shape {
                hands_over,
                position,
                lean,
                standing,
                leg_lunge,
            } => {
                let result = match (hands_over, position, lean, standing, leg_lunge) {
                    _ => "WP.C22CDC",
                };
                result.to_owned()
            }
            WallType::Hit {
                position,
                standing,
                hands,
            } => {
                let mut result = String::with_capacity(6);
                result.push_str("WH.");
                result.push_str(position.to_str(None));
                result.push_str(hands.to_str(Some("B")));
                result.push_str(if standing { "U" } else { "D" });
                result
            }
            WallType::Dodge { t, duration } => {
                let mut result = String::with_capacity(10);
                result.push_str("WA.");
                result.push_str(t.to_str());
                result.push_str(".");
                result.push_str(duration.to_string().as_str());
                result
            }
            WallType::Coin { x, y } => {
                let mut result = String::with_capacity(9);
                result.push_str("CN.");
                result.push_str(x.to_string().as_str());
                result.push_str(".");
                result.push_str(y.to_string().as_str());
                result
            }
        }
    }
}
