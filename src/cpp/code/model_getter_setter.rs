use super::*;

pub(super) fn write_model_getter_setter(
    w: &mut Vec<u8>,
    index: &str,
    name: &str,
    ip: &ItemProperty,
    o: &Object,
) -> Result<()> {
    let lcname = snake_case(&o.name);

    let mut idx = index;

    // getter
    let mut r = property_type(ip);

    if o.object_type == ObjectType::List {
        idx = ", row";
        writeln!(w, "{} {}::{}(int row) const\n{{", r, o.name, name)?;
    } else {
        writeln!(
            w,
            "{} {}::{}(const QModelIndex& index) const\n{{",
            r, o.name, name
        )?;
    }

    if ip.type_name() == "QString" {
        writeln!(w, "    QString s;")?;
        writeln!(
            w,
            "    {}_data_{}(m_d{}, &s, set_{});",
            lcname,
            snake_case(name),
            idx,
            ip.type_name().to_lowercase()
        )?;

        writeln!(w, "    return s;")?;
    } else if ip.type_name() == "QByteArray" {
        writeln!(w, "    QByteArray b;")?;
        writeln!(
            w,
            "    {}_data_{}(m_d{}, &b, set_{});",
            lcname,
            snake_case(name),
            idx,
            ip.type_name().to_lowercase()
        )?;

        writeln!(w, "    return b;")?;
    } else if ip.optional {
        writeln!(w, "    QVariant v;")?;
        writeln!(
            w,
            "    v = {}_data_{}(m_d{});",
            lcname,
            snake_case(name),
            idx
        )?;

        writeln!(w, "    return v;")?;
    } else {
        writeln!(
            w,
            "    return {}_data_{}(m_d{});",
            lcname,
            snake_case(name),
            idx
        )?;
    }

    writeln!(w, "}}\n")?;

    if !ip.write {
        return Ok(());
    }

    //setter
    if r == "QVariant" || ip.is_complex() {
        r = format!("const {}&", r);
    }

    if o.object_type == ObjectType::List {
        idx = ", row";
        writeln!(
            w,
            "bool {}::set{}(int row, {} value)\n{{",
            o.name,
            upper_initial(name),
            r
        )?;
    } else {
        writeln!(
            w,
            "bool {}::set{}(const QModelIndex& index, {} value)\n{{",
            o.name,
            upper_initial(name),
            r
        )?;
    }

    writeln!(w, "    bool set = false;")?;

    if ip.optional {
        let mut test = "value.isNull()".to_string();
        if !ip.is_complex() {
            test += " || !value.isValid()";
        }

        writeln!(w, "    if ({}) {{", test)?;

        writeln!(
            w,
            "        set = {}_set_data_{}_none(m_d{});",
            lcname,
            snake_case(name),
            idx
        )?;

        writeln!(w, "    }} else {{")?;
    }

    if ip.optional && !ip.is_complex() {
        writeln!(
            w,
            "    if (!value.canConvert(qMetaTypeId<{}>())) {{
        return false;
    }}",
            ip.type_name()
        )?;
        writeln!(
            w,
            "    set = {}_set_data_{}(m_d{}, value.value<{}>());",
            lcname,
            snake_case(name),
            idx,
            ip.type_name()
        )?;
    } else {
        let mut val = "value";
        if ip.is_complex() {
            if ip.type_name() == "QString" {
                val = "value.utf16(), value.length()";
            } else {
                val = "value.data(), value.length()";
            }
        }

        writeln!(
            w,
            "    set = {}_set_data_{}(m_d{}, {});",
            lcname,
            snake_case(name),
            idx,
            val
        )?;
    }

    if ip.optional {
        writeln!(w, "    }}")?;
    }

    if o.object_type == ObjectType::List {
        writeln!(
            w,
            "    if (set) {{
        QModelIndex index = createIndex(row, 0, row);
        Q_EMIT dataChanged(index, index);
    }}
    return set;
}}
"
        )?;
    } else {
        writeln!(
            w,
            "    if (set) {{
        Q_EMIT dataChanged(index, index);
    }}
    return set;
}}
"
        )?;
    }

    Ok(())
}
