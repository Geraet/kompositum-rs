// Copyright(c) 2021 René Hansen.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)

pub mod builder;
pub mod component;
pub mod composite;
pub mod leaf;
pub mod printer;
mod tests;
pub mod visitor;

pub use multimap::MultiMap;

pub use crate::{component::Component, composite::Composite, leaf::Leaf, visitor::Visitor};

type Children = Vec<Box<dyn Component>>;
pub type IDType = u64;
