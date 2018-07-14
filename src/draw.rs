use jss::properties::transforms_push_to_builder;
use resources::images::ImageInfo;
use utils::random_string;

use common::{
  PropertiesCollection,
  DrawingProperties,
  layout_into_rect,
  RectBuilder,
  Draw,
};

use webrender::api::{
  LayoutPrimitiveInfo,
  DisplayListBuilder,
  ComplexClipRegion,
  GlyphRasterSpace,
  ImageRendering,
  TransformStyle,
  MixBlendMode,
  BorderRadius,
  LayoutSize,
  AlphaType,
  ClipMode,
  ColorF,
};

#[derive(Clone, Debug)]
pub enum NodeType {
  Image(ImageInfo),
  Text(()),
  Div,
}

#[derive(Clone, Debug)]
pub struct DrawingNode {
  pub children: Vec<DrawingNode>,
  pub style: DrawingProperties,
  pub node_type: NodeType,
  pub tag: String,
}

impl DrawingNode {
  pub fn new(style: DrawingProperties, tag: Option<String>, node_type: Option<NodeType>) -> DrawingNode {
    let node_type = node_type.unwrap_or(NodeType::Div);
    let tag = tag.unwrap_or(random_string(10));

    DrawingNode {
      children: vec![],
      node_type,
      style,
      tag,
    }
  }

  pub fn push(&mut self, children: DrawingNode) {
    self.children.push(children);
  }
}

impl Draw for DrawingNode {
  fn draw(
    &self,
    mut builder: DisplayListBuilder,
    mut properties: PropertiesCollection,
  ) -> (DisplayListBuilder, PropertiesCollection) {
    let apperance = self.style.apperance.clone();
    let layout = self.style.layout.clone();

    let container_size = (layout.width(), layout.height());
    let primitive = LayoutPrimitiveInfo::new(layout_into_rect(&layout));

    // Get Transforms and clip that over stacking_context container
    let transforms = &apperance.transform.unwrap_or(Vec::new());
    let (mut builder, properties) = transforms_push_to_builder(
      &primitive,
      transforms.clone(),
      container_size.clone(),
      (self.tag.clone(), 10),
      properties,
      builder,
    );

    builder.push_stacking_context(
      &primitive,
      None,
      TransformStyle::Flat,
      MixBlendMode::Normal,
      Vec::new(),
      GlyphRasterSpace::Screen,
    );

    // Define content (inside stacking_context) bounds
    let content_bounds = (0., 0.).by(layout.width(), layout.height());
    let content_primitive = LayoutPrimitiveInfo::new(content_bounds.clone());

    let border_radius: BorderRadius = {
      if let Some(border_radius) = &apperance.border_radius {
        BorderRadius::from(border_radius.clone())
      } else {
        BorderRadius::zero()
      }
    };

    // Content clip for border-radius
    let clip = ComplexClipRegion::new(content_bounds.clone(), border_radius, ClipMode::Clip);
    let clip_id = builder.define_clip(content_bounds.clone(), vec![clip], None);
    builder.push_clip_id(clip_id);

    // Push background layer
    if let Some(background) = &self.style.apperance.background {
      let sizes = (self.style.layout.width(), self.style.layout.height());
      builder = background.push_to_builder(builder, &content_primitive, sizes);
    }

    // Push image
    match &self.node_type {
      NodeType::Image(image) => builder.push_image(
        &content_primitive,
        LayoutSize::new(layout.width(), layout.height()),
        LayoutSize::zero(),
        ImageRendering::Auto,
        AlphaType::Alpha,
        image.key,
      ),
      _ => {}
    }

    (builder, properties)
  }
}
