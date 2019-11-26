use super::*;
mod setters;

pub(super) fn properties(write_buf: &mut Vec<u8>, obj: &Object) -> Result<()> {
    for (name, prop) in obj.properties.iter() {
        if prop.is_object() {
            object(write_buf, obj, name, prop)?;
        } else if prop.is_complex() {
            complex(write_buf, obj, name, prop)?;
        } else if prop.optional {
            non_complex_optional(write_buf, obj, name)?;
        } else {
            non_complex_non_optional(write_buf, obj, name, prop)?;
        }

        if prop.write {
            setters::setters(write_buf, obj, name, prop)?;
        }
    }

    Ok(())
}

fn object(write_buf: &mut Vec<u8>, obj: &Object, name: &str, prop: &Property) -> Result<()> {
    writeln!(
        write_buf,
        "
const {typ}* {obj_name}::{prop_name}() const
{{
    return m_{prop_name};
}}
{typ}* {obj_name}::{prop_name}()
{{
    return m_{prop_name};
}}",
        typ = prop.type_name(),
        obj_name = obj.name,
        prop_name = name
    )?;

    Ok(())
}

fn complex(write_buf: &mut Vec<u8>, obj: &Object, name: &str, prop: &Property) -> Result<()> {
    let base = format!("{}_{}", snake_case(&obj.name), snake_case(name));
    writeln!(
        write_buf,
        "
   {typ} {obj_name}::{name}() const
   {{
       {typ} v;
       {base}_get(m_d, &v, set_{typ_lower_case});
       return v;
   }}",
        typ = prop.type_name(),
        obj_name = obj.name,
        name = name,
        base = base,
        typ_lower_case = prop.type_name().to_lowercase()
    )?;
    Ok(())
}

fn non_complex_optional(write_buf: &mut Vec<u8>, obj: &Object, name: &str) -> Result<()> {
    let base = format!("{}_{}", snake_case(&obj.name), snake_case(name));

    writeln!(
        write_buf,
        "
QVariant {obj_name}::{name}() const
{{
    QVariant v;
    auto r = {base}_get(m_d);
    if (r.some) {{
        v.setValue(r.value);
    }}
    return r;
}}",
        obj_name = obj.name,
        name = name,
        base = base
    )?;

    Ok(())
}

fn non_complex_non_optional(
    write_buf: &mut Vec<u8>,
    obj: &Object,
    name: &str,
    prop: &Property,
) -> Result<()> {
    let base = format!("{}_{}", snake_case(&obj.name), snake_case(name));

    writeln!(
        write_buf,
        "
{typ} {obj_name}::{name}() const
{{
    return {base}_get(m_d);
}}",
        typ = prop.type_name(),
        obj_name = obj.name,
        name = name,
        base = base
    )?;

    Ok(())
}
