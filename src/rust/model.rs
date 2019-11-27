use super::*;
use codegen::{Function as Func, *};

pub(super) fn model_name(object: &Object) -> Option<String> {
    match object.object_type {
        ObjectType::Tree => Some(format!("{name}Tree", name = object.name)),
        ObjectType::List => Some(format!("{name}List", name = object.name)),
        _ => None,
    }
}

pub(super) fn push_model(scope: &mut Scope, object: &Object) {
    if object.object_type == ObjectType::Object {
        return;
    }

    let model = model_def(object);
    let imp = model_imp(object, &model);

    scope.push_struct(model);
    scope.push_impl(imp);
}

fn model_def(object: &Object) -> Struct {
    match object.object_type {
        ObjectType::Tree => tree_model_def(object),
        ObjectType::List => list_model_def(object),
        _ => unreachable!(),
    }
}

fn tree_model_def(object: &Object) -> Struct {
    let mut model = Struct::new(&model_name(object).unwrap());
    let qobj = qobject(&object.name);

    let index_c_decl = "index: COption<usize>";
    let dest_c_decl = " dest: COption<usize>";

    let qobj_fn_ptr = format!("fn(*mut {qobj})", qobj = qobj);
    let begin_index_fn_ptr = format!(
        "fn(*mut {qobj}, {index_c_decl}, usize, usize)",
        qobj = qobj,
        index_c_decl = index_c_decl
    );

    model
        .vis("pub")
        .derive("Clone")
        .field("pub(super) qobject", format!("*mut {obj}", obj = &qobj))
        .field("pub(super) layout_about_to_be_changed", &qobj_fn_ptr)
        .field("pub(super) layout_changed", &qobj_fn_ptr)
        .field("pub(super) begin_reset_model", &qobj_fn_ptr)
        .field("pub(super) end_reset_model", &qobj_fn_ptr)
        .field("pub(super) end_insert_rows", &qobj_fn_ptr)
        .field("pub(super) end_move_rows", &qobj_fn_ptr)
        .field("pub(super) end_remove_rows", &qobj_fn_ptr)
        .field("pub(super) begin_insert_rows", &begin_index_fn_ptr)
        .field("pub(super) begin_remove_rows", &begin_index_fn_ptr)
        .field(
            "pub(super) data_changed",
            format!("fn(*mut {qobj}, usize, usize)", qobj = qobj),
        )
        .field(
            "pub(super) begin_move_rows",
            format!(
                "fn(*mut {qobj}, {index_c_decl}, usize, usize, {dest_c_decl}, usize)",
                qobj = qobj,
                index_c_decl = index_c_decl,
                dest_c_decl = dest_c_decl
            ),
        );

    model
}

fn list_model_def(object: &Object) -> Struct {
    let mut model = Struct::new(&model_name(object).unwrap());
    let qobj = qobject(&object.name);
    let qobj_fn_ptr = format!("fn(*mut {qobj})", qobj = qobj);

    let begin_index_fn_ptr = format!("fn(*mut {qobj},  usize, usize)", qobj = qobj,);

    model
        .vis("pub")
        .derive("Clone")
        .field("pub(super) qobject", format!("*mut {obj}", obj = &qobj))
        .field("pub(super) layout_about_to_be_changed", &qobj_fn_ptr)
        .field("pub(super) layout_changed", &qobj_fn_ptr)
        .field("pub(super) begin_reset_model", &qobj_fn_ptr)
        .field("pub(super) end_reset_model", &qobj_fn_ptr)
        .field("pub(super) end_insert_rows", &qobj_fn_ptr)
        .field("pub(super) end_move_rows", &qobj_fn_ptr)
        .field("pub(super) end_remove_rows", &qobj_fn_ptr)
        .field("pub(super) begin_insert_rows", &begin_index_fn_ptr)
        .field("pub(super) begin_remove_rows", &begin_index_fn_ptr)
        .field(
            "pub(super) data_changed",
            format!("fn(*mut {qobj}, usize, usize)", qobj = qobj),
        )
        .field(
            "pub(super) begin_move_rows",
            format!("fn(*mut {qobj}, usize, usize, usize)", qobj = qobj),
        );

    model
}

fn model_imp(object: &Object, model_struct: &Struct) -> Impl {
    match object.object_type {
        ObjectType::Tree => tree_model_imp(model_struct),
        ObjectType::List => list_model_imp(model_struct),
        _ => unreachable!(),
    }
}

fn list_model_imp(model_struct: &Struct) -> Impl {
    let mut imp = Impl::new(model_struct.ty());

    imp.push_fn(layout_about_to_be_changed())
        .push_fn(layout_changed())
        .push_fn(begin_reset_model())
        .push_fn(end_reset_model())
        .push_fn(end_insert_rows())
        .push_fn(end_move_rows())
        .push_fn(end_remove_rows())
        .push_fn(list_begin_insert_rows())
        .push_fn(list_begin_remove_rows())
        .push_fn(data_changed())
        .push_fn(list_begin_move_rows());

    imp
}

