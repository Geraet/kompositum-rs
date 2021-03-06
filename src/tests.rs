// Copyright(c) 2021 René Hansen.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)

#[cfg(test)]
mod tests {
    use crate::{builder, printer::Printer, IDType};
    use multimap::MultiMap;

    const TREE_MAP_DEF: &[(IDType, IDType)] = &[(1, 2), (1, 3), (1, 4), (4, 5), (4, 6), (1, 7)];

    #[test]
    fn test_from_builder() {
        let tree_map: MultiMap<IDType, IDType> = TREE_MAP_DEF.iter().cloned().collect();

        let root = builder::build_composite(1, &tree_map);
        root.accept(&mut Printer::new());
    }

    #[test]
    fn test_component_debug_print() {
        let tree_map: MultiMap<IDType, IDType> = TREE_MAP_DEF.iter().cloned().collect();

        let root = builder::build_composite(1, &tree_map);
        println!("{:?}", &root);
    }
}
