use std::error::Error;
use std::fs;
use std::path::PathBuf;

use tree_sitter::Parser;

pub fn parse_zrpc_file(file_path: &PathBuf, parser: &mut Parser) -> bool {
    let source_code = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(_) => return false,
    };

    match parser.parse(&source_code, None) {
        Some(tree) => !tree.root_node().has_error(),
        None => false,
    }
}

pub fn init_parser(parser: &mut Parser) -> Result<(), Box<dyn Error>> {
    parser.set_language(&tree_sitter_zrpc::LANGUAGE.into())?;
    Ok(())
}
