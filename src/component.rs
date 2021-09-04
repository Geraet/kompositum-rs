// Copyright(c) 2021 René Hansen.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)

use crate::{IDType, Visitor};

pub trait Component {
    fn get_uid(&self) -> IDType;
    fn accept(&self, visitor: &mut dyn Visitor);
}
