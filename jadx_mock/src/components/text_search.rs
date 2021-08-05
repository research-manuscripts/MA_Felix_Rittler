use crate::elements::structured_item_box;
use crate::elements::CustomCheckBox;
use crate::elements::ProjectNodeDescription;
use orbtk::prelude::*;
use rust_decimal::prelude::ToPrimitive;

widget!(TextSearch {
    class_checkbox_selected: bool,
    method_checkbox_selected: bool,
    field_checkbox_selected: bool,
    code_checkbox_selected: bool
});

impl Template for TextSearch {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let item_count = 370;
        let lower_border = 1;
        let higher_border = 100;
        let pagination_text = format!(
            "Showing results {} to {} of {}",
            lower_border, higher_border, item_count
        );

        let definitions_container = Container::new().child(
            Stack::new()
                .orientation("horizontal")
                .spacing(7)
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .selected(("class_checkbox_selected", id))
                        .build(ctx),
                )
                .child(TextBlock::new().style("text").text("Class").build(ctx))
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .selected(("method_checkbox_selected", id))
                        .build(ctx),
                )
                .child(TextBlock::new().style("text").text("Method").build(ctx))
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .selected(("field_checkbox_selected", id))
                        .build(ctx),
                )
                .child(TextBlock::new().style("text").text("Field").build(ctx))
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .selected(("code_checkbox_selected", id))
                        .build(ctx),
                )
                .child(TextBlock::new().style("text").text("Code").build(ctx))
                .build(ctx),
        );

        let options_container = Container::new().child(
            Stack::new()
                .orientation("horizontal")
                .spacing(7)
                .child(CustomCheckBox::new().style("windows_checkbox").build(ctx))
                .child(TextBlock::new().style("text").text("Case insensitive").build(ctx))
                .build(ctx),
        );

        let width = self.width.expect("Width has to be set");
        let table_column_width = 0.5 * width;
        let table_grid_columns = format!("{}, {}", table_column_width, table_column_width);

        let definitions_container_width = (0.65 * width - 30.0) as i32;
        let options_container_width = (0.35 * width - 30.0) as i32;

        self.name("Preferences").child(
            Container::new()
                .margin((10, 10, 20, 10))
                .child(
                    Stack::new()
                        .orientation("vertical")
                        .spacing(20)
                        .child(
                            Stack::new()
                                .spacing(5)
                                .child(
                                    TextBlock::new()
                                        .style("text")
                                        .margin(0)
                                        .text("Search for text:")
                                        .build(ctx),
                                )
                                .child(TextBox::new().style("windows_textbox").build(ctx))
                                .build(ctx),
                        )
                        .child(
                            Stack::new()
                                .attach(Grid::column(0))
                                .spacing(20)
                                .orientation("horizontal")
                                .child(
                                    Container::new()
                                        .child(structured_item_box(
                                            ctx,
                                            definitions_container,
                                            "Search definitions of:",
                                            50,
                                            definitions_container_width,
                                        ))
                                        .build(ctx),
                                )
                                .child(
                                    Container::new()
                                        .child(structured_item_box(
                                            ctx,
                                            options_container,
                                            "Search options:",
                                            50,
                                            options_container_width,
                                        ))
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .child(
                            Container::new()
                                .style("rule")
                                .background("#ffffff")
                                .height(400.0)
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
