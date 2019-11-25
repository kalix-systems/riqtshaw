//let (index_decl, index) = if object.object_type == ObjectType::Tree {
//    (", index: usize", "index")
//} else {
//    (", row: c_int", "to_usize(row).unwrap_or(0)")
//};

// complex and non-optional
//
//                writeln!(
//                    r,
//                    "
//#[no_mangle]
//pub unsafe extern \"C\" fn {}_data_{}(
//    ptr: *const {}{},
//    d: *mut {},
//    set: fn(*mut {4}, *const c_char, len: c_int),
//) {{
//    let o = &*ptr;
//    let data = o.{1}({});
//    let s: *const c_char = data.as_ptr() as (*const c_char);
//    set(d, s, to_c_int(data.len()));
//}}",
//                    lcname,
//                    snake_case(name),
//                    object.name,
//                    index_decl,
//                    ip.type_name(),
//                    index
//                )?;

// complex and optional
//
//                writeln!(
//                    r,
//                    "
//#[no_mangle]
//pub unsafe extern \"C\" fn {}_data_{}(
//    ptr: *const {}{},
//    d: *mut {},
//    set: fn(*mut {4}, *const c_char, len: c_int),
//) {{
//    let o = &*ptr;
//    let data = o.{1}({});
//    if let Some(data) = data {{
//        let s: *const c_char = data.as_ptr() as (*const c_char);
//        set(d, s, to_c_int(data.len()));
//    }}
//}}",
//                    lcname,
//                    snake_case(name),
//                    object.name,
//                    index_decl,
//                    ip.type_name(),
//                    index
//                )?;

// non-complex
//
//                writeln!(
//                    r,
//                    "
//#[no_mangle]
//pub unsafe extern \"C\" fn {}_data_{}(ptr: *const {}{}) -> {} {{
//    let o = &*ptr;
//    o.{1}({}){}
//}}",
//                    lcname,
//                    snake_case(name),
//                    object.name,
//                    index_decl,
//                    rust_c_type(ip),
//                    index,
//                    if ip.optional { ".into()" } else { "" }
//                )?;

// write and optional
//
//                writeln!(
//                    r,
//                    "
//#[no_mangle]
//pub unsafe extern \"C\" fn {}_set_data_{}_none(ptr: *mut {}{}) -> bool {{
//    (&mut *ptr).set_{1}({}, None)
//}}",
//                    lcname,
//                    snake_case(name),
//                    object.name,
//                    index_decl,
//                    index
//                )?;

// QString write
//
//
//
//                    writeln!(
//                        r,
//                        "
//#[no_mangle]
//pub unsafe extern \"C\" fn {}_set_data_{}(
//    ptr: *mut {}{},
//    s: *const c_ushort, len: c_int,
//) -> bool {{
//    let o = &mut *ptr;
//    let mut v = String::new();
//    set_string_from_utf16(&mut v, s, len);
//    o.set_{1}({}, {})
//}}",
//                        lcname,
//                        snake_case(name),
//                        object.name,
//                        index_decl,
//                        index,
//                        val
//                    )?;

// QByteArray write
//
//                    writeln!(
//                        r,
//                        "
//#[no_mangle]
//pub unsafe extern \"C\" fn {}_set_data_{}(
//    ptr: *mut {}{},
//    s: *const c_char, len: c_int,
//) -> bool {{
//    let o = &mut *ptr;
//    let slice = qba_slice!(s, len);
//    o.set_{1}({}, {})
//}}",
//                        lcname,
//                        snake_case(name),
//                        object.name,
//                        index_decl,
//                        index,
//                        if ip.optional { "Some(slice)" } else { "slice" }
//                    )?;

// non-complex write
//                    writeln!(
//                        r,
//                        "
//#[no_mangle]
//pub unsafe extern \"C\" fn {}_set_data_{}(
//    ptr: *mut {}{},
//    v: {},
//) -> bool {{
//    (&mut *ptr).set_{1}({}, {})
//}}",
//                        lcname,
//                        snake_case(name),
//                        object.name,
//                        index_decl,
//                        type_,
//                        index,
//                        val
//                    )?;
