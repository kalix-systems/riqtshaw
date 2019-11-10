use super::*;

pub(super) fn r_constructor_args_decl(
    r: &mut Vec<u8>,
    name: &str,
    o: &Object,
    conf: &Config,
) -> Result<()> {
    write!(r, "    {}: *mut {}QObject", snake_case(name), o.name)?;
    for (p_name, p) in &o.properties {
        if let Type::Object(object) = &p.property_type {
            writeln!(r, ",")?;
            r_constructor_args_decl(r, p_name, object, conf)?;
        } else {
            write!(
                r,
                ",\n    {}_{}_changed: fn(*mut {}QObject)",
                snake_case(name),
                snake_case(p_name),
                o.name
            )?;
        }
    }
    if o.object_type == ObjectType::List {
        write!(
            r,
            ",\n    {}_new_data_ready: fn(*mut {}QObject)",
            snake_case(name),
            o.name
        )?;
    } else if o.object_type == ObjectType::Tree {
        write!(
            r,
            ",\n    {}_new_data_ready: fn(*mut {}QObject, index: COption<usize>)",
            snake_case(name),
            o.name
        )?;
    }
    if o.object_type != ObjectType::Object {
        let index_decl = if o.object_type == ObjectType::Tree {
            " index: COption<usize>,"
        } else {
            ""
        };
        let dest_decl = if o.object_type == ObjectType::Tree {
            " index: COption<usize>,"
        } else {
            ""
        };
        write!(
            r,
            ",
    {2}_layout_about_to_be_changed: fn(*mut {0}QObject),
    {2}_layout_changed: fn(*mut {0}QObject),
    {2}_data_changed: fn(*mut {0}QObject, usize, usize),
    {2}_begin_reset_model: fn(*mut {0}QObject),
    {2}_end_reset_model: fn(*mut {0}QObject),
    {2}_begin_insert_rows: fn(*mut {0}QObject,{1} usize, usize),
    {2}_end_insert_rows: fn(*mut {0}QObject),
    {2}_begin_move_rows: fn(*mut {0}QObject,{1} usize, usize,{3} usize),
    {2}_end_move_rows: fn(*mut {0}QObject),
    {2}_begin_remove_rows: fn(*mut {0}QObject,{1} usize, usize),
    {2}_end_remove_rows: fn(*mut {0}QObject)",
            o.name,
            index_decl,
            snake_case(name),
            dest_decl
        )?;
    }
    Ok(())
}

