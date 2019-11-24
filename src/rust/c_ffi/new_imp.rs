use super::*;

pub(super) fn new(object: &Object) -> Func {
    let name = snake_case(&object.name);
    let mut func = Func::new(&format!("{}_new", &name));

    func.extern_abi("C")
        .vis("pub")
        .attr("no_mangle")
        .ret(format!("*mut {}", object.name));

    new_args(object, &name, &mut func);
    new_ctor(object, &name, &mut func);

    func.line(format!("Box::into_raw(Box::new(d_{name}))", name = name));

    func
}

pub(super) fn new_args(object: &Object, name: &str, func: &mut Func) {
    func.arg(&snake_case(name), format!("*mut {}", qobject(&object.name)));

    for (prop_name, prop) in object.properties.iter() {
        match &prop.property_type {
            Type::Object(object) => {
                new_args(object, prop_name, func);
            }
            _ => {
                func.arg(
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

    if object.object_type == ObjectType::Object {
        return;
    }

    let qobj = qobject(&object.name);
    let lc_name = snake_case(&name);

    match object.object_type {
        ObjectType::List => {
            func.arg(
                &format!("{}_new_data_ready", &lc_name),
                &format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_layout_about_to_be_changed", &lc_name),
                format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_layout_changed", &lc_name),
                format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_data_changed", &lc_name),
                format!("fn(*mut {}, usize, usize)", &qobj),
            )
            .arg(
                &format!("{}_begin_reset_model", &lc_name),
                format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_end_reset_model", &lc_name),
                format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_begin_insert_rows", &lc_name),
                format!("fn(*mut {}, usize, usize)", &qobj),
            )
            .arg(
                &format!("{}_end_insert_rows", &lc_name),
                format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_begin_move_rows", &lc_name),
                format!("fn(*mut {}, usize, usize, usize)", &qobj),
            )
            .arg(
                &format!("{}_end_move_rows", &lc_name),
                format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_begin_remove_rows", &lc_name),
                format!("fn(*mut {}, usize, usize)", &qobj),
            )
            .arg(
                &format!("{}_end_remove_rows", &lc_name),
                format!("fn(*mut {})", &qobj),
            );
        }
        ObjectType::Tree => {
            let tree_index = "index: COption<usize>";
            func.arg(
                &format!("{}_new_data_ready", &lc_name),
                &format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_layout_about_to_be_changed", &lc_name),
                format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_layout_changed", &lc_name),
                format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_data_changed", &lc_name),
                format!("fn(*mut {}, usize, usize)", &qobj),
            )
            .arg(
                &format!("{}_begin_reset_model", &lc_name),
                format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_end_reset_model", &lc_name),
                format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_begin_insert_rows", &lc_name),
                format!(
                    "fn(*mut {qobj}, {index}, usize, usize)",
                    qobj = &qobj,
                    index = tree_index
                ),
            )
            .arg(
                &format!("{}_end_insert_rows", &lc_name),
                format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_begin_move_rows", &lc_name),
                format!(
                    "fn(*mut {qobj}, usize, {index}, usize, {index}, usize)",
                    qobj = &qobj,
                    index = "index: COption<usize>"
                ),
            )
            .arg(
                &format!("{}_end_move_rows", &lc_name),
                format!("fn(*mut {})", &qobj),
            )
            .arg(
                &format!("{}_begin_remove_rows", &lc_name),
                format!(
                    "fn(*mut {qobj}, {index}, usize, usize)",
                    qobj = &qobj,
                    index = tree_index
                ),
            )
            .arg(
                &format!("{}_end_remove_rows", &lc_name),
                format!("fn(*mut {})", &qobj),
            );
        }
        _ => unreachable!(),
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
    block.after(";");

    Some(block)
}
