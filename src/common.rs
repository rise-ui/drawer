use webrender::api::{DisplayListBuilder, LayoutRect, LayoutPoint, LayoutSize};
use jss::properties::Apperance;
use yoga::Layout;

pub trait Draw {
  fn draw(&self, mut builder: DisplayListBuilder) -> DisplayListBuilder;
}

#[derive(Clone, Debug)]
pub struct DrawingProperties {
  pub apperance: Apperance,
  pub layout: Layout,
}

pub trait RectBuilder {
  fn to(&self, x2: f32, y2: f32) -> LayoutRect;
  fn by(&self, w: f32, h: f32) -> LayoutRect;
}

impl RectBuilder for (f32, f32) {
  fn to(&self, x2: f32, y2: f32) -> LayoutRect {
    LayoutRect::new(LayoutPoint::new(self.0, self.1), LayoutSize::new(x2 - self.0, y2 - self.1))
  }

  fn by(&self, w: f32, h: f32) -> LayoutRect {
    LayoutRect::new(LayoutPoint::new(self.0, self.1), LayoutSize::new(w, h))
  }
}

pub fn layout_into_rect(layout: &Layout) -> LayoutRect {
  (layout.left(), layout.top()).by(layout.width(), layout.height())
}

pub fn layout_into_size(layout: &Layout) -> LayoutSize {
  LayoutSize::new(layout.width(), layout.height())
}
