use super::*;
use codegen::Struct;
use std::io::Write;

pub(crate) fn ptr_bundle(object: &Object) -> Struct {
    let name = &object.name;
    let mut bundle = Struct::new(&ptr_bundle_name(object));

    bundle.repr("C").vis("pub").derive("Clone").derive("Copy");

    fields(object, &name, &mut bundle);

    bundle
}

fn fields(object: &Object, name: &str, bundle: &mut Struct) {
    bundle.field(&snake_case(name), format!("*mut {}", qobject(&object.name)));

    for (prop_name, prop) in object.properties.iter() {
        match &prop.property_type {
            Type::Object(object) => {
                fields(object, prop_name, bundle);
            }
            _ => {
                bundle.field(
                    &format!(
                        "{name}_{prop_name}_changed",
                        name = snake_case(name),
                        prop_name = snake_case(prop_name)
                    ),
                    format!("fn(*mut {})", qobject(&object.name)),
                );
            }
        }
    }

    let qobj = qobject(&object.name);
    let lc_name = snake_case(&name);

    match object.object_type {
        ObjectType::List => {
            bundle
                .field(
                    &format!("{}_new_data_ready", &lc_name),
                    &format!("fn(*mut {})", &qobj),
                )
                .field(
                    &format!("{}_layout_about_to_be_changed", &lc_name),
                    format!("fn(*mut {})", &qobj),
                )
                .field(
                    &format!("{}_layout_changed", &lc_name),
                    format!("fn(*mut {})", &qobj),
                )
                .field(
                    &format!("{}_data_changed", &lc_name),
                    format!("fn(*mut {}, usize, usize)", &qobj),
                )
                .field(
                    &format!("{}_begin_reset_model", &lc_name),
                    format!("fn(*mut {})", &qobj),
                )
                .field(
                    &format!("{}_end_reset_model", &lc_name),
                    format!("fn(*mut {})", &qobj),
                )
                .field(
                    &format!("{}_begin_insert_rows", &lc_name),
                    format!("fn(*mut {}, usize, usize)", &qobj),
                )
                .field(
                    &format!("{}_end_insert_rows", &lc_name),
                    format!("fn(*mut {})", &qobj),
                )
                .field(
                    &format!("{}_begin_move_rows", &lc_name),
                    format!("fn(*mut {}, usize, usize, usize)", &qobj),
                )
                .field(
                    &format!("{}_end_move_rows", &lc_name),
                    format!("fn(*mut {})", &qobj),
                )
                .field(
                    &format!("{}_begin_remove_rows", &lc_name),
                    format!("fn(*mut {}, usize, usize)", &qobj),
                )
                .field(
                    &format!("{}_end_remove_rows", &lc_name),
                    format!("fn(*mut {})", &qobj),
                );
        }
        ObjectType::Object => {}
    };

    for (signal_name, signal) in object.signals.iter() {
        let mut buf = Vec::new();
        write!(&mut buf, "fn(*mut {qobject}, ", qobject = &qobj).unwrap();

        for arg in signal.arguments.iter() {
            write!(&mut buf, "{typ}, ", typ = arg.argument_type.rust_type()).unwrap();
        }

        write!(&mut buf, ")").unwrap();

        bundle.field(
            &format!(
                "{name}_{signal_name}",
                name = &lc_name,
                signal_name = signal_name
            ),
            String::from_utf8(buf).unwrap(),
        );
    }
}
