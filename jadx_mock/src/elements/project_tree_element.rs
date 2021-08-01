use orbtk::prelude::behaviors::MouseBehavior;
use orbtk::prelude::themes::orbtk_fonts;
use orbtk::prelude::*;

use crate::components::ProjectTreeNode;

#[derive(Default, AsAny)]
pub struct ProjectTreeElementState {}

fn count_children(nodes: Vec<ProjectTreeNode>) -> i32 {
    let mut count = nodes.len() as i32;
    for child_node in nodes {
        count += count_children(child_node.nodes);
    }

    return count;
}

impl ProjectTreeElementState {
    fn generate(&mut self, ctx: &mut Context) {
        let nodes = ctx
            .get_widget(ctx.entity())
            .get::<ProjectTreeNode>("tree")
            .clone()
            .nodes;

        if let Some(items_panel) = ctx.entity_of_child("children") {
            ctx.clear_children_of(items_panel);
            let mut count_children_of_neighbor_above = 0;
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

impl State for ProjectTreeElementState {
    fn init(&mut self, registry: &mut Registry, ctx: &mut Context) {
        self.generate(ctx);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.generate(ctx);
    }
}

widget!(
    /// The `TopMenuButton` widget can be clicked by user. It's used to perform an action.
    ///
    /// **style:** `button`
    ProjectTreeElement<ProjectTreeElementState> {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the foreground property.
        foreground: Brush,

        /// Sets or shares the image property
        image: Image,

        /// Sets or share the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the spacing between icon and text.
        spacing: f64,

        /// Indicates if the widget is hovered by the mouse cursor.
        hover: bool,

        /// Defines the margin around the inner border.
        container_margin: Thickness,

        text: String,

        tree: ProjectTreeNode
    }
);

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
                                    .columns("20, 20, *")
                                    .child(
                                        Container::new()
                                            .border_brush("#000000")
                                            .border_radius(0)
                                            .margin((5, 3, 5, 2))
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
                                        ImageWidget::new()
                                            .attach(Grid::column(1))
                                            .v_align("center")
                                            .h_align("start")
                                            .image(("image", id))
                                            .build(ctx),
                                    )
                                    .child(
                                        TextBlock::new()
                                            .style("text")
                                            .v_align("center")
                                            .attach(Grid::column(2))
                                            .text(id)
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
