use orbtk::prelude::behaviors::MouseBehavior;
use orbtk::prelude::themes::{colors, orbtk_fonts};
use orbtk::prelude::*;

widget!(
    ///
    /// Custom implementation of the Button from OrbTK to support buttons with icon + text.
    TopMenuEntryButton: MouseHandler {
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

        /// Sets or shares the text property.
        text: String,

        shortcut_text: String,

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

        column_layout: Blocks
    }
);

impl Template for TopMenuEntryButton {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let image;
        if self.image.is_some() {
            image = ImageWidget::new()
                .v_align("center")
                .image(("image", id))
                .attach(Grid::column(0))
                .build(ctx);
        } else {
            image = Container::new().width(16).height(16).build(ctx);
        }
        self.name("TopMenuButton")
            .style("top_menu_entry_button")
            .height(36.0)
            .min_width(64.0)
            .background(colors::LYNCH_COLOR)
            .border_radius(4.0)
            .border_width(0.0)
            .border_brush("transparent")
            .padding((16.0, 16.0, 16.0, 16.0))
            .foreground(colors::LINK_WATER_COLOR)
            .text("")
            .shortcut_text("")
            .image("src/assets/icons-16/folder_add.png")
            .font_size(orbtk_fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
            .pressed(false)
            .spacing(8.0)
            .container_margin(0)
            .column_layout("")
            .child(
                Container::new()
                    .padding(id)
                    .background(id)
                    .border_radius(id)
                    .border_width(id)
                    .border_brush(id)
                    .child(
                        MouseBehavior::new()
                            .pressed(id)
                            .enabled(id)
                            .target(id.0)
                            .child(
                                Container::new()
                                    .child(
                                        Grid::new()
                                            .columns(("column_layout", id))
                                            .h_align("start")
                                            .child(image)
                                            .child(
                                                TextBlock::new()
                                                    .v_align("center")
                                                    .foreground(id)
                                                    .text(id)
                                                    .font_size(id)
                                                    .font(id)
                                                    .attach(Grid::column(1))
                                                    .build(ctx),
                                            )
                                            .child(
                                                TextBlock::new()
                                                    .v_align("center")
                                                    .foreground(id)
                                                    .text(("shortcut_text", id))
                                                    .font_size(id)
                                                    .font(id)
                                                    .attach(Grid::column(2))
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
