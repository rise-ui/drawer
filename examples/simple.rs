extern crate app_units;
extern crate euclid;
extern crate gleam;
extern crate glutin;
extern crate webrender;
extern crate winit;
extern crate drawer;
extern crate yoga;
extern crate ordered_float;

#[path = "common/boilerplate.rs"]
mod boilerplate;

use ordered_float::OrderedFloat;
use boilerplate::Example;
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
    let root = drawer::DrawingNode {
      style: drawer::DrawingProperties {
        apperance: vec![],
        layout: yoga::Layout::new(0.0.into(), 0.0.into(), 0.0.into(), 0.0.into(), 400.0.into(), 400.0.into()),
      },

      children: vec![
        drawer::DrawingNode {
          style: drawer::DrawingProperties {
            apperance: vec![],
            layout: yoga::Layout::new(25.0.into(), 0.0.into(), 25.0.into(), 0.0.into(), 100.0.into(), 100.0.into()),
          },

          children: vec![],
        },
        drawer::DrawingNode {
          style: drawer::DrawingProperties {
            apperance: vec![],
            layout: yoga::Layout::new(150.0.into(), 0.0.into(), 25.0.into(), 0.0.into(), 100.0.into(), 100.0.into()),
          },

          children: vec![],
        },
      ],
    };

    *builder = drawer::render(root, builder.clone());
    // builder.print_display_list();
    // println!("\n\n\n");
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
