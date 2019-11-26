use super::*;

pub(super) fn block<T, F: Fn(&mut Vec<u8>, T) -> Result<()>>(
    buf: &mut Vec<u8>,
    before: &str,
    after: &str,
    content: F,
    args: T,
) -> Result<()> {
    writeln!(buf, "{} {{", before)?;

    content(buf, args)?;

    writeln!(buf, "}}")?;
    write!(buf, "{}", after)?;

    Ok(())
}

pub fn define_ffi_getters(o: &Object, w: &mut Vec<u8>) -> Result<()> {
    // define ffi getters for from-rust FFI types
    let index_decl = if o.object_type == ObjectType::Tree {
        "quintptr"
    } else {
        "int"
    };

    let lcname = snake_case(&o.name);

    for (obj_name, item_prop) in &o.item_properties {
        if item_prop.is_complex() {
            writeln!(
            w,
            "void {snake_class_name}_data_{prop_name}(const {camel_class_name}::Private*, {index_type}, {prop_type});",
            snake_class_name = lcname,
            prop_name = snake_case(obj_name),
            camel_class_name = o.name,
            index_type = index_decl,
            prop_type = item_prop.c_get_type()
        )?;
        } else {
            writeln!(
            w,
            "{return_type} {snake_class_name}_data_{snake_data_name}(const {camel_class_name}::Private*, {prop_type});",
            return_type = item_prop.cpp_set_type(),
            snake_class_name = lcname,
            snake_data_name = snake_case(obj_name),
            camel_class_name = o.name,
            prop_type =  index_decl
        )?;
        }

        if item_prop.write {
            let a = format!("bool {}_set_data_{}", lcname, snake_case(obj_name));
            let b = format!("({}::Private*, {}", o.name, index_decl);
            if item_prop.type_name() == "QString" {
                writeln!(w, "{}{}, const ushort* s, int len);", a, b)?;
            } else if item_prop.type_name() == "QByteArray" {
                writeln!(w, "{}{}, const char* s, int len);", a, b)?;
            } else {
                writeln!(w, "{}{}, {});", a, b, item_prop.c_set_type())?;
            }
            if item_prop.optional {
                writeln!(w, "{}_none{});", a, b)?;
            }
        }
    }

    Ok(())
}

pub(super) fn write_qaim_data_function(o: &Object, w: &mut Vec<u8>) -> Result<()> {
    writeln!(
        w,
        "int {}::role(const char* name) const {{
    auto names = roleNames();
    auto i = names.constBegin();
    while (i != names.constEnd()) {{
        if (i.value() == name) {{
            return i.key();
        }}
        ++i;
    }}
    return -1;
}}
QHash<int, QByteArray> {0}::roleNames() const {{
    QHash<int, QByteArray> names = QAbstractItemModel::roleNames();",
        o.name
    )?;
    for (i, (name, _)) in o.item_properties.iter().enumerate() {
        writeln!(w, "    names.insert(Qt::UserRole + {}, \"{}\");", i, name)?;
    }
    writeln!(w, "return names;}}")?;

    Ok(())
}

pub(super) fn property_type(prop: &ItemProperty) -> String {
    if prop.optional && !prop.item_property_type.is_complex() {
        return "QVariant".into();
    }
    match &prop.item_property_type {
        Type::Simple(_) => prop.type_name().to_string(),
        Type::Object(obj) => obj.name.clone() + "Ref".into(),
    }
}

pub(super) fn upper_initial(name: &str) -> String {
    format!("{}{}", &name[..1].to_uppercase(), &name[1..])
}

pub(super) fn lower_initial(name: &str) -> String {
    format!("{}{}", &name[..1].to_lowercase(), &name[1..])
}

pub(super) fn write_property(name: &str, prop: &Property) -> String {
    if prop.write {
        format!("WRITE set{} ", upper_initial(name))
    } else {
        "".into()
    }
}

pub(super) fn base_type(o: &Object) -> &str {
    if o.object_type != ObjectType::Object {
        return "QAbstractItemModel";
    }

    "QObject"
}

pub(super) fn get_return_type(prop: &Property) -> String {
    let mut t = if prop.optional && !prop.is_complex() {
        "QVariant"
    } else {
        prop.type_name()
    }
    .to_string();

    if prop.is_object() {
        t.push_str("*");
    }

    return t;
}

pub(super) fn model_is_writable(o: &Object) -> bool {
    let mut write = false;

    for p in o.item_properties.values() {
        write |= p.write;
    }

    write
}

pub(super) fn initialize_members_zero(w: &mut Vec<u8>, o: &Object) -> Result<()> {
    for (name, p) in &o.properties {
        if p.is_object() {
            writeln!(w, "    m_{}(new {}(false, this)),", name, p.type_name())?;
        }
    }

    Ok(())
}

pub(super) fn initialize_members(
    w: &mut Vec<u8>,
    prefix: &str,
    o: &Object,
    conf: &Config,
) -> Result<()> {
    for (name, p) in &o.properties {
        if let Type::Object(object) = &p.property_type {
            writeln!(
                w,
                "    {}m_{}->m_d = {}_{}_get({0}m_d);",
                prefix,
                name,
                snake_case(&o.name),
                snake_case(name)
            )?;
            initialize_members(w, &format!("m_{}->", name), object, conf)?;
        }
    }

    Ok(())
}

pub(super) fn connect(w: &mut Vec<u8>, d: &str, o: &Object, conf: &Config) -> Result<()> {
    for (name, p) in &o.properties {
        if let Type::Object(object) = &p.property_type {
            connect(w, &format!("{}->m_{}", d, name), object, conf)?;
        }
    }

    if o.object_type != ObjectType::Object {
        writeln!(
            w,
            "    connect({}, &{1}::newDataReady, {0}, [this](const QModelIndex& i) {{
        {0}->fetchMore(i);
    }}, Qt::QueuedConnection);",
            d, o.name
        )?;
    }

    Ok(())
}

