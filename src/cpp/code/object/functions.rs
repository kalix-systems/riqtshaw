use super::*;

pub(super) fn functions(writebuf: &mut Vec<u8>, obj: &Object) -> Result<()> {
    let lcname = snake_case(&obj.name);

    for (name, func) in obj.functions.iter() {
        let base = format!("{}_{}", lcname, snake_case(name));

        write!(writebuf, "{} {}::{}(", func.type_name(), obj.name, name)?;

        for (i, arg) in func.arguments.iter().enumerate() {
            write!(
                writebuf,
                "{} {}{}",
                arg.argument_type.cpp_set_type(),
                arg.name,
                if i + 1 < func.arguments.len() {
                    ", "
                } else {
                    ""
                }
            )?;
        }

        writeln!(
            writebuf,
            "){}\n{{",
            if func.mutable { "" } else { " const" }
        )?;

        let mut arg_list = String::new();

        for arg in func.arguments.iter() {
            if arg.type_name() == "QString" {
                arg_list.push_str(&format!(", {}.utf16(), {0}.size()", arg.name));
            } else if arg.type_name() == "QByteArray" {
                arg_list.push_str(&format!(", {}.data(), {0}.size()", arg.name));
            } else {
                arg_list.push_str(&format!(", {}", arg.name));
            }
        }

        if func.return_type.name() == "QString" {
            writeln!(
                writebuf,
                "    {} s;
    {}(m_d{}, &s, set_qstring);
    return s;",
                func.type_name(),
                base,
                arg_list
            )?;
        } else if func.return_type.name() == "QByteArray" {
            writeln!(
                writebuf,
                "    {} s;
    {}(m_d{}, &s, set_qbytearray);
    return s;",
                func.type_name(),
                base,
                arg_list
            )?;
        } else {
            writeln!(writebuf, "    return {}(m_d{});", base, arg_list)?;
        }

        writeln!(writebuf, "}}")?;
    }
    Ok(())
}
