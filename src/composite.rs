// Copyright(c) 2021 René Hansen.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)

use crate::{Children, Component, IDType, Visitor};

pub struct Composite {
    uid: IDType,
    children: Children,
}

impl Composite {
    pub fn new(uid: u64) -> Self {
        Self {
            uid,
            children: Children::new(),
        }
    }
}

impl Component for Composite {
    fn get_uid(&self) -> IDType {
        self.uid
    }

    fn accept(&self, v: &mut dyn Visitor) {
        v.visit_composite(self);
    }
}

impl Composite {
    pub fn add_child(&mut self, component: Box<dyn Component>) {
        self.children.push(component);
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    pub fn visit_children(&self, v: &mut dyn Visitor) {
        for child in &self.children {
            child.accept(v);
        }
    }
}
