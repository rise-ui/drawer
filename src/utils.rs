use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use jss::traits::TStyleContext;
use jss::types::DimensionType;
use yoga::Layout;

use dom::events::*;
use dom::traits::*;
use dom::setup::*;

/// reflow dimension context in all tree elements for recalculate by runtime property (styles with calc function)
/// Call that before main iterate drawing tree per frame
pub fn preset_dimensions_tree<'a>(node: &mut DOMArenaRefMut<'a, BasicEvent>) {
    // preset dimensions for calculate
    let layout = {
        let value = node.raw.try_value();
        let layout = value.and_then(|value| Some(value.layout_node.get_layout()));
        layout.unwrap_or(Layout::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0))
    };

    // Set parent dimensions for current node
    let parent_layout = node
        .parent_id()
        .and_then(|id| Some(node.get_mut(id)))
        .and_then(|parent| parent.raw.try_value().and_then(|raw| Some(raw.layout_node.get_layout())));

    if let Some(styles) = node.raw.try_value_mut().and_then(|node| Some(&mut node.styles)) {
        styles.context.set_dimension(DimensionType::Parent, parent_layout);
    }

    // Set current node dimensions to style context
    if let Some(styles) = node.raw.try_value_mut().and_then(|node| Some(&mut node.styles)) {
        styles.context.set_dimension(DimensionType::Parent, Some(layout.clone()));
    }

    let mut next_child_id = node.first_child_id();
    while let Some(child_id) = next_child_id {
        {
            let mut child_ref = node.get_mut(child_id);
            preset_dimensions_tree(&mut child_ref);
        }

        next_child_id = node.get(child_id).next_sibling_id();
    }
}

pub fn random_string(length: usize) -> String {
    let mut rng = thread_rng();
    let string = rng.sample_iter(&Alphanumeric).take(length).collect();
    string
}
