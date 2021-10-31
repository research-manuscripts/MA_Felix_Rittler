use crate::elements::ProjectNodeDescription;
use orbtk::prelude::*;

widget!(RenameDialogue {
    /// Icon of entity to rename
    entity_icon: Image,
    /// Name of entity to rename
    entity_name: String
});

/// Width of rename dialogue
pub const RENAME_DIALOGUE_WIDTH: f64 = 250.0;
/// Height of rename dialogue
pub const RENAME_DIALOGUE_HEIGHT: f64 = 150.0;

/// Templating rename dialogue consisting of entity to rename, a textbox
/// to enter a new name and buttons to confirm and cancel
impl Template for RenameDialogue {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("RenameDialogue").child(
            Container::new()
                .margin((10, 10, 20, 10))
                .child(
                    Stack::new()
                        .orientation("vertical")
                        .spacing(20)
                        .child(
                            Stack::new()
                                .orientation("horizontal")
                                .spacing(5)
                                .child(TextBlock::new().style("text").margin(0).text("Rename").build(ctx))
                                .child(
                                    ProjectNodeDescription::new()
                                        .style("text")
                                        .margin(0)
                                        .text(("entity_name", id))
                                        .image(("entity_icon", id))
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .child(TextBox::new().style("windows_textbox").build(ctx))
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
                                        .text("Rename")
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
