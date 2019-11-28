use super::*;

pub(super) fn write_header_item_model(header_buf: &mut Vec<u8>, obj: &Object) -> Result<()> {
    writeln!(header_buf, include_str!("../cpp/header_item_model.hpp"))?;

    if model_is_writable(obj) {
        writeln!(
            header_buf,
            "bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;"
        )?;
    }

    match obj.object_type {
        ObjectType::List => list(header_buf, obj)?,
        _ => unreachable!(),
    }

    writeln!(
        header_buf,
        "
Q_SIGNALS:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
private:
    QHash<QPair<int,Qt::ItemDataRole>, QVariant> m_headerData;
    void initHeaderData();
    void updatePersistentIndexes();"
    )?;

    Ok(())
}

fn list(header_buf: &mut Vec<u8>, obj: &Object) -> Result<()> {
    for (name, item_prop) in obj.item_properties.iter() {
        let read_type = property_type(item_prop);

        let read_write_type =
            if read_type == "QVariant" || item_prop.item_property_type.is_complex() {
                format!("const {}&", read_type)
            } else {
                read_type.clone()
            };

        writeln!(
            header_buf,
            "Q_INVOKABLE {read_type} {name}(int row) const;",
            read_type = read_type,
            name = name
        )?;

        if item_prop.write {
            writeln!(
                header_buf,
                "Q_INVOKABLE bool set{name}(int row, {read_write_type} value);",
                name = upper_initial(name),
                read_write_type = read_write_type
            )?;
        }
    }

    Ok(())
}
