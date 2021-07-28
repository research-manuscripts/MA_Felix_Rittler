use crate::components::Preferences;
use crate::elements::ImageButton;
use orbtk::prelude::*;

#[derive(AsAny, Default)]
struct ToolbarState {
    show_window: bool,
}

impl ToolbarState {
    fn show_window(&mut self) {
        self.show_window = true;
    }
}

impl State for ToolbarState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if self.show_window {
            ctx.show_window(|ctx| {
                Window::new()
                    .title("Präferenzen")
                    .style("popup_window")
                    .position((120.0, 120.0))
                    .size(860.0, 740.0)
                    .resizeable(true)
                    .child(Preferences::new().build(ctx))
                    .build(ctx)
            });
            self.show_window = false;
        }
    }
}

widget!(Toolbar<ToolbarState>);

impl Template for Toolbar {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Toolbar").child(
            Stack::new()
                .orientation("vertical")
                .child(
                    Container::new()
                        .v_align("center")
                        .style("toolbar")
                        .height(30)
                        .child(
                            Stack::new()
                                .orientation("horizontal")
                                .v_align("center")
                                .spacing(0)
                                .child(
                                    ImageButton::new()
                                        .image("src/assets/icons-16/grey_background/folder.png")
                                        .v_align("center")
                                        .height(20)
                                        .build(ctx),
                                )
                                .child(
                                    ImageButton::new()
                                        .image("src/assets/icons-16/grey_background/folder_add.png")
                                        .v_align("center")
                                        .margin((0, 0, 5, 0))
                                        .build(ctx),
                                )
                                .child(Container::new().style("small_rule").build(ctx))
                                .child(
                                    ImageButton::new()
                                        .image(
                                            "src/assets/icons-16/grey_background/disk_multiple.png",
                                        )
                                        .v_align("center")
                                        .margin((5, 0, 0, 0))
                                        .build(ctx),
                                )
                                .child(
                                    ImageButton::new()
                                        .image(
                                            "src/assets/icons-16/grey_background/database_save.png",
                                        )
                                        .margin((0, 0, 5, 0))
                                        .v_align("center")
                                        .build(ctx),
                                )
                                .child(Container::new().style("small_rule").build(ctx))
                                .child(
                                    ImageButton::new()
                                        .image(
                                            "src/assets/icons-16/grey_background/sync.png",
                                        )
                                        .v_align("center")
                                        .margin((5, 0, 0, 0))
                                        .build(ctx),
                                )
                                .child(
                                    ImageButton::new()
                                        .image(
                                            "src/assets/icons-16/grey_background/empty_logical_package_obj.png",
                                        )
                                        .margin((0, 0, 5, 0))
                                        .v_align("center")
                                        .build(ctx),
                                )
                                .child(Container::new().style("small_rule").build(ctx))
                                .child(
                                    ImageButton::new()
                                        .image(
                                            "src/assets/icons-16/grey_background/wand.png",
                                        )
                                        .v_align("center")
                                        .margin((5, 0, 0, 0))
                                        .build(ctx),
                                )
                                .child(
                                    ImageButton::new()
                                        .image(
                                            "src/assets/icons-16/grey_background/magnifier.png",
                                        )
                                        .margin((0, 0, 5, 0))
                                        .v_align("center")
                                        .build(ctx),
                                )
                                .child(Container::new().style("small_rule").build(ctx))
                                .child(
                                    ImageButton::new()
                                        .image(
                                            "src/assets/icons-16/grey_background/icon_back.png",
                                        )
                                        .v_align("center")
                                        .margin((5, 0, 0, 0))
                                        .build(ctx),
                                )
                                .child(
                                    ImageButton::new()
                                        .image(
                                            "src/assets/icons-16/grey_background/icon_forward.png",
                                        )
                                        .margin((0, 0, 5, 0))
                                        .v_align("center")
                                        .build(ctx),
                                )
                                .child(Container::new().style("small_rule").build(ctx))
                                .child(
                                    ImageButton::new()
                                        .image(
                                            "src/assets/icons-16/grey_background/lock_edit.png",
                                        )
                                        .v_align("center")
                                        .margin((5, 0, 5, 0))
                                        .build(ctx),
                                )
                                .child(Container::new().style("small_rule").build(ctx))
                                .child(
                                    ImageButton::new()
                                        .image(
                                            "src/assets/icons-16/grey_background/report.png",
                                        )
                                        .v_align("center")
                                        .margin((5, 0, 5, 0))
                                        .build(ctx),
                                )
                                .child(Container::new().style("small_rule").build(ctx))
                                .child(
                                    ImageButton::new()
                                        .image(
                                            "src/assets/icons-16/grey_background/wrench.png",
                                        )
                                        .on_click(move |states, _| -> bool {
                                            let state = states.get_mut::<ToolbarState>(id);
                                            state.show_window();
                                            false
                                        })
                                        .v_align("center")
                                        .margin((5, 0, 5, 0))
                                        .build(ctx),
                                )
                                .child(Container::new().style("small_rule").build(ctx))
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
