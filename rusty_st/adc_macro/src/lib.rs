#![no_std]

extern crate alloc;
use alloc::{fmt::format, string::ToString, vec::Vec};
use proc_macro::TokenStream;

extern crate proc_macro;

#[proc_macro]
pub fn create_set_fn(input: TokenStream) -> TokenStream {
    let binding = input.to_string();
    let args: Vec<&str> = binding.split(',').collect();

    if args.len() != 4 {
        panic!("This macro only takes four arguments");
    }

    let reg = args[0].trim();
    let reg_field = args[1].trim();
    let conf_enum = args[2].trim();
    let bits = args[3].trim() == "true";

    let function_body = if bits {
        format(format_args!(
            "self.register_block
                .{}
                .write(|w| unsafe {{ w.{}().bits(config_value as u8) }});",
            reg, reg_field
        ))
    } else {
        format(format_args!(
            "self.register_block
                .{}
                .write(|w| unsafe {{ w.{}().bit(
                    match config_value as u8 {{
                        0 => false,
                        _ => true,
                    }} )
                }});",
            reg, reg_field
        ))
    };

    let string = format(format_args!(
        "
        /// set the {} field of the {} register 
        pub fn set_{}(&self, config_value: {}) {{
            {}
        }}",
        reg_field, reg, reg_field, conf_enum, function_body
    ));

    string.parse().unwrap()
}
