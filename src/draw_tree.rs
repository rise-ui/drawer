use webrender::api::{DisplayListBuilder, PropertyBindingKey};
use common::{Draw, PropertiesCollection};
use draw::DrawingNode;

fn builder_node(
  root: &DrawingNode,
  mut builder: DisplayListBuilder,
  mut properties: PropertiesCollection,
) -> (DisplayListBuilder, PropertiesCollection) {
  // Draw root, open main context
  let drawed_result = root.draw(builder, properties);
  properties = drawed_result.1;
  builder = drawed_result.0;

  for node in root.children.iter() {
    // Draw child
    let drawed_result = builder_node(node, builder, properties);
    properties = drawed_result.1;
    builder = drawed_result.0;
  }

  // Close context
  builder = close_node_context(builder);
  (builder, properties)
}

fn close_node_context(mut builder: DisplayListBuilder) -> DisplayListBuilder {
  builder.pop_stacking_context();
  builder.pop_clip_id();
  builder
}

pub struct Drawer {
  binding_keys: PropertiesCollection,
  root: DrawingNode,
}

impl Drawer {
  pub fn new(root: DrawingNode) -> Drawer {
    Drawer {
      binding_keys: PropertiesCollection::new(),
      root,
    }
  }

  pub fn render(&mut self, builder: DisplayListBuilder) -> (DisplayListBuilder, PropertiesCollection) {
    let result = builder_node(&self.root, builder, self.binding_keys.clone());
    self.binding_keys = result.1.clone();
    result
  }
}
