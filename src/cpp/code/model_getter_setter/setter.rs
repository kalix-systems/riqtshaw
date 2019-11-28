use super::*;

pub(super) fn item_prop_write(
    write_buf: &mut Vec<u8>,
    name: &str,
    item_prop: &ItemProperty,
    obj: &Object,
    mut read_type: String,
    idx: &str,
) -> Result<()> {
    if read_type == "QVariant" || item_prop.is_complex() {
        read_type = format!("const {}&", read_type);
    }

    writeln!(
        write_buf,
        "bool {}::set{}(int row, {} value)\n{{",
        obj.name,
        upper_initial(name),
        read_type
    )?;

    writeln!(write_buf, "bool set = false;")?;

    if item_prop.optional {
        optional(write_buf, item_prop, name, idx, obj, setter_body)?;
    } else {
        setter_body(write_buf, item_prop, name, idx, obj)?;
    }

    writeln!(
        write_buf,
        "
    if (set) {{
        QModelIndex index = createIndex(row, 0, row);
        Q_EMIT dataChanged(index, index);
    }}
    return set;
}}
"
    )?;

    Ok(())
}

fn optional_non_complex(
    write_buf: &mut Vec<u8>,
    item_prop: &ItemProperty,
    name: &str,
    idx: &str,
    obj: &Object,
) -> Result<()> {
    writeln!(
        write_buf,
        "    if (!value.canConvert(qMetaTypeId<{}>())) {{
        return false;
    }}",
        item_prop.type_name()
    )?;

    writeln!(
        write_buf,
        "    set = {}_set_data_{}(m_d{}, value.value<{}>());",
        snake_case(&obj.name),
        snake_case(name),
        idx,
        item_prop.type_name()
    )?;

    Ok(())
}

fn optional<F: Fn(&mut Vec<u8>, &ItemProperty, &str, &str, &Object) -> Result<()>>(
    write_buf: &mut Vec<u8>,
    item_prop: &ItemProperty,
    name: &str,
    idx: &str,
    obj: &Object,
    content: F,
) -> Result<()> {
    let test = if item_prop.is_complex() {
        "value.isNull()"
    } else {
        "value.isNull() || !value.isValid()"
    };

    writeln!(write_buf, "    if ({}) {{", test)?;

    writeln!(
        write_buf,
        "        set = {}_set_data_{}_none(m_d{});",
        snake_case(&obj.name),
        snake_case(name),
        idx
    )?;

    writeln!(write_buf, "    }} else {{")?;

    content(write_buf, item_prop, name, idx, obj)?;

    writeln!(write_buf, "    }}")?;
    Ok(())
}

fn setter_body(
    write_buf: &mut Vec<u8>,
    item_prop: &ItemProperty,
    name: &str,
    idx: &str,
    obj: &Object,
) -> Result<()> {
    if item_prop.optional && !item_prop.is_complex() {
        optional_non_complex(write_buf, item_prop, name, idx, obj)?;

        Ok(())
    } else {
        let val = match item_prop.item_property_type {
            Type::Simple(SimpleType::QString) => "value.utf16(), value.length()",
            Type::Simple(SimpleType::QByteArray) => "value.data(), value.length()",
            _ => "value",
        };

        writeln!(
            write_buf,
            "    set = {}_set_data_{}(m_d{}, {});",
            snake_case(&obj.name),
            snake_case(name),
            idx,
            val
        )?;

        Ok(())
    }
}
