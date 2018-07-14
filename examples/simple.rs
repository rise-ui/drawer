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
use boilerplate::Example;
use boilerplate::resources as assets;
use webrender::api::*;

lazy_static! {
  static ref STYLES: jss::Stylesheet =
    jss::parse_json_stylesheet(include_str!("common/styles.json")).unwrap();
}

fn get_default_apperance(name: &str) -> jss::Apperance {
  use jss::PrepareStyleExt;

  let style = STYLES.get(&name.to_string()).unwrap().clone();
  let style = style.default.unwrap();

  let styles = style.get_prepared_styles();
  styles.0.clone()
}

struct App {
  is_init: bool,
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
    let mut resources = assets::resources();
    let box_image_apperance = get_default_apperance("box_image");
    let box_one_apperance = get_default_apperance("box_one");
    let box_two_apperance = get_default_apperance("box_two");
    let window_apperance = get_default_apperance("window");

    let mut root = drawer::DrawingNode::new(
      drawer::DrawingProperties {
        layout: yoga::Layout::new(0.0.into(), 0.0.into(), 0.0.into(), 0.0.into(), 400.0.into(), 400.0.into()),
        apperance: window_apperance,
      },
      None,
      None,
    );

    let children_one = drawer::DrawingNode::new(
      drawer::DrawingProperties {
        layout: yoga::Layout::new(
          25.0.into(),
          0.0.into(),
          25.0.into(),
          0.0.into(),
          100.0.into(),
          100.0.into(),
        ),
        apperance: box_one_apperance,
      },
      None,
      None,
    );

    let children_two = drawer::DrawingNode::new(
      drawer::DrawingProperties {
        layout: yoga::Layout::new(
          150.0.into(),
          0.0.into(),
          25.0.into(),
          0.0.into(),
          100.0.into(),
          100.0.into(),
        ),
        apperance: box_two_apperance,
      },
      None,
      None,
    );

    let image_source = assets::images::ImageSource::bundled("jerk");
    let image = resources.image_loader.get_image(&image_source).unwrap();

    let children_image = drawer::DrawingNode::new(
      drawer::DrawingProperties {
        layout: yoga::Layout::new(
          270.0.into(),
          0.0.into(),
          25.0.into(),
          0.0.into(),
          100.0.into(),
          100.0.into(),
        ),
        apperance: box_image_apperance,
      },
      None,
      Some(drawer::NodeType::Image(image.clone())),
    );

    root.push(children_one);
    root.push(children_two);
    root.push(children_image);

    let mut drawer_rise = drawer::Drawer::new(root);
    let result = drawer_rise.render(builder.clone());
    *builder = result.0;
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

fn main() {
  boilerplate::main_wrapper(
    &mut App {
      is_init: false,
    },
    None,
  );
}
