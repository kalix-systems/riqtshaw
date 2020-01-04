use crate::configuration::*;
use std::collections::BTreeMap;

pub struct Obj<'a> {
    name: Option<&'a str>,
    functions: Functions,
    item_properties: ItemProperties,
    object_type: ObjectType,
    properties: Properties,
    signals: Signals,
}

impl Default for Obj<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Obj<'a> {
    pub fn new() -> Self {
        Self {
            name: None,
            functions: BTreeMap::new(),
            item_properties: BTreeMap::new(),
            properties: BTreeMap::new(),
            signals: BTreeMap::new(),
            object_type: ObjectType::Object,
        }
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name.replace(name);
        self
    }

    pub fn funcs(mut self, functions: BTreeMap<String, Function>) -> Self {
        self.functions = functions;
        self
    }

    pub fn item_props(mut self, item_props: BTreeMap<String, ItemProperty>) -> Self {
        self.item_properties = item_props;
        self
    }

    pub fn props(mut self, properties: BTreeMap<String, Property>) -> Self {
        self.properties = properties;
        self
    }

    pub fn signals(mut self, signals: BTreeMap<String, Signal>) -> Self {
        self.signals = signals;
        self
    }

    pub fn list(mut self) -> Self {
        self.object_type = ObjectType::List;
        self
    }

    pub fn build(self) -> Option<Object> {
        let Self {
            name,
            functions,
            item_properties,
            object_type,
            properties,
            signals,
        } = self;

        let name = name?.to_owned();

        Some(Object {
            name,
            functions,
            item_properties,
            properties,
            object_type,
            signals,
        })
    }
}
