use super::*;
use codegen::*;

pub(super) fn write_rust_interface_object(r: &mut Vec<u8>, object: &Object) -> Result<()> {
    let mut scope = Scope::new();
    let lcname = snake_case(&object.name);

    scope.new_struct(&qobject(&object.name)).vis("pub");

    push_emitter(&mut scope, object);
    push_model(&mut scope, object);
    push_trait(&mut scope, object);
    c_ffi::push_new(&mut scope, object);

    writeln!(r, "{}", scope.to_string())?;

    writeln!(
        r,
        "
#[no_mangle]
pub unsafe extern \"C\" fn {lcname}_free(ptr: *mut {object_name}) {{
    Box::from_raw(ptr).emit().clear();
}}",
        lcname = lcname,
        object_name = object.name
    )?;

    for (name, p) in &object.properties {
        let base = format!("{}_{}", lcname, snake_case(name));
        if p.is_object() {
            writeln!(
                r,
                "
#[no_mangle]
pub unsafe extern \"C\" fn {}_get(ptr: *mut {}) -> *mut {} {{
    (&mut *ptr).{}_mut()
}}",
                base,
                object.name,
                rust_type(p),
                snake_case(name)
            )?;
        } else if p.is_complex() && !p.optional {
            if p.rust_by_function {
                writeln!(
                    r,
                    "
#[no_mangle]
pub unsafe extern \"C\" fn {}_get(
    ptr: *const {},
    p: *mut {},
    set: fn(*mut {2}, *const c_char, c_int),
) {{
    let o = &*ptr;
    o.{}(|v| {{
        let s: *const c_char = v.as_ptr() as (*const c_char);
        set(p, s, to_c_int(v.len()));
    }});
}}",
                    base,
                    object.name,
                    p.type_name(),
                    snake_case(name)
                )?;
            } else {
                writeln!(
                    r,
                    "
#[no_mangle]
pub unsafe extern \"C\" fn {}_get(
    ptr: *const {},
    p: *mut {},
    set: fn(*mut {2}, *const c_char, c_int),
) {{
    let o = &*ptr;
    let v = o.{}();
    let s: *const c_char = v.as_ptr() as (*const c_char);
    set(p, s, to_c_int(v.len()));
}}",
                    base,
                    object.name,
                    p.type_name(),
                    snake_case(name)
                )?;
            }
            if p.write && p.type_name() == "QString" {
                writeln!(
                    r,
                    "
#[no_mangle]
pub unsafe extern \"C\" fn {}_set(ptr: *mut {}, v: *const c_ushort, len: c_int) {{
    let o = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_{}(s);
}}",
                    base,
                    object.name,
                    snake_case(name)
                )?;
            } else if p.write {
                writeln!(
                    r,
                    "
#[no_mangle]
pub unsafe extern \"C\" fn {}_set(ptr: *mut {}, v: *const c_char, len: c_int) {{
    let o = &mut *ptr;
    let v = qba_slice!(v, len);

    o.set_{}(v);
}}",
                    base,
                    object.name,
                    snake_case(name)
                )?;
            }
        } else if p.is_complex() {
            writeln!(
                r,
                "
#[no_mangle]
pub unsafe extern \"C\" fn {}_get(
    ptr: *const {},
    p: *mut {},
    set: fn(*mut {2}, *const c_char, c_int),
) {{
    let o = &*ptr;
    let v = o.{}();
    if let Some(v) = v {{
        let s: *const c_char = v.as_ptr() as (*const c_char);
        set(p, s, to_c_int(v.len()));
    }}
}}",
                base,
                object.name,
                p.type_name(),
                snake_case(name)
            )?;
            if p.write && p.type_name() == "QString" {
                writeln!(
                    r,
                    "
#[no_mangle]
pub unsafe extern \"C\" fn {}_set(ptr: *mut {}, v: *const c_ushort, len: c_int) {{
    let o = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_{}(Some(s));
}}",
                    base,
                    object.name,
                    snake_case(name)
                )?;
            } else if p.write {
                writeln!(
                    r,
                    "
#[no_mangle]
pub unsafe extern \"C\" fn {}_set(ptr: *mut {}, v: *const c_char, len: c_int) {{
    let o = &mut *ptr;
    let v = qba_slice!(v, len);
    o.set_{}(Some(v.into()));
}}",
                    base,
                    object.name,
                    snake_case(name)
                )?;
            }
        } else if p.optional {
            writeln!(
                r,
                "
#[no_mangle]
pub unsafe extern \"C\" fn {}_get(ptr: *const {}) -> COption<{}> {{
    match (&*ptr).{}() {{
        Some(value) => COption {{ data: value, some: true }},
        None => COption {{ data: {2}::default(), some: false}}
    }}
}}",
                base,
                object.name,
                p.property_type.rust_type(),
                snake_case(name)
            )?;
            if p.write {
                writeln!(
                    r,
                    "
#[no_mangle]
pub unsafe extern \"C\" fn {}_set(ptr: *mut {}, v: {}) {{
    (&mut *ptr).set_{}(Some(v));
}}",
                    base,
                    object.name,
                    p.property_type.rust_type(),
                    snake_case(name)
                )?;
            }
        } else {
            writeln!(
                r,
                "
#[no_mangle]
pub unsafe extern \"C\" fn {}_get(ptr: *const {}) -> {} {{
    (&*ptr).{}()
}}",
                base,
                object.name,
                rust_type(p),
                snake_case(name)
            )?;
            if p.write {
                writeln!(
                    r,
                    "
#[no_mangle]
pub unsafe extern \"C\" fn {}_set(ptr: *mut {}, v: {}) {{
    (&mut *ptr).set_{}(v);
}}",
                    base,
                    object.name,
                    rust_type(p),
                    snake_case(name)
                )?;
            }
        }
        if p.write && p.optional {
            writeln!(
                r,
                "
#[no_mangle]
pub unsafe extern \"C\" fn {}_set_none(ptr: *mut {}) {{
    let o = &mut *ptr;
    o.set_{}(None);
}}",
                base,
                object.name,
                snake_case(name)
            )?;
        }
    }

    for f in &object.functions {
        write_function(r, f, &lcname, object)?;
    }

    if object.object_type == ObjectType::List {
        writeln!(
            r,
            "
#[no_mangle]
pub unsafe extern \"C\" fn {1}_row_count(ptr: *const {0}) -> c_int {{
    to_c_int((&*ptr).row_count())
}}

