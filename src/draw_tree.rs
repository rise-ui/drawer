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
    let result = builder_node(node, builder, properties);
    properties = result.1;
    builder = result.0;
  }

  // Close context
  let (builder, properties) = close_node_context(&root, builder, properties);
  (builder, properties)
}

fn close_node_context(
  node: &DrawingNode,
  mut builder: DisplayListBuilder,
  properties: PropertiesCollection,
) -> (DisplayListBuilder, PropertiesCollection) {
  // Close content clip zone (aka border-radius)
  builder.pop_clip_id();
  // Pop of context node
  builder.pop_stacking_context();

  // Close clip id if found transform for container
  if properties.contains_key(&node.tag) {
    builder.pop_clip_id();
  }

  (builder, properties)
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
