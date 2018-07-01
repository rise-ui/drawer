/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate app_units;
extern crate euclid;
extern crate gleam;
extern crate glutin;
extern crate webrender;
extern crate winit;

#[path = "common/boilerplate.rs"]
mod boilerplate;

use boilerplate::{Example, make_rotation, make_skew, HandyDandyRectBuilder};
use webrender::api::*;

fn main() {
  let mut app = App {};
  boilerplate::main_wrapper(&mut app, None);
}

struct App {}

impl Example for App {
  // Make this the only example to test all shaders for compile errors.
  const PRECACHE_SHADERS: bool = true;

  fn render(
    &mut self,
    api: &RenderApi,
    builder: &mut DisplayListBuilder,
    txn: &mut Transaction,
    _: DeviceUintSize,
    _pipeline_id: PipelineId,
    _document_id: DocumentId,
  ) {
    // Basic layer declaration
    let window_bounds = LayoutRect::new(LayoutPoint::zero(), builder.content_size());
    // rect wrapper
    let window_layer = LayoutPrimitiveInfo::new(window_bounds);

    // Create new context zone for drawing
    builder.push_stacking_context(
      &window_layer,
      None,
      TransformStyle::Flat,
      MixBlendMode::Normal,
      Vec::new(),
      GlyphRasterSpace::Screen,
    );

    // Create zone for clipping content inside stacking context
    let complex = ComplexClipRegion::new(
      // Set zone for clipping by coord with size
      LayoutRect::new(LayoutPoint::zero(), builder.content_size()),
      // Set border radius for clip
      BorderRadius::uniform(20.0),
      ClipMode::Clip,
    );

    // Define clip block with params
    let id = builder.define_clip(window_bounds, vec![complex], None);
    // Push clip block to layout
    builder.push_clip_id(id);

    /******** Content Zone of First Stacking Context ********/
    // Push rectangle layer inside clip zone
    builder.push_rect(&window_layer, ColorF::new(1., 1., 1., 1.));

    // Transformation
    let rotation_transform = make_rotation(&LayoutPoint::new(50., 50.), 45., 0.0, 0.0, 1.0);
    // create clip zone of transform container

    let boxes = (50, 50).by(100, 100);
    let boxes_primitive = LayoutPrimitiveInfo::new(boxes.clone());

    let transformed_frame = builder.push_reference_frame(
      &boxes_primitive,
      Some(PropertyBinding::Binding(PropertyBindingKey::new(42), rotation_transform)),
      None,
    );
    builder.push_clip_id(transformed_frame);

    builder.push_stacking_context(
      &boxes_primitive,
      None,
      TransformStyle::Flat,
      MixBlendMode::Normal,
      Vec::new(),
      GlyphRasterSpace::Screen,
    );

    let stops = vec![
      GradientStop {
        offset: 0.0,
        color: ColorF::new(0.84, 0.2, 0.41, 1.0),
      },
      GradientStop {
        offset: 0.5,
        color: ColorF::new(0.8, 0.68, 0.43, 1.0),
      },
    ];

    let gradient = builder.create_gradient(
      LayoutPoint::new(0.0, 0.0),
      LayoutPoint::new(100., 100.),
      stops,
      ExtendMode::Clamp,
    );
    builder.push_gradient(
      &boxes_primitive,
      gradient,
      LayoutSize::new(100.0, 100.0),
      LayoutSize::new(0.0, 0.0),
    );

    builder.pop_clip_id();
    builder.pop_stacking_context();
    /*******************************************************/

    // Close clip zone
    builder.pop_clip_id();
    // Close stacking context layer
    builder.pop_stacking_context();
  }

  fn on_event(&mut self, event: winit::WindowEvent, api: &RenderApi, document_id: DocumentId) -> bool {
    let mut txn = Transaction::new();

    if !txn.is_empty() {
      txn.generate_frame();
      api.send_transaction(document_id, txn);
    }

    false
  }
}
