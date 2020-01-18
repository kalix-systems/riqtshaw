use super::*;
mod functions;
mod properties;

pub(super) fn write_cpp_object(w: &mut Vec<u8>, obj: &Object, conf: &Config) -> Result<()> {
    let lcname = snake_case(&obj.name);

    write!(
        w,
        "
{name}::{name}(bool /*owned*/, QObject *parent):
    {typ}(parent),",
        name = obj.name,
        typ = base_type(obj)
    )?;

    initialize_members_zero(w, obj)?;

    write!(w, "m_d(nullptr), m_ownsPrivate(false)")?;

    // start block
    write!(w, "{{")?;

    if obj.object_type != ObjectType::Object {
        writeln!(w, "initHeaderData();")?;
    }

    // end block
    writeln!(w, "}}")?;

    writeln!(
        w,
        "
{name}::{name}(QObject *parent):
    {typ}(parent),",
        name = obj.name,
        typ = base_type(obj)
    )?;

    initialize_members_zero(w, obj)?;

    write!(
        w,
        "m_d({name}_new( new {class_name}PtrBundle {{ this",
        name = lcname,
        class_name = obj.name
    )?;

    constructor_args(w, "", obj, conf)?;
    writeln!(
        w,
        "}})),
    m_ownsPrivate(true)"
    )?;

    // start block
    writeln!(w, "{{")?;

    initialize_members(w, "", obj, conf)?;

    connect(w, "this", obj)?;
    connect_fetch_more(w, "this", obj)?;

    if obj.object_type != ObjectType::Object {
        writeln!(w, "initHeaderData();")?;
    }

    // end block
    writeln!(w, "}}")?;

    writeln!(
        w,
        "
{}::~{0}() {{
    if (m_ownsPrivate) {{
        {1}_free(m_d);
    }}
}}",
        obj.name, lcname
    )?;

    write_abstract_item_header_data_function(obj, w)?;

    properties::properties(w, obj)?;
    functions::functions(w, obj)?;

    Ok(())
}

fn initialize_members_zero(w: &mut Vec<u8>, o: &Object) -> Result<()> {
    for (name, p) in &o.properties {
        if p.is_object() {
            writeln!(w, "    m_{}(new {}(false, this)),", name, p.type_name())?;
        }
    }

    Ok(())
}

fn initialize_members(w: &mut Vec<u8>, prefix: &str, o: &Object, conf: &Config) -> Result<()> {
    for (name, p) in o.properties.iter() {
        if let Type::Object(object) = &p.property_type {
            writeln!(
                w,
                "    {}m_{}->m_d = {}_{}_get({0}m_d);",
                prefix,
                name,
                snake_case(&o.name),
                snake_case(name)
            )?;
            initialize_members(
                w,
                &format!("{prefix}m_{name}->", prefix = prefix, name = name),
                object,
                conf,
            )?;
        }
    }

    Ok(())
}

fn connect(w: &mut Vec<u8>, d: &str, o: &Object) -> Result<()> {
    for (name, p) in o.properties.iter() {
        if let Type::Object(object) = &p.property_type {
            let new_d = &format!("{}->m_{}", d, name);
            connect(w, new_d, object)?;
        }
    }

    for Connection {
        signal: signal_name,
        function: function_name,
    } in o.connections.iter()
    {
        if !o.signals.contains_key(signal_name) {
            panic!("Signal {} does not exist", signal_name);
        }

        let function = o
            .functions
            .get(function_name)
            .unwrap_or_else(|| panic!("Function {} does not exist", function_name));

        assert!(
            function.arguments.is_empty(),
            "Functions in connections cannot take arguments"
        );

        writeln!(
            w,
            "
            connect({d}, &{class_name}::{signal}, {d}, [this]() {{
                    {d}->{func}();
            }}, Qt::QueuedConnection);
",
            d = d,
            class_name = o.name,
            func = function_name,
            signal = signal_name,
        )?;
    }

    Ok(())
}

fn connect_fetch_more(w: &mut Vec<u8>, d: &str, o: &Object) -> Result<()> {
    for (name, p) in o.properties.iter() {
        if let Type::Object(object) = &p.property_type {
            let new_d = &format!("{}->m_{}", d, name);
            connect_fetch_more(w, new_d, object)?;
        }
    }

    if o.object_type != ObjectType::Object {
        writeln!(
            w,
            "connect({}, &{1}::newDataReady, {0}, [this](const QModelIndex& i) {{
        {0}->fetchMore(i);
    }}, Qt::QueuedConnection);",
            d, o.name
        )?;
    }

    Ok(())
}

fn write_abstract_item_header_data_function(o: &Object, w: &mut Vec<u8>) -> Result<()> {
    if o.object_type == ObjectType::Object {
        return Ok(());
    };

    writeln!(w, "void {}::initHeaderData() {{", o.name)?;
    for col in 0..o.column_count() {
        for (name, ip) in &o.item_properties {
            let empty = Vec::new();

            let roles = ip.roles.get(col).unwrap_or(&empty);

            if roles.contains(&"display".to_string()) {
                writeln!(
                    w,
                    "m_headerData.insert(qMakePair({}, Qt::DisplayRole), QVariant(\"{}\"));",
                    col, name
                )?;
            }
        }
    }
    writeln!(w, "}}")?;

    Ok(())
}

fn constructor_args(
    write_buf: &mut Vec<u8>,
    prefix: &str,
    obj: &Object,
    conf: &Config,
) -> Result<()> {
    for (name, prop) in obj.properties.iter() {
        if let Type::Object(object) = &prop.property_type {
            write!(
                write_buf,
                ", {prefix}m_{name}",
                prefix = prefix,
                name = name
            )?;
            constructor_args(
                write_buf,
                &format!("{prefix}m_{name}->", prefix = prefix, name = name),
                object,
                conf,
            )?;
        } else {
            write!(write_buf, ",\n{}", changed_f(obj, name))?;
        }
    }

    if let ObjectType::List = obj.object_type {
        writeln!(
            write_buf,
            include_str!("../../cpp/list_constructor_lambdas.cpp_string"),
            name = obj.name,
            col_count = obj.column_count() - 1
        )?;
    }

    for (signal_name, signal) in obj.signals.iter() {
        write!(write_buf, ",")?;

        write!(write_buf, "[](const {name}* o", name = obj.name)?;

        for arg in signal.arguments.iter() {
            write!(
                write_buf,
                ", {typ} {arg_name}",
                typ = arg.argument_type.cpp_set_type(),
                arg_name = arg.name
            )?;
        }

        write!(write_buf, ")")?;

        let mut body = Block::new();
        body.line(format!(
            "Q_EMIT o->{signal_name}(",
            signal_name = signal_name
        ));

        for (i, arg) in signal.arguments.iter().enumerate() {
            if i != 0 {
                body.line(", ");
            }

            body.line(&arg.name);
        }

        body.line(");");

        write!(write_buf, "{}", body)?;
    }

    Ok(())
}
