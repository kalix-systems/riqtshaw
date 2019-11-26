use super::*;

pub(super) fn properties(w: &mut Vec<u8>, obj: &Object) -> Result<()> {
    let lcname = snake_case(&obj.name);

    for (name, p) in obj.properties.iter() {
        let base = format!("{}_{}", lcname, snake_case(name));

        if p.is_object() {
            writeln!(
                w,
                "
const {}* {}::{}() const
{{
    return m_{2};
}}
{0}* {1}::{2}()
{{
    return m_{2};
}}",
                p.type_name(),
                obj.name,
                name
            )?;
        } else if p.is_complex() {
            writeln!(
                w,
                "
{} {}::{}() const
{{
    {0} v;
    {3}_get(m_d, &v, set_{4});
    return v;
}}",
                p.type_name(),
                obj.name,
                name,
                base,
                p.type_name().to_lowercase()
            )?;
        } else if p.optional {
            writeln!(
                w,
                "
QVariant {}::{}() const
{{
    QVariant v;
    auto r = {2}_get(m_d);
    if (r.some) {{
        v.setValue(r.value);
    }}
    return r;
}}",
                obj.name, name, base
            )?;
        } else {
            writeln!(
                w,
                "
{} {}::{}() const
{{
    return {}_get(m_d);
}}",
                p.type_name(),
                obj.name,
                name,
                base
            )?;
        }

        if p.write {
            let t = if p.optional && !p.is_complex() {
                "const QVariant&"
            } else {
                p.property_type.cpp_set_type()
            };

            writeln!(
                w,
                "void {}::set{}({} v) {{",
                obj.name,
                upper_initial(name),
                t
            )?;

            if p.optional {
                if p.is_complex() {
                    writeln!(w, "if (v.isNull()) {{")?;
                } else {
                    writeln!(
                        w,
                        "if (v.isNull() || !v.canConvert<{}>()) {{",
                        p.type_name()
                    )?;
                }

                writeln!(w, "{}_set_none(m_d);", base)?;

                writeln!(w, "}} else {{")?;

                if p.type_name() == "QString" {
                    writeln!(
                        w,
                        "{}_set(m_d, reinterpret_cast<const ushort*>(v.data()), v.size());",
                        base
                    )?;
                } else if p.type_name() == "QByteArray" {
                    writeln!(w, "{}_set(m_d, v.data(), v.size());", base)?;
                } else if p.optional {
                    writeln!(w, "{}_set(m_d, v.value<{}>());", base, p.type_name())?;
                } else {
                    writeln!(w, "{}_set(m_d, v);", base)?;
                }

                writeln!(w, "}}")?;
            } else if p.type_name() == "QString" {
                writeln!(
                    w,
                    "{}_set(m_d, reinterpret_cast<const ushort*>(v.data()), v.size());",
                    base
                )?;
            } else if p.type_name() == "QByteArray" {
                writeln!(w, "{}_set(m_d, v.data(), v.size());", base)?;
            } else {
                writeln!(w, "{}_set(m_d, v);", base)?;
            }

            writeln!(w, "}}")?;
        }
    }
    Ok(())
}
