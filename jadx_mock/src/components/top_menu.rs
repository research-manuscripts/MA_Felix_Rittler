use orbtk::prelude::*;

use crate::components::About;
use crate::elements::TopMenuEntryButton;

#[derive(PartialEq, Copy, Clone)]
enum TopMenuAction {
    CloseMenu,
    OpenMenu(TopMenuType),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TopMenuType {
    File,
    View,
    Navigation,
    Tools,
    Help,
    None,
}

impl Default for TopMenuType {
    fn default() -> Self {
        return TopMenuType::None;
    }
}

into_property_source!(TopMenuType);

#[derive(Default, AsAny, Copy, Clone)]
struct TopMenuState {
    action: Option<TopMenuAction>,
    opened_menu: TopMenuType,
    popup: Option<Entity>,
    show_about: bool,
}

impl TopMenuState {
    fn toggle_menu(&mut self, menu: TopMenuType) -> bool {
        if self.is_menu_opened(menu) {
            self.action = Some(TopMenuAction::CloseMenu);
        } else {
            self.action = Some(TopMenuAction::OpenMenu(menu));
        }

        false
    }

    fn show_about(&mut self) {
        self.show_about = true;
    }

    fn is_menu_opened(self, menu: TopMenuType) -> bool {
        match menu {
            TopMenuType::File => true,
            TopMenuType::View => true,
            TopMenuType::Navigation => true,
            TopMenuType::Tools => true,
            TopMenuType::Help => true,
            TopMenuType::None => false,
        }
    }

    fn open_menu(&mut self, ctx: &mut Context, menu: TopMenuType) {
        // close menu thats been opened before
        if let Some(popup) = self.popup {
            self.opened_menu = TopMenuType::None;

            // TODO: Different position for overlay?
            ctx.remove_child_from_overlay(popup);
        }

        // open new menu
        let current_entity = ctx.entity();
        let build_context = &mut ctx.build_context();

        let popup: Option<Entity>;
        match menu {
            // creates a popup then attach it to the overlay
            TopMenuType::File => {
                popup = Some(create_file_menu(current_entity, build_context));
            }
            TopMenuType::View => {
                popup = Some(create_view_menu(current_entity, build_context));
            }
            TopMenuType::Navigation => {
                popup = Some(create_navigation_menu(current_entity, build_context));
            }
            TopMenuType::Tools => {
                popup = Some(create_tools_menu(current_entity, build_context));
            }
            TopMenuType::Help => {
                popup = Some(create_help_menu(current_entity, build_context));
            }
            TopMenuType::None => {
                popup = Option::None;
            }
        }
        if let Some(value) = popup {
            build_context.append_child_to_overlay(value);
            self.opened_menu = menu;
            self.popup = Some(value);
        }
    }
}

impl State for TopMenuState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.open_menu(ctx, self.opened_menu)
    }

    fn update(&mut self, _reg: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                // creates a popup then attach it to the overlay
                TopMenuAction::OpenMenu(t) => self.open_menu(ctx, t),
                // delete popup from widget tree.
                TopMenuAction::CloseMenu => {
                    if let Some(popup) = self.popup {
                        self.opened_menu = TopMenuType::None;
                        ctx.remove_child_from_overlay(popup);
                    }
                }
            }

            self.action = None;
        }

        if self.show_about {
            ctx.show_window(|ctx| {
                let height = 220;
                Window::new()
                    .title("About")
                    .style("popup_window")
                    .position((120.0, 120.0))
                    .size(250.0, height)
                    .resizeable(true)
                    .child(About::new().height(height).build(ctx))
                    .build(ctx)
            });
            self.show_about = false;
        }
    }
}

widget!(TopMenu<TopMenuState> {
    opened_menu: TopMenuType
});

