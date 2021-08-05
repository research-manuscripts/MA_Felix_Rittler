use orbtk::prelude::*;

widget!(
    /// The `ProjectNodeDescription` widget can be clicked by user. It's used to perform an action.
    ///
    /// **style:** `button`
    ProjectNodeDescription {
        /// Sets or shares the text property
        text: String,

        /// Sets or shares the image property
        image: Image
    }
);

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
