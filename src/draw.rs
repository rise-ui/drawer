use common::{
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

#[derive(Clone, Debug)]
pub struct DrawingNode {
  pub style: DrawingProperties,
  pub children: Vec<DrawingNode>,
}

impl Draw for DrawingNode {
  fn draw(&self, mut builder: DisplayListBuilder) -> DisplayListBuilder {
    let container_bounds = layout_into_rect(&self.style.layout);
    let primitive = LayoutPrimitiveInfo::new(container_bounds.clone());

    builder.push_stacking_context(&primitive, None, TransformStyle::Flat, MixBlendMode::Normal, Vec::new(), GlyphRasterSpace::Screen);

    let content_bounds = (0., 0.).by(self.style.layout.width(), self.style.layout.height());
    let content_primitive = LayoutPrimitiveInfo::new(content_bounds.clone());

    let clip = ComplexClipRegion::new(content_bounds.clone(), BorderRadius::uniform(20.), ClipMode::Clip);
    let clip_id = builder.define_clip(content_bounds.clone(), vec![clip], None);

    builder.push_clip_id(clip_id);
    builder.push_rect(&content_primitive, ColorF::new(1., 1., 1., 1.));

    builder
  }
}
