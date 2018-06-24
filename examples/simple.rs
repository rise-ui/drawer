extern crate app_units;
extern crate euclid;
extern crate gleam;
extern crate glutin;
extern crate webrender;
extern crate winit;
extern crate drawer;
extern crate yoga;
extern crate ordered_float;
extern crate jss;

#[macro_use]
extern crate lazy_static;

#[path = "common/boilerplate.rs"]
mod boilerplate;

use ordered_float::OrderedFloat;
use boilerplate::Example;
use webrender::api::*;
use std::collections::HashMap;

lazy_static! {
  static ref STYLES: jss::Stylesheet = jss::parse_json_stylesheet(include_str!("common/styles.json")).unwrap();
}

fn main() {
  let mut app = App {};
  boilerplate::main_wrapper(&mut app, None);
}

struct App {}

fn get_default_apperance(name: &str) -> jss::Apperance {
  use jss::PrepareStyleExt;

  let style = STYLES.get(&name.to_string()).unwrap().clone();
  let style = style.default.unwrap();

  let styles = style.get_prepared_styles();
  styles.0.clone()
}

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
    let window_apperance = get_default_apperance("window");
    let box_one_apperance = get_default_apperance("box_one");
    let box_two_apperance = get_default_apperance("box_two");

    let root = drawer::DrawingNode {
      style: drawer::DrawingProperties {
        apperance: window_apperance,
        layout: yoga::Layout::new(0.0.into(), 0.0.into(), 0.0.into(), 0.0.into(), 400.0.into(), 400.0.into()),
      },

      children: vec![
        drawer::DrawingNode {
          style: drawer::DrawingProperties {
            apperance: box_one_apperance,
            layout: yoga::Layout::new(25.0.into(), 0.0.into(), 25.0.into(), 0.0.into(), 100.0.into(), 100.0.into()),
          },

          children: vec![],
        },
        drawer::DrawingNode {
          style: drawer::DrawingProperties {
            apperance: box_two_apperance,
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
