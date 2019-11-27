use super::*;
use codegen::{Block, Function as Func, Scope};

mod funcs;
mod item_props;
mod models;
mod new_imp;
mod properties;
mod ptr_bundle;

pub(super) use item_props::push_item_props;
pub(super) use models::push_models;
pub(super) use properties::push_properties;
use ptr_bundle::ptr_bundle;

pub(super) fn push_ptr_bundle(scope: &mut Scope, object: &Object) {
    scope.push_struct(ptr_bundle(object));
}

pub(super) fn push_new(scope: &mut Scope, object: &Object) {
    scope.push_fn(new_imp::new(object));
}

pub(super) fn push_functions(scope: &mut Scope, object: &Object) {
    for (fn_name, fn_def) in object.functions.iter() {
        scope.push_fn(funcs::function((fn_name, fn_def), object));
    }
}

pub(super) fn push_clear(scope: &mut Scope, object: &Object) {
    let mut func = Func::new(&format!("{}_free", snake_case(&object.name)));

    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", object.name))
        .line("Box::from_raw(ptr).emit().clear();");

    scope.push_fn(func);
}

fn ptr_bundle_name(object: &Object) -> String {
    let name = &object.name;
    format!("{}PtrBundle", name)
}
