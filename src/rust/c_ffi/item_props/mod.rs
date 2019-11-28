use super::*;

mod list;

pub(crate) fn push_item_props(scope: &mut Scope, object: &Object) {
    if let ObjectType::List = object.object_type {
        for (name, item_prop) in &object.item_properties {
            if item_prop.is_complex() {
                scope.push_fn(list::complex_data(object, name, item_prop));
            } else {
                scope.push_fn(list::non_complex_data(object, name, item_prop));
            }

            if item_prop.write {
                match item_prop.item_property_type {
                    SimpleType::QString => {
                        scope.push_fn(list::qstring_set(object, name, item_prop));
                    }
                    SimpleType::QByteArray => {
                        scope.push_fn(list::qbytearray_set(object, name, item_prop));
                    }
                    _ => {
                        scope.push_fn(list::non_complex_set(object, name, item_prop));
                    }
                }
            }

            if item_prop.write && item_prop.optional {
                scope.push_fn(list::set_none(object, name, item_prop));
            }
        }
    }
}