#[no_mangle]
pub unsafe extern \"C\" fn {1}_insert_rows(ptr: *mut {0}, row: c_int, count: c_int) -> bool {{
    match (to_usize(row), to_usize(count)) {{
        (Some(row), Some(count)) => {{
            (&mut *ptr).insert_rows(row, count)
        }}
        _ => false
    }}
}}

#[no_mangle]
pub unsafe extern \"C\" fn {1}_remove_rows(ptr: *mut {0}, row: c_int, count: c_int) -> bool {{
    match (to_usize(row), to_usize(count)) {{
        (Some(row), Some(count)) => {{
            (&mut *ptr).remove_rows(row, count)
        }}
        _ => false
    }}
}}

#[no_mangle]
pub unsafe extern \"C\" fn {1}_can_fetch_more(ptr: *const {0}) -> bool {{
    (&*ptr).can_fetch_more()
}}

#[no_mangle]
pub unsafe extern \"C\" fn {1}_fetch_more(ptr: *mut {0}) {{
    (&mut *ptr).fetch_more()
}}

#[no_mangle]
pub unsafe extern \"C\" fn {1}_sort(
    ptr: *mut {0},
    column: u8,
    order: SortOrder,
) {{
    (&mut *ptr).sort(column, order)
}}",
            object.name, lcname
        )?;
    } else if object.object_type == ObjectType::Tree {
        writeln!(
            r,
            "
#[no_mangle]
pub unsafe extern \"C\" fn {1}_row_count(
    ptr: *const {0},
    index: COption<usize>,
) -> c_int {{
    to_c_int((&*ptr).row_count(index.into()))
}}

#[no_mangle]
pub unsafe extern \"C\" fn {1}_can_fetch_more(
    ptr: *const {0},
    index: COption<usize>,
) -> bool {{
    (&*ptr).can_fetch_more(index.into())
}}

#[no_mangle]
pub unsafe extern \"C\" fn {1}_fetch_more(ptr: *mut {0}, index: COption<usize>) {{
    (&mut *ptr).fetch_more(index.into())
}}

#[no_mangle]
pub unsafe extern \"C\" fn {1}_sort(
    ptr: *mut {0},
    column: u8,
    order: SortOrder
) {{
    (&mut *ptr).sort(column, order)
}}

#[no_mangle]
pub unsafe extern \"C\" fn {1}_check_row(
    ptr: *const {0},
    index: usize,
    row: c_int,
) -> COption<usize> {{
    match to_usize(row) {{
        Some(row) => (&*ptr).check_row(index, row).into(),
        other => other.into()
    }}
}}

#[no_mangle]
pub unsafe extern \"C\" fn {1}_index(
    ptr: *const {0},
    index: COption<usize>,
    row: c_int,
) -> usize {{
    (&*ptr).index(index.into(), to_usize(row).unwrap_or(0))
}}

