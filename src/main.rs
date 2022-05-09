use anyhow::Result;
use onshape_level_gen::util::*;
use std::{env, fs, path::Path};

fn process(path: &str) -> Result<()> {
    let init_difficulty_limit = 1.2f32;
    let (samples, output_params) = get_data_from_ogg(path)?;
    let notes = get_notes_from_samples(samples, init_difficulty_limit).map_err(|err| anyhow::anyhow!("{:?}", err))?;

    let walls = get_walls_from_notes(&notes);

    let yml_content = gen_yml(&output_params, walls);

    println!("Writing to file: {}", output_params.output_file);
    fs::write(output_params.output_file, yml_content)?;

    println!("Done\n");

    Ok(())
}

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();

    match args.as_slice() {
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
