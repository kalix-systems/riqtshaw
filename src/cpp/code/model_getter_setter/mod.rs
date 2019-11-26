use super::*;

mod getter;
mod setter;

pub(super) fn write_model_getter_setter(
    write_buf: &mut Vec<u8>,
    index: &str,
    name: &str,
    item_prop: &ItemProperty,
    obj: &Object,
) -> Result<()> {
    let read_type = property_type(item_prop);

    let idx = match obj.object_type {
        ObjectType::List => {
            writeln!(
                write_buf,
                "{} {}::{}(int row) const\n{{",
                read_type, obj.name, name
            )?;
            ", row"
        }
        ObjectType::Tree => {
            writeln!(
                write_buf,
                "{} {}::{}(const QModelIndex& index) const\n{{",
                read_type, obj.name, name
            )?;
            index
        }
        _ => unreachable!(),
    };

    match item_prop.item_property_type {
        Type::Simple(SimpleType::QString) => {
            getter::qstring(write_buf, name, idx, item_prop, obj)?;
        }
        Type::Simple(SimpleType::QByteArray) => {
            getter::qbytearray(write_buf, name, idx, item_prop, obj)?;
        }
        _ => {
            if item_prop.optional {
                getter::non_complex_optional(write_buf, name, idx, obj)?;
            } else {
                getter::non_complex_non_optional(write_buf, name, idx, obj)?;
            }
        }
    }

    writeln!(write_buf, "}}\n")?;

    if !item_prop.write {
        return Ok(());
    }

    setter::item_prop_write(write_buf, name, item_prop, obj, read_type, idx)
}
