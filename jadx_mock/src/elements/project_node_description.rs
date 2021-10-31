use orbtk::prelude::*;

widget!(
    /// Description of a project node containing just icon and text
    ProjectNodeDescription {
        text: String,
        image: Image
    }
);

/// Templating the ProjectNodeDescription containing an icon and text
impl Template for ProjectNodeDescription {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("ProjectNodeDescription").child(
            Grid::new()
                .columns("20,*")
                .child(
                    ImageWidget::new()
                        .attach(Grid::column(0))
                        .v_align("center")
                        .h_align("start")
                        .image(("image", id))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .v_align("center")
                        .attach(Grid::column(1))
                        .text(id)
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
