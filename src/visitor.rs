// Copyright(c) 2021 René Hansen.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)

use crate::{Composite, Leaf};

pub trait Visitor {
    fn visit_composite(&mut self, c: &Composite);
    fn visit_leaf(&mut self, l: &Leaf);
}
