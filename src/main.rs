use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use walkdir::WalkDir;

use zrpc::*;

fn print_usage() {
    println!("\nZrpc Usage:");
    println!("  -c, --clear        Remove existing zrpc_bindings directory");
    println!("  -r, --recursive    Recursively search for .zrpc files in subdirectories");
    println!("  -l, --language     Target language for bindings (supported: 'c++', 'rs')");
    println!("  -p, --path         Source directory containing .zrpc files (default: current dir)");
    println!(
        "  -od, --out-dir     Output directory for generated bindings (default: 'zrpc_bindings')"
    );
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

    if args.len() <= 1 {
        print_usage();
        return Ok(());
    }

    let mut i = 1;

    let mut lang = "";
    let mut out_dir = PathBuf::from("zrpc_bindings");
    let mut path = std::env::current_dir()?;
    let mut recursive = false;

    while i < args.len() {
        match args[i].as_str() {
            "-c" | "--clear" => {
                let mut found = false;
                for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                    if entry.file_type().is_dir() && entry.file_name() == "zrpc_bindings" {
                        std::fs::remove_dir_all(entry.path()).expect("Failed to clean up");
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("Nothing to cleanup!");
                }
                return Ok(());
            }

            "-r" | "--recursive" => recursive = true,

            "-l" | "--language" => match args[i + 1].as_str() {
                "c++" => {
                    lang = "c++";
                    i += 1;
                }
                "rs" => {
                    lang = "rs";
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

            "-od" | "--out-dir" => {
                if args[i + 1].as_str().len() <= 0 {
                    println!("\n\tOutput directory path wasn't specified!");
                    return Ok(());
                } else {
                    let temp_buf: PathBuf = PathBuf::from_str(args[i + 1].as_str())?;
                    if !temp_buf.exists() {
                        println!("\n\tSpecified output directory path doesn't exist");
                        return Ok(());
                    }
                    out_dir = temp_buf;
                    i += 1;
                }
            }

            _ => {
                println!("Incorrecet args! {}", args[i].to_string());
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
            cpp_gen::create_zrpc_bindings(&schemas, &out_dir)?;
        }
        "rs" => {
            rust_gen::create_zrpc_bindings(&schemas, &out_dir)?;
        }

        _ => {
            println!("\nOutput language wasn't specified!");
            return Ok(());
        }
    }

    Ok(())
}
