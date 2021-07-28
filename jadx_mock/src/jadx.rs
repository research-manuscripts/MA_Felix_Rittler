use crate::components::Toolbar;
use crate::components::TopMenu;
use orbtk::prelude::*;

widget!(Jadx);

impl Template for Jadx {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Jadx").child(
            Stack::new()
                .orientation("vertical")
                .child(TopMenu::new().build(ctx))
                .child(Container::new().style("rule").build(ctx))
                .child(Toolbar::new().build(ctx))
                .child(Container::new().style("rule").build(ctx))
                .child(
                    Grid::new()
                        .columns("380, auto")
                        .v_align("top")
                        .h_align("start")
                        .child(
                            Container::new()
                                .h_align("start")
                                .v_align("top")
                                .attach(Grid::column(0))
                                .style("rule")
                                .width(360)
                                .height(480)
                                .build(ctx),
                        )
                        .child(
                            Container::new()
                                .attach(Grid::column(1))
                                .v_align("top")
                                .child(
                                    Container::new()
                                        .style("rule")
                                        .width(380)
                                        .height(480)
                                        .h_align("end")
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
