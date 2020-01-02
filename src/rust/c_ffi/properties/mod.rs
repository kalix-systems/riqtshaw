use super::*;

mod complex_non_optional;
mod complex_optional;
mod non_complex_non_optional;
mod non_complex_optional;

pub(crate) fn push_properties(scope: &mut Scope, object: &Object) {
    for (prop_name, property) in object.properties.iter() {
        match &property.property_type {
            Type::Object(_) => {
                scope.push_fn(object_get(object, prop_name, property));
            }
            Type::Simple(simp_type) => {
                simple_prop(scope, simp_type, prop_name, property, object);
            }
        }

        if property.write && property.optional {
            scope.push_fn(set_none(object, prop_name));
        }
    }
}

fn simple_prop(
    scope: &mut Scope,
    simp_type: &SimpleType,
    prop_name: &str,
    property: &Property,
    object: &Object,
) {
    match (property.is_complex(), property.optional) {
        (true, false) => {
            if property.rust_by_value {
                scope.push_fn(complex_non_optional::get_by_function(
                    object, prop_name, property,
                ));
            } else {
                scope.push_fn(complex_non_optional::getter(object, prop_name, &property));
            }

            if property.write {
                match simp_type {
                    SimpleType::QString => {
                        scope.push_fn(complex_non_optional::qstring_setter(object, prop_name));
                    }
                    SimpleType::QByteArray => {
                        scope.push_fn(complex_non_optional::qbytearray_setter(object, prop_name));
                    }
                    _ => {}
                }
            }
        }
        (true, true) => {
            scope.push_fn(complex_optional::getter(object, prop_name, &property));

            if property.write {
                match simp_type {
                    SimpleType::QString => {
                        scope.push_fn(complex_optional::qstring_setter(object, prop_name));
                    }
                    SimpleType::QByteArray => {
                        scope.push_fn(complex_optional::qbytearray_setter(object, prop_name));
                    }
                    _ => {}
                }
            }
        }
        (false, true) => {
            scope.push_fn(non_complex_optional::getter(object, prop_name, property));

            if property.write {
                scope.push_fn(non_complex_optional::setter(object, prop_name, property));
            }
        }
        (false, false) => {
            scope.push_fn(non_complex_non_optional::getter(
                object, prop_name, property,
            ));

            if property.write {
                scope.push_fn(non_complex_non_optional::setter(
                    object, prop_name, property,
                ));
            }
        }
    }
}

pub(super) fn set_none(object: &Object, prop_name: &str) -> Func {
    let mut func = Func::new(&format!("{}_set_none", base(object, prop_name)));
    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .arg("ptr", format!("*mut {}", object.name))
        .line("let obj = &mut *ptr;")
        .line(format!("obj.set_{}(None);", snake_case(prop_name)));

    func
}

fn object_get(object: &Object, prop_name: &str, property: &Property) -> Func {
    let mut func = Func::new(&format!("{}_get", base(object, prop_name)));
    func.extern_abi("C")
        .attr("no_mangle")
        .vis("pub unsafe")
        .ret(format!("*mut {}", rust_type(property)))
        .arg("ptr", format!("*mut {}", object.name))
        .line(&format!("(&mut *ptr).{}_mut()", snake_case(prop_name)));

    func
}

fn base(object: &Object, prop_name: &str) -> String {
    format!("{}_{}", snake_case(&object.name), snake_case(prop_name))
}
