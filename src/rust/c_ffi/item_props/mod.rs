use super::*;

mod list;
mod tree;

pub(crate) fn push_item_props(scope: &mut Scope, object: &Object) {
    if object.object_type == ObjectType::Object {
        return;
    }

    for (name, item_prop) in &object.item_properties {
        if let crate::configuration::Type::Object(item_obj) = &item_prop.item_property_type {
            match object.object_type {
                ObjectType::List => {
                    scope.push_fn(list::object_get(object, name, item_obj));
                }
                ObjectType::Tree => unimplemented!(),
                _ => {}
            }
        } else if item_prop.is_complex() {
            match object.object_type {
                ObjectType::List => {
                    scope.push_fn(list::complex_data(object, name, item_prop));
                }
                ObjectType::Tree => unimplemented!(),
                _ => {}
            }
        } else {
            match object.object_type {
                ObjectType::List => {
                    scope.push_fn(list::non_complex_data(object, name, item_prop));
                }
                ObjectType::Tree => unimplemented!(),
                _ => {}
            }
        }

        if item_prop.write {
            match item_prop.item_property_type {
                crate::configuration::Type::Simple(SimpleType::QString) => {
                    match object.object_type {
                        ObjectType::List => {
                            scope.push_fn(list::qstring_set(object, name, item_prop));
                        }
                        ObjectType::Tree => unimplemented!(),
                        _ => {}
                    }
                }
                crate::configuration::Type::Simple(SimpleType::QByteArray) => {
                    match object.object_type {
                        ObjectType::List => {
                            scope.push_fn(list::qbytearray_set(object, name, item_prop));
                        }
                        ObjectType::Tree => unimplemented!(),
                        _ => {}
                    }
                }
                crate::configuration::Type::Simple(_) => match object.object_type {
                    ObjectType::List => {
                        scope.push_fn(list::non_complex_set(object, name, item_prop));
                    }
                    ObjectType::Tree => unimplemented!(),
                    _ => {}
                },
                _ => {}
            }
        }

        if item_prop.write && item_prop.optional {
            match object.object_type {
                ObjectType::List => {
                    scope.push_fn(list::set_none(object, name, item_prop));
                }
                ObjectType::Tree => unimplemented!(),
                _ => {}
            }
        }
    }
}
