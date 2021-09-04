// Copyright(c) 2021 René Hansen.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)

mod builder;
mod component;
mod composite;
mod leaf;
mod printer;
mod tests;
mod visitor;

use crate::{component::Component, composite::Composite, leaf::Leaf, visitor::Visitor};

type Children = Vec<Box<dyn Component>>;
type IDType = u64;