pub(super) fn r_constructor_args(
    r: &mut Vec<u8>,
    name: &str,
    o: &Object,
    conf: &Config,
) -> Result<()> {
    for (name, p) in &o.properties {
        if let Type::Object(object) = &p.property_type {
            r_constructor_args(r, name, object, conf)?;
        }
    }
    writeln!(
        r,
        "    let {}_emit = {}Emitter {{
        qobject: Arc::new(AtomicPtr::new({0})),",
        snake_case(name),
        o.name
    )?;
    for (p_name, p) in &o.properties {
        if p.is_object() {
            continue;
        }
        writeln!(
            r,
            "        {}_changed: {}_{0}_changed,",
            snake_case(p_name),
            snake_case(name)
        )?;
    }
    if o.object_type != ObjectType::Object {
        writeln!(
            r,
            "        new_data_ready: {}_new_data_ready,",
            snake_case(name)
        )?;
    }
    let mut model = String::new();
    if o.object_type != ObjectType::Object {
        let type_ = if o.object_type == ObjectType::List {
            "List"
        } else {
            "Tree"
        };
        model.push_str(", model");
        writeln!(
            r,
            "    }};
    let model = {}{} {{
        qobject: {},
        layout_about_to_be_changed: {2}_layout_about_to_be_changed,
        layout_changed: {2}_layout_changed,
        data_changed: {2}_data_changed,
        begin_reset_model: {2}_begin_reset_model,
        end_reset_model: {2}_end_reset_model,
        begin_insert_rows: {2}_begin_insert_rows,
        end_insert_rows: {2}_end_insert_rows,
        begin_move_rows: {2}_begin_move_rows,
        end_move_rows: {2}_end_move_rows,
        begin_remove_rows: {2}_begin_remove_rows,
        end_remove_rows: {2}_end_remove_rows,",
            o.name,
            type_,
            snake_case(name)
        )?;
    }
    write!(
        r,
        "    }};\n    let d_{} = {}::new({0}_emit{}",
        snake_case(name),
        o.name,
        model
    )?;
    for (name, p) in &o.properties {
        if p.is_object() {
            write!(r, ",\n        d_{}", snake_case(name))?;
        }
    }
    writeln!(r, ");")
}

pub(super) fn write_function(
    r: &mut Vec<u8>,
    (name, f): (&String, &Function),
    lcname: &str,
    o: &Object,
) -> Result<()> {
    let lc = snake_case(name);
    write!(
        r,
        "
#[no_mangle]
pub unsafe extern \"C\" fn {}_{}(ptr: *{} {}",
        lcname,
        lc,
        if f.mutable { "mut" } else { "const" },
        o.name
    )?;

    // write all the input arguments, for QString and QByteArray, write
    // pointers to their content and the length which is int in Qt
    for a in &f.arguments {
        write!(r, ", ")?;
        if a.argument_type.name() == "QString" {
            write!(r, "{}_str: *const c_ushort, {0}_len: c_int", a.name)?;
        } else if a.argument_type.name() == "QByteArray" {
            write!(r, "{}_str: *const c_char, {0}_len: c_int", a.name)?;
        } else {
            write!(r, "{}: {}", a.name, a.argument_type.rust_type())?;
        }
    }

    // If the return type is QString or QByteArray, append a pointer to the
    // variable that will be set to the argument list. Also add a setter
    // function.
    if f.return_type.is_complex() {
        writeln!(
            r,
            ", d: *mut {}, set: fn(*mut {0}, str: *const c_char, len: c_int)) {{",
            f.return_type.name()
        )?;
    } else if f.return_type == SimpleType::Void {
        writeln!(r, ") {{")?;
    } else {
        writeln!(r, ") -> {} {{", f.return_type.rust_type())?;
    }

    for a in &f.arguments {
        if a.argument_type.name() == "QString" {
            writeln!(
                r,
                "    let mut {} = String::new();
    set_string_from_utf16(&mut {0}, {0}_str, {0}_len);",
                a.name
            )?;
        } else if a.argument_type.name() == "QByteArray" {
            writeln!(r, "let {} = {{ qba_slice!({0}_str, {0}_len) }};", a.name)?;
        }
    }

    if f.mutable {
        writeln!(r, "    let o = &mut *ptr;")?;
    } else {
        writeln!(r, "    let o = &*ptr;")?;
    }
    if f.return_type.is_complex() {
        write!(r, "    let r = o.{}(", lc)?;
    } else {
        write!(r, "    o.{}(", lc)?;
    }
    for (i, a) in f.arguments.iter().enumerate() {
        if i > 0 {
            write!(r, ", ")?;
        }
        write!(r, "{}", a.name)?;
    }
    write!(r, ")")?;
    if f.return_type.is_complex() {
        writeln!(r, ";")?;
        writeln!(
            r,
            "    let s: *const c_char = r.as_ptr() as (*const c_char);
    set(d, s, r.len() as i32);"
        )?;
    } else {
        writeln!(r)?;
    }
    writeln!(r, "}}")
}

pub(super) fn write_rust_types(conf: &Config, r: &mut Vec<u8>) -> Result<()> {
    let mut has_option = false;
    let mut has_string = false;
    let mut has_byte_array = false;
    let mut has_list_or_tree = false;

    for o in conf.objects.values() {
        has_list_or_tree |= o.object_type != ObjectType::Object;
        for p in o.properties.values() {
            has_option |= p.optional;
            has_string |= p.property_type == Type::Simple(SimpleType::QString);
            has_byte_array |= p.property_type == Type::Simple(SimpleType::QByteArray);
        }
        for p in o.item_properties.values() {
            has_option |= p.optional;
            has_string |= p.item_property_type == SimpleType::QString;
            has_byte_array |= p.item_property_type == SimpleType::QByteArray;
        }
        for f in o.functions.values() {
            has_string |= f.return_type == SimpleType::QString;
            has_byte_array |= f.return_type == SimpleType::QByteArray;
            for a in &f.arguments {
                has_string |= a.argument_type == SimpleType::QString;
                has_byte_array |= a.argument_type == SimpleType::QByteArray;
            }
        }
    }

    if has_byte_array {
        writeln!(
            r,
            "

macro_rules! qba_slice {{
    ($qba: expr, $qba_len: expr) => {{
        match (to_usize($qba_len), $qba.is_null()) {{
            (Some(len), false) => ::std::slice::from_raw_parts($qba as *const u8, len),
            _ => &[],
        }}
    }}
}}

       "
        )?;
    }

    if has_option || has_list_or_tree {
        writeln!(
            r,
            "

#[repr(C)]
pub struct COption<T> {{
    data: T,
    some: bool,
}}

impl<T> COption<T> {{
    #![allow(dead_code)]
    fn into(self) -> Option<T> {{
        if self.some {{
            Some(self.data)
        }} else {{
            None
        }}
    }}
}}

impl<T> From<Option<T>> for COption<T>
where
    T: Default,
{{
    fn from(t: Option<T>) -> COption<T> {{
        if let Some(v) = t {{
            COption {{
                data: v,
                some: true,
            }}
        }} else {{
            COption {{
                data: T::default(),
                some: false,
            }}
        }}
    }}
}}"
        )?;
    }
    if has_string {
        writeln!(
            r,
            "

pub enum QString {{}}

fn set_string_from_utf16(s: &mut String, str: *const c_ushort, len: c_int) {{
    let utf16 = unsafe {{
        match to_usize(len) {{
            Some(len) => ::std::slice::from_raw_parts(str, len),
            None => &[],
        }}
    }};
    let characters = decode_utf16(utf16.iter().cloned())
        .map(|r| r.unwrap());
    s.clear();
    s.extend(characters);
}}
"
        )?;
    }
    if has_byte_array {
        writeln!(
            r,
            "

pub enum QByteArray {{}}"
        )?;
    }
    if has_list_or_tree {
        writeln!(
            r,
            "

#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub enum SortOrder {{
    Ascending = 0,
    Descending = 1,
}}

#[repr(C)]
pub struct QModelIndex {{
    row: c_int,
    internal_id: usize,
}}"
        )?;
    }

    if has_string || has_byte_array || has_list_or_tree {
        writeln!(
            r,
            "

fn to_usize(n: c_int) -> Option<usize> {{
    use std::convert::TryInto;

    n.try_into().ok()
}}

fn to_c_int(n: usize) -> c_int {{
    // saturate
    n.min(c_int::max_value() as usize) as c_int
}}
"
        )?;
    }

    Ok(())
}
