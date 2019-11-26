use super::*;

pub(super) fn setters(
    write_buf: &mut Vec<u8>,
    obj: &Object,
    name: &str,
    prop: &Property,
) -> Result<()> {
    let base = format!("{}_{}", snake_case(&obj.name), snake_case(name));

    let t = if prop.optional && !prop.is_complex() {
        "const QVariant&"
    } else {
        prop.property_type.cpp_set_type()
    };

    writeln!(
        write_buf,
        "void {}::set{}({} v) {{",
        obj.name,
        upper_initial(name),
        t
    )?;

    if prop.optional {
        if prop.is_complex() {
            writeln!(write_buf, "if (v.isNull()) {{")?;
        } else {
            writeln!(
                write_buf,
                "if (v.isNull() || !v.canConvert<{}>()) {{",
                prop.type_name()
            )?;
        }

        writeln!(write_buf, "{}_set_none(m_d);", base)?;

        writeln!(write_buf, "}} else {{")?;

        if prop.type_name() == "QString" {
            writeln!(
                write_buf,
                "{}_set(m_d, reinterpret_cast<const ushort*>(v.data()), v.size());",
                base
            )?;
        } else if prop.type_name() == "QByteArray" {
            writeln!(write_buf, "{}_set(m_d, v.data(), v.size());", base)?;
        } else if prop.optional {
            writeln!(
                write_buf,
                "{}_set(m_d, v.value<{}>());",
                base,
                prop.type_name()
            )?;
        } else {
            writeln!(write_buf, "{}_set(m_d, v);", base)?;
        }

        writeln!(write_buf, "}}")?;
    } else {
        non_optional(write_buf, prop, name, obj)?;
    }

    writeln!(write_buf, "}}")?;

    Ok(())
}

fn non_optional(write_buf: &mut Vec<u8>, prop: &Property, name: &str, obj: &Object) -> Result<()> {
    let base = format!("{}_{}", snake_case(&obj.name), snake_case(name));

    match prop.property_type {
        Type::Simple(SimpleType::QString) => {
            writeln!(
                write_buf,
                "{}_set(m_d, reinterpret_cast<const ushort*>(v.data()), v.size());",
                base
            )?;
        }
        Type::Simple(SimpleType::QByteArray) => {
            writeln!(write_buf, "{}_set(m_d, v.data(), v.size());", base)?;
        }
        _ => {
            writeln!(write_buf, "{}_set(m_d, v);", base)?;
        }
    }

    Ok(())
}
