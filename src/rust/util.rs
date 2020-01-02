use super::*;

pub(super) fn qobject(name: &str) -> String {
    format!("{name}QObject", name = name)
}

pub(super) fn emitter(name: &str) -> String {
    format!("{name}Emitter", name = name)
}

pub(super) fn rust_type(p: &Property) -> String {
    if p.optional {
        return format!("Option<{}>", p.property_type.rust_type());
    }

    p.property_type.rust_type().to_string()
}

pub(super) fn rust_type_(p: &ItemProperty) -> String {
    if p.optional {
        return format!("Option<{}>", p.item_property_type.rust_type());
    }
    p.item_property_type.rust_type().to_string()
}

pub(super) fn rust_return_type(p: &Property) -> String {
    let value = p.rust_by_value;
    let mut type_: String = p.property_type.rust_type().to_string();

    if type_ == "String" {
        type_ = if value {
            "String".into()
        } else {
            "&str".into()
        };
    }

    if type_ == "Vec<u8>" {
        type_ = if value {
            "Vec<u8>".into()
        } else {
            "&[u8]".into()
        };
    }

    if p.optional {
        return format!("Option<{}>", type_);
    }

    type_
}

pub(super) fn rust_return_type_(p: &ItemProperty) -> String {
    let mut type_: String = p.item_property_type.rust_type().to_string();

    if type_ == "String" && !p.rust_by_value {
        type_ = "str".to_string();
    }
    if type_ == "Vec<u8>" && !p.rust_by_value {
        type_ = "[u8]".to_string();
    }
    if p.item_property_type.is_complex() && !p.rust_by_value {
        type_ = "&".to_string() + &type_;
    }
    if p.optional {
        return "Option<".to_string() + &type_ + ">";
    }
    type_
}

pub(super) fn rust_c_type(p: &ItemProperty) -> String {
    if p.optional {
        return format!("COption<{}>", p.item_property_type.rust_type());
    }
    p.item_property_type.rust_type().to_string()
}
