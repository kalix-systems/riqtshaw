use super::*;
use codegen::{Block, Formatter, Function as Func, Scope};

mod funcs;
mod item_props;
mod models;
mod new_imp;
mod properties;

pub(super) use item_props::push_item_props;
pub(super) use models::push_models;
pub(super) use properties::push_properties;

pub(super) fn push_new(scope: &mut Scope, object: &Object) {
    push_to_scope(scope, new_imp::new(object));
}

pub(super) fn push_functions(scope: &mut Scope, object: &Object) {
    let mut buf = String::new();
    let mut fmt = Formatter::new(&mut buf);

    for (fn_name, fn_def) in object.functions.iter() {
        funcs::function((fn_name, fn_def), object)
            .fmt(false, &mut fmt)
            .unwrap();
    }

    scope.raw(&buf);
}

pub(super) fn push_clear(scope: &mut Scope, object: &Object) {
    let mut func = Func::new(&format!("{}_free", snake_case(&object.name)));
    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", object.name))
        .line("Box::from_raw(ptr).emit().clear();");

    push_to_scope(scope, func);
}

fn push_to_scope(scope: &mut Scope, func: Func) {
    let mut buf = String::new();
    let mut fmt = Formatter::new(&mut buf);

    func.fmt(false, &mut fmt).unwrap();

    scope.raw(&buf);
}
