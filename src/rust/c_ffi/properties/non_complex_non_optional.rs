use super::*;

pub(super) fn getter(object: &Object, prop_name: &str, property: &Property) -> Func {
    let mut func = Func::new(&format!("{}_get", base(object, prop_name)));
    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*const {}", &object.name))
        .ret(rust_type(property))
        .line(&format!("(&*ptr).{}()", snake_case(prop_name)));

    func
}

pub(super) fn setter(object: &Object, prop_name: &str, property: &Property) -> Func {
    let mut func = Func::new(&format!("{}_set", base(object, prop_name)));
    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", &object.name))
        .arg("value", rust_type(property))
        .line(&format!("(&mut *ptr).set_{}(value)", snake_case(prop_name)));
    func
}
