// Copyright(c) 2021 René Hansen.
// Distributed under the MIT License (http://opensource.org/licenses/MIT)

use multimap::MultiMap;

use crate::{component::Component, composite::Composite, leaf::Leaf, IDType};

type ParentChildTreeDef = MultiMap<IDType, IDType>;

pub fn build_composite(uid: IDType, tree_map: &ParentChildTreeDef) -> Box<dyn Component> {
    let has_children = tree_map.contains_key(&uid);
    if has_children {
        let mut composite = Composite::new(uid);
        match tree_map.get_vec(&uid) {
            Some(children) => {
                children.iter().for_each(|child| {
                    let new_child = build_composite(*child, &tree_map);
                    composite.add_child(new_child);
                });
            }
            None => (),
        }

        return Box::new(composite);
    }

    Box::new(Leaf::new(uid))
}
