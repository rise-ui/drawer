use webrender::api::DisplayListBuilder;
use draw::DrawingNode;
use common::Draw;

fn close_node_context(mut builder: DisplayListBuilder) -> DisplayListBuilder {
  builder.pop_stacking_context();
  builder.pop_clip_id();
  builder
}

pub fn render(root: DrawingNode, mut builder: DisplayListBuilder) -> DisplayListBuilder {
  // Draw root, open main context
  builder = root.draw(builder);

  for node in root.children {
    // Draw child
    builder = render(node, builder);
  }

  // Close context
  builder = close_node_context(builder);
  builder
}
