use crate::configuration::*;

pub struct Prop {
    optional: bool,
    property_type: Type,
    rust_by_value: bool,
    write: bool,
}

impl Default for Prop {
    fn default() -> Self {
        Self::new()
    }
}

impl Prop {
    pub fn new() -> Self {
        Self {
            optional: false,
            property_type: Type::Simple(SimpleType::Void),
            rust_by_value: false,
            write: false,
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn get_by_value(mut self) -> Self {
        self.rust_by_value = true;
        self
    }

    pub fn simple(mut self, typ: SimpleType) -> Self {
        self.property_type = Type::Simple(typ);
        self
    }

    pub fn object(mut self, typ: Object) -> Self {
        self.property_type = Type::Object(std::rc::Rc::new(typ));
        self
    }

    pub fn write(mut self) -> Self {
        self.write = true;
        self
    }

    pub fn build(self) -> Property {
        let Prop {
            property_type,
            rust_by_value,
            optional,
            write,
        } = self;

        Property {
            optional,
            rust_by_value,
            write,
            property_type,
        }
    }
}
