use super::*;

mod list;
mod tree;

pub(crate) fn push_item_props(scope: &mut Scope, object: &Object) {
    if object.object_type == ObjectType::Object {
        return;
    }

    for (name, item_prop) in &object.item_properties {
        if item_prop.is_complex() {
            match object.object_type {
                ObjectType::List => {
                    push_to_scope(scope, list::complex_data(object, name, item_prop));
                }
                _ => {}
            }
        } else {
            match object.object_type {
                ObjectType::List => {
                    push_to_scope(scope, list::non_complex_data(object, name, item_prop));
                }
                _ => {}
            }
        }

        if item_prop.write {
            match item_prop.item_property_type {
                crate::configuration::Type::Simple(SimpleType::QString) => {
                    match object.object_type {
                        ObjectType::List => {
                            push_to_scope(scope, list::qstring_set(object, name, item_prop));
                        }
                        _ => {}
                    }
                }
                crate::configuration::Type::Simple(SimpleType::QByteArray) => {
                    match object.object_type {
                        ObjectType::List => {
                            push_to_scope(scope, list::qbytearray_set(object, name, item_prop));
                        }
                        _ => {}
                    }
                }
                crate::configuration::Type::Simple(_) => match object.object_type {
                    ObjectType::List => {
                        push_to_scope(scope, list::non_complex_set(object, name, item_prop));
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if item_prop.write && item_prop.optional {
            match object.object_type {
                ObjectType::List => {
                    push_to_scope(scope, list::set_none(object, name, item_prop));
                }
                _ => {}
            }
        }
    }
}
