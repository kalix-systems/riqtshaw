use super::*;

pub(super) fn write_cpp_object_properties(w: &mut Vec<u8>, o: &Object, lcname: &str) -> Result<()> {
    for (name, p) in &o.properties {
        let base = format!("{}_{}", lcname, snake_case(name));

        if p.is_object() {
            writeln!(
                w,
                "const {}* {}::{}() const
{{
    return m_{2};
}}
{0}* {1}::{2}()
{{
    return m_{2};
}}",
                p.type_name(),
                o.name,
                name
            )?;
        } else if p.is_complex() {
            writeln!(
                w,
                "{} {}::{}() const
{{
    {0} v;
    {3}_get(m_d, &v, set_{4});
    return v;
}}",
                p.type_name(),
                o.name,
                name,
                base,
                p.type_name().to_lowercase()
            )?;
        } else if p.optional {
            writeln!(
                w,
                "QVariant {}::{}() const
{{
    QVariant v;
    auto r = {2}_get(m_d);
    if (r.some) {{
        v.setValue(r.value);
    }}
    return r;
}}",
                o.name, name, base
            )?;
        } else {
            writeln!(
                w,
                "{} {}::{}() const
{{
    return {}_get(m_d);
}}",
                p.type_name(),
                o.name,
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

            writeln!(w, "void {}::set{}({} v) {{", o.name, upper_initial(name), t)?;

            if p.optional {
                if p.is_complex() {
                    writeln!(w, "    if (v.isNull()) {{")?;
                } else {
                    writeln!(
                        w,
                        "    if (v.isNull() || !v.canConvert<{}>()) {{",
                        p.type_name()
                    )?;
                }

                writeln!(w, "        {}_set_none(m_d);", base)?;

                writeln!(w, "    }} else {{")?;

                if p.type_name() == "QString" {
                    writeln!(
                        w,
                        "    {}_set(m_d, reinterpret_cast<const ushort*>(v.data()), v.size());",
                        base
                    )?;
                } else if p.type_name() == "QByteArray" {
                    writeln!(w, "    {}_set(m_d, v.data(), v.size());", base)?;
                } else if p.optional {
                    writeln!(
                        w,
                        "        {}_set(m_d, v.value<{}>());",
                        base,
                        p.type_name()
                    )?;
                } else {
                    writeln!(w, "        {}_set(m_d, v);", base)?;
                }
                writeln!(w, "    }}")?;
            } else if p.type_name() == "QString" {
                writeln!(
                    w,
                    "    {}_set(m_d, reinterpret_cast<const ushort*>(v.data()), v.size());",
                    base
                )?;
            } else if p.type_name() == "QByteArray" {
                writeln!(w, "    {}_set(m_d, v.data(), v.size());", base)?;
            } else {
                writeln!(w, "    {}_set(m_d, v);", base)?;
            }

            writeln!(w, "}}")?;
        }
    }
    Ok(())
}

pub(super) fn write_cpp_object(w: &mut Vec<u8>, o: &Object, conf: &Config) -> Result<()> {
    let lcname = snake_case(&o.name);

    writeln!(
        w,
        "{}::{0}(bool /*owned*/, QObject *parent):
    {}(parent),",
        o.name,
        base_type(o)
    )?;

    initialize_members_zero(w, o)?;

    writeln!(
        w,
        "    m_d(nullptr),
    m_ownsPrivate(false)
{{"
    )?;

    if o.object_type != ObjectType::Object {
        writeln!(w, "initHeaderData();")?;
    }

    writeln!(
        w,
        "}}
{}::{0}(QObject *parent):
    {}(parent),",
        o.name,
        base_type(o)
    )?;

    initialize_members_zero(w, o)?;

    write!(w, "    m_d({}_new(this", lcname)?;

    constructor_args(w, "", o, conf)?;

    writeln!(
        w,
        ")),
    m_ownsPrivate(true)
{{"
    )?;

    initialize_members(w, "", o, conf)?;

    connect(w, "this", o, conf)?;

    if o.object_type != ObjectType::Object {
        writeln!(w, "    initHeaderData();")?;
    }

    writeln!(
        w,
        "}}

{}::~{0}() {{
    if (m_ownsPrivate) {{
        {1}_free(m_d);
    }}
}}",
        o.name, lcname
    )?;

    write_abstract_item_header_data_function(o, w)?;

    write_cpp_object_properties(w, o, &lcname)?;

    for (name, f) in &o.functions {
        let base = format!("{}_{}", lcname, snake_case(name));

        write!(w, "{} {}::{}(", f.type_name(), o.name, name)?;

        for (i, a) in f.arguments.iter().enumerate() {
            write!(
                w,
                "{} {}{}",
                a.argument_type.cpp_set_type(),
                a.name,
                if i + 1 < f.arguments.len() { ", " } else { "" }
            )?;
        }

        writeln!(w, "){}\n{{", if f.mutable { "" } else { " const" })?;

        let mut arg_list = String::new();

        for a in &f.arguments {
            if a.type_name() == "QString" {
                arg_list.push_str(&format!(", {}.utf16(), {0}.size()", a.name));
            } else if a.type_name() == "QByteArray" {
                arg_list.push_str(&format!(", {}.data(), {0}.size()", a.name));
            } else {
                arg_list.push_str(&format!(", {}", a.name));
            }
        }

        if f.return_type.name() == "QString" {
            writeln!(
                w,
                "    {} s;
    {}(m_d{}, &s, set_qstring);
    return s;",
                f.type_name(),
                base,
                arg_list
            )?;
        } else if f.return_type.name() == "QByteArray" {
            writeln!(
                w,
                "    {} s;
    {}(m_d{}, &s, set_qbytearray);
    return s;",
                f.type_name(),
                base,
                arg_list
            )?;
        } else {
            writeln!(w, "    return {}(m_d{});", base, arg_list)?;
        }

        writeln!(w, "}}")?;
    }

    Ok(())
}
