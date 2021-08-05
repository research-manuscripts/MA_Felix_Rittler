use orbtk::prelude::*;

widget!(About);

impl Template for About {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let height = self.height.expect("Height of About has to be set.");

        let row1_height = if height > 170.0 { 170 } else { height as i32 - 40 };
        let row2_height = height as i32 - row1_height;
        let grid_row_layout = format!("{}, {}", row1_height, row2_height);

        self.name("Preferences").child(
            Grid::new()
                .rows(grid_row_layout)
                .child(
                    Stack::new()
                        .orientation("vertical")
                        .attach(Grid::row(0))
                        .spacing(20)
                        .margin((0, 20, 0, 0)).height(row1_height as f64)
                        .child(
                            Stack::new()
                                .orientation("horizontal")
                                .spacing(5)
                                .h_align("center")
                                .child(
                                    ImageWidget::new()
                                        .image("src/assets/jadx-logo-grey-background-48px.png")
                                        .build(ctx),
                                )
                                .child(
                                    TextBlock::new()
                                        .style("text")
                                        .text("jadx")
                                        .v_align("center")
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .child(
                            TextBlock::new()
                                .style("text")
                                .h_align("center")
                                .text("Dex to Java decompiler")
                                .build(ctx),
                        )
                        .child(
                            TextBlock::new()
                                .h_align("center")
                                .style("text")
                                .text("jadx version: 1.2.0")
                                .build(ctx),
                        )
                        .child(
                            Stack::new()
                                .orientation("vertical")
                                .h_align("center")
                                .spacing(5)
                                .child(
                                    TextBlock::new()
                                        .h_align("center")
                                        .style("text")
                                        .text("Java VM: OpenJDK 64-Bit Server VM")
                                        .build(ctx),
                                )
                                .child(
                                    TextBlock::new()
                                        .style("text")
                                        .h_align("center")
                                        .text("Java version: 15.0.1")
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .margin((0, 20, 0, 0))
                        .attach(Grid::row(1))
                        .v_align("end")
                        .style("windows_button")
                        .text("Close")
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
