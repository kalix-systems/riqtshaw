use super::*;

pub(super) fn getter(object: &Object, prop_name: &str, property: &Property) -> Func {
    let mut func = Func::new(&format!("{}_get", base(object, prop_name)));
    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*const {}", &object.name))
        .ret(&format!("COption<{}>", property.property_type.rust_type()));

    let mut match_block = Block::new(&format!("match (&*ptr).{}()", snake_case(prop_name)));

    match_block.line("Some(value) => COption { data: value, some: true },");

    match_block.line(format!(
        "None => COption {{ data: {}::default(), some: false}},",
        property.property_type.rust_type()
    ));

    func.push_block(match_block);

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
