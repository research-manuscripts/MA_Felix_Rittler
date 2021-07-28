use orbtk::prelude::*;

pub fn structured_item_box(
    ctx: &mut BuildContext,
    child: Container,
    heading: &str,
    height: i32,
    width: i32,
) -> Entity {
    Container::new()
        .style("rule")
        .height(height)
        .width(width)
        .child(
            Container::new()
                .margin((10, -9, 0, 0))
                .child(
                    Stack::new()
                        .orientation("vertical")
                        .child(
                            Container::new()
                                .background("#F0F0F0")
                                .padding((5, 0, 5, 0))
                                .width(95)
                                .margin((0, 0, 0, 10))
                                .height(16)
                                .child(
                                    TextBlock::new()
                                        .text(heading)
                                        .style("box_heading")
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .child(child.build(ctx))
                        .build(ctx),
                )
                .build(ctx),
        )
        .build(ctx)
}
