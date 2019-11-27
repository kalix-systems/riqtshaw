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

        // let mut forward_decl = Block::new();

        for object in conf.objects.values() {
            // typedef struct
            let mut typedef_block = Block::new();

            typedef_block.before(format!("typedef struct {}PtrBundle", object.name));
            write_extern_typedefs(&mut typedef_block, object)?;

            typedef_block.after(format!("{}PtrBundle;", object.name));

            extern_block.push_block(typedef_block);
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

fn write_extern_typedefs(block: &mut Block, obj: &Object) -> Result<()> {
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
            write_extern_typedefs(block, object)?;
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

            for (item_prop_name, item_prop) in obj.item_properties.iter() {
                if let Type::Object(item_obj) = &item_prop.item_property_type {
                    block.line(format!(
                        "{ptr_bundle}* (*{item_obj}_ptr_bundle_factory)({class_name}*);",
                        class_name = obj.name,
                        item_obj = snake_case(&item_prop_name),
                        ptr_bundle = format!("{}PtrBundle", &item_obj.name)
                    ));
                }
            }
        }
        ObjectType::Object => {}
        ObjectType::Tree => unimplemented!(),
    }

    Ok(())
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
            ObjectType::List | ObjectType::Tree => {
                item_model::write_header_item_model(header_buf, obj)?;
            }
        }

        object::qsignals(header_buf, obj)?;

        Ok(())
    })?;

    Ok(())
}

fn write_type_def(header_buf: &mut Vec<u8>, name: &str) -> Result<()> {
    writeln!(
        header_buf,
        "
class {name};
typedef {name}* {name}Ref;
Q_DECLARE_METATYPE({name}Ref);
         ",
        name = name
    )?;

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
