
use orbtk::prelude::behaviors::MouseBehavior;
use orbtk::prelude::themes::{orbtk_fonts};
use orbtk::prelude::*;

widget!(
    /// Self-implemented button that is just an image
    ImageButton: MouseHandler {
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
        container_margin: Thickness
    }
);

impl Template for ImageButton {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("ImageButton")
            .style("image_button")
            .height(36.0)
            .min_width(64.0)
            .background("transparent")
            .border_radius(0.0)
            .border_width(0.0)
            .border_brush("transparent")
            .padding(0)
            .foreground("transparent")
            .image("")
            .font_size(orbtk_fonts::FONT_SIZE_12)
            .font("Roboto-Regular")
            .pressed(false)
            .container_margin(0)
            .child(
                MouseBehavior::new()
                    .pressed(id)
                    .enabled(id)
                    .target(id.0)
                    .child(
                        Container::new()
                            .background(id)
                            .border_radius(id)
                            .border_width(id)
                            .border_brush(id)
                            .padding(id)
                            .child(
                                ImageWidget::new()
                                    .v_align("center")
                                    .image(("image", id))
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
