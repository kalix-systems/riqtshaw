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

        block(
            header_buf,
            "extern \"C\"",
            "",
            |header_buf, conf| {
                for object in conf.objects.values() {
                    // typedef struct
                    block(
                        header_buf,
                        &format!("typedef struct {}PtrBundle", object.name),
                        &format!("{}PtrBundle;", object.name),
                        |header_buf, _| {
                            write_extern_typedefs(header_buf, object, conf)?;
                            Ok(())
                        },
                        (),
                    )?;
                }
                Ok(())
            },
            conf,
        )?;

        for object in conf.objects.values() {
            write_header_object(header_buf, object, conf)?;
        }

        Ok(())
    })?;

    write_if_different(h_file, &header_buf)?;

    Ok(())
}

fn write_extern_typedefs(w: &mut Vec<u8>, o: &Object, conf: &Config) -> Result<()> {
    let lcname = snake_case(&o.name);

    for (prop_name, prop) in o.properties.iter() {
        if let Type::Object(object) = &prop.property_type {
            writeln!(w, "void (*)({class_name}*);", class_name = o.name,)?;
            write_extern_typedefs(w, object, conf)?;
        } else {
            writeln!(
                w,
                "void (*{snake_class_name}_{p_name}_changed)({class_name}*);",
                snake_class_name = lcname,
                p_name = snake_case(prop_name),
                class_name = o.name,
            )?;
        }
    }

    match o.object_type {
        ObjectType::List => {
            write!(
                w,
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
                class_name = o.name,
                snake_class_name = lcname,
            )?;
        }
        ObjectType::Object => {}
        ObjectType::Tree => unimplemented!(),
    }

    writeln!(w, "")?;

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
