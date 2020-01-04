use super::*;

mod list;

pub(crate) fn push_models(scope: &mut Scope, object: &Object) {
    if let ObjectType::List = object.object_type {
        use list::*;
        scope.push_fn(row_count(object));
        scope.push_fn(insert_rows(object));
        scope.push_fn(remove_rows(object));
        scope.push_fn(can_fetch_more(object));
        scope.push_fn(fetch_more(object));
        scope.push_fn(sort(object));
    }
}
