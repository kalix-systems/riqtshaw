use super::*;

pub(super) fn getter(object: &Object, prop_name: &str, property: &Property) -> Func {
    let mut func = Func::new(&format!("{}_get", base(object, prop_name)));
    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*const {}", &object.name))
        .arg("prop", format!("*mut {}", property.type_name()))
        .arg(
            "set",
            format!("fn(*mut {}, *const c_char, c_int)", property.type_name()),
        )
        .line("let obj = &*ptr;")
        .line(format!("let value = obj.{}();", snake_case(prop_name)));

    let mut block = Block::new("if let Some(value) = value");
    block.line("let str_: *const c_char = value.as_ptr() as (*const c_char);");
    block.line("set(prop, str_, to_c_int(value.len()));");

    func.push_block(block);

    func
}

pub(super) fn qbytearray_setter(object: &Object, prop_name: &str) -> Func {
    let mut func = Func::new(&format!("{}_set", base(object, prop_name)));
    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", &object.name))
        .arg("value", "*const c_char")
        .arg("len", "c_int")
        .line("let obj = &mut *ptr;")
        .line("let value = qba_slice!(value, len);")
        .line(format!(
            "obj.set_{}(Some(value.into()));",
            snake_case(prop_name)
        ));

    func
}

pub(super) fn qstring_setter(object: &Object, prop_name: &str) -> Func {
    let mut func = Func::new(&format!("{}_set", base(object, prop_name)));
    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", &object.name))
        .arg("value", "*const c_ushort")
        .arg("len", "c_int")
        .line("let obj = &mut *ptr;")
        .line("let mut s = String::new();")
        .line("set_string_from_utf16(&mut s, value, len);")
        .line(format!("obj.set_{}(Some(s));", snake_case(prop_name)));

    func
}
