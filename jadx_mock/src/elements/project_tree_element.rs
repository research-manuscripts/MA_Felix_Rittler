use orbtk::prelude::behaviors::MouseBehavior;
use orbtk::prelude::themes::orbtk_fonts;
use orbtk::prelude::*;

use crate::components::ProjectTreeNode;
use crate::elements::ProjectNodeDescription;

///
/// State of a ProjectTreeElement
#[derive(Default, AsAny)]
pub struct ProjectTreeElementState {}

///
/// Utility function to count all children of a ProjectTreeNode
fn count_children(nodes: Vec<ProjectTreeNode>) -> i32 {
    let mut count = nodes.len() as i32;
    for child_node in nodes {
        count += count_children(child_node.nodes);
    }

    return count;
}

///
/// Implementing state of a ProjectTreeElement.
/// Inits and renders the children elements
impl ProjectTreeElementState {
    fn generate(&mut self, ctx: &mut Context) {
        let nodes = ctx
            .get_widget(ctx.entity())
            .get::<ProjectTreeNode>("tree")
            .clone()
            .nodes;

        // Get Stack defined below in Template method from ProjectTreeElement
        if let Some(items_panel) = ctx.entity_of_child("children") {
            // remove all the old items
            ctx.clear_children_of(items_panel);
            let mut count_children_of_neighbor_above = 0;

            // Render all child elements and add them to the context
            for node in nodes {
                let margin_top = count_children_of_neighbor_above * 20;
                count_children_of_neighbor_above = count_children(node.nodes.clone());

                let child = ProjectTreeElement::new()
                    .style("body")
                    .v_align("center")
                    .margin((0, margin_top, 0, 0))
                    .text(node.text.clone())
                    .image(node.image.clone())
                    .tree(node)
                    .build(&mut ctx.build_context());

                let bctx = &mut ctx.build_context();
                bctx.append_child(items_panel, child);
            }
        }
    }
}

///
/// Implement trait state for ProjectTreeElementState
impl State for ProjectTreeElementState {
    fn init(&mut self, _registry: &mut Registry, ctx: &mut Context) {
        self.generate(ctx);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.generate(ctx);
    }
}

widget!(
    ///
    /// Element in the project tree
    /// Implements a custom Button
    ProjectTreeElement<ProjectTreeElementState> {
        background: Brush,
        border_radius: f64,
        border_width: Thickness,
        border_brush: Brush,
        padding: Thickness,
        foreground: Brush,
        image: Image,
        font_size: f64,
        font: String,
        pressed: bool,
        spacing: f64,
        hover: bool,
        container_margin: Thickness,
        text: String,
        tree: ProjectTreeNode
    }
);

///
/// Templating a ProjectTreeElement containing the "+" icon, the description with icon and name.
/// The Stack containing the child elements is filled from ProjectTreeElementState!
///
/// This is a custom button wrapping a ProjectNodeDescription
impl Template for ProjectTreeElement {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("ProjectTreeElement")
            .style("project_tree_element")
            .height(16.0)
            .min_width(64.0)
            .background("transparent")
            .border_radius(0.0)
            .border_width(0.0)
            .border_brush("transparent")
            .padding(0)
            .foreground("transparent")
            .image("")
            .font_size(orbtk_fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
            .pressed(false)
            .container_margin(0)
            .child(
                Stack::new()
                    .orientation("vertical")
                    .spacing(5)
                    .child(
                        MouseBehavior::new()
                            .pressed(id)
                            .enabled(id)
                            .target(id.0)
                            .child(
                                Grid::new()
                                    .columns("20, *")
                                    .child(
                                        Container::new()
                                            .border_brush("#000000")
                                            .border_radius(0)
                                            .margin((5, 3, 5, 2))
                                            .attach(Grid::column(0))
                                            .border_width(1)
                                            .child(
                                                TextBlock::new()
                                                    .v_align("center")
                                                    .h_align("center")
                                                    .style("text")
                                                    .text("+")
                                                    .build(ctx),
                                            )
                                            .build(ctx),
                                    )
                                    .child(
                                        ProjectNodeDescription::new()
                                            .attach(Grid::column(1))
                                            .text(id)
                                            .image(id)
                                            .build(ctx),
                                    )
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .child(
                        Grid::new()
                            .columns("20, *")
                            .child(
                                Stack::new()
                                    .attach(Grid::column(1))
                                    .spacing(5)
                                    .id("children")
                                    .orientation("vertical")
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
