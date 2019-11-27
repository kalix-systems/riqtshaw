use super::*;
use codegen::*;

pub(super) fn rust_interface_module(object: &Object) -> Scope {
    let mut scope = Scope::new();
    scope.import("super", "*");

    scope.new_struct(&qobject(&object.name)).vis("pub");

    push_emitter(&mut scope, object);
    push_model(&mut scope, object);
    push_trait(&mut scope, object);

    c_ffi::push_new(&mut scope, object);
    c_ffi::push_clear(&mut scope, object);
    c_ffi::push_functions(&mut scope, object);
    c_ffi::push_properties(&mut scope, object);
    c_ffi::push_models(&mut scope, object);
    c_ffi::push_item_props(&mut scope, object);
    c_ffi::push_ptr_bundle(&mut scope, object);

    scope
}
