mod builder;
mod component;
mod composite;
mod leaf;
mod printer;
mod visitor;

use crate::{
    component::Component, composite::Composite, leaf::Leaf, printer::Printer, visitor::Visitor,
};

type Children = Vec<Box<dyn Component>>;
type IDType = u64;

#[cfg(test)]
mod tests {
    use super::*;
    use multimap::MultiMap;

    #[test]
    fn from_builder() {
        const TREE_MAP_DEF: &[(IDType, IDType)] = &[(1, 2), (1, 3), (1, 4), (4, 5), (4, 6), (1, 7)];
        let tree_map: MultiMap<IDType, IDType> = TREE_MAP_DEF.iter().cloned().collect();

        let root = builder::build_composite(1, &tree_map);

        root.accept(&mut Printer::new());
    }
}
