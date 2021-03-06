use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::rc::Rc;

pub type Objects = BTreeMap<String, Rc<Object>>;
pub type Signals = BTreeMap<String, Signal>;
pub type Properties = BTreeMap<String, Property>;
pub type ItemProperties = BTreeMap<String, ItemProperty>;
pub type Functions = BTreeMap<String, Function>;
pub type Connections = Vec<Connection>;
pub type Hooks = (Signals, Connections);

pub struct Config {
    pub out_dir: PathBuf,
    pub cpp_file: PathBuf,
    pub objects: Objects,
    pub rust: Rust,
    pub overwrite_implementation: bool,
}

impl Config {
    pub fn types(&self) -> BTreeSet<String> {
        let mut ops = BTreeSet::new();

        for object in self.objects.values() {
            for p in object.properties.values() {
                ops.insert(p.type_name().into());
            }

            for p in object.item_properties.values() {
                ops.insert(p.type_name().into());
            }

            for f in object.functions.values() {
                ops.insert(f.return_type.name().into());

                for a in &f.arguments {
                    ops.insert(a.type_name().into());
                }
            }
        }

        ops
    }

    pub fn optional_types(&self) -> BTreeSet<String> {
        let mut ops = BTreeSet::new();
        for o in self.objects.values() {
            for p in o.properties.values() {
                if p.optional {
                    ops.insert(p.type_name().into());
                }
            }
            for p in o.item_properties.values() {
                if p.optional {
                    ops.insert(p.type_name().into());
                }
            }
            if o.object_type != ObjectType::Object {
                ops.insert("quintptr".into());
            }
        }
        ops
    }

    pub fn has_list(&self) -> bool {
        self.objects
            .values()
            .any(|o| o.object_type == ObjectType::List)
    }
}

#[derive(PartialEq, Debug)]
pub struct Object {
    pub name: String,
    pub functions: Functions,
    pub item_properties: ItemProperties,
    pub object_type: ObjectType,
    pub properties: Properties,
    pub signals: Signals,
    pub connections: Connections,
}

impl Object {
    pub fn non_object_property_names(&self) -> impl Iterator<Item = &String> {
        self.properties
            .iter()
            .filter(|(_, property)| !property.is_object())
            .map(|(prop_name, _)| prop_name)
    }

    pub fn object_properties(&self) -> impl Iterator<Item = (&String, &Property)> {
        self.properties
            .iter()
            .filter(|(_, property)| property.is_object())
    }
}

impl Object {
    pub fn contains_object(&self) -> bool {
        self.properties.values().any(|p| p.is_object())
    }

    pub fn column_count(&self) -> usize {
        let mut column_count = 1;
        for ip in self.item_properties.values() {
            column_count = column_count.max(ip.roles.len());
        }
        column_count
    }
}

#[derive(PartialEq, Debug)]
pub struct Property {
    pub optional: bool,
    pub property_type: Type,
    pub rust_by_value: bool,
    pub write: bool,
}

impl Property {
    pub fn is_object(&self) -> bool {
        self.property_type.is_object()
    }

    pub fn is_complex(&self) -> bool {
        self.property_type.is_complex()
    }

    pub fn c_get_type(&self) -> String {
        let name = self.property_type.name();
        name.to_string() + "*, " + &name.to_lowercase() + "_set"
    }
}

impl Property {
    pub fn type_name(&self) -> &str {
        self.property_type.name()
    }
}

pub struct Rust {
    pub dir: PathBuf,
    pub implementation_module: String,
    pub interface_module: String,
}

#[derive(PartialEq, Debug)]
pub enum ObjectType {
    Object,
    List,
}

#[derive(PartialEq, Clone, Debug)]
pub enum SimpleType {
    QString,
    QByteArray,
    Bool,
    Float,
    Double,
    Void,
    Qint8,
    Qint16,
    Qint32,
    Qint64,
    QUint8,
    QUint16,
    QUint32,
    QUint64,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CopyType {
    Bool,
    Float,
    Double,
    Void,
    Qint8,
    Qint16,
    Qint32,
    Qint64,
    QUint8,
    QUint16,
    QUint32,
    QUint64,
}

impl CopyType {
    pub fn name(&self) -> &str {
        match self {
            CopyType::Bool => "bool",
            CopyType::Float => "float",
            CopyType::Double => "double",
            CopyType::Void => "void",
            CopyType::Qint8 => "qint8",
            CopyType::Qint16 => "qint16",
            CopyType::Qint32 => "qint32",
            CopyType::Qint64 => "qint64",
            CopyType::QUint8 => "quint8",
            CopyType::QUint16 => "quint16",
            CopyType::QUint32 => "quint32",
            CopyType::QUint64 => "quint64",
        }
    }

    pub fn cpp_set_type(&self) -> &str {
        self.name()
    }

    pub fn c_set_type(&self) -> &str {
        self.name()
    }

    pub fn rust_type(&self) -> &str {
        match self {
            CopyType::Bool => "bool",
            CopyType::Float => "f32",
            CopyType::Double => "f64",
            CopyType::Void => "()",
            CopyType::Qint8 => "i8",
            CopyType::Qint16 => "i16",
            CopyType::Qint32 => "i32",
            CopyType::Qint64 => "i64",
            CopyType::QUint8 => "u8",
            CopyType::QUint16 => "u16",
            CopyType::QUint32 => "u32",
            CopyType::QUint64 => "u64",
        }
    }

