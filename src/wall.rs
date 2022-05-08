//! This file contains implementations for Wall module.
//!
use super::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

impl Wall {
    pub fn choose_shape_patterns(main_pose: &str, mut rng: &mut ThreadRng) -> String {
        let available_chars = match main_pose {
            "CUC" => ['0', '1', '2', '3', '4', '5', '7', '8', '9', 'A', 'B'].as_slice(),
            "CDC" => ['0', '2', '3', '4', '5', '7', '8', '9', 'A', 'B'].as_slice(),
            "LUC" => ['3', '8'].as_slice(),
            "RUC" => ['3', '8'].as_slice(),
            _ => ['2'].as_slice(),
        };

        format!(
            "{}{}{}",
            available_chars.choose(&mut rng).unwrap(),
            available_chars.choose(&mut rng).unwrap(),
            main_pose
        )
    }

    pub fn new(
        note: &Note,
        rng: &mut ThreadRng,
        time2prev: f32,
        time2next: f32,
        figures: &mut Vec<i32>,
        prev_wall: &mut Option<Wall>,
        acc_coins: &mut u8,
    ) -> Self {
        let select: i32 = if (time2prev > 0.5f32
            || matches!(
                prev_wall,
                Some(Wall {
                    time: _,
                    t: WallType::Coin { x: _, y: _ }
                })
            ))
            && (time2next > 0.5f32 || *acc_coins >= 8)
        {
            *acc_coins = 0;
            figures.pop().unwrap_or(3)
        } else {
            *acc_coins += 1;
            3
        };

        let wall_type = match select {
            0 => WallType::Shape {
                position: rand::random(),
                standing: rand::random(),
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
                    duration: d.saturating_sub(50),
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

    pub fn to_code(&self, mut rng: &mut ThreadRng) -> String {
        match self.t {
            WallType::Shape { position, standing } => {
                let mut result = String::with_capacity(9);
                result.push_str("WP.C");
                result.push_str(
                    match (position, standing) {
                        (_, true) => Self::choose_shape_patterns("CDC", &mut rng),
                        (_, false) => Self::choose_shape_patterns("CUC", &mut rng),
                    }
                    .as_str(),
                );
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
