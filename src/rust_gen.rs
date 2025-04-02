use crate::utils::{init_parser, parse_zrpc_file};
use std::{error::Error, path::PathBuf};
use tree_sitter::Parser;

use std::fs::File;
use std::io::prelude::*;

pub fn create_zrpc_bindings(input: &Vec<PathBuf>, out_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut parser: Parser = Parser::new();

    init_parser(&mut parser)?;

    for path in input.into_iter() {
        transform(path, &mut parser, out_dir)?
    }

    Ok(())
}

fn transform(
    file_path: &PathBuf,
    parser: &mut Parser,
    out_dir: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    if !parse_zrpc_file(file_path, parser) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to parse file: {:?}", file_path),
        )) as Box<dyn Error>);
    }

    let output_path = out_dir.join("rust");
    std::fs::create_dir_all(&output_path)?;

    let file_stem = file_path.file_stem().unwrap();

    let source_file = format!("rust/{}.rs", file_stem.to_string_lossy());

    let temp_source_file_path: PathBuf = out_dir.join(source_file);

    let mut scheme_fs = File::open(file_path).unwrap();
    let mut scheme_str: String = String::new();
    scheme_fs.read_to_string(&mut scheme_str)?;

    let mut source_fs = File::create(&temp_source_file_path)?;

    Ok(())
}
