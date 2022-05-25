use anyhow::Result;
use onshape_level_gen::{util::*, Difficulty, LevelDifficulty};
use std::{env, fs, path::Path};

fn process(path: &str) -> Result<()> {
    let init_difficulty_limit = 1.2f32;
    let path = Path::new(path);
    let (samples, sound_file_info) = get_data_from_ogg(path)?;
    let notes = get_notes_from_samples(samples, init_difficulty_limit)
        .map_err(|err| anyhow::anyhow!("{:?}", err))?;

    let easy = LevelDifficulty {
        speed: 30,
        difficulty: Difficulty::Easy,
        min_interval: 0.5,
        max_consecutive_coins: 8,
    };
    let medium = LevelDifficulty {
        speed: 40,
        difficulty: Difficulty::Medium,
        min_interval: 0.3,
        max_consecutive_coins: 3,
    };
    let hard = LevelDifficulty {
        speed: 50,
        difficulty: Difficulty::Hard,
        min_interval: 0.2,
        max_consecutive_coins: 2,
    };

    for ld in [easy, medium, hard].iter() {
        let walls = get_walls_from_notes(&notes, ld);
        let yml_content = gen_yml(&sound_file_info, ld, walls);

        let output_file = path
            .with_extension(&format!("{}.yml", ld.difficulty))
            .to_str()
            .unwrap_or(&"output_file")
            .to_owned();
            
        println!("Writing to file: {}", output_file);
        fs::write(output_file, yml_content)?;
    }

    println!("All jobs done\n");

    Ok(())
}

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();

    match args.as_slice() {
        [_] => {
            println!(
                "===== onshapelevelgen v{}: A tool to generate OnShape VR game levels from music =====
\nUsage: onshapelevelgen [your_ogg_vorbis_file | folder_contains_ogg_files]", env!("CARGO_PKG_VERSION")
            );
        }
        [_, target] => {
            let target_path = Path::new(target);
            if target_path.is_dir() {
                let paths: Vec<String> = match fs::read_dir(target_path) {
                    Ok(paths) => paths
                        .filter_map(|x| x.ok())
                        .filter(|x| {
                            if let Some("ogg") = x.path().extension().and_then(|x| x.to_str()) {
                                true
                            } else {
                                false
                            }
                        })
                        .filter_map(|x| x.path().into_os_string().into_string().ok())
                        .collect(),
                    Err(_) => vec![],
                };
                for path in paths {
                    process(path.as_str())?
                }
            } else {
                process(target.as_str())?
            }
        }
        _ => panic!("Invalid arguments"),
    };

    Ok(())
}