pub(super) fn role_name(role: &str) -> String {
    match role {
        "display" => "DisplayRole".into(),
        "decoration" => "DecorationRole".into(),
        "edit" => "EditRole".into(),
        "toolTip" => "ToolTipRole".into(),
        "statustip" => "StatusTipRole".into(),
        "whatsthis" => "WhatsThisRole".into(),
        _ => panic!("Unknown role {}", role),
    }
}

pub(super) fn is_column_write(o: &Object, col: usize) -> bool {
    o.item_properties
        .values()
        .any(|ip| ip.write && (col == 0 || (ip.roles.len() > col && !ip.roles[col].is_empty())))
}

fn write_function_c_decl(
    w: &mut Vec<u8>,
    (name, f): (&String, &Function),
    lcname: &str,
    o: &Object,
) -> Result<()> {
    let lc = snake_case(name);

    if f.return_type.is_complex() {
        write!(w, "void")?;
    } else {
        write!(w, "{}", f.type_name())?;
    }

    let name = format!("{}_{}", lcname, lc);

    write!(
        w,
        " {}({}{}::Private*",
        name,
        if f.mutable { "" } else { "const " },
        o.name
    )?;

    // write all the input arguments, for QString and QByteArray, write
    // pointers to their content and the length
    for a in &f.arguments {
        if a.type_name() == "QString" {
            write!(w, ",const ushort*, int")?;
        } else if a.type_name() == "QByteArray" {
            write!(w, ",const char*, int")?;
        } else {
            write!(w, ",{}", a.type_name())?;
        }
    }

    // If the return type is QString or QByteArray, append a pointer to the
    // variable that will be set to the argument list. Also add a setter
    // function.
    if f.return_type.name() == "QString" {
        write!(w, ", QString*, qstring_set")?;
    } else if f.return_type.name() == "QByteArray" {
        write!(w, ", QByteArray*, qbytearray_set")?;
    }

    writeln!(w, ");")?;

    Ok(())
}

pub(super) fn write_object_c_decl(w: &mut Vec<u8>, o: &Object, conf: &Config) -> Result<()> {
    let lcname = snake_case(&o.name);

    write!(w, "{}::Private* {}_new(", o.name, lcname)?;

    constructor_args_decl(w, o, conf)?;

    writeln!(w, ");")?;

    writeln!(w, "void {}_free({}::Private*);", lcname, o.name)?;

    for (prop_name, prop) in &o.properties {
        let base = format!("{}_{}", lcname, snake_case(prop_name));

        if prop.is_object() {
            writeln!(
                w,
                "{}::Private* {}_get(const {}::Private*);",
                prop.type_name(),
                base,
                o.name
            )?;
        } else if prop.is_complex() {
            writeln!(
                w,
                "void {}_get(const {}::Private*, {});",
                base,
                o.name,
                prop.c_get_type()
            )?;
        } else if prop.optional {
            writeln!(
                w,
                "option_{} {}_get(const {}::Private*);",
                prop.type_name(),
                base,
                o.name
            )?;
        } else {
            writeln!(
                w,
                "{} {}_get(const {}::Private*);",
                prop.type_name(),
                base,
                o.name
            )?;
        }

        if prop.write {
            let mut t = prop.property_type.c_set_type();

            if t == "qstring_t" {
                t = "const ushort *str, int len";
            } else if t == "qbytearray_t" {
                t = "const char* bytes, int len";
            }

            writeln!(w, "    void {}_set({}::Private*, {});", base, o.name, t)?;

            if prop.optional {
                writeln!(w, "    void {}_set_none({}::Private*);", base, o.name)?;
            }
        }
    }

    for f in &o.functions {
        write_function_c_decl(w, f, &lcname, o)?;
    }
    Ok(())
}

fn constructor_args_decl(w: &mut Vec<u8>, o: &Object, conf: &Config) -> Result<()> {
    write!(w, "{}*", o.name)?;

    for p in o.properties.values() {
        if let Type::Object(object) = &p.property_type {
            write!(w, ", ")?;
            constructor_args_decl(w, object, conf)?;
        } else {
            writeln!(w, ", void (*)({}*)", o.name)?;
        }
    }

    if o.object_type == ObjectType::List {
        write!(
            w,
            ",
        void (*)(const {}*),
        void (*)({0}*),
        void (*)({0}*),
        void (*)({0}*, quintptr, quintptr),
        void (*)({0}*),
        void (*)({0}*),
        void (*)({0}*, int, int),
        void (*)({0}*),
        void (*)({0}*, int, int, int),
        void (*)({0}*),
        void (*)({0}*, int, int),
        void (*)({0}*)",
            o.name
        )?;
    }

    if o.object_type == ObjectType::Tree {
        write!(
            w,
            ",
        void (*)(const {0}*, option_quintptr),
        void (*)({0}*),
        void (*)({0}*),
        void (*)({0}*, quintptr, quintptr),
        void (*)({0}*),
        void (*)({0}*),
        void (*)({0}*, option_quintptr, int, int),
        void (*)({0}*),
        void (*)({0}*, option_quintptr, int, int, option_quintptr, int),
        void (*)({0}*),
        void (*)({0}*, option_quintptr, int, int),
        void (*)({0}*)",
            o.name
        )?;
    }

    Ok(())
}

pub(super) fn changed_f(o: &Object, p_name: &str) -> String {
    lower_initial(&o.name) + &upper_initial(p_name) + "Changed"
}
