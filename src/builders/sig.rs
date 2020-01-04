use crate::configuration::*;

#[derive(Default)]
pub struct Sig<'a> {
    arguments: Vec<(&'a str, CopyType)>,
}

impl<'a> Sig<'a> {
    pub fn new() -> Sig<'a> {
        Self { arguments: vec![] }
    }

    pub fn arg(mut self, name: &'a str, typ: CopyType) -> Self {
        self.arguments.push((name, typ));
        self
    }

    pub fn build(self) -> Signal {
        let Self { arguments } = self;

        Signal {
            arguments: arguments
                .into_iter()
                .map(|(name, typ)| CopyArgument {
                    name: name.to_owned(),
                    argument_type: typ,
                })
                .collect(),
        }
    }
}
