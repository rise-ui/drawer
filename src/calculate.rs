use dom::node::DOMNodeId;
use dom::events::*;
use dom::traits::*;
use dom::setup::*;
use yoga::Layout;

#[derive(Debug, Default)]
pub struct CursorComputed {
    pub ids: Vec<DOMNodeId<BasicEvent>>,
    pub window: (f32, f32),
}

impl CursorComputed {
    pub fn reset(&mut self) {
        self.ids = vec![];
    }

    /// Calculate states for elements by cursor position
    pub fn calculate_hover<'a>(
        &mut self,
        node: &mut DOMArenaRefMut<'a, BasicEvent>,
        zone: ((f32, f32), (f32, f32)), // top-left, bottom-right
        cursor: (f32, f32), // x, y
    ) {
        let layout = {
            let value = node.raw.try_value();
            let layout = value.and_then(|value| Some(value.layout_node.get_layout()));
            layout.unwrap_or(Layout::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0))
        };

        // Current zone on screen contains current node
        let height = layout.height();
        let width = layout.width();

        let (mut x1, mut x2) = zone.0.clone();

        x1 += layout.left();
        x2 += layout.top();

        let y1 = x1 + width;
        let y2 = x2 + height;

        let current_bounds = ((x1, x2), (y1, y2));
        if cursor.1 >= x2 && cursor.1 <= y2 && cursor.0 >= x1 && cursor.0 <= y1 {
            self.ids.push(node.id());
        }

        let mut next_child_id = node.first_child_id();

        while let Some(child_id) = next_child_id {
            {
                let mut child_ref = node.get_mut(child_id);
                self.calculate_hover(&mut child_ref, current_bounds, cursor);
            }

            next_child_id = node.get(child_id).next_sibling_id();
        }
    }
}
