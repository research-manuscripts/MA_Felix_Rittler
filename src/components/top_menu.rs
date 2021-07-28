use orbtk::prelude::*;

use crate::elements::TopMenuEntryButton;

#[derive(PartialEq, Copy, Clone)]
enum TopMenuAction {
    CloseMenu,
    OpenMenu(TopMenuType),
}

#[derive(PartialEq, Copy, Clone)]
enum TopMenuType {
    File,
    View,
    Navigation,
    Tools,
    Help,
}

#[derive(Default, AsAny, Copy, Clone)]
struct TopMenuState {
    action: Option<TopMenuAction>,
    opened_menu: Option<TopMenuType>,
    popup: Option<Entity>,
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

    fn is_menu_opened(self, menu: TopMenuType) -> bool {
        match self.opened_menu {
            Some(y) => y == menu,
            None => false,
        }
    }

    fn open_menu(&mut self, ctx: &mut Context, menu: TopMenuType) {
        // close menu thats been opened before
        if let Some(popup) = self.popup {
            self.opened_menu = Option::None;

            // TODO: Different position for overlay?
            ctx.remove_child_from_overlay(popup);
        }

        // open new menu
        let current_entity = ctx.entity();
        let build_context = &mut ctx.build_context();

        let popup;
        match menu {
            // creates a popup then attach it to the overlay
            TopMenuType::File => {
                popup = create_file_menu(current_entity, build_context);
            }
            TopMenuType::View => {
                popup = create_view_menu(current_entity, build_context);
            }
            TopMenuType::Navigation => {
                popup = create_navigation_menu(current_entity, build_context);
            }
            TopMenuType::Tools => {
                popup = create_tools_menu(current_entity, build_context);
            }
            TopMenuType::Help => {
                popup = create_help_menu(current_entity, build_context);
            }
        }
        build_context.append_child_to_overlay(popup);
        self.opened_menu = Option::Some(menu);
        self.popup = Some(popup);
    }
}

impl State for TopMenuState {
    fn init(&mut self, _: &mut Registry, _ctx: &mut Context) {
        self.opened_menu = Option::None;
        self.popup = None;
    }

    fn update(&mut self, _reg: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                // creates a popup then attach it to the overlay
                TopMenuAction::OpenMenu(t) => self.open_menu(ctx, t),
                // delete popup from widget tree.
                TopMenuAction::CloseMenu => {
                    if let Some(popup) = self.popup {
                        self.opened_menu = Option::None;
                        ctx.remove_child_from_overlay(popup);
                    }
                }
            }

            self.action = None;
        }
    }
}

widget!(TopMenu<TopMenuState>);

impl Template for TopMenu {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("TopMenu").h_align("start").child(
            Stack::new()
                .orientation("horizontal")
                .h_align("start")
                .child(
                    Button::new()
                        .style("top_menu_button")
                        .text("Datei")
                        .on_click(move |states, _| -> bool {
                            let state = states.get_mut::<TopMenuState>(id);
                            state.toggle_menu(TopMenuType::File)
                        })
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .style("top_menu_button")
                        .text("Anzeigen")
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
                        .text("Hilfe")
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
                                .text("Datei öffnen...")
                                .shortcut_text("Strg+O")
                                .image("src/assets/icons-16/grey_background/folder.png")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Datei hinzufügen...")
                                .image("src/assets/icons-16/grey_background/folder_add.png")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Neues Projekt")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Projekt speichern")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Projekt speichern als...")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Alles speichern")
                                .shortcut_text("Strg+S")
                                .image("src/assets/icons-16/grey_background/disk_multiple.png")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Als Gradle-Projekt speichern")
                                .shortcut_text("Strg+E")
                                .image("src/assets/icons-16/grey_background/database_save.png")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Aktuelle Projekte")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Präferenzen")
                                .shortcut_text("Strg+Umschalt+P")
                                .image("src/assets/icons-16/grey_background/wrench.png")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new()
                                .column_layout(column_layout)
                                .text("Beenden")
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
        .style("top_menu").margin((39,23,0,0))
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
                            TopMenuEntryButton::new().column_layout(column_layout)
                                .text("Codepaket erweitern")
                                .image("src/assets/icons-16/grey_background/empty_logical_package_obj.png")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new().column_layout(column_layout)
                                .text("Mit Editor synchronisieren")
                                .image("src/assets/icons-16/grey_background/sync.png")
                                .shortcut_text("Strg+T")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new().column_layout(column_layout)
                                .text("Speicherverbrauchsleiste anzeigen")
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
        .style("top_menu").margin((100,23,0,0))
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
                            TopMenuEntryButton::new().column_layout(column_layout)
                                .text("Textsuche")
                                .image("src/assets/icons-16/grey_background/wand.png")
                                .shortcut_text("Strg+Umschalt+F")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new().column_layout(column_layout)
                                .text("Klassen-Suche")
                                .image("src/assets/icons-16/grey_background/magnifier.png")
                                .shortcut_text("Strg+N")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new().column_layout(column_layout)
                                .text("Zurück")
                                .image("src/assets/icons-16/grey_background/icon_back.png")
                                .shortcut_text("Strg+Alt+Links")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new().column_layout(column_layout)
                                .text("Vorwärts")
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
        .style("top_menu").margin((170,23,0,0))
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
                            TopMenuEntryButton::new().column_layout(column_layout)
                                .text("Deobfuscation")
                                .image("src/assets/icons-16/grey_background/lock_edit.png")
                                .shortcut_text("Strg+Alt+D")
                                .build(ctx),
                        )
                        .child(
                            TopMenuEntryButton::new().column_layout(column_layout)
                                .text("Log-Anzeige")
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
        .style("top_menu").margin((211,23,0,0))
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
                            TopMenuEntryButton::new().column_layout(column_layout)
                                .text("Über")
                                .image("src/assets/icons-16/grey_background/jadx-logo.png")
                                .build(ctx),
                        )
                        .build(ctx),
                )
                .build(ctx),
        )
        .build(ctx)
}