use anyhow::Result;
use core::panic;
use onshape_level_gen::{util::*, *};
use std::env;

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();

    let args = env::args().collect::<Vec<String>>();

    match args.as_slice() {
        [_, target] => {
            let samples = get_data_from_ogg(target.as_str())?;
            let notes =
                get_notes_from_samples(samples).map_err(|err| anyhow::anyhow!("{:?}", err))?;

            let walls: Vec<Wall> = notes
                .windows(2)
                .map(|notes| {
                    let time2next = notes[1].time - notes[0].time;
                    Wall::new(&notes[0], time2next, &mut rng)
                })
                .collect();
        }
        _ => panic!("Invalid arguments"),
    };

    Ok(())
}
