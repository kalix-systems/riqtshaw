use super::*;

pub(super) fn function((fn_name, fn_def): (&str, &Function), object: &Object) -> Func {
    let mut func = Func::new(&format!(
        "{obj_name}_{fn_name}",
        obj_name = snake_case(&object.name),
        fn_name = snake_case(fn_name)
    ));

    func.attr("no_mangle").vis("pub unsafe").extern_abi("C");

    if fn_def.mutable {
        func.arg("ptr", format!("*mut {}", object.name));
        func.line("let obj = &mut *ptr;");
    } else {
        func.arg("ptr", format!("*const {}", object.name));
        func.line("let obj = &*ptr;");
    }

    for arg in fn_def.arguments.iter() {
        match arg.argument_type {
            SimpleType::QString => {
                func.arg(&format!("{}_str", arg.name), "*const c_ushort");
                func.arg(&format!("{}_len", arg.name), "c_int");

                func.line(&format!("let mut {} = String::new();", arg.name));
                func.line(&format!(
                    "set_string_from_utf16(&mut {name}, {name}_str, {name}_len);",
                    name = arg.name
                ));
            }
            SimpleType::QByteArray => {
                func.arg(&format!("{}_str", arg.name), "*const c_char");
                func.arg(&format!("{}_len", arg.name), "c_int");

                func.line(&format!(
                    "let {name} = {{ qba_slice!({name}_str, {name}_len) }};",
                    name = arg.name
                ));
            }
            _ => {
                func.arg(&arg.name, arg.argument_type.rust_type());
            }
        }
    }

    match &fn_def.return_type {
        ret @ SimpleType::QByteArray | ret @ SimpleType::QString => {
            func.arg("data", format!("*mut {}", ret.name()));
            func.arg(
                "set",
                format!("fn(*mut {}, str_: *const c_char, len: c_int)", ret.name()),
            );

            func.line(format!("let ret = obj.{}(", snake_case(&fn_name)));
            write_arg_names(&fn_def, &mut func);
            func.line(";");

            func.line("let str_: *const c_char = ret.as_ptr() as (*const c_char);");
            func.line("set(data, str_, ret.len() as i32);");
        }
        ret => {
            func.ret(ret.rust_type());

            func.line(format!("obj.{}(", snake_case(&fn_name)));
            write_arg_names(&fn_def, &mut func);
        }
    }

    func
}

fn write_arg_names(fn_def: &Function, func: &mut Func) {
    for arg in fn_def.arguments.iter() {
        func.line(&format!("{},", arg.name));
    }

    func.line(")");
}
