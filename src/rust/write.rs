use super::*;

pub(super) fn write_rust_interface_object(
    r: &mut Vec<u8>,
    o: &Object,
    conf: &Config,
) -> Result<()> {
    let lcname = snake_case(&o.name);
    writeln!(
        r,
        "
pub struct {}QObject {{}}

pub struct {0}Emitter {{
    qobject: Arc<AtomicPtr<{0}QObject>>,",
        o.name
    )?;
    for (name, p) in &o.properties {
        if p.is_object() {
            continue;
        }
        writeln!(
            r,
            "    {}_changed: fn(*mut {}QObject),",
            snake_case(name),
            o.name
        )?;
    }
    if o.object_type == ObjectType::List {
        writeln!(r, "    new_data_ready: fn(*mut {}QObject),", o.name)?;
    } else if o.object_type == ObjectType::Tree {
        writeln!(
            r,
            "    new_data_ready: fn(*mut {}QObject, index: COption<usize>),",
            o.name
        )?;
    }
    writeln!(
        r,
        "}}

impl {0}Emitter {{
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> {0}Emitter {{
        {0}Emitter {{
            qobject: self.qobject.clone(),",
        o.name
    )?;
    for (name, p) in &o.properties {
        if p.is_object() {
            continue;
        }
        writeln!(
            r,
            "            {}_changed: self.{0}_changed,",
            snake_case(name),
        )?;
    }
    if o.object_type != ObjectType::Object {
        writeln!(r, "            new_data_ready: self.new_data_ready,")?;
    }
    writeln!(
        r,
        "        }}
    }}
    fn clear(&self) {{
        let n: *const {0}QObject = null();
        self.qobject.store(n as *mut {0}QObject, Ordering::SeqCst);
    }}",
        o.name
    )?;

    for (name, p) in &o.properties {
        if p.is_object() {
            continue;
        }
        writeln!(
            r,
            "    pub fn {}_changed(&mut self) {{
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {{
            (self.{0}_changed)(ptr);
        }}
    }}",
            snake_case(name)
        )?;
    }

    if o.object_type == ObjectType::List {
        writeln!(
            r,
            "    pub fn new_data_ready(&mut self) {{
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {{
            (self.new_data_ready)(ptr);
        }}
    }}"
        )?;
    } else if o.object_type == ObjectType::Tree {
        writeln!(
            r,
            "    pub fn new_data_ready(&mut self, item: Option<usize>) {{
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {{
            (self.new_data_ready)(ptr, item.into());
        }}
    }}"
        )?;
    }

    let mut model_struct = String::new();
    if o.object_type != ObjectType::Object {
        let type_ = if o.object_type == ObjectType::List {
            "List"
        } else {
            "Tree"
        };
        model_struct = format!(", model: {}{}", o.name, type_);
        let mut index = "";
        let mut index_decl = "";
        let mut index_c_decl = "";
        let mut dest = "";
        let mut dest_decl = "";
        let mut dest_c_decl = "";
        if o.object_type == ObjectType::Tree {
            index_decl = " index: Option<usize>,";
            index_c_decl = " index: COption<usize>,";
            index = " index.into(),";
            dest_decl = " dest: Option<usize>,";
            dest_c_decl = " dest: COption<usize>,";
            dest = " dest.into(),";
        }
        writeln!(
            r,
            "}}

#[derive(Clone)]
pub struct {0}{1} {{
    qobject: *mut {0}QObject,
    layout_about_to_be_changed: fn(*mut {0}QObject),
    layout_changed: fn(*mut {0}QObject),
    data_changed: fn(*mut {0}QObject, usize, usize),
    begin_reset_model: fn(*mut {0}QObject),
    end_reset_model: fn(*mut {0}QObject),
    begin_insert_rows: fn(*mut {0}QObject,{4} usize, usize),
    end_insert_rows: fn(*mut {0}QObject),
    begin_move_rows: fn(*mut {0}QObject,{4} usize, usize,{7} usize),
    end_move_rows: fn(*mut {0}QObject),
    begin_remove_rows: fn(*mut {0}QObject,{4} usize, usize),
    end_remove_rows: fn(*mut {0}QObject),
}}

impl {0}{1} {{
    pub fn layout_about_to_be_changed(&mut self) {{
        (self.layout_about_to_be_changed)(self.qobject);
    }}
    pub fn layout_changed(&mut self) {{
        (self.layout_changed)(self.qobject);
    }}
    pub fn data_changed(&mut self, first: usize, last: usize) {{
        (self.data_changed)(self.qobject, first, last);
    }}
    pub fn begin_reset_model(&mut self) {{
        (self.begin_reset_model)(self.qobject);
    }}
    pub fn end_reset_model(&mut self) {{
        (self.end_reset_model)(self.qobject);
    }}
    pub fn begin_insert_rows(&mut self,{2} first: usize, last: usize) {{
        (self.begin_insert_rows)(self.qobject,{3} first, last);
    }}
    pub fn end_insert_rows(&mut self) {{
        (self.end_insert_rows)(self.qobject);
    }}
    pub fn begin_move_rows(&mut self,{2} first: usize, last: usize,{5} destination: usize) {{
        (self.begin_move_rows)(self.qobject,{3} first, last,{6} destination);
    }}
    pub fn end_move_rows(&mut self) {{
        (self.end_move_rows)(self.qobject);
    }}
    pub fn begin_remove_rows(&mut self,{2} first: usize, last: usize) {{
        (self.begin_remove_rows)(self.qobject,{3} first, last);
    }}
    pub fn end_remove_rows(&mut self) {{
        (self.end_remove_rows)(self.qobject);
    }}",
            o.name, type_, index_decl, index, index_c_decl, dest_decl, dest, dest_c_decl
        )?;
    }

    write!(
        r,
        "}}

pub trait {}Trait {{
    fn new(emit: {0}Emitter{}",
        o.name, model_struct
    )?;
    for (name, p) in &o.properties {
        if p.is_object() {
            write!(r, ",\n        {}: {}", snake_case(name), p.type_name())?;
        }
    }
    writeln!(
        r,
        ") -> Self;
    fn emit(&mut self) -> &mut {}Emitter;",
        o.name
    )?;
    for (name, p) in &o.properties {
        let lc = snake_case(name).to_lowercase();
        if p.is_object() {
            writeln!(r, "    fn {}(&self) -> &{};", lc, rust_type(p))?;
            writeln!(r, "    fn {}_mut(&mut self) -> &mut {};", lc, rust_type(p))?;
        } else {
            if p.rust_by_function {
                write!(
                    r,
                    "    fn {}<F>(&self, getter: F) where F: FnOnce({});",
                    lc,
                    rust_return_type(p)
                )?;
            } else {
                writeln!(r, "    fn {}(&self) -> {};", lc, rust_return_type(p))?;
            }
            if p.write {
                if p.type_name() == "QByteArray" {
                    if p.optional {
                        writeln!(r, "    fn set_{}(&mut self, value: Option<&[u8]>);", lc)?;
                    } else {
                        writeln!(r, "    fn set_{}(&mut self, value: &[u8]);", lc)?;
                    }
                } else {
                    writeln!(r, "    fn set_{}(&mut self, value: {});", lc, rust_type(p))?;
                }
            }
        }
    }
    for (name, f) in &o.functions {
        let lc = snake_case(name);
        let mut arg_list = String::new();
        if !f.arguments.is_empty() {
            for a in &f.arguments {
                let t = if a.argument_type.name() == "QByteArray" {
                    "&[u8]"
                } else {
                    a.argument_type.rust_type()
                };
                arg_list.push_str(&format!(", {}: {}", a.name, t));
            }
        }
        writeln!(
            r,
            "    fn {}(&{}self{}) -> {};",
            lc,
            if f.mutable { "mut " } else { "" },
            arg_list,
            f.return_type.rust_type()
        )?;
    }
    if o.object_type == ObjectType::List {
        writeln!(
            r,
            "    fn row_count(&self) -> usize;
    fn insert_rows(&mut self, _row: usize, _count: usize) -> bool {{ false }}
    fn remove_rows(&mut self, _row: usize, _count: usize) -> bool {{ false }}
    fn can_fetch_more(&self) -> bool {{
        false
    }}
    fn fetch_more(&mut self) {{}}
    fn sort(&mut self, _: u8, _: SortOrder) {{}}"
        )?;
    } else if o.object_type == ObjectType::Tree {
        writeln!(
            r,
            "    fn row_count(&self, _: Option<usize>) -> usize;
    fn can_fetch_more(&self, _: Option<usize>) -> bool {{
        false
    }}
    fn fetch_more(&mut self, _: Option<usize>) {{}}
    fn sort(&mut self, _: u8, _: SortOrder) {{}}
    fn check_row(&self, index: usize, row: usize) -> Option<usize>;
    fn index(&self, item: Option<usize>, row: usize) -> usize;
    fn parent(&self, index: usize) -> Option<usize>;
    fn row(&self, index: usize) -> usize;"
        )?;
    }
    if o.object_type != ObjectType::Object {
        for (name, ip) in &o.item_properties {
            let name = snake_case(name);
            writeln!(
                r,
                "    fn {}(&self, index: usize) -> {};",
                name,
                rust_return_type_(ip)
            )?;
            if ip.write {
                if ip.item_property_type.name() == "QByteArray" {
                    if ip.optional {
                        writeln!(
                            r,
                            "    fn set_{}(&mut self, index: usize, _: Option<&[u8]>) -> bool;",
                            name
                        )?;
                    } else {
                        writeln!(
                            r,
                            "    fn set_{}(&mut self, index: usize, _: &[u8]) -> bool;",
                            name
                        )?;
                    }
                } else {
                    writeln!(
                        r,
                        "    fn set_{}(&mut self, index: usize, _: {}) -> bool;",
                        name,
                        rust_type_(ip)
                    )?;
                }
            }
        }
    }
    writeln!(
        r,
        "}}

#[no_mangle]
pub extern \"C\" fn {}_new(",
        lcname
    )?;
    r_constructor_args_decl(r, &lcname, o, conf)?;
    writeln!(r, ",\n) -> *mut {} {{", o.name)?;
    r_constructor_args(r, &lcname, o, conf)?;
    writeln!(
        r,
        "    Box::into_raw(Box::new(d_{}))
}}

#[no_mangle]
pub unsafe extern \"C\" fn {0}_free(ptr: *mut {}) {{
    Box::from_raw(ptr).emit().clear();
}}",
        lcname, o.name
    )?;

    for (name, p) in &o.properties {
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
                o.name,
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
                    o.name,
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
                    o.name,
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
                    o.name,
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
                    o.name,
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
                o.name,
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
                    o.name,
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
                    o.name,
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
                o.name,
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
                    o.name,
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
                o.name,
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
                    o.name,
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
                o.name,
                snake_case(name)
            )?;
        }
    }
    for f in &o.functions {
        write_function(r, f, &lcname, o)?;
    }
    if o.object_type == ObjectType::List {
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
            o.name, lcname
        )?;
    } else if o.object_type == ObjectType::Tree {
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
            o.name, lcname
        )?;
    }
    if o.object_type != ObjectType::Object {
        let (index_decl, index) = if o.object_type == ObjectType::Tree {
            (", index: usize", "index")
        } else {
            (", row: c_int", "to_usize(row).unwrap_or(0)")
        };
        for (name, ip) in &o.item_properties {
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
                    o.name,
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
                    o.name,
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
                    o.name,
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
                        o.name,
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
                        o.name,
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
                        o.name,
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
                    o.name,
                    index_decl,
                    index
                )?;
            }
        }
    }
    Ok(())
}
