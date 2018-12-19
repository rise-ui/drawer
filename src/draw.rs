use jss::convert::{WebrenderStyles, AppearanceWrapper, WebrenderBackground};
use jss::types::{PropertiesAppearance};

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
    ComplexClipRegion,
    LayoutTransform,
    TransformStyle,
    MixBlendMode,
    RasterSpace,
    LayoutPoint,
    LayoutRect,
    LayoutSize,
    ClipMode,
};

pub fn render_node<'a>(
    builder_props: &mut PropertiesCollection<DOMNodeId<BasicEvent>>,
    builder: &mut DisplayListBuilder,

    node: &mut DOMArenaRefMut<'a, BasicEvent>,
) {
    // Open Stacking Context

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
        builder,
        context,
    });

    let container_size = (dimensions.width(), dimensions.height());

    let node_id = node.id();

    // Push clip for start of transforms display context

    let mut transform_frame_exists = false;
    if let Some(id_index) = node_id.to_u64() {
        let frame_primitive = LayoutPrimitiveInfo::new(
            (dimensions.left(), dimensions.top()).by(dimensions.width(), dimensions.height()),
        );

        transform_frame_exists = properties.transforms.push_builder(&frame_primitive, (node_id, id_index), builder_props, builder);
    }

    let stacking_primitive = {
        let left = if transform_frame_exists { 0.0 } else { dimensions.left() };
        let top = if transform_frame_exists { 0.0 } else { dimensions.top() };

        LayoutPrimitiveInfo::new((left, top).by(dimensions.width(), dimensions.height()))
    };

    // Declare stacking context of content
    builder.push_stacking_context(
        &stacking_primitive,
        None,
        TransformStyle::Flat,
        MixBlendMode::Normal,
        &Vec::new(),
        RasterSpace::Screen,
    );

    // Define content (inside stacking_context) bounds
    let content_bounds = (0., 0.).by(dimensions.width(), dimensions.height());
    let content_primitive = LayoutPrimitiveInfo::new(content_bounds.clone());

    // Content clip for border-radius
    let rounded_corners_clip = ComplexClipRegion::new(
        content_bounds.clone(),
        properties.borders.border_radius.clone(), 
        ClipMode::Clip
    );

    let rounded_corners_clip_id = builder.define_clip(
        content_bounds.clone(),
        vec![rounded_corners_clip],
        None
    );

    // Push clip of border-radius
    builder.push_clip_id(rounded_corners_clip_id);

    // Push background
    match properties.background {
        WebrenderBackground::Color(color) => {
            builder.push_rect(&content_primitive, color);
        },

        WebrenderBackground::Gradient(gradient) => {
            builder.push_gradient(
                &content_primitive,
                gradient,
                LayoutSize::new(0., 0.),
                LayoutSize::new(0., 0.)
            );
        },
    }

    // Push borders
    builder.push_border(
        &content_primitive,
        properties.borders.widths,
        properties.borders.details
    );

    // Iter childrens for draw
    let mut next_child_id = node.first_child_id();
    while let Some(child_id) = next_child_id {
        {
            let mut child_ref = node.get_mut(child_id);
            render_node(builder_props, builder, &mut child_ref);
        }

        next_child_id = node.get(child_id).next_sibling_id();
    }

    // CLOSE AREA
    
    // Close clip border-radius zone
    builder.pop_clip_id();
    // Close stacking context
    builder.pop_stacking_context();
    // Close transforms clip zone
    if transform_frame_exists {
        builder.pop_clip_id();
        builder.pop_reference_frame();
    }
}