fn tree_model_imp(model_struct: &Struct) -> Impl {
    let mut imp = Impl::new(model_struct.ty());

    imp.push_fn(layout_about_to_be_changed())
        .push_fn(layout_changed())
        .push_fn(begin_reset_model())
        .push_fn(end_reset_model())
        .push_fn(end_insert_rows())
        .push_fn(end_move_rows())
        .push_fn(end_remove_rows())
        .push_fn(tree_begin_insert_rows())
        .push_fn(data_changed())
        .push_fn(tree_begin_remove_rows())
        .push_fn(tree_begin_move_rows());

    imp
}

fn layout_about_to_be_changed() -> Func {
    let mut func = Func::new("layout_about_to_be_changed");

    func.vis("pub")
        .arg_mut_self()
        .line("(self.layout_about_to_be_changed)(self.qobject);");

    func
}

fn layout_changed() -> Func {
    let mut func = Func::new("layout_changed");
    func.vis("pub")
        .arg_mut_self()
        .line("(self.layout_changed)(self.qobject)");

    func
}

fn data_changed() -> Func {
    let mut func = Func::new("data_changed");
    func.vis("pub")
        .arg_mut_self()
        .arg("first", "usize")
        .arg("last", "usize")
        .line("(self.data_changed)(self.qobject, first, last);");

    func
}

fn begin_reset_model() -> Func {
    let mut func = Func::new("begin_reset_model");

    func.vis("pub")
        .arg_mut_self()
        .line("(self.begin_reset_model)(self.qobject);");

    func
}

fn end_reset_model() -> Func {
    let mut func = Func::new("end_reset_model");

    func.vis("pub")
        .arg_mut_self()
        .line("(self.end_reset_model)(self.qobject);");

    func
}

fn end_remove_rows() -> Func {
    let mut func = Func::new("end_remove_rows");

    func.vis("pub")
        .arg_mut_self()
        .line("(self.end_remove_rows)(self.qobject);");

    func
}

fn end_move_rows() -> Func {
    let mut func = Func::new("end_move_rows");

    func.vis("pub")
        .arg_mut_self()
        .line("(self.end_move_rows)(self.qobject);");

    func
}

fn end_insert_rows() -> Func {
    let mut func = Func::new("end_insert_rows");

    func.vis("pub")
        .arg_mut_self()
        .line("(self.end_insert_rows)(self.qobject);");

    func
}

fn tree_begin_insert_rows() -> Func {
    let mut func = Func::new("begin_insert_rows");

    func.vis("pub")
        .arg_mut_self()
        .arg("index", "Option<usize>")
        .arg("first", "usize")
        .arg("last", "usize")
        .line("(self.begin_insert_rows)(self.qobject, index.into(), first, last);");

    func
}

fn list_begin_insert_rows() -> Func {
    let mut func = Func::new("begin_insert_rows");

    func.vis("pub")
        .arg_mut_self()
        .arg("first", "usize")
        .arg("last", "usize")
        .line("(self.begin_insert_rows)(self.qobject, first, last);");

    func
}

fn tree_begin_remove_rows() -> Func {
    let mut func = Func::new("begin_remove_rows");

    func.vis("pub")
        .arg_mut_self()
        .arg("index", "Option<usize>")
        .arg("first", "usize")
        .arg("last", "usize")
        .line("(self.begin_remove_rows)(self.qobject, index.into(), first, last);");

    func
}

fn list_begin_remove_rows() -> Func {
    let mut func = Func::new("begin_remove_rows");

    func.vis("pub")
        .arg_mut_self()
        .arg("first", "usize")
        .arg("last", "usize")
        .line("(self.begin_remove_rows)(self.qobject, first, last);");

    func
}

fn tree_begin_move_rows() -> Func {
    let mut func = Func::new("begin_move_rows");

    func.vis("pub")
        .arg_mut_self()
        .arg("index", "Option<usize>")
        .arg("first", "usize")
        .arg("last", "usize")
        .arg("dest", "Option<usize>")
        .arg("destination", "usize")
        .line(
            "(self.begin_move_rows)(self.qobject, index.into(), first, last, dest.into(), destination);",
        );

    func
}

fn list_begin_move_rows() -> Func {
    let mut func = Func::new("begin_move_rows");

    func.vis("pub")
        .arg_mut_self()
        .arg("first", "usize")
        .arg("last", "usize")
        .arg("destination", "usize")
        .line("(self.begin_move_rows)(self.qobject, first, last, destination);");

    func
}
