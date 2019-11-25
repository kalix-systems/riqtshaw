#![allow(unused)]

use super::*;

const ROW_IX: &str = "to_usize(row).unwrap_or(0)";

pub(super) fn complex_data(object: &Object, name: &str, item_prop: &ItemProperty) -> Func {
    let mut func = Func::new(&format!(
        "{}_data_{}",
        snake_case(&object.name),
        snake_case(name)
    ));

    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*const {}", &object.name))
        .arg("row", "c_int")
        .arg("d", format!("*mut {}", item_prop.type_name()))
        .arg(
            "set",
            format!(
                "fn(*mut {}, *const c_char, len: c_int)",
                item_prop.type_name()
            ),
        )
        .line("let obj = &*ptr;")
        .line(&format!(
            "let data = obj.{name}({row_ix});",
            name = snake_case(name),
            row_ix = ROW_IX
        ));

    if item_prop.optional {
        let mut block = Block::new("if let Some(data) = data");
        block
            .line("let str_: *const c_char = data.as_ptr() as (*const c_char);")
            .line("set(d, str_, to_c_int(data.len()));");

        func.push_block(block);
    } else {
        func.line("let str_: *const c_char = data.as_ptr() as *const c_char;")
            .line("set(d, str_, to_c_int(data.len()));");
    }

    func
}

pub(super) fn non_complex_data(object: &Object, name: &str, item_prop: &ItemProperty) -> Func {
    let mut func = Func::new(&format!(
        "{}_data_{}",
        snake_case(&object.name),
        snake_case(name)
    ));

    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*const {}", &object.name))
        .arg("row", "c_int")
        .ret(rust_c_type(item_prop))
        .line("let obj = &*ptr;");

    if item_prop.optional {
        func.line(&format!(
            "obj.{name}({row_ix}).into()",
            name = snake_case(name),
            row_ix = ROW_IX
        ));
    } else {
        func.line(&format!(
            "obj.{name}({row_ix})",
            name = snake_case(name),
            row_ix = ROW_IX
        ));
    }

    func
}

pub(super) fn non_complex_set(object: &Object, name: &str, item_prop: &ItemProperty) -> Func {
    let mut func = Func::new(&format!(
        "{}_set_data_{}",
        snake_case(&object.name),
        snake_case(name)
    ));

    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", &format!("*mut {}", object.name))
        .arg("row", "c_int")
        .arg("value", item_prop.item_property_type.rust_type())
        .ret("bool");

    if item_prop.optional {
        func.line(&format!(
            "(&mut *ptr).set_{name}({row_ix}, Some(value))",
            name = snake_case(name),
            row_ix = ROW_IX
        ));
    } else {
        func.line(&format!(
            "(&mut *ptr).set_{name}({row_ix}, value)",
            name = snake_case(name),
            row_ix = ROW_IX
        ));
    }

    func
}

pub(super) fn set_none(object: &Object, name: &str, item_prop: &ItemProperty) -> Func {
    let mut func = Func::new(&format!(
        "{}_set_data_{}_none",
        snake_case(&object.name),
        snake_case(name)
    ));

    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", &object.name))
        .arg("row", "c_int")
        .ret("bool")
        .line(format!(
            "(&mut *ptr).set_{name}({row_ix}, None)",
            name = snake_case(name),
            row_ix = ROW_IX
        ));

    func
}

pub(super) fn qstring_set(object: &Object, name: &str, item_prop: &ItemProperty) -> Func {
    let mut func = Func::new(&format!(
        "{}_set_data_{}",
        snake_case(&object.name),
        snake_case(name)
    ));

    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", &object.name))
        .arg("row", "c_int")
        .arg("str_", "*const c_ushort")
        .arg("len", "c_int")
        .ret("bool")
        .line("let obj = &mut *ptr;")
        .line("let mut value = String::new();")
        .line("set_string_from_utf16(&mut value, str_, len);");

    if item_prop.optional {
        func.line(&format!(
            "obj.set_{name}({row_ix}, Some(value))",
            name = snake_case(name),
            row_ix = ROW_IX
        ));
    } else {
        func.line(&format!(
            "obj.set_{name}({row_ix}, value)",
            name = snake_case(name),
            row_ix = ROW_IX
        ));
    }

    func
}

pub(super) fn qbytearray_set(object: &Object, name: &str, item_prop: &ItemProperty) -> Func {
    let mut func = Func::new(&format!(
        "{}_set_data_{}",
        snake_case(&object.name),
        snake_case(name)
    ));

    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", &object.name))
        .arg("row", "c_int")
        .arg("bs", "*const c_ushort")
        .arg("len", "c_int")
        .ret("bool")
        .line("let obj = &mut *ptr;")
        .line("let slice = qba_slice!(bs, len);");

    if item_prop.optional {
        func.line(&format!(
            "obj.set_{name}({row_ix}, Some(slice))",
            name = snake_case(name),
            row_ix = ROW_IX
        ));
    } else {
        func.line(&format!(
            "obj.set_{name}({row_ix}, slice)",
            name = snake_case(name),
            row_ix = ROW_IX
        ));
    }

    func
}
