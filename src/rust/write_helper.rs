use super::*;

pub(super) fn r_constructor_args_decl(
    r: &mut Vec<u8>,
    name: &str,
    o: &Object,
    conf: &Config,
) -> Result<()> {
    write!(r, "    {}: *mut {}QObject", snake_case(name), o.name)?;
    for (p_name, p) in &o.properties {
        if let Type::Object(object) = &p.property_type {
            writeln!(r, ",")?;
            r_constructor_args_decl(r, p_name, object, conf)?;
        } else {
            write!(
                r,
                ",\n    {}_{}_changed: fn(*mut {}QObject)",
                snake_case(name),
                snake_case(p_name),
                o.name
            )?;
        }
    }
    if o.object_type == ObjectType::List {
        write!(
            r,
            ",\n    {}_new_data_ready: fn(*mut {}QObject)",
            snake_case(name),
            o.name
        )?;
    } else if o.object_type == ObjectType::Tree {
        write!(
            r,
            ",\n    {}_new_data_ready: fn(*mut {}QObject, index: COption<usize>)",
            snake_case(name),
            o.name
        )?;
    }
    if o.object_type != ObjectType::Object {
        let index_decl = if o.object_type == ObjectType::Tree {
            " index: COption<usize>,"
        } else {
            ""
        };
        let dest_decl = if o.object_type == ObjectType::Tree {
            " index: COption<usize>,"
        } else {
            ""
        };
        write!(
            r,
            ",
    {2}_layout_about_to_be_changed: fn(*mut {0}QObject),
    {2}_layout_changed: fn(*mut {0}QObject),
    {2}_data_changed: fn(*mut {0}QObject, usize, usize),
    {2}_begin_reset_model: fn(*mut {0}QObject),
    {2}_end_reset_model: fn(*mut {0}QObject),
    {2}_begin_insert_rows: fn(*mut {0}QObject,{1} usize, usize),
    {2}_end_insert_rows: fn(*mut {0}QObject),
    {2}_begin_move_rows: fn(*mut {0}QObject,{1} usize, usize,{3} usize),
    {2}_end_move_rows: fn(*mut {0}QObject),
    {2}_begin_remove_rows: fn(*mut {0}QObject,{1} usize, usize),
    {2}_end_remove_rows: fn(*mut {0}QObject)",
            o.name,
            index_decl,
            snake_case(name),
            dest_decl
        )?;
    }
    Ok(())
}

pub(super) fn r_constructor_args(
    r: &mut Vec<u8>,
    name: &str,
    o: &Object,
    conf: &Config,
) -> Result<()> {
    for (name, p) in &o.properties {
        if let Type::Object(object) = &p.property_type {
            r_constructor_args(r, name, object, conf)?;
        }
    }
    writeln!(
        r,
        "    let {}_emit = {}Emitter {{
        qobject: Arc::new(AtomicPtr::new({0})),",
        snake_case(name),
        o.name
    )?;
    for (p_name, p) in &o.properties {
        if p.is_object() {
            continue;
        }
        writeln!(
            r,
            "        {}_changed: {}_{0}_changed,",
            snake_case(p_name),
            snake_case(name)
        )?;
    }
    if o.object_type != ObjectType::Object {
        writeln!(
            r,
            "        new_data_ready: {}_new_data_ready,",
            snake_case(name)
        )?;
    }
    let mut model = String::new();
    if o.object_type != ObjectType::Object {
        let type_ = if o.object_type == ObjectType::List {
            "List"
        } else {
            "Tree"
        };
        model.push_str(", model");
        writeln!(
            r,
            "    }};
    let model = {}{} {{
        qobject: {},
        layout_about_to_be_changed: {2}_layout_about_to_be_changed,
        layout_changed: {2}_layout_changed,
        data_changed: {2}_data_changed,
        begin_reset_model: {2}_begin_reset_model,
        end_reset_model: {2}_end_reset_model,
        begin_insert_rows: {2}_begin_insert_rows,
        end_insert_rows: {2}_end_insert_rows,
        begin_move_rows: {2}_begin_move_rows,
        end_move_rows: {2}_end_move_rows,
        begin_remove_rows: {2}_begin_remove_rows,
        end_remove_rows: {2}_end_remove_rows,",
            o.name,
            type_,
            snake_case(name)
        )?;
    }
    write!(
        r,
        "    }};\n    let d_{} = {}::new({0}_emit{}",
        snake_case(name),
        o.name,
        model
    )?;
    for (name, p) in &o.properties {
        if p.is_object() {
            write!(r, ",\n        d_{}", snake_case(name))?;
        }
    }
    writeln!(r, ");")
}

pub(super) fn write_function(
    r: &mut Vec<u8>,
    (name, func): (&String, &Function),
    lcname: &str,
    object: &Object,
) -> Result<()> {
    let lower_case = snake_case(name);
    write!(
        r,
        "
#[no_mangle]
pub unsafe extern \"C\" fn {lcname}_{lower_case}(ptr: *{is_mut} {object_name}",
        lcname = lcname,
        lower_case = lower_case,
        is_mut = if func.mutable { "mut" } else { "const" },
        object_name = object.name
    )?;

    // write all the input arguments, for QString and QByteArray, write
    // pointers to their content and the length which is int in Qt
    for arg in func.arguments.iter() {
        write!(r, ", ")?;
        if arg.argument_type.name() == "QString" {
            write!(r, "{}_str: *const c_ushort, {0}_len: c_int", arg.name)?;
        } else if arg.argument_type.name() == "QByteArray" {
            write!(r, "{}_str: *const c_char, {0}_len: c_int", arg.name)?;
        } else {
            write!(r, "{}: {}", arg.name, arg.argument_type.rust_type())?;
        }
    }

    // If the return type is QString or QByteArray, append a pointer to the
    // variable that will be set to the argument list. Also add a setter
    // function.
    if func.return_type.is_complex() {
        writeln!(
            r,
            ", d: *mut {}, set: fn(*mut {0}, str: *const c_char, len: c_int)) {{",
            func.return_type.name()
        )?;
    } else if func.return_type == SimpleType::Void {
        writeln!(r, ") {{")?;
    } else {
        writeln!(r, ") -> {} {{", func.return_type.rust_type())?;
    }

    for arg in &func.arguments {
        if arg.argument_type.name() == "QString" {
            writeln!(
                r,
                "    let mut {} = String::new();
    set_string_from_utf16(&mut {0}, {0}_str, {0}_len);",
                arg.name
            )?;
        } else if arg.argument_type.name() == "QByteArray" {
            writeln!(r, "let {} = {{ qba_slice!({0}_str, {0}_len) }};", arg.name)?;
        }
    }

    if func.mutable {
        writeln!(r, "    let o = &mut *ptr;")?;
    } else {
        writeln!(r, "    let o = &*ptr;")?;
    }

    if func.return_type.is_complex() {
        write!(r, "    let r = o.{}(", lower_case)?;
    } else {
        write!(r, "    o.{}(", lower_case)?;
    }

    for (i, a) in func.arguments.iter().enumerate() {
        if i > 0 {
            write!(r, ", ")?;
        }
        write!(r, "{}", a.name)?;
    }

    write!(r, ")")?;

    if func.return_type.is_complex() {
        writeln!(r, ";")?;
        writeln!(
            r,
            "    let s: *const c_char = r.as_ptr() as (*const c_char);
    set(d, s, r.len() as i32);"
        )?;
    } else {
        writeln!(r)?;
    }
    writeln!(r, "}}")
}
