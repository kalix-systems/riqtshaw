use super::*;
use codegen::{Function as CGFunc, *};

const CLONE_DOC: &str = "Clone the emitter

The emitter can only be cloned when it is mutable. The emitter calls
into C++ code which may call into Rust again. If emmitting is possible
from immutable structures, that might lead to access to a mutable
reference. That is undefined behaviour and forbidden.";

pub(super) fn push_emitter(scope: &mut Scope, object: &Object) {
    let emitter_struct = emitter_def(object);
    let emitter_imp = emitter_impl(object, &emitter_struct);

    scope.push_struct(emitter_struct);
    scope.push_impl(emitter_imp);
}

fn emitter_def(object: &Object) -> Struct {
    let mut emitter = Struct::new(&emitter(&object.name));

    emitter.vis("pub").field(
        "pub(super) qobject",
        &format!("Arc<AtomicPtr<{}>>", qobject(&object.name)),
    );

    for prop_name in object
        .properties
        .iter()
        .filter(|(_, prop)| !prop.is_object())
        .map(|(name, _)| name)
    {
        emitter.field(
            &prop_changed_field(&prop_name),
            format!("fn(*mut {qobject})", qobject = qobject(&object.name)),
        );
    }

    match object.object_type {
        ObjectType::List => {
            emitter.field(
                "pub(super) new_data_ready",
                format!("fn(*mut {qobject})", qobject = qobject(&object.name)),
            );
        }
        _ => {}
    }

    emitter
}

fn emitter_impl(object: &Object, emit_struct: &Struct) -> Impl {
    let mut imp = Impl::new(emit_struct.ty());

    imp.push_fn(clone_fn(object, emit_struct));

    imp.push_fn(clear_fn(object));

    for prop_name in object.non_object_property_names() {
        imp.push_fn(prop_change_fn(prop_name));
    }

    match object.object_type {
        ObjectType::List => {
            imp.push_fn(list_new_data_ready());
        }
        _ => {}
    }

    imp
}

fn clone_fn(object: &Object, emit_struct: &Struct) -> CGFunc {
    let name = emitter(&object.name);

    let mut clone = CGFunc::new("clone");

    clone
        .vis("pub")
        .arg_mut_self()
        .ret(emit_struct.ty())
        .doc(CLONE_DOC);

    let mut clone_body = Block::new(&name);
    clone_body.line("qobject: self.qobject.clone(),");

    for prop_name in object.non_object_property_names() {
        clone_body.line(format!(
            "{prop_changed}: self.{prop_changed},",
            prop_changed = prop_changed(prop_name)
        ));
    }

    if object.object_type != ObjectType::Object {
        clone_body.line("new_data_ready: self.new_data_ready,");
    }

    clone.push_block(clone_body);

    clone
}

fn clear_fn(object: &Object) -> CGFunc {
    let mut clear = CGFunc::new("clear");

    clear
        .vis("pub")
        .arg_ref_self()
        .line(format!(
            "let n: *const {qobject} = null();",
            qobject = qobject(&object.name)
        ))
        .line(format!(
            "self.qobject.store(n as *mut {qobject}, Ordering::SeqCst);",
            qobject = qobject(&object.name)
        ));

    clear
}

fn prop_change_fn(prop_name: &str) -> CGFunc {
    let mut func = CGFunc::new(&prop_changed(prop_name));

    func.vis("pub")
        .arg_mut_self()
        .line("let ptr = self.qobject.load(Ordering::SeqCst);")
        .line("");

    let mut block = Block::new("if !ptr.is_null()");

    block.line(format!(
        "(self.{prop_changed})(ptr);",
        prop_changed = prop_changed(prop_name)
    ));

    func.push_block(block);

    func
}

fn list_new_data_ready() -> CGFunc {
    let mut func = CGFunc::new("new_data_ready");

    func.vis("pub")
        .arg_mut_self()
        .line("let ptr = self.qobject.load(Ordering::SeqCst);");

    let mut block = Block::new("if !ptr.is_null()");
    block.line("(self.new_data_ready)(ptr);");

    func.push_block(block);

    func
}

fn prop_changed(prop_name: &str) -> String {
    format!("{prop_name}_changed", prop_name = snake_case(prop_name))
}

fn prop_changed_field(prop_name: &str) -> String {
    format!(
        "pub(super) {prop_name}_changed",
        prop_name = snake_case(prop_name)
    )
}
