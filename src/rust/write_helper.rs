use super::*;

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

    for arg in func.arguments.iter() {
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
