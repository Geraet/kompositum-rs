// Copyright(c) 2021 René Hansen.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)

use crate::{component::Component, Composite, Leaf, Visitor};

pub struct Printer {
    indent: i32,
}

impl Printer {
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    pub fn do_indentation(&self) {
        let mut i = self.indent;
        while i > 0 {
            print!("+--");
            i -= 1;
        }
    }
}

impl Visitor for Printer {
    fn visit_composite(&mut self, composite: &Composite) {
        self.do_indentation();
        self.indent += 1;

        if !composite.has_children() {
            println!("- Composite ({}): empty", &composite.get_uid());
        } else {
            println!("+ Composite ({}):", composite.get_uid());
            composite.visit_children(self);
        }

        self.indent -= 1;
    }

    fn visit_leaf(&mut self, l: &Leaf) {
        self.do_indentation();
        println!("- Leaf ({})", l.get_uid());
    }
}