impl Template for TopMenu {
    fn template(mut self, id: Entity, ctx: &mut BuildContext) -> Self {
        // init opened menu
        match self
            .opened_menu
            .as_ref()
            .unwrap_or(&PropertySource::Value(TopMenuType::None))
        {
            PropertySource::Source(_) => self.state.opened_menu = TopMenuType::None,
            PropertySource::KeySource(_, _) => self.state.opened_menu = TopMenuType::None,
            PropertySource::Value(t) => {
                self.state.opened_menu = t.clone();
            }
        }

        self.name("TopMenu").h_align("start").child(
            Stack::new()
                .orientation("horizontal")
                .h_align("start")
                .child(
                    Button::new()
                        .style("top_menu_button")
                        .text("File")
                        .on_click(move |states, _| -> bool {
                            let state = states.get_mut::<TopMenuState>(id);
                            state.toggle_menu(TopMenuType::File)
                        })
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .style("top_menu_button")
                        .text("View")
                        .min_size(0, 0)
                        .on_click(move |states, _| -> bool {
                            let state = states.get_mut::<TopMenuState>(id);
                            state.toggle_menu(TopMenuType::View)
                        })
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .text("Navigation")
                        .style("top_menu_button")
                        .on_click(move |states, _| -> bool {
                            let state = states.get_mut::<TopMenuState>(id);
                            state.toggle_menu(TopMenuType::Navigation)
                        })
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .style("top_menu_button")
                        .text("Tools")
                        .on_click(move |states, _| -> bool {
                            let state = states.get_mut::<TopMenuState>(id);
                            state.toggle_menu(TopMenuType::Tools)
                        })
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .style("top_menu_button")
                        .text("Help")
                        .on_click(move |states, _| -> bool {
                            let state = states.get_mut::<TopMenuState>(id);
                            state.toggle_menu(TopMenuType::Help)
                        })
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

fn create_file_menu(target: Entity, ctx: &mut BuildContext) -> Entity {
    let column_layout = "30, 160, 120";
    Popup::new()
        .style("top_menu")
        .margin((0, 23, 0, 0))
        .target(target)
        .open(true)
        .width(310.0)
        .height(320.0)
        .child(
            Container::new()
                .style("top_menu_container")
                .v_align("top")
                .child(
                    Stack::new()
                        .style("top_menu_container")
                        .orientation("vertical")
                        .v_align("top")
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Open file...")
                                .shortcut_text("Strg+O")
                                .image("src/assets/icons-16/grey_background/folder.png")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Add files...")
                                .image("src/assets/icons-16/grey_background/folder_add.png")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("New project")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Save project")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Save project as...")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Save all")
                                .shortcut_text("Strg+S")
                                .image("src/assets/icons-16/grey_background/disk_multiple.png")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Save as gradle project")
                                .shortcut_text("Strg+E")
                                .image("src/assets/icons-16/grey_background/database_save.png")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Recent projects")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Preferences")
                                .shortcut_text("Strg+Umschalt+P")
                                .image("src/assets/icons-16/grey_background/wrench.png")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Exit")
                                .image("src/assets/icons-16/grey_background/cross.png")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
        .build(ctx)
}

fn create_view_menu(target: Entity, ctx: &mut BuildContext) -> Entity {
    let column_layout = "30, 200, 120";
    Popup::new()
        .style("top_menu")
        .margin((30, 23, 0, 0))
        .target(target)
        .open(true)
        .width(350.0)
        .height(96.0)
        .child(
            Container::new()
                .style("top_menu_container")
                .child(
                    Stack::new()
                        .orientation("vertical")
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Show flatten packages")
                                .image("src/assets/icons-16/grey_background/empty_logical_package_obj.png")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Sync with editor")
                                .image("src/assets/icons-16/grey_background/sync.png")
                                .shortcut_text("Strg+T")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Show memory usage bar")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
        .build(ctx)
}

fn create_navigation_menu(target: Entity, ctx: &mut BuildContext) -> Entity {
    let column_layout = "30, 85, 110";
    Popup::new()
        .style("top_menu")
        .margin((67, 23, 0, 0))
        .target(target)
        .open(true)
        .width(250.0)
        .height(128.0)
        .child(
            Container::new()
                .style("top_menu_container")
                .child(
                    Stack::new()
                        .orientation("vertical")
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Text search")
                                .image("src/assets/icons-16/grey_background/wand.png")
                                .shortcut_text("Strg+Umschalt+F")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Class search")
                                .image("src/assets/icons-16/grey_background/magnifier.png")
                                .shortcut_text("Strg+N")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Back")
                                .image("src/assets/icons-16/grey_background/icon_back.png")
                                .shortcut_text("Strg+Alt+Links")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Forward")
                                .image("src/assets/icons-16/grey_background/icon_forward.png")
                                .shortcut_text("Strg+Alt+Rechts")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
        .build(ctx)
}

fn create_tools_menu(target: Entity, ctx: &mut BuildContext) -> Entity {
    let column_layout = "30, 90, 120";
    Popup::new()
        .style("top_menu")
        .margin((137, 23, 0, 0))
        .target(target)
        .open(true)
        .width(240.0)
        .height(64.0)
        .child(
            Container::new()
                .style("top_menu_container")
                .child(
                    Stack::new()
                        .orientation("vertical")
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Deobfuscation")
                                .image("src/assets/icons-16/grey_background/lock_edit.png")
                                .shortcut_text("Strg+Alt+D")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Log Viewer")
                                .image("src/assets/icons-16/grey_background/report.png")
                                .shortcut_text("Strg+Umschalt+L")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
        .build(ctx)
}

fn create_help_menu(target: Entity, ctx: &mut BuildContext) -> Entity {
    let column_layout = "30, 200, 120";
    Popup::new()
        .style("top_menu")
        .margin((178, 23, 0, 0))
        .target(target)
        .open(true)
        .width(80.0)
        .height(32.0)
        .child(
            Container::new()
                .style("top_menu_container")
                .child(
                    Stack::new()
                        .orientation("vertical")
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("About")
                                .on_click(move |states, _| -> bool {
                                    let state = states.get_mut::<TopMenuState>(target);
                                    state.show_about();
                                    false
                                })
                                .image("src/assets/icons-16/grey_background/jadx-logo.png")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
        .build(ctx)
}
