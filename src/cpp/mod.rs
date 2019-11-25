//! `cpp` is the module that generates the cpp code for the bindings

use crate::configuration::*;
use crate::configuration_private::*;
use crate::util::{snake_case, write_if_different};
use std::io::{Result, Write};

mod headers;
pub use headers::write_header;

mod code;
pub use code::write_cpp;

mod helpers;
use helpers::*;

#[allow(unused)]
fn add_factory_lambdas(write_buf: &mut Vec<u8>, object: &Object) -> Result<()> {
    let qobject_type_properties = object.item_properties.iter().filter(|(_, prop)| {
        if let Type::Object(_) = prop.item_property_type {
            true
        } else {
            false
        }
    });

    for (_, prop) in qobject_type_properties {
        let nested_model_name = if let Type::Object(obj) = &prop.item_property_type {
            obj.name.clone()
        } else {
            break;
        };

        writeln!(
            write_buf,
            ",
            []() {{
                {nested_model_name} alloc = new Messages;
                 return alloc;
                }}",
            nested_model_name = nested_model_name,
        )?;
    }

    Ok(())
}
