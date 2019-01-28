use jss::convert::{WebrenderStyles, AppearanceWrapper, WebrenderBackground};
use jss::types::{PropertiesAppearance};
use jss::convert::transforms;

use common::{PropertiesCollection, RectBuilder};
use yoga::Layout;

use num_traits::cast::ToPrimitive;
use dom::node::DOMNodeId;
use dom::events::*;
use dom::traits::*;
use dom::setup::*;

use webrender::api::{
    LayoutPrimitiveInfo,
    DisplayListBuilder,
    PropertyBindingKey,
    ReferenceFrameKind,
    ComplexClipRegion,
    SpaceAndClipInfo,
    LayoutTransform,
    PropertyBinding,
    TransformStyle,
    MixBlendMode,
    RasterSpace,
    LayoutPoint,
    PipelineId,
    DocumentId,
    LayoutRect,
    LayoutSize,
    SpatialId,
    ClipMode,
    ClipId,
};

pub struct Drawer<'a, 'b> {
    pub builder_props: &'a mut PropertiesCollection<DOMNodeId<BasicEvent>>,
    pub builder: &'b mut DisplayListBuilder,

    pub spatial_ids: Vec<SpatialId>,
    pub clip_ids: Vec<ClipId>,

    pub pipeline_id: PipelineId,
    pub document_id: DocumentId,
}

impl <'a, 'b>Drawer<'a, 'b> {
    pub fn new(
        builder_props: &'a mut PropertiesCollection<DOMNodeId<BasicEvent>>,
        builder: &'b mut DisplayListBuilder,
        pipeline_id: PipelineId,
        document_id: DocumentId,
    ) -> Self {
        let root_space_and_clip = SpaceAndClipInfo::root_scroll(pipeline_id);
        let root_spatial_id = root_space_and_clip.spatial_id;
        let root_clip_id = root_space_and_clip.clip_id;

        Drawer {
            spatial_ids: vec![ root_spatial_id ],
            clip_ids: vec![ root_clip_id ],

            builder_props,
            builder,

            pipeline_id,
            document_id,
        }
    }

    fn pop_clip_id(&mut self) {
        self.clip_ids.pop().is_some();
    }

    fn pop_spatial_id(&mut self) {
        self.spatial_ids.pop().is_some();
    }

    pub fn built_node<'c>(&mut self, node: &mut DOMArenaRefMut<'c, BasicEvent>) {
        // Open Node Context

        let current_spatial_id = {
            let id = self.spatial_ids.last().unwrap();
            id.clone()
        };

        let current_clip_id = {
            let id = self.clip_ids.last().unwrap();
            id.clone()
        };

        // Get current layout positions & sizes
        let (dimensions, appearance, layout, context) = {
            let raw = node.raw.try_value();

            let dimensions = raw
                .and_then(|node| Some(node.layout_node.get_layout()))
                .unwrap_or(Layout::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0));

            // @TODO: need handle as link for set in wrapper for convert without copy
            let appearance = raw
                .and_then(|node| Some(node.styles.computed.appearance.clone()))
                .unwrap_or(PropertiesAppearance::default());

            // @TODO: need handle as link for set in wrapper for convert without copy
            let layout = raw.and_then(|node| Some(node.styles.computed.layout.clone())).unwrap_or(vec![]);

            let context = raw.and_then(|node| Some(node.styles.context.clone())).unwrap_or_default();
            (dimensions, appearance, layout, context)
        };

        // @TODO: need handle as link for set in wrapper for convert without copy
        let properties = WebrenderStyles::from(AppearanceWrapper {
            appearance: &appearance,
            layout: &layout,
            builder: &mut self.builder,
            context,
        });

        let container_size = (dimensions.width(), dimensions.height());
        let node_id = node.id();

        // Push clip for start of transforms display context
        let id_index = node_id.to_u64().unwrap();
        
        // (node_id, id_index)
        let frame_primitive = (dimensions.left(), dimensions.top()).by(dimensions.width(), dimensions.height());
        let transform = transforms::multiply(properties.transforms.transforms, container_size).unwrap_or_default();
        let binding_key = PropertyBindingKey::new(id_index);
        
        // Add dynamic binding property
        self.builder_props.insert(node_id, binding_key);
        
        // Generate push transform frame for transform area
        let property_transform = PropertyBinding::Binding(binding_key, transform);
        let transformed_frame_spatial_id = self.builder.push_reference_frame(
            &frame_primitive,
            current_spatial_id,
            TransformStyle::Flat,
            property_transform,
            ReferenceFrameKind::Transform,
        );

        // Push spatial to ids stack
        self.spatial_ids.push(transformed_frame_spatial_id);
    
        let stacking_primitive = LayoutPrimitiveInfo::new((0., 0.).by(dimensions.width(), dimensions.height()));

        // Declare stacking context of content
        self.builder.push_stacking_context(
            &stacking_primitive,
            transformed_frame_spatial_id,
            None,
            TransformStyle::Flat,
            MixBlendMode::Normal,
            &Vec::new(),
            RasterSpace::Screen,
            true,
        );

        // Define content (inside stacking_context) bounds
        let content_bounds = (0., 0.).by(dimensions.width(), dimensions.height());
        let content_primitive = LayoutPrimitiveInfo::new(content_bounds.clone());

        // Content clip for border-radius
        let rounded_corners_clip = ComplexClipRegion::new(
            content_bounds.clone(),
            properties.borders.border_radius.clone(),
            ClipMode::Clip,
        );

        let node_space_and_clip = SpaceAndClipInfo {
            spatial_id: transformed_frame_spatial_id,
            clip_id: current_clip_id,
        };

        let rounded_corners_clip_id = self.builder.define_clip(
            &node_space_and_clip,
            content_bounds.clone(),
            vec![rounded_corners_clip],
            None
        );

        // Push clip to ids stack
        self.clip_ids.push(rounded_corners_clip_id);

        // Push background
        let node_space_and_clip_content = SpaceAndClipInfo {
            spatial_id: transformed_frame_spatial_id,
            clip_id: rounded_corners_clip_id,
        };

        match properties.background {
            WebrenderBackground::Color(color) => {
                self.builder.push_rect(
                    &content_primitive,
                    &node_space_and_clip_content,
                    color
                );
            }

            WebrenderBackground::Gradient(gradient) => {
                self.builder.push_gradient(
                    &content_primitive,
                    &node_space_and_clip_content,
                    gradient,
                    LayoutSize::new(0., 0.),
                    LayoutSize::new(0., 0.),
                );
            }
        }

        // Push borders
        self.builder.push_border(
            &content_primitive,
            &node_space_and_clip_content,

            properties.borders.widths,
            properties.borders.details
        );

        // Iter childrens for draw
        let mut next_child_id = node.first_child_id();
        while let Some(child_id) = next_child_id {
            {
                let mut child_ref = node.get_mut(child_id);
                self.built_node(&mut child_ref);
            }

            next_child_id = node.get(child_id).next_sibling_id();
        }

        // CLOSE AREA

        // Close clip border-radius zone
        
        // Close stacking context
        self.pop_clip_id();
        self.builder.pop_stacking_context();
        
        // Close transforms clip zone
        self.builder.pop_reference_frame();
        self.pop_spatial_id();
    }
}