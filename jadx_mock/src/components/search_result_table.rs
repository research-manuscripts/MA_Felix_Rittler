use crate::elements::ProjectNodeDescription;
use orbtk::prelude::*;

widget!(SearchResultTable { table_height: f64 });

impl Template for SearchResultTable {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let item_count = 370;
        let lower_border = 1;
        let higher_border = 100;
        let pagination_text = format!(
            "Showing results {} to {} of {}",
            lower_border, higher_border, item_count
        );

        let table_height_prop = self.table_height.as_ref().expect("Height of text search has to be set.");
        let table_height: f64;
        match table_height_prop {
            PropertySource::Source(_) => table_height = 0.0,
            PropertySource::KeySource(_, _) => table_height = 0.0,
            PropertySource::Value(t) => table_height = *t,
        }

        let width = self.width.expect("Width has to be set");
        let table_column_width = 0.5 * width;
        let table_grid_columns = format!("{}, {}", table_column_width, table_column_width);

        self.name("Preferences").child(
            Container::new()
                .child(
                    Stack::new()
                        .orientation("vertical")
                        .spacing(20)
                        .child(
                            Container::new()
                                .style("rule")
                                .background("#ffffff")
                                .height(table_height)
                                .child(
                                    Grid::new()
                                        .columns(table_grid_columns)
                                        .rows("30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30")
                                        .child(
                                            TextBlock::new()
                                                .style("text")
                                                .text("Node")
                                                .margin(5)
                                                .attach(Grid::column(0))
                                                .attach(Grid::row(0))
                                                .build(ctx),
                                        )
                                        .child(
                                            TextBlock::new()
                                                .style("text")
                                                .text("Code")
                                                .margin(5)
                                                .attach(Grid::column(1))
                                                .attach(Grid::row(0))
                                                .build(ctx),
                                        )
                                        .child(
                                            ProjectNodeDescription::new()
                                                .margin((5, 0, 10, 0))
                                                .attach(Grid::column(0))
                                                .attach(Grid::row(1))
                                                .text("Test")
                                                .image("src/assets/icons-16/class_obj.png")
                                                .build(ctx),
                                        )
                                        .child(
                                            ProjectNodeDescription::new()
                                                .margin((5, 0, 10, 0))
                                                .attach(Grid::column(0))
                                                .attach(Grid::row(2))
                                                .text("Test")
                                                .image("src/assets/icons-16/methpub_obj.png")
                                                .build(ctx),
                                        )
                                        .child(
                                            ProjectNodeDescription::new()
                                                .margin((5, 0, 10, 0))
                                                .attach(Grid::column(0))
                                                .attach(Grid::row(3))
                                                .text("Test")
                                                .image("src/assets/icons-16/methpri_obj.png")
                                                .build(ctx),
                                        )
                                        .child(
                                            TextBlock::new()
                                                .margin(5)
                                                .attach(Grid::column(1))
                                                .attach(Grid::row(1))
                                                .style("text")
                                                .v_align("center")
                                                .text("Log.e(f308TAG, Could not retrieve Resources field")
                                                .build(ctx),
                                        )
                                        .child(
                                            TextBlock::new()
                                                .margin(5)
                                                .attach(Grid::column(1))
                                                .attach(Grid::row(2))
                                                .style("text")
                                                .v_align("center")
                                                .text("Log.e(f308TAG, Could not retrieve Resources field")
                                                .build(ctx),
                                        )
                                        .child(
                                            TextBlock::new()
                                                .margin(5)
                                                .attach(Grid::column(1))
                                                .attach(Grid::row(3))
                                                .style("text")
                                                .v_align("center")
                                                .text("Log.e(f308TAG, Could not retrieve Resources field")
                                                .build(ctx),
                                        )
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .child(
                            Stack::new()
                                .orientation("horizontal")
                                .spacing(5)
                                .child(Button::new().style("wide_windows_button").text("<-").build(ctx))
                                .child(Button::new().text("->").style("wide_windows_button").build(ctx))
                                .child(
                                    TextBlock::new()
                                        .style("text")
                                        .v_align("center")
                                        .text(pagination_text)
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
