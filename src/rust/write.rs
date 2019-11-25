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
    c_ffi::push_clear(&mut scope, object);
    c_ffi::push_functions(&mut scope, object);
    c_ffi::push_properties(&mut scope, object);
    c_ffi::push_models(&mut scope, object);

    writeln!(r, "{}", scope.to_string())?;

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
