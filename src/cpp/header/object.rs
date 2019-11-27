use super::*;

pub(super) fn register_friend_classes(
    header_buf: &mut Vec<u8>,
    conf: &Config,
    obj: &Object,
) -> Result<()> {
    for object in conf.objects.values().filter(|o| o.name != obj.name) {
        writeln!(header_buf, "friend class {};", object.name)?;
    }

    Ok(())
}

pub(super) fn private_properties(header_buf: &mut Vec<u8>, obj: &Object) -> Result<()> {
    writeln!(header_buf, "private:")?;

    for (name, prop) in obj.object_properties() {
        writeln!(
            header_buf,
            "{type_name}* const m_{prop_name};",
            type_name = prop.type_name(),
            prop_name = name
        )?;
    }

    writeln!(
        header_buf,
        "Private * m_d;
        bool m_ownsPrivate;"
    )?;

    for (name, prop) in obj.properties.iter() {
        writeln!(
            header_buf,
            "Q_PROPERTY({ret} {name} READ {name} {write_prop}NOTIFY {name}Changed FINAL)",
            ret = get_return_type(&prop),
            name = name,
            write_prop = write_property(name, prop)
        )?;
    }

    writeln!(
        header_buf,
        "explicit {constructor_name}(bool owned, QObject *parent);",
        constructor_name = obj.name
    )?;

    Ok(())
}

pub(super) fn public_properties(header_buf: &mut Vec<u8>, obj: &Object) -> Result<()> {
    writeln!(
        header_buf,
        "
public:
    explicit {constructor_name}(QObject *parent = nullptr);
    ~{constructor_name}() override;",
        constructor_name = obj.name
    )?;

    for (name, p) in obj.properties.iter() {
        if p.is_object() {
            writeln!(
                header_buf,
                "const {type_name}* {name}() const;
                 {type_name}* {name}();",
                type_name = p.type_name(),
                name = name
            )?;
        } else {
            let (typ, typ2) = if p.optional && !p.is_complex() {
                ("QVariant", "const QVariant&")
            } else {
                (p.type_name(), p.property_type.cpp_set_type())
            };

            writeln!(header_buf, "{typ} {name}() const;", typ = typ, name = name)?;

            if p.write {
                writeln!(header_buf, "void set{}({} v);", upper_initial(name), typ2)?;
            }
        }
    }

    Ok(())
}

pub(super) fn functions(header_buf: &mut Vec<u8>, obj: &Object) -> Result<()> {
    for (name, func) in obj.functions.iter() {
        write!(
            header_buf,
            "Q_INVOKABLE {ret} {name}(",
            ret = func.return_type.name(),
            name = name
        )?;

        for (i, arg) in func.arguments.iter().enumerate() {
            if i != 0 {
                write!(header_buf, ", ")?;
            }

            write!(
                header_buf,
                "{typ} {arg_name}",
                typ = arg.argument_type.cpp_set_type(),
                arg_name = arg.name
            )?;
        }

        if func.mutable {
            writeln!(header_buf, ");")?;
        } else {
            writeln!(header_buf, ") const;",)?;
        }
    }

    Ok(())
}

pub(super) fn qsignals(header_buf: &mut Vec<u8>, obj: &Object) -> Result<()> {
    writeln!(header_buf, "Q_SIGNALS:")?;

    for name in obj.properties.keys() {
        writeln!(header_buf, "void {}Changed();", name)?;
    }

    Ok(())
}

pub(super) fn qobject_block<F: Fn(&mut Vec<u8>, &Object, &Config) -> Result<()>>(
    header_buf: &mut Vec<u8>,
    obj: &Object,
    conf: &Config,
    content: F,
) -> Result<()> {
    writeln!(
        header_buf,
        "class {} : public {} {{ Q_OBJECT",
        obj.name,
        base_type(obj)
    )?;

    content(header_buf, obj, conf)?;

    writeln!(header_buf, "}};")?;

    Ok(())
}