#[no_mangle]
pub unsafe extern \"C\" fn {1}_parent(ptr: *const {0}, index: usize) -> QModelIndex {{
    if let Some(parent) = (&*ptr).parent(index) {{
        QModelIndex {{
            row: to_c_int((&*ptr).row(parent)),
            internal_id: parent,
        }}
    }} else {{
        QModelIndex {{
            row: -1,
            internal_id: 0,
        }}
    }}
}}
#[no_mangle]
pub unsafe extern \"C\" fn {1}_row(ptr: *const {0}, index: usize) -> c_int {{
    to_c_int((&*ptr).row(index))
}}",
            object.name, lcname
        )?;
    }
    if object.object_type != ObjectType::Object {
        let (index_decl, index) = if object.object_type == ObjectType::Tree {
            (", index: usize", "index")
        } else {
            (", row: c_int", "to_usize(row).unwrap_or(0)")
        };
        for (name, ip) in &object.item_properties {
            if ip.is_complex() && !ip.optional {
                writeln!(
                    r,
                    "
#[no_mangle]
pub unsafe extern \"C\" fn {}_data_{}(
    ptr: *const {}{},
    d: *mut {},
    set: fn(*mut {4}, *const c_char, len: c_int),
) {{
    let o = &*ptr;
    let data = o.{1}({});
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}}",
                    lcname,
                    snake_case(name),
                    object.name,
                    index_decl,
                    ip.type_name(),
                    index
                )?;
            } else if ip.is_complex() {
                writeln!(
                    r,
                    "
#[no_mangle]
pub unsafe extern \"C\" fn {}_data_{}(
    ptr: *const {}{},
    d: *mut {},
    set: fn(*mut {4}, *const c_char, len: c_int),
) {{
    let o = &*ptr;
    let data = o.{1}({});
    if let Some(data) = data {{
        let s: *const c_char = data.as_ptr() as (*const c_char);
        set(d, s, to_c_int(data.len()));
    }}
}}",
                    lcname,
                    snake_case(name),
                    object.name,
                    index_decl,
                    ip.type_name(),
                    index
                )?;
            } else {
                writeln!(
                    r,
                    "
#[no_mangle]
pub unsafe extern \"C\" fn {}_data_{}(ptr: *const {}{}) -> {} {{
    let o = &*ptr;
    o.{1}({}){}
}}",
                    lcname,
                    snake_case(name),
                    object.name,
                    index_decl,
                    rust_c_type(ip),
                    index,
                    if ip.optional { ".into()" } else { "" }
                )?;
            }
            if ip.write {
                let val = if ip.optional { "Some(v)" } else { "v" };
                if ip.type_name() == "QString" {
                    writeln!(
                        r,
                        "
#[no_mangle]
pub unsafe extern \"C\" fn {}_set_data_{}(
    ptr: *mut {}{},
    s: *const c_ushort, len: c_int,
) -> bool {{
    let o = &mut *ptr;
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_{1}({}, {})
}}",
                        lcname,
                        snake_case(name),
                        object.name,
                        index_decl,
                        index,
                        val
                    )?;
                } else if ip.type_name() == "QByteArray" {
                    writeln!(
                        r,
                        "
#[no_mangle]
pub unsafe extern \"C\" fn {}_set_data_{}(
    ptr: *mut {}{},
    s: *const c_char, len: c_int,
) -> bool {{
    let o = &mut *ptr;
    let slice = qba_slice!(s, len);
    o.set_{1}({}, {})
}}",
                        lcname,
                        snake_case(name),
                        object.name,
                        index_decl,
                        index,
                        if ip.optional { "Some(slice)" } else { "slice" }
                    )?;
                } else {
                    let type_ = ip.item_property_type.rust_type();
                    writeln!(
                        r,
                        "
#[no_mangle]
pub unsafe extern \"C\" fn {}_set_data_{}(
    ptr: *mut {}{},
    v: {},
) -> bool {{
    (&mut *ptr).set_{1}({}, {})
}}",
                        lcname,
                        snake_case(name),
                        object.name,
                        index_decl,
                        type_,
                        index,
                        val
                    )?;
                }
            }
            if ip.write && ip.optional {
                writeln!(
                    r,
                    "
#[no_mangle]
pub unsafe extern \"C\" fn {}_set_data_{}_none(ptr: *mut {}{}) -> bool {{
    (&mut *ptr).set_{1}({}, None)
}}",
                    lcname,
                    snake_case(name),
                    object.name,
                    index_decl,
                    index
                )?;
            }
        }
    }
    Ok(())
}
