use crate::elements::structured_item_box;
use crate::elements::CustomCheckBox;
use crate::elements::SmallNumericBox;
use crate::elements::CustomComboBox;
use orbtk::prelude::*;

widget!(Preferences);

impl Template for Preferences {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let decompile_container = Container::new().child(
            Grid::new()
                .columns("300, 50")
                .rows("50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50")
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Processing threads count")
                        .attach(Grid::column(0))
                        .attach(Grid::row(0))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Excluded packages")
                        .attach(Grid::column(0))
                        .attach(Grid::row(1))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Auto start background decompilation")
                        .attach(Grid::column(0))
                        .attach(Grid::row(2))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Show inconsistent code")
                        .attach(Grid::column(0))
                        .attach(Grid::row(3))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Escape unicode")
                        .attach(Grid::column(0))
                        .attach(Grid::row(4))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Replace constants")
                        .attach(Grid::column(0))
                        .attach(Grid::row(5))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Respect bytecode access modifiers")
                        .attach(Grid::column(0))
                        .attach(Grid::row(6))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Use import statements")
                        .attach(Grid::column(0))
                        .attach(Grid::row(7))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Inline anonymous classes")
                        .attach(Grid::column(0))
                        .attach(Grid::row(8))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("File system is case sensitive")
                        .attach(Grid::column(0))
                        .attach(Grid::row(9))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Fallback mode (simple dump)")
                        .attach(Grid::column(0))
                        .attach(Grid::row(10))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Don't decode resources")
                        .attach(Grid::column(0))
                        .attach(Grid::row(11))
                        .build(ctx),
                )
                .child(
                    NumericBox::new()
                        .style("preference_numeric_box")
                        .val(1)
                        .min(1)
                        .attach(Grid::column(1))
                        .attach(Grid::row(0))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(1))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(2))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(3))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(4))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(5))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(6))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(7))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(8))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(9))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(10))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(11))
                        .build(ctx),
                )
                .build(ctx),
        );
        let deobfuscate_container = Container::new().child(
            Grid::new()
                .columns("300, 80")
                .rows("30, 30, 30, 30, 30, 30")
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Enable deobfuscation")
                        .attach(Grid::column(0))
                        .attach(Grid::row(0))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Force rewrite deobfuscation map file")
                        .attach(Grid::column(0))
                        .attach(Grid::row(1))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Minimum name length")
                        .attach(Grid::column(0))
                        .attach(Grid::row(2))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Maximum name length")
                        .attach(Grid::column(0))
                        .attach(Grid::row(3))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Use source file name as class name alias")
                        .attach(Grid::column(0))
                        .attach(Grid::row(4))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Parse Kotlin metadata for class and package names")
                        .attach(Grid::column(0))
                        .attach(Grid::row(5))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(0))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(1))
                        .build(ctx),
                )
                .child(
                    SmallNumericBox::new()
                        .style("preference_numeric_box")
                        .val(1)
                        .min(1)
                        .attach(Grid::column(1))
                        .attach(Grid::row(2))
                        .build(ctx),
                )
                .child(
                    NumericBox::new()
                        .style("preference_numeric_box")
                        .val(1)
                        .min(1)
                        .attach(Grid::column(1))
                        .attach(Grid::row(3))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(4))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(5))
                        .build(ctx),
                )
                .build(ctx),
        );
        let rename_container = Container::new().child(
            Grid::new()
                .columns("350, 50")
                .rows("30, 30, 30")
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("System case sensitivity")
                        .attach(Grid::column(0))
                        .attach(Grid::row(0))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("To be valid identifier")
                        .attach(Grid::column(0))
                        .attach(Grid::row(1))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("To be printable")
                        .attach(Grid::column(0))
                        .attach(Grid::row(2))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(0))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(1))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(2))
                        .build(ctx),
                )
                .build(ctx),
        );
        let project_container = Container::new().child(
            Grid::new()
                .columns("350, 50")
                .rows("30")
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Auto save")
                        .attach(Grid::column(0))
                        .attach(Grid::row(0))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(0))
                        .build(ctx),
                )
                .build(ctx),
        );

        let themes = ["default", "idea", "eclipse", "vs", "dark", "monokai"];
        let editor_container = Container::new().child(
            Grid::new()
                .columns("280, 50")
                .rows("30, 30")
                .child(
                    TextBlock::new()
                        .style("text")
                        // TODO Make font changeable
                        .text("Editor font: Consolas plain 13")
                        .attach(Grid::column(0))
                        .attach(Grid::row(0))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Editor theme")
                        .attach(Grid::column(0))
                        .attach(Grid::row(1))
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .style("windows_button")
                        .text("Change")
                        .attach(Grid::column(1))
                        .attach(Grid::row(0))
                        .build(ctx),
                )
                .child(
                    CustomComboBox::new()
                        .style("preference_combo_box")
                        .count(6)
                        .attach(Grid::column(1))
                        .attach(Grid::row(1))
                        .items_builder(move |bc, index| {
                            TextBlock::new()
                                .v_align("center")
                                .text(themes[index])
                                .build(bc)
                        })
                        .selected_index(0)
                        .build(ctx),
                )
                .build(ctx),
        );

        let languages = ["English", "Español", "Deutsch"];
        let other_container = Container::new().child(
            Grid::new()
                .columns("280, 50")
                .rows("30, 30, 30, 30")
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Language")
                        .attach(Grid::column(0))
                        .attach(Grid::row(0))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Check for updates on startup")
                        .attach(Grid::column(0))
                        .attach(Grid::row(1))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Generate methods CFG graphs (in 'dot' format)")
                        .attach(Grid::column(0))
                        .attach(Grid::row(2))
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .style("text")
                        .text("Generate RAW CFG graphs")
                        .attach(Grid::column(0))
                        .attach(Grid::row(3))
                        .build(ctx),
                )
                .child(
                    CustomComboBox::new()
                        .style("preference_combo_box")
                        .count(3)
                        .attach(Grid::column(1))
                        .attach(Grid::row(0))
                        .items_builder(move |bc, index| {
                            TextBlock::new()
                                .v_align("center")
                                .text(languages[index])
                                .build(bc)
                        })
                        .selected_index(0)
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(1))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(2))
                        .build(ctx),
                )
                .child(
                    CustomCheckBox::new()
                        .style("windows_checkbox")
                        .attach(Grid::column(1))
                        .attach(Grid::row(3))
                        .build(ctx),
                )
                .build(ctx),
        );
        self.name("Preferences").child(
            Stack::new()
                .orientation("vertical")
                .spacing(20)
                .child(
                    Grid::new()
                        .columns("450, 400")
                        .child(
                            Stack::new()
                                .attach(Grid::column(0))
                                .orientation("vertical")
                                .child(
                                    Container::new()
                                        .margin((10, 20, 0, 0))
                                        .child(structured_item_box(
                                            ctx,
                                            deobfuscate_container,
                                            "Deobfuscation",
                                            200,
                                            400,
                                        ))
                                        .build(ctx),
                                )
                                .child(
                                    Container::new()
                                        .margin((10, 20, 0, 0))
                                        .child(structured_item_box(
                                            ctx,
                                            rename_container,
                                            "Rename",
                                            110,
                                            400,
                                        ))
                                        .build(ctx),
                                )
                                .child(
                                    Container::new()
                                        .margin((10, 20, 0, 0))
                                        .child(structured_item_box(
                                            ctx,
                                            project_container,
                                            "Project",
                                            50,
                                            400,
                                        ))
                                        .build(ctx),
                                )
                                .child(
                                    Container::new()
                                        .margin((10, 20, 0, 0))
                                        .child(structured_item_box(
                                            ctx,
                                            editor_container,
                                            "Editor",
                                            80,
                                            400,
                                        ))
                                        .build(ctx),
                                )
                                .child(
                                    Container::new()
                                        .margin((10, 20, 0, 0))
                                        .child(structured_item_box(
                                            ctx,
                                            other_container,
                                            "Other",
                                            140,
                                            400,
                                        ))
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .child(
                            Stack::new()
                                .attach(Grid::column(1))
                                .orientation("vertical")
                                .child(
                                    Container::new()
                                        .margin((0, 20, 0, 0))
                                        .child(structured_item_box(
                                            ctx,
                                            decompile_container,
                                            "Decompilation",
                                            600,
                                            370,
                                        ))
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .child(
                    Container::new()
                        .border_width(1)
                        .border_brush("#000000")
                        .border_radius(0)
                        .child(
                            Grid::new()
                                .columns("420, 420")
                                .h_align("start")
                                .width(1000)
                                .child(
                                    Stack::new()
                                        .orientation("horizontal")
                                        .h_align("start")
                                        .attach(Grid::column(0))
                                        .child(
                                            Button::new()
                                                .margin(10)
                                                .border_brush("#000000")
                                                .border_width(1)
                                                .border_radius(0)
                                                .style("windows_button")
                                                .text("Reset")
                                                .build(ctx),
                                        )
                                        .child(
                                            Button::new()
                                                .margin(10)
                                                .border_brush("#000000")
                                                .border_width(1)
                                                .border_radius(0)
                                                .style("windows_button")
                                                .text("Copy to clipboard")
                                                .build(ctx),
                                        )
                                        .build(ctx),
                                )
                                .child(
                                    Stack::new()
                                        .orientation("horizontal")
                                        .attach(Grid::column(1))
                                        .h_align("end")
                                        .child(
                                            Button::new()
                                                .margin(10)
                                                .border_brush("#000000")
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
                                                .border_brush("#000000")
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
                .build(ctx),
        )
    }
}
