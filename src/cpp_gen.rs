use crate::utils::{get_scheme_count, init_parser, parse_zrpc_file};
use std::os::unix::fs::FileExt;
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

    let output_path = out_dir.join("c++");
    std::fs::create_dir_all(&output_path)?;

    let file_stem = file_path.file_stem().unwrap();

    let source_file = format!("c++/{}.cpp", file_stem.to_string_lossy());
    let header_file = format!("c++/{}.h", file_stem.to_string_lossy());

    let temp_source_file_path: PathBuf = out_dir.join(source_file);
    let temp_header_file_path: PathBuf = out_dir.join(header_file);

    let mut scheme_fs = File::open(file_path).unwrap();
    let mut scheme_str: String = String::new();
    scheme_fs.read_to_string(&mut scheme_str)?;

    let source_fs = File::create(&temp_source_file_path)?;
    let header_fs = File::create(&temp_header_file_path)?;

    let mut source_fs_offset = 0;
    let mut header_fs_offset = 0;

    let scheme_count = get_scheme_count(&scheme_str);
    let route_count = get_scheme_count(&scheme_str);

    for _ in 0..scheme_count {
        println!("");
    }

    for _ in 0..route_count {
        println!("");
    }

    let import_statement = format!(
        "#include {:?}\n\n",
        temp_header_file_path.file_name().unwrap()
    );

    println!("{import_statement}");

    source_fs.write_at(import_statement.as_bytes(), source_fs_offset)?;

    Ok(())

    /*
        Type: Scheme

        File struct .h

        @ifndef STRUCT_NAME_H
        @define STRUCT_NAME_H

        struct SchemeName {
            @constructor noexcept
            @constructor __args__(...) noexcept
            @destructror noexcept

            @repeat {
                @getter get_field_name() const;
            }

            @repeat {
                @setter set_field_name();
            }

            @serialize_to_ptr __in__(schem&), __out__(data*);
            @serialize __in__(scheme&), __out__(data[N]);
            @deserialize_to_ptr __in__(data*), __out__(scheme*);
            @deserialize __in__(data*), __out__(scheme);

            @compress -> lz4
            @decompress -> lz4

            private:
                @repeat {
                    type field_name_m
                }
        }

        @endif // STRUCT_NAME_H

        File struct .cpp

        @import statement => #include "source_file_name"

        @constructor noexcept
        @constructor __args__(...) noexcept
        @destructor noexcept

        @repeat => {
            field_name_get();
            field_name_set();
        }

        @serialize -> uint8_t*
        @serialize -> uint8_t[N]
        @deserialize -> scheme_type*
        @deserialize -> scheme_type

        @compress -> lz4
        @decompress -> lz4
    */

    /*
        Type: Route

        File struct .h

        @ifndef ROUTE_NAME_H
        @define ROUTE_NAME_H

        @repeat {
            @import "scheme_name.h"
        }

        struct RouteName {

            @constructor
            @constructor __args__(...)
            @destructror

            @callback_on_response -> user_defined : type callback_resposne(response_type response)
            @callback_on_error -> user_defined: type callback_error(error_type error)

            # Transport carrier is a set of USER|ROUTE defined callbacks,
            # which must provide USER|ROUTE with necessary API calls, which
            # will be used by USER|ROUTE.

            # _send(data*, size) is a USER defined send API call, which ROUTE
            # will invoke after serializing request args.
            @transport_carrier_send -> user_defined: type send(data*, size);

            # _receive(data*, size) is a ROUTE defined API callback, which USER needs
            # to connect to the QUIC's receive loop, in order to provide ROUTE access to
            # incoming udp packets.
            @transport_carrier_receive -> user_defined: on_receive_callback(data*, size);

            # _ordering(data[]) is a USER defined filter, containing instructions on how the data
            # must be sorted.
            @transport_carrier_ordering -> user_defined: order_filter(data[]);

            # _decrypt(data*) is a USER defined function for data decryption
            @transport_carrier_decrypt -> user_defined: decrypt(data*);

            # _encrypt(data*) is a USER defined function for data encryption
            @transport_carrier_encrypt -> user_defined: encrypt(data*);

            @invoke_request

            @get_status

            @repeat {
                @set_request_field_name;
            }

            @repeat {
                @get_request_field_name;
            }

            @repeat {
                @set_settings_field_name;
            }

            @repeat {
                @get_settingst_field_name;
            }

            private:
                @settings {
                    timeout
                    auth_credentials
                    ...
                }

                uint8_t* data;
                type response;
                type request;
        }

        @endif // ROUTE_NAME_H

        File struct .cpp

        @import statement => #include "source_file_name"

        @constructor noexcept
        @constructor __args__(...) noexcept
        @destructor noexcept

        @invoke_request

        @get_status

        @repeat {
            @set_request_field_name;
        }

        @repeat {
            @get_request_field_name;
        }

        @repeat {
            @set_settings_field_name;
        }

        @repeat {
            @get_settingst_field_name;
        }
    */
}
