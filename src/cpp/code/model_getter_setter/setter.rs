use super::*;

pub(super) fn item_prop_write(
    write_buf: &mut Vec<u8>,
    name: &str,
    item_prop: &ItemProperty,
    obj: &Object,
    mut read_type: String,
    mut idx: &str,
) -> Result<()> {
    let lcname = snake_case(&obj.name);

    if read_type == "QVariant" || item_prop.is_complex() {
        read_type = format!("const {}&", read_type);
    }

    match obj.object_type {
        ObjectType::List => {
            idx = ", row";
            writeln!(
                write_buf,
                "bool {}::set{}(int row, {} value)\n{{",
                obj.name,
                upper_initial(name),
                read_type
            )?;
        }
        ObjectType::Tree => {
            writeln!(
                write_buf,
                "bool {}::set{}(const QModelIndex& index, {} value)\n{{",
                obj.name,
                upper_initial(name),
                read_type
            )?;
        }
        _ => unreachable!(),
    }

    writeln!(write_buf, "bool set = false;")?;

    if item_prop.optional {
        let mut test = "value.isNull()".to_string();
        if !item_prop.is_complex() {
            test += " || !value.isValid()";
        }

        writeln!(write_buf, "    if ({}) {{", test)?;

        writeln!(
            write_buf,
            "        set = {}_set_data_{}_none(m_d{});",
            lcname,
            snake_case(name),
            idx
        )?;

        writeln!(write_buf, "    }} else {{")?;
    }

    if item_prop.optional && !item_prop.is_complex() {
        optional_non_complex(write_buf, item_prop, name, idx, obj)?;
    } else {
        let mut val = "value";

        if item_prop.is_complex() {
            if item_prop.type_name() == "QString" {
                val = "value.utf16(), value.length()";
            } else {
                val = "value.data(), value.length()";
            }
        }

        writeln!(
            write_buf,
            "    set = {}_set_data_{}(m_d{}, {});",
            lcname,
            snake_case(name),
            idx,
            val
        )?;
    }

    if item_prop.optional {
        writeln!(write_buf, "    }}")?;
    }

    match obj.object_type {
        ObjectType::List => {
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
        }
        ObjectType::Tree => {
            writeln!(
                write_buf,
                "
    if (set) {{
        Q_EMIT dataChanged(index, index);
    }}
    return set;
}}
"
            )?;
        }
        _ => unreachable!(),
    }

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
