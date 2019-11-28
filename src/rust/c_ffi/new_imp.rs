use super::*;

pub(super) fn new(object: &Object) -> Func {
    let name = snake_case(&object.name);
    let mut func = Func::new(&format!("{}_new", &name));

    func.extern_abi("C")
        .vis("pub unsafe")
        .attr("no_mangle")
        .ret(format!("*mut {}", object.name))
        .arg("ptr_bundle", format!("*mut {}", ptr_bundle_name(object)))
        .line("let ptr_bundle = *ptr_bundle;")
        .line("");

    let mut block = Block::new(&format!("let {}", ptr_bundle_name(object)));

    fields(object, &name, &mut block);
    block.after(" = ptr_bundle;");

    func.push_block(block);

    new_ctor(object, &name, &mut func);

    func.line(format!("Box::into_raw(Box::new(d_{name}))", name = name));

    func
}

fn fields(object: &Object, name: &str, block: &mut Block) {
    block.line(&snake_case(name)).line(",");

    for (prop_name, prop) in object.properties.iter() {
        match &prop.property_type {
            Type::Object(object) => {
                fields(object, prop_name, block);
            }
            _ => {
                block.line(&format!(
                    "{name}_{prop_name}_changed,",
                    name = snake_case(name),
                    prop_name = snake_case(prop_name)
                ));
            }
        }
    }

    let lc_name = snake_case(&name);

    match object.object_type {
        ObjectType::List => {
            block
                .line(format!("{}_new_data_ready,", &lc_name))
                .line(format!("{}_layout_about_to_be_changed,", &lc_name))
                .line(format!("{}_layout_changed,", &lc_name))
                .line(format!("{}_data_changed,", &lc_name))
                .line(format!("{}_begin_reset_model,", &lc_name))
                .line(format!("{}_end_reset_model,", &lc_name))
                .line(format!("{}_begin_insert_rows,", &lc_name))
                .line(format!("{}_end_insert_rows,", &lc_name))
                .line(format!("{}_begin_move_rows,", &lc_name))
                .line(format!("{}_end_move_rows,", &lc_name))
                .line(format!("{}_begin_remove_rows,", &lc_name))
                .line(format!("{}_end_remove_rows,", &lc_name));

            for (item_prop_name, item_prop) in object.item_properties.iter() {
                if let Type::Object(_) = &item_prop.item_property_type {
                    block.line(&format!("{},", ptr_bundle_factory_name(item_prop_name)));
                }
            }
        }
        ObjectType::Object => {}
        _ => unimplemented!(),
    }
}

pub(super) fn new_ctor(object: &Object, name: &str, func: &mut Func) {
    for (prop_name, prop) in object.properties.iter() {
        if let Type::Object(object) = &prop.property_type {
            new_ctor(object, prop_name, func);
        }
    }

    // construct emitter
    let mut emit_ctor = Block::new(&format!(
        "let {name}_emit = {emitter}",
        name = snake_case(name),
        emitter = emitter(&object.name)
    ));

    emit_ctor.line(format!(
        "qobject: Arc::new(AtomicPtr::new({})),",
        snake_case(name)
    ));

    for prop_name in object.non_object_property_names() {
        emit_ctor.line(&format!(
            "{}_changed: {}_{0}_changed,",
            snake_case(prop_name),
            snake_case(name)
        ));
    }

    if object.object_type != ObjectType::Object {
        emit_ctor.line(&format!(
            "new_data_ready: {}_new_data_ready,",
            snake_case(name)
        ));
    }

    emit_ctor.after(";");
    func.push_block(emit_ctor);

    match model(object, name) {
        Some(block) => {
            func.push_block(block);

            func.line(&format!(
                "let d_{lc_name} = {object_name}::new({lc_name}_emit, model",
                lc_name = snake_case(name),
                object_name = object.name
            ));
        }
        None => {
            func.line(&format!(
                "let d_{lc_name} = {object_name}::new({lc_name}_emit",
                lc_name = snake_case(name),
                object_name = object.name
            ));
        }
    }

    for (name, _) in object.object_properties() {
        func.line(format!(", d_{}", snake_case(name)));
    }
    func.line(");");
}

pub(super) fn model(object: &Object, name: &str) -> Option<Block> {
    // construct model
    let mut block = Block::new(&format!("let model = {}", model_name(object)?));

    block.line(format!(
        "
        qobject: {name},
        layout_about_to_be_changed: {name}_layout_about_to_be_changed,
        layout_changed: {name}_layout_changed,
        data_changed: {name}_data_changed,
        begin_reset_model: {name}_begin_reset_model,
        end_reset_model: {name}_end_reset_model,
        begin_insert_rows: {name}_begin_insert_rows,
        end_insert_rows: {name}_end_insert_rows,
        begin_move_rows: {name}_begin_move_rows,
        end_move_rows: {name}_end_move_rows,
        begin_remove_rows: {name}_begin_remove_rows,
        end_remove_rows: {name}_end_remove_rows,
        ",
        name = snake_case(name)
    ));

    for (item_prop_name, item_prop) in object.item_properties.iter() {
        if let Type::Object(_) = &item_prop.item_property_type {
            block.line(&format!("{},", ptr_bundle_factory_name(item_prop_name)));
        }
    }
    block.after(";");

    Some(block)
}
