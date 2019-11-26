pub(crate) struct Block {
    before: Option<String>,
    after: Option<String>,
    lines: Vec<String>,
}

impl Block {
    pub(crate) fn new() -> Block {
        Block {
            before: None,
            after: None,
            lines: Vec::new(),
        }
    }

    pub(crate) fn before<T: ToString>(&mut self, before: T) -> &mut Block {
        self.before.replace(before.to_string());
        self
    }

    pub(crate) fn after<T: ToString>(&mut self, after: T) -> &mut Block {
        self.after.replace(after.to_string());
        self
    }

    pub(crate) fn line<T: ToString>(&mut self, line: T) -> &mut Block {
        self.lines.push(line.to_string());
        self
    }

    pub(crate) fn push_block(&mut self, block: Block) -> &mut Block {
        self.line(block);
        self
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(before) = self.before.as_ref() {
            write!(w, "{} ", before)?;
        }

        write!(w, "{{")?;

        for line in self.lines.iter() {
            writeln!(w, "{}", line)?;
        }

        write!(w, "}}")?;

        if let Some(after) = self.after.as_ref() {
            write!(w, "{}", after)?;
        }

        Ok(())
    }
}