    pub fn rust_type_init(&self) -> &str {
        match self {
            CopyType::Bool => "false",
            CopyType::Float | CopyType::Double => "0.0",
            CopyType::Void => "()",
            _ => "0",
        }
    }
}

impl SimpleType {
    pub fn name(&self) -> &str {
        match self {
            SimpleType::QString => "QString",
            SimpleType::QByteArray => "QByteArray",
            SimpleType::Bool => "bool",
            SimpleType::Float => "float",
            SimpleType::Double => "double",
            SimpleType::Void => "void",
            SimpleType::Qint8 => "qint8",
            SimpleType::Qint16 => "qint16",
            SimpleType::Qint32 => "qint32",
            SimpleType::Qint64 => "qint64",
            SimpleType::QUint8 => "quint8",
            SimpleType::QUint16 => "quint16",
            SimpleType::QUint32 => "quint32",
            SimpleType::QUint64 => "quint64",
        }
    }

    pub fn cpp_set_type(&self) -> &str {
        match self {
            SimpleType::QString => "const QString&",
            SimpleType::QByteArray => "const QByteArray&",
            _ => self.name(),
        }
    }

    pub fn c_set_type(&self) -> &str {
        match self {
            SimpleType::QString => "qstring_t",
            SimpleType::QByteArray => "qbytearray_t",
            _ => self.name(),
        }
    }

    pub fn rust_type(&self) -> &str {
        match self {
            SimpleType::QString => "String",
            SimpleType::QByteArray => "Vec<u8>",
            SimpleType::Bool => "bool",
            SimpleType::Float => "f32",
            SimpleType::Double => "f64",
            SimpleType::Void => "()",
            SimpleType::Qint8 => "i8",
            SimpleType::Qint16 => "i16",
            SimpleType::Qint32 => "i32",
            SimpleType::Qint64 => "i64",
            SimpleType::QUint8 => "u8",
            SimpleType::QUint16 => "u16",
            SimpleType::QUint32 => "u32",
            SimpleType::QUint64 => "u64",
        }
    }

    pub fn rust_type_init(&self) -> &str {
        match self {
            SimpleType::QString => "String::new()",
            SimpleType::QByteArray => "Vec::new()",
            SimpleType::Bool => "false",
            SimpleType::Float | SimpleType::Double => "0.0",
            SimpleType::Void => "()",
            _ => "0",
        }
    }

    pub fn is_complex(&self) -> bool {
        self == &SimpleType::QString || self == &SimpleType::QByteArray
    }
}

#[derive(PartialEq, Debug)]
pub enum Type {
    Simple(SimpleType),
    Object(Rc<Object>),
}

impl Type {
    pub fn is_object(&self) -> bool {
        match self {
            Type::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_complex(&self) -> bool {
        match self {
            Type::Simple(simple) => simple.is_complex(),
            _ => false,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Type::Simple(s) => s.name(),
            Type::Object(o) => &o.name,
        }
    }

    pub fn cpp_set_type(&self) -> &str {
        match self {
            Type::Simple(s) => s.cpp_set_type(),
            Type::Object(o) => &o.name,
        }
    }

    pub fn c_set_type(&self) -> &str {
        match self {
            Type::Simple(s) => s.c_set_type(),
            Type::Object(o) => &o.name,
        }
    }

    pub fn rust_type(&self) -> &str {
        match self {
            Type::Simple(s) => s.rust_type(),
            Type::Object(o) => &o.name,
        }
    }

    pub fn rust_type_init(&self) -> &str {
        match self {
            Type::Simple(s) => s.rust_type_init(),
            Type::Object(_) => unimplemented!(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ItemProperty {
    pub item_property_type: SimpleType,
    pub optional: bool,
    pub roles: Vec<Vec<String>>,
    pub rust_by_value: bool,
    pub write: bool,
}

impl ItemProperty {
    pub fn type_name(&self) -> &str {
        self.item_property_type.name()
    }
}

impl ItemProperty {
    pub fn is_complex(&self) -> bool {
        self.item_property_type.is_complex()
    }

    pub fn cpp_set_type(&self) -> String {
        let typ = self.item_property_type.cpp_set_type().to_string();

        if self.optional {
            return "option_".to_string() + &typ;
        }

        typ
    }

    pub fn c_get_type(&self) -> String {
        let name = self.item_property_type.name();
        name.to_string() + "*, " + &name.to_lowercase() + "_set"
    }
    pub fn c_set_type(&self) -> &str {
        self.item_property_type.c_set_type()
    }
}

#[derive(PartialEq, Debug)]
pub struct Function {
    pub return_type: SimpleType,
    pub mutable: bool,
    pub arguments: Vec<Argument>,
}

#[derive(PartialEq, Debug)]
pub struct Signal {
    pub arguments: Vec<CopyArgument>,
}

impl Function {
    pub fn type_name(&self) -> &str {
        self.return_type.name()
    }
}

#[derive(PartialEq, Debug)]
pub struct CopyArgument {
    pub name: String,
    pub argument_type: CopyType,
}

#[derive(PartialEq, Debug)]
pub struct Argument {
    pub name: String,
    pub argument_type: SimpleType,
}

impl Argument {
    pub fn type_name(&self) -> &str {
        self.argument_type.name()
    }
}

#[derive(PartialEq, Debug)]
pub struct Connection {
    pub(crate) signal: String,
    pub(crate) function: String,
}

impl Connection {
    pub fn new(signal: String, function: String) -> Self {
        Self { signal, function }
    }
}
