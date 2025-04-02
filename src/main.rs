use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use zrpc::*;

fn print_usage() {
    println!("\n\tZrpc Usage:");
}

fn get_schemas(dir_path: PathBuf, recursive: bool) -> Vec<PathBuf> {
    let mut result = Vec::new();

    if let Ok(entries) = fs::read_dir(&dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "zrpc" {
                        result.push(path);
                    }
                }
            } else if path.is_dir() && recursive {
                let mut sub_results = get_schemas(path, recursive);
                result.append(&mut sub_results);
            }
        }
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        print_usage();
        return Ok(());
    }

    let mut i = 1;

    let mut lang = "";
    let mut path = std::env::current_dir()?;
    let mut recursive = false;

    while i < args.len() {
        match args[i].as_str() {
            "-r" | "--recursive" => recursive = true,

            "-l" | "--language" => match args[i + 1].as_str() {
                "c++" => {
                    lang = "c++";
                    i += 1;
                }
                "rust" => {
                    lang = "rust";
                    i += 1;
                }

                _ => {
                    println!("\n\tUnsupported language!");
                    print_usage();
                    return Ok(());
                }
            },

            "-p" | "--path" => {
                if args[i + 1].as_str().len() <= 0 {
                    println!("\n\tPath wasn't specified!");
                    return Ok(());
                } else {
                    let temp_buf: PathBuf = PathBuf::from_str(args[i + 1].as_str())?;
                    if !temp_buf.exists() {
                        println!("\n\tSpecified path doesn't exist");
                        return Ok(());
                    }
                    path = temp_buf;
                    i += 1;
                }
            }

            _ => {
                print_usage();
                return Ok(());
            }
        }
        i += 1;
    }

    let schemas = get_schemas(path, recursive);

    if schemas.len() < 1 {
        println!("\n\tNo zrpc schemas was found!");
        return Ok(());
    }

    if lang == "" {
        println!("\n\tOutput lang wasn't specified.\n\tTry using --lang \'c++\' or --lang \'rs\'");
        return Ok(());
    }

    match lang {
        "c++" => {
            cpp_gen::create_zrpc_bindings(&schemas)?;
        }
        "rs" => {
            rust_gen::create_zrpc_bindings(&schemas)?;
        }

        _ => {
            println!("\nOutput language wasn't specified!");
            return Ok(());
        }
    }

    Ok(())
}
