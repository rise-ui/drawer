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
  TransformStyle,
  MixBlendMode,
  BorderRadius,
  ClipMode,
  ColorF,
};

use jss::properties::{
  transforms_push_to_builder
};

#[derive(Clone, Debug)]
pub struct DrawingNode {
  pub children: Vec<DrawingNode>,
  pub style: DrawingProperties,
  pub tag: String,
}

impl DrawingNode {
  pub fn new(style: DrawingProperties, tag: Option<String>) -> DrawingNode {
    let tag = match tag {
      None => random_string(10),
      Some(name) => name,
    };

    DrawingNode {
      children: vec![],
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

    let content_bounds = (0., 0.).by(layout.width(), layout.height());
    let content_primitive = LayoutPrimitiveInfo::new(content_bounds.clone());

    let border_radius: BorderRadius = {
      if let Some(border_radius) = &apperance.border_radius {
        BorderRadius::from(border_radius.clone())
      } else {
        BorderRadius::zero()
      }
    };

    let clip = ComplexClipRegion::new(content_bounds.clone(), border_radius, ClipMode::Clip);
    let clip_id = builder.define_clip(content_bounds.clone(), vec![clip], None);
    builder.push_clip_id(clip_id);

    if let Some(background) = &self.style.apperance.background {
      let sizes = (self.style.layout.width(), self.style.layout.height());
      builder = background.push_to_builder(builder, &content_primitive, sizes);
    }

    (builder, properties)
  }
}
