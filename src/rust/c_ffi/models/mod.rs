mod list;

use super::*;

pub(crate) fn push_models(scope: &mut Scope, object: &Object) {
    match object.object_type {
        ObjectType::List => {
            use list::*;
            scope.push_fn(row_count(object));
            scope.push_fn(insert_rows(object));
            scope.push_fn(remove_rows(object));
            scope.push_fn(can_fetch_more(object));
            scope.push_fn(fetch_more(object));
            scope.push_fn(sort(object));
        }
        _ => {}
    }
}
