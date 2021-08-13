use crate::components::SearchResultTable;
use crate::elements::structured_item_box;
use crate::elements::CustomCheckBox;
use crate::mock_data_generator::fill_checkbox;
use crate::mock_data_generator::generate_name;
use crate::mock_data_generator::generate_search_results;
use orbtk::prelude::*;

widget!(TextSearch {
    class_checkbox_selected: bool,
    method_checkbox_selected: bool,
    field_checkbox_selected: bool,
    code_checkbox_selected: bool
});

impl Template for TextSearch {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let height = self.height.expect("Height of text search has to be set.");
        let table_height = height - 270.0;

        let margin_left = 10;
        let margin_top = 10;
        let margin_right = 20;
        let margin_bottom = 10;

        let width = self.width.expect("Width has to be set");
        let table_width = width - margin_left as f64 - margin_right as f64;
        let definitions_container_width = (0.65 * width - 30.0) as i32;
        let options_container_width = (0.35 * width - 30.0) as i32;

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
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .selected(fill_checkbox())
                        .build(ctx),
                )
                .child(TextBlock::new().style("text").text("Case insensitive").build(ctx))
                .build(ctx),
        );

        self.name("TextSearch").child(
            Container::new()
                .margin((margin_left, margin_top, margin_right, margin_bottom))
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
                                .child(TextBox::new().style("windows_textbox").text(generate_name()).build(ctx))
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
                            SearchResultTable::new()
                                .table_height(table_height)
                                .items(generate_search_results())
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
