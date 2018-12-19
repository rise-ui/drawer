/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate app_units;
extern crate euclid;
extern crate gleam;
extern crate glutin;
extern crate drawer;
extern crate jss;
extern crate winit;
extern crate yoga;
extern crate dom;

use dom::events::*;
use dom::node::*;
use dom::traits::*;
use dom::tree::*;
use dom::types::*;
use dom::setup::*;

use jss::types::*;
use yoga::Direction;

use jss::webrender;

#[path = "common/boilerplate.rs"]
mod boilerplate;

use boilerplate::{Example, HandyDandyRectBuilder};
use webrender::ShaderPrecacheFlags;
use webrender::api::*;
use euclid::vec2;

struct App {
    pub props: drawer::PropertiesCollection<DOMNodeId<BasicEvent>>,
    pub dom: DOMTree<BasicEvent>,
}

fn get_dom_tree() -> DOMTree<BasicEvent> {
    let container_style = StyleBuilder::default().case(Case::Ignore).parse_from_str(r#"{
        "justify-content": "space-between",
        "background": "rgba(0,0,0,0.3)",
        "flex-direction": "row",
        "align-items": "center",

        "padding-left": "20px",
        "padding-right": "20px",

        "border-top-left-radius": "15px",
        "border-top-right-radius": "15px",
        "border-bottom-left-radius": "15px",
        "border-bottom-right-radius": "15px"
    }"#).unwrap();

    let item_style = StyleBuilder::default().case(Case::Ignore).parse_from_str(r#"{
        "justify-content": "space-between",
        "background": "rgb(255,255,255)",
        "align-items": "center",
        "margin-top": "10px",
        "height": "250px",
        "width": "250px",

        "border-top-color": "rgba(0,0,0,0.6)",
        "border-top-width": 10,

        "border-top-left-radius": "10px",
        "border-top-right-radius": "10px",
        "border-bottom-left-radius": "10px",
        "border-bottom-right-radius": "10px",

        "transform": [
            "rotate(40deg,40deg)"
        ]
    }"#).unwrap();

    let tree: DOMTree<BasicEvent> = {
        let mut fragment = DOMTree::default();
        
        {
            let mut parent = fragment.root_mut();
            {
                let mut parent = parent.append(DOMNode::from((
                    DOMTagName::from(KnownElementName::Div),
                    vec![ DOMAttribute::from((DOMAttributeName::from("name"), DOMAttributeValue::from("body"))) ],
                    container_style
                )));

                {
                    let mut first_item = parent.append(DOMNode::from((
                        DOMTagName::from(KnownElementName::Div),
                        vec![ DOMAttribute::from((DOMAttributeName::from("name"), DOMAttributeValue::from("item"))) ],
                        item_style.clone()
                    )));
                }

                {
                    let mut second_item = parent.append(DOMNode::from((
                        DOMTagName::from(KnownElementName::Div),
                        vec![ DOMAttribute::from((DOMAttributeName::from("name"), DOMAttributeValue::from("item"))) ],
                        item_style.clone()
                    )));
                }

                {
                    let mut three_item = parent.append(DOMNode::from((
                        DOMTagName::from(KnownElementName::Div),
                        vec![ DOMAttribute::from((DOMAttributeName::from("name"), DOMAttributeValue::from("item"))) ],
                        item_style.clone()
                    )));
                }
            }
        }

        fragment
    };

    tree
}

fn main() {
    let props: drawer::PropertiesCollection<DOMNodeId<BasicEvent>> = drawer::PropertiesCollection::default();
    let dom = get_dom_tree();

    let mut app = App { dom, props };

    boilerplate::main_wrapper(&mut app, None);
}

impl Example for App {
    // Make this the only example to test all shaders for compile errors.
    const PRECACHE_SHADER_FLAGS: ShaderPrecacheFlags = ShaderPrecacheFlags::FULL_COMPILE;

    fn render(
        &mut self,
        api: &RenderApi,
        builder: &mut DisplayListBuilder,
        txn: &mut Transaction,
        _: DeviceIntSize,
        _pipeline_id: PipelineId,
        _document_id: DocumentId,
    ) { 
        let mut document = self.dom.document_mut();

        // Recalculate tree & layout
        {
            document.build_layout();
            document.value_mut().reflow_subtree(1000, 500, Direction::LTR);
        }

        drawer::render_node(
            &mut self.props,
            builder,
            &mut document
        );

        builder.print_display_list();
    }

    fn on_event(
        &mut self,
        event: winit::WindowEvent,
        api: &RenderApi,
        document_id: DocumentId,
    ) -> bool {
        let mut txn = Transaction::new();

        if !txn.is_empty() {
            txn.generate_frame();
            api.send_transaction(document_id, txn);
        }

        false
    }
}
