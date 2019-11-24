use super::*;
use codegen::{Block, Formatter, Function as Func, Scope};

mod new_imp;

pub(super) fn push_new(scope: &mut Scope, object: &Object) {
    let mut buf = String::new();
    let mut fmt = Formatter::new(&mut buf);
    new_imp::new(object).fmt(false, &mut fmt).unwrap();

    scope.raw(&buf);
}

pub(super) fn push_clear(scope: &mut Scope, object: &Object) {
    let mut buf = String::new();
    let mut fmt = Formatter::new(&mut buf);

    let mut func = Func::new(&format!("{}_free", snake_case(&object.name)));
    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", object.name))
        .line("Box::from_raw(ptr).emit().clear();");

    func.fmt(false, &mut fmt).unwrap();

    scope.raw(&buf);
}
