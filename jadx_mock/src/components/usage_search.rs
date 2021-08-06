use crate::components::SearchResultTable;
use crate::elements::ProjectNodeDescription;
use orbtk::prelude::*;

widget!(UsageSearch);

impl Template for UsageSearch {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let height = self.height.expect("Height of usage search has to be set.");
        let table_height = height - 170.0;

        let margin_left = 10;
        let margin_top = 10;
        let margin_right = 20;
        let margin_bottom = 10;

        let width = self.width.expect("Width has to be set");
        let table_width = width - margin_left as f64 - margin_right as f64;

        self.name("UsageSearch").child(
            Container::new()
                .margin((margin_left, margin_top, margin_right, margin_bottom))
                .child(
                    Stack::new()
                        .orientation("vertical")
                        .spacing(20)
                        .child(
                            Stack::new()
                                .spacing(5)
                                .orientation("horizontal")
                                .child(
                                    TextBlock::new()
                                        .style("text")
                                        .margin(0)
                                        .text("Usage for:")
                                        .build(ctx),
                                )
                                .child(
                                    ProjectNodeDescription::new()
                                        .text("p008io.noties.markwon.AbstractMarkwonPlugin")
                                        .image("src/assets/icons-16/grey_background/class_obj.png")
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .child(
                            SearchResultTable::new()
                                .table_height(table_height)
                                .width(table_width)
                                .build(ctx),
                        )
                        .child(
                            Stack::new()
                                .orientation("horizontal")
                                .h_align("end")
                                .child(
                                    Button::new()
                                        .margin(10)
                                        .border_width(1)
                                        .border_radius(0)
                                        .style("windows_button")
                                        .text("Save")
                                        .build(ctx),
                                )
                                .child(
                                    Button::new()
                                        .margin(10)
                                        .style("windows_button")
                                        .border_width(5)
                                        .border_radius(0)
                                        .text("Cancel")
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
