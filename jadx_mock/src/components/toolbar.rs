use crate::components::Preferences;
use crate::components::TextSearch;
use crate::elements::ImageButton;
use orbtk::prelude::*;

#[derive(AsAny, Default)]
struct ToolbarState {
    show_preferences: bool,
    show_text_search: bool,
    show_class_search: bool
}

impl ToolbarState {
    fn show_preferences(&mut self) {
        self.show_preferences = true;
    }

    fn show_text_search(&mut self) {
        self.show_text_search = true;
    }

    fn show_class_search(&mut self) {
        self.show_class_search = true;
    }
}

impl State for ToolbarState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if self.show_preferences {
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
            self.show_preferences = false;
        }
        if self.show_text_search {
            ctx.show_window(|ctx| {
                let height = 740.0;
                let width = 860.0;
                Window::new()
                    .title("Text Search")
                    .style("popup_window")
                    .position((120.0, 120.0))
                    .size(width, height)
                    .resizeable(true)
                    .child(TextSearch::new().width(width).height(height).code_checkbox_selected(true).build(ctx))
                    .build(ctx)
            });
            self.show_text_search = false;
        }
        if self.show_class_search {
            ctx.show_window(|ctx| {
                let height = 740.0;
                let width = 860.0;
                Window::new()
                    .title("Text Search")
                    .style("popup_window")
                    .position((120.0, 120.0))
                    .size(width, height)
                    .resizeable(true)
                    .child(TextSearch::new().width(width).height(height).class_checkbox_selected(true).build(ctx))
                    .build(ctx)
            });
            self.show_text_search = false;
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
                                        .on_click(move |states, _| -> bool {
                                            let state = states.get_mut::<ToolbarState>(id);
                                            state.show_text_search();
                                            false
                                        })
                                        .v_align("center")
                                        .margin((5, 0, 0, 0))
                                        .build(ctx),
                                )
                                .child(
                                    ImageButton::new()
                                        .image(
                                            "src/assets/icons-16/grey_background/magnifier.png",
                                        )
                                        .on_click(move |states, _| -> bool {
                                            let state = states.get_mut::<ToolbarState>(id);
                                            state.show_class_search();
                                            false
                                        })
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
                                            state.show_preferences();
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
