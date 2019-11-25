#![allow(unused)]

mod list;
mod tree;

use super::*;

pub(crate) fn push_models(scope: &mut Scope, object: &Object) {
    match object.object_type {
        ObjectType::Tree => {
            use tree::*;
            push_to_scope(scope, row_count(object));
            push_to_scope(scope, insert_rows(object));
            push_to_scope(scope, remove_rows(object));
            push_to_scope(scope, can_fetch_more(object));
            push_to_scope(scope, fetch_more(object));
            push_to_scope(scope, sort(object));
            push_to_scope(scope, check_row(object));
            push_to_scope(scope, index(object));
            push_to_scope(scope, parent(object));
            push_to_scope(scope, row(object));
        }
        ObjectType::List => {
            use list::*;
            push_to_scope(scope, row_count(object));
            push_to_scope(scope, insert_rows(object));
            push_to_scope(scope, remove_rows(object));
            push_to_scope(scope, can_fetch_more(object));
            push_to_scope(scope, fetch_more(object));
            push_to_scope(scope, sort(object));
        }
        _ => {}
    }
}
