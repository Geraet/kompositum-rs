// Copyright(c) 2021 René Hansen.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)

use crate::{Component, IDType, Visitor};

pub struct Leaf {
    uid: IDType,
}

impl Leaf {
    pub fn new(uid: IDType) -> Self {
        Self { uid }
    }
}

impl Component for Leaf {
    fn get_uid(&self) -> IDType {
        self.uid
    }

    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_leaf(&self);
    }
}
