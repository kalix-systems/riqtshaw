use super::*;
mod item_model;
mod object;

/// Entry point for producing the
/// generated C++ header code
pub fn write_header(conf: &Config) -> Result<()> {
    let mut h_file = conf.out_dir.join(&conf.cpp_file);

    h_file.set_extension("h");

    let mut header_buf = Vec::new();

    let guard = h_file
        .file_name()
        .unwrap()
        .to_string_lossy()
        .replace(".", "_")
        .to_uppercase();

    guard_scope(&mut header_buf, &guard, conf, |header_buf, conf| {
        for name in conf.objects.keys() {
            write_type_def(header_buf, name)?;
        }

        let mut extern_block = Block::new();
        extern_block.before("extern \"C\"");

        let mut def_blocks = Vec::new();

        for object in conf.objects.values() {
            extern_block.line(format!(
                "using {name}PtrBundle = struct {name}PtrBundle;",
                name = object.name
            ));

            // typedef struct
            let mut typedef_block = Block::new();

            typedef_block.before(format!("struct {}PtrBundle", object.name));

            write_extern_typedefs(&mut typedef_block, object);

            typedef_block.after(";");

            def_blocks.push(typedef_block);
        }

        for def_block in def_blocks {
            extern_block.push_block(def_block);
        }

        writeln!(header_buf, "{}", extern_block)?;

        for object in conf.objects.values() {
            write_header_object(header_buf, object, conf)?;
        }

        Ok(())
    })?;

    write_if_different(h_file, &header_buf)?;

    Ok(())
}

fn write_extern_typedefs(block: &mut Block, obj: &Object) {
    let lcname = snake_case(&obj.name);

    // first item in the bundle struct is a pointer to the
    // type that the bundle is used to construct.
    block.line(format!(
        "{class_name}* {snake_class_name};",
        class_name = obj.name,
        snake_class_name = lcname,
    ));

    for (prop_name, prop) in obj.properties.iter() {
        if let Type::Object(object) = &prop.property_type {
            write_extern_typedefs(block, object);
        } else {
            block.line(format!(
                "void (*{snake_class_name}_{p_name}_changed)({class_name}*);",
                snake_class_name = lcname,
                p_name = snake_case(prop_name),
                class_name = obj.name,
            ));
        }
    }

    match obj.object_type {
        ObjectType::List => {
            block.line(format!(
                "
             void (*{snake_class_name}_new_data_ready)(const {class_name}*);
             void (*{snake_class_name}_layout_about_to_be_changed)({class_name}*);
             void (*{snake_class_name}_layout_changed)({class_name}*);
             void (*{snake_class_name}_data_changed)({class_name}*, quintptr, quintptr);
             void (*{snake_class_name}_begin_reset_model)({class_name}*);
             void (*{snake_class_name}_end_reset_model)({class_name}*);
             void (*{snake_class_name}_begin_insert_rows)({class_name}*, int, int);
             void (*{snake_class_name}_end_insert_rows)({class_name}*);
             void (*{snake_class_name}_begin_move_rows)({class_name}*, int, int, int);
             void (*{snake_class_name}_end_move_rows)({class_name}*);
             void (*{snake_class_name}_begin_remove_rows)({class_name}*, int, int);
             void (*{snake_class_name}_end_remove_rows)({class_name}*);",
                class_name = obj.name,
                snake_class_name = lcname,
            ));
        }
        ObjectType::Object => {}
    };

    for (signal_name, signal) in obj.signals.iter() {
        block.line(format!(
            "void (*{snake_class_name}_{signal_name})(const {class_name}*",
            snake_class_name = lcname,
            signal_name = signal_name,
            class_name = obj.name
        ));

        for arg in signal.arguments.iter() {
            block.line(",");
            block.line(arg.argument_type.c_set_type());
        }

        block.line(");");
    }
}

fn write_header_object(header_buf: &mut Vec<u8>, obj: &Object, conf: &Config) -> Result<()> {
    object::qobject_block(header_buf, obj, conf, |header_buf, obj, conf| {
        object::register_friend_classes(header_buf, conf, obj)?;

        writeln!(header_buf, "public: class Private;")?;

        object::private_properties(header_buf, obj)?;

        object::public_properties(header_buf, obj)?;

        object::functions(header_buf, obj)?;

        match obj.object_type {
            ObjectType::Object => {}
            ObjectType::List => {
                item_model::write_header_item_model(header_buf, obj)?;
            }
        }

        object::qsignals(header_buf, obj)?;

        Ok(())
    })?;

    Ok(())
}

fn write_type_def(header_buf: &mut Vec<u8>, name: &str) -> Result<()> {
    writeln!(header_buf, "class {name};", name = name)?;

    Ok(())
}

fn guard_scope<F: Fn(&mut Vec<u8>, &Config) -> Result<()>>(
    header_buf: &mut Vec<u8>,
    guard: &str,
    conf: &Config,
    content: F,
) -> Result<()> {
    writeln!(
        header_buf,
        "
/* generated by riqtshaw */
#ifndef {guard}
#define {guard}

#include <QtCore/QObject>
#include <QtCore/QAbstractItemModel>
",
        guard = guard
    )?;

    content(header_buf, conf)?;

    writeln!(header_buf, "#endif // {guard}", guard = guard)?;

    Ok(())
}
