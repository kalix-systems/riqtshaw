use super::*;

pub(super) fn qstring(
    write_buf: &mut Vec<u8>,
    name: &str,
    idx: &str,
    item_prop: &ItemProperty,
    obj: &Object,
) -> Result<()> {
    writeln!(write_buf, "    QString s;")?;
    writeln!(
        write_buf,
        "    {}_data_{}(m_d{}, &s, set_{});",
        snake_case(&obj.name),
        snake_case(name),
        idx,
        item_prop.type_name().to_lowercase()
    )?;

    writeln!(write_buf, "    return s;")?;

    Ok(())
}

pub(super) fn qbytearray(
    write_buf: &mut Vec<u8>,
    name: &str,
    idx: &str,
    item_prop: &ItemProperty,
    obj: &Object,
) -> Result<()> {
    writeln!(write_buf, "    QByteArray b;")?;
    writeln!(
        write_buf,
        "    {}_data_{}(m_d{}, &b, set_{});",
        snake_case(&obj.name),
        snake_case(name),
        idx,
        item_prop.type_name().to_lowercase()
    )?;

    writeln!(write_buf, "    return b;")?;

    Ok(())
}

pub(super) fn non_complex_optional(
    write_buf: &mut Vec<u8>,
    name: &str,
    idx: &str,
    obj: &Object,
) -> Result<()> {
    writeln!(write_buf, "    QVariant v;")?;
    writeln!(
        write_buf,
        "    v = {}_data_{}(m_d{});",
        snake_case(&obj.name),
        snake_case(name),
        idx
    )?;

    writeln!(write_buf, "    return v;")?;
    Ok(())
}

pub(super) fn non_complex_non_optional(
    write_buf: &mut Vec<u8>,
    name: &str,
    idx: &str,
    obj: &Object,
) -> Result<()> {
    writeln!(
        write_buf,
        "    return {}_data_{}(m_d{});",
        snake_case(&obj.name),
        snake_case(name),
        idx
    )?;

    Ok(())
}
