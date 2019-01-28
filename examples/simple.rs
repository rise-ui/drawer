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
use yoga::Direction;

use drawer::{Drawer, PropertiesCollection, CursorComputed};
use jss::webrender;

#[path = "common/boilerplate.rs"]
mod boilerplate;

#[path = "common/utils.rs"]
mod utils;

use webrender::ShaderPrecacheFlags;
use boilerplate::Example;
use webrender::api::*;
use euclid::vec2;

struct App {
    pub props: PropertiesCollection<DOMNodeId<BasicEvent>>,
    pub hovered: CursorComputed,
    pub dom: DOMTree<BasicEvent>,
}

fn main() {
    let props: PropertiesCollection<DOMNodeId<BasicEvent>> = PropertiesCollection::default();
    let hovered = CursorComputed::default();
    let mut dom = utils::get_sample_dom_tree();

    // Recalculate tree & layout
    {
        let mut document = dom.document_mut();
        document.build_layout();
        document.value_mut().reflow_subtree(1000, 500, Direction::LTR);
    }

    let mut app = App {
        hovered,
        props,
        dom,
    };

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
        println!("{:#?}", &self.hovered.ids);
        
        let mut document = self.dom.document_mut();
        document.calculate_styles(); // calculate inner styles with new layout props
        document.value_mut().reflow_subtree(1000, 500, Direction::LTR); // recalculate yoga

        let mut list_builder = Drawer::new(
            &mut self.props,
            builder,

            _pipeline_id,
            _document_id,
        );

        list_builder.built_node(&mut document);
        // builder.print_display_list();
    }

    fn on_event(&mut self, event: winit::WindowEvent, api: &RenderApi, document_id: DocumentId) -> bool {
        let mut document = self.dom.document_mut();
        let mut txn = Transaction::new();

        let mut need_redraw = false;

        match event {
            winit::WindowEvent::CursorMoved { position, .. } => {
                let cursor = (position.x as f32, position.y as f32);

                self.hovered.reset();
                self.hovered.calculate_hover(&mut document, ((0., 0.), (1000., 500.)), cursor);
                need_redraw = true;
            },

            _ => {},
        }

        if !txn.is_empty() || need_redraw {
            txn.generate_frame();
            api.send_transaction(document_id, txn);
            
            return true;
        }

        false
    }
}
