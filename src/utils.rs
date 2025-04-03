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

pub fn get_scheme_count(scheme_str: &str) -> u32 {
    let split = scheme_str.split(" ");
    let mut count: u32 = 0;

    for item in split.into_iter() {
        if item == ZrpcKeywords::Scheme.as_str() {
            count += 1;
        }
    }

    count
}

pub fn get_route_count(route_str: &str) -> u32 {
    let split = route_str.split(" ");
    let mut count: u32 = 0;

    for item in split.into_iter() {
        if item == ZrpcKeywords::Route.as_str() {
            count += 1;
        }
    }

    count
}

pub fn get_scheme_names(src_str: &str) -> Vec<String> {
    let mut names = Vec::new();

    let final_count = get_scheme_count(src_str);
    let mut count = 0;
    let split: Vec<&str> = src_str.split(" ").collect();

    let mut iter = split.into_iter();
    while let Some(item) = iter.next() {
        if item == ZrpcKeywords::Scheme.as_str() {
            names.push(iter.next().unwrap().to_string());
            count += 1;
            if count == final_count {
                return names;
            }
        }
    }

    names
}

pub fn get_route_names(src_str: &str) -> Vec<String> {
    let mut names = Vec::new();

    let final_count = get_route_count(src_str);
    let mut count = 0;
    let split: Vec<&str> = src_str.split(" ").collect();

    let mut iter = split.into_iter();
    while let Some(item) = iter.next() {
        if item == ZrpcKeywords::Route.as_str() {
            names.push(iter.next().unwrap().to_string());
            count += 1;
            if count == final_count {
                return names;
            }
        }
    }

    names
}

pub enum ZrpcKeywords {
    Scheme,
    Route,
}

impl ZrpcKeywords {
    pub fn as_str(&self) -> &str {
        match self {
            ZrpcKeywords::Scheme => "scheme",
            ZrpcKeywords::Route => "route",
        }
    }
}

pub struct ZrpcField {
    pub field_name: String,
    pub field_type: ZrpcTypes,
}

pub enum ZrpcTypes {
    U32id,
    U64id,
    Uname,
    Uemail,
    Upassword,
    Uavatar,
    Ubanner,
    Time,
    Lang,
    Text,
    Bytes,
    UserDefined,
}

/*
    as_lang_str()

    Returns language specific string created based on provided field name
    and type used:

    ZrpcTypes::U32id.as_cpp_str(user_id, None) -> "uint8_t user_id[32];"

    Note!

    Second argument inside as_lang_str func, is a user defined type, primarily
    used when user defines type with existing scheme:

    scheme User {
        user_id:   u32_id
        user_name: name
    }

    scheme Message {
        receiver: User
        sender:   User
    }

    ZrpcTypes::UserDefined.as_cpp_str(receiver, Some(User)) ->

    #include "user.h"

    User receiver;

    Read the docs for more information.
    https://github.com/Akzestia/zrpc/blob/main/docs/types.md
*/
impl ZrpcTypes {
    pub fn as_cpp_str(&self, field_name: &str, user_defined_type: Option<&str>) -> String {
        match self {
            ZrpcTypes::U32id => {
                format!("uint8_t {}[32];", field_name)
            }
            ZrpcTypes::U64id => {
                format!("uint8_t {}[64];", field_name)
            }
            ZrpcTypes::Uname => {
                format!("uint8_t {}[32];", field_name)
            }
            ZrpcTypes::Uemail => {
                format!("uint8_t {}[16];", field_name)
            }
            ZrpcTypes::Upassword => {
                format!("uint8_t {}[16];", field_name)
            }
            ZrpcTypes::Uavatar => {
                //Max 6mb
                format!("uint8_t* {};", field_name)
            }
            ZrpcTypes::Ubanner => {
                //Max 32mb
                format!("uint8_t* {};", field_name)
            }
            ZrpcTypes::Time => {
                format!("uint64_t {};", field_name)
            }
            ZrpcTypes::Lang => {
                format!("char {}[3]", field_name)
            }
            ZrpcTypes::Text => {
                format!("uint8_t* {};", field_name)
            }
            ZrpcTypes::Bytes => {
                format!("uint8_t* {};", field_name)
            }
            ZrpcTypes::UserDefined => {
                format!("{} {}", user_defined_type.unwrap_or("void*"), field_name)
            }
        }
    }

    pub fn as_rust_str(&self, field_name: &str, user_defined_type: Option<&str>) -> String {
        match self {
            ZrpcTypes::U32id => {
                format!("[u8; 32] {}", field_name)
            }
            ZrpcTypes::U64id => {
                format!("[u8; 64] {}", field_name)
            }
            ZrpcTypes::Uname => {
                format!("[u8; 32] {}", field_name)
            }
            ZrpcTypes::Uemail => {
                format!("[u8; 16] {}", field_name)
            }
            ZrpcTypes::Upassword => {
                format!("[u8; 16] {}", field_name)
            }
            ZrpcTypes::Uavatar => {
                //Max 6mb
                format!("Vec<u8> {}", field_name)
            }
            ZrpcTypes::Ubanner => {
                //Max 32mb
                format!("Vec<u8> {}", field_name)
            }
            ZrpcTypes::Time => {
                format!("u64 {}", field_name)
            }
            ZrpcTypes::Lang => {
                format!("[u8; 3] {}", field_name)
            }
            ZrpcTypes::Text => {
                format!("Vec<u8> {}", field_name)
            }
            ZrpcTypes::Bytes => {
                format!("Vec<u8> {}", field_name)
            }
            ZrpcTypes::UserDefined => {
                format!(
                    "{} {}",
                    user_defined_type.unwrap_or("Box<dyn Any>"),
                    field_name
                )
            }
        }
    }
}

/*

    Lang Factory

    LangFactory is a language specific factory for generating
    corresponding strings. For example:

    C++ {
        #import
        #define
        #ifdef
        ...
        void callback();
    }

    Rust {
        use name::type::*
        ...
        fn callback() -> void;
    }

    Please note, that Factory mustn't implement any of ZrpcTypes!
*/

pub struct CppFactory {}

impl CppFactory {
    pub fn import_str(file_name: &str) -> String {
        format!("#import {}", file_name)
    }
    pub fn struct_str(name: &str) -> String {
        format!("struct {} {{\n\n}}", name)
    }

    pub fn ifndef_block_str(name: &str) -> String {
        let upper_case_name = format!("{}_H", name.to_uppercase());
        let ifndef_block_str = format!(
            "#ifndef {}\n#define {}\n\n#endif // {}",
            upper_case_name, upper_case_name, upper_case_name
        );

        ifndef_block_str
    }
}

/*
    @Note!

    Rust bindings are stored in the same file (both definitions and implementations),
    for the convenience of use.
*/
pub struct RustFactory {}

impl RustFactory {}
