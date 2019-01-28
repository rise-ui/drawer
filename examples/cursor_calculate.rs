extern crate drawer;
extern crate yoga;
extern crate dom;
extern crate jss;

use yoga::{Layout, Direction};
use dom::events::*;
use dom::node::*;
use dom::traits::*;
use dom::tree::*;

#[path = "common/utils.rs"]
mod utils;

fn main() {
    let mut dom = utils::get_sample_dom_tree();
    let mut document = dom.document_mut();

    // Recalculate tree & layout
    {
        document.build_layout();
        document.value_mut().reflow_subtree(1000, 500, Direction::LTR);
    }

    let mut computed_hovered = drawer::CursorComputed::default();
    computed_hovered.window = (1000., 500.);
    
    let window_bounds = ((0., 0.), (1000., 500.));

    computed_hovered.calculate_hover(&mut document, window_bounds, (100., 150.));
    println!("{:#?}", computed_hovered);
    computed_hovered.reset();
    
    computed_hovered.calculate_hover(&mut document, window_bounds, (750., 150.));
    println!("{:#?}", computed_hovered);
    computed_hovered.reset();
}
