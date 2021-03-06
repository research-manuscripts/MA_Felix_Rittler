use orbtk::prelude::*;
use orbtk::prelude::behaviors::*;
use orbtk::prelude::themes::*;

widget!(
    ///
    /// Custom implementation of the OrbTK Checkbox to adjust parameters originally not adjustable in OrbTK, e.g. height.
    /// Template: https://github.com/redox-os/orbtk/blob/c7a44eedb7785fae0cfa3e7de5a4eb231ddb6a77/orbtk_widgets/src/check_box.rs
    ///
    /// The `CheckBox` widget can be switch its selected state. It contains a selection box and a text.
    ///
    /// **style:** `check-box`
    CustomCheckBox: MouseHandler {
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

        /// Sets or shares the text property.
        text: String,

        /// Sets or share the font size property.
        font_size: f64,

        /// Sets or shares the font property.
        font: String,

        /// Sets or shares the icon property.
        icon: String,

        /// Sets or shares the icon brush property.
        icon_brush: Brush,

        /// Sets or share the icon font size property.
        icon_size: f64,

        /// Sets or shares the icon font property.
        icon_font: String,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the selected property.
        selected: bool,

        /// Indicates if the widget is hovered by the mouse cursor.
        hover: bool
    }
);

///
/// Templating custom checkbox
impl Template for CustomCheckBox {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("CheckBox")
            .style("check_box")
            .selected(false)
            .height(14.0).width(12.0)
            .background(colors::LYNCH_COLOR)
            .border_radius(0.0)
            .border_width(0.0)
            .border_brush("transparent")
            .padding((0.0, 0.0, 0.0, 0.0))
            .foreground(colors::LINK_WATER_COLOR)
            .text("")
            .font_size(orbtk_fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
            .icon(material_icons_font::MD_CHECK)
            .icon_font("MaterialIcons-Regular")
            .icon_size(orbtk_fonts::ICON_FONT_SIZE_12)
            .icon_brush(colors::LINK_WATER_COLOR)
            .pressed(false)
            .child(
                MouseBehavior::new()
                    .pressed(id)
                    .enabled(id)
                    .target(id.0)
                    .child(
                        SelectionBehavior::new()
                            .selected(id)
                            .enabled(id)
                            .target(id.0)
                            .child(
                                Stack::new()
                                    .orientation("horizontal")
                                    .spacing(8.0)
                                    .child(
                                        Container::new()
                                            .size(12.0, 12.0)
                                            .background(id)
                                            .border_radius(id)
                                            .border_width(id)
                                            .border_brush(id)
                                            .padding(id)
                                            .opacity(id)
                                            .child(
                                                FontIconBlock::new()
                                                    .v_align("center")
                                                    .h_align("center")
                                                    .icon(id)
                                                    .icon_brush(id)
                                                    .icon_size(id)
                                                    .icon_font(id)
                                                    .opacity(id)
                                                    .build(ctx),
                                            )
                                            .build(ctx),
                                    )
                                    .child(
                                        TextBlock::new()
                                            .v_align("center")
                                            .foreground(id)
                                            .text(id)
                                            .font_size(id)
                                            .font(id)
                                            .opacity(id)
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
