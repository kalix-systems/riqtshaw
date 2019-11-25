use super::*;
use codegen::*;

pub(super) fn push_trait(scope: &mut Scope, object: &Object) {
    let mut trait_def = Trait::new(&format!("{name}Trait", name = object.name));

    trait_def.vis("pub");

    // constructor
    let new = trait_def
        .new_fn("new")
        .ret("Self")
        .arg("emit", emitter(&object.name));

    if let Some(model) = model_name(object) {
        new.arg("model", model);
    }

    for (name, prop) in object.object_properties() {
        new.arg(&snake_case(name), prop.type_name());
    }

    // emitter
    trait_def
        .new_fn("emit")
        .arg_mut_self()
        .ret(format!("&mut {emitter}", emitter = emitter(&object.name)));

    for (name, property) in object.properties.iter() {
        let lc_name = snake_case(name);

        if property.is_object() {
            let typ = rust_type(property);

            trait_def
                .new_fn(&lc_name)
                .arg_ref_self()
                .ret(format!("&{typ}", typ = &typ));

            trait_def
                .new_fn(&format!("{}_mut", &lc_name))
                .arg_mut_self()
                .ret(format!("&mut {typ}", typ = typ));
        } else {
            if property.rust_by_function {
                trait_def
                    .new_fn(&lc_name)
                    .arg_ref_self()
                    .generic("F")
                    .bound(
                        "F",
                        &format!("FnOnce({ret_type})", ret_type = rust_return_type(property)),
                    )
                    .arg("getter", "F");
            } else {
                trait_def
                    .new_fn(&lc_name)
                    .arg_ref_self()
                    .ret(rust_return_type(property));
            }

            if property.write {
                let setter_name = format!("set_{prop}", prop = snake_case(name));
                let setter = trait_def.new_fn(&setter_name).arg_mut_self();

                match (property.type_name() == "QByteArray", property.optional) {
                    (true, true) => {
                        setter.arg("value", "Option<&[u8]>");
                    }
                    (true, false) => {
                        setter.arg("value", "&[u8]");
                    }
                    (false, _) => {
                        setter.arg("value", rust_type(property));
                    }
                }
            }
        }
    }

    for (name, func) in object.functions.iter() {
        let name = snake_case(name);

        let trait_func = trait_def.new_fn(&name);

        if func.mutable {
            trait_func.arg_mut_self();
        } else {
            trait_func.arg_ref_self();
        }

        for arg in func.arguments.iter() {
            let typ = match arg.argument_type {
                SimpleType::QByteArray => "&[u8]",
                _ => arg.argument_type.rust_type(),
            };

            trait_func.arg(&arg.name, typ);
        }

        trait_func.ret(func.return_type.rust_type());
    }

    match object.object_type {
        ObjectType::List => {
            trait_def.new_fn("row_count").arg_ref_self().ret("usize");
            trait_def
                .new_fn("insert_rows")
                .arg_mut_self()
                .arg("_row", "usize")
                .arg("_count", "usize")
                .ret("bool")
                .line("false");

            trait_def
                .new_fn("remove_rows")
                .arg_mut_self()
                .arg("_row", "usize")
                .arg("_count", "usize")
                .ret("bool")
                .line("false");

            trait_def
                .new_fn("can_fetch_more")
                .arg_ref_self()
                .ret("bool")
                .line("false");

            trait_def.new_fn("fetch_more").arg_mut_self().line("");

            trait_def
                .new_fn("sort")
                .arg_mut_self()
                .arg("_", "u8")
                .arg("_", "SortOrder")
                .line("");
        }
        ObjectType::Tree => {
            trait_def
                .new_fn("row_count")
                .arg_ref_self()
                .arg("_", "Option<usize>")
                .ret("usize");

            trait_def
                .new_fn("can_fetch_more")
                .arg_ref_self()
                .arg("_", "Option<usize>")
                .ret("bool")
                .line("false");

            trait_def
                .new_fn("fetch_more")
                .arg_mut_self()
                .arg("_", "Option<usize>")
                .line("");

            trait_def
                .new_fn("sort")
                .arg_mut_self()
                .arg("_", "u8")
                .arg("_", "SortOrder")
                .line("");

            trait_def
                .new_fn("check_row")
                .arg_ref_self()
                .arg("index", "usize")
                .arg("row", "usize")
                .ret("Option<usize>");

            trait_def
                .new_fn("index")
                .arg("item", "Option<usize>")
                .arg("row", "usize")
                .ret("usize");

            trait_def
                .new_fn("parent")
                .arg("index", "usize")
                .ret("Option<usize>");

            trait_def.new_fn("row").arg("index", "usize").ret("usize");
        }
        _ => {}
    }

    if object.object_type != ObjectType::Object {
        for (name, item_prop) in object.item_properties.iter() {
            let name = snake_case(name);

            trait_def
                .new_fn(&name)
                .arg_ref_self()
                .arg("index", "usize")
                .ret(rust_return_type_(item_prop));

            if !item_prop.write {
                continue;
            }

            let setter_name = format!("set_{name}", name = name);
            let setter = trait_def
                .new_fn(&setter_name)
                .arg_mut_self()
                .arg("index", "usize")
                .ret("bool");

            match (&item_prop.item_property_type, item_prop.optional) {
                (crate::configuration::Type::Simple(SimpleType::QByteArray), true) => {
                    setter.arg("_", "Option<&[u8]>");
                }
                (crate::configuration::Type::Simple(SimpleType::QByteArray), false) => {
                    setter.arg("_", "&[u8]");
                }
                _ => {
                    setter.arg("_", rust_type_(item_prop));
                }
            }
        }
    }

    scope.push_trait(trait_def);
}
