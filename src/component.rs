// Copyright(c) 2021 René Hansen.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)

use std::fmt;

use crate::{IDType, Visitor};

pub trait Component {
    fn get_uid(&self) -> IDType;
    fn accept(&self, visitor: &mut dyn Visitor);
}

// e.g. println!("{:?}", component); prints "Component { uid: 1 }"
impl fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Component")
            .field("uid", &self.get_uid())
            .finish()
    }
}
