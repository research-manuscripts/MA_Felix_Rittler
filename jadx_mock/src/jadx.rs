use crate::components::EditorTabNavigationMock;
use crate::components::Preferences;
use crate::components::ProjectTreeWidget;
use crate::components::RenameDialogue;
use crate::components::TextSearch;
use crate::components::Toolbar;
use crate::components::TopMenu;
use crate::components::UsageSearch;
use crate::generator::constants::IconSet;
use crate::generator::*;
use orbtk::prelude::*;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::random;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
enum WindowType {
    Preferences,
    TextSearch,
    ClassSearch,
    UsageSearch,
    RenameDialogue,
    None
}

impl Distribution<WindowType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WindowType {
        match rng.gen_range(0..=5) {
            0 => WindowType::Preferences,
            1 => WindowType::TextSearch,
            2 => WindowType::ClassSearch,
            3 => WindowType::UsageSearch,
            4 => WindowType::RenameDialogue,
            _ => WindowType::None,
        }
    }
}

impl Default for WindowType {
    fn default() -> Self {
        return random();
    }
}

#[derive(AsAny, Default)]
struct JadxState {
    additional_window: WindowType,
}

impl State for JadxState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        match self.additional_window {
            WindowType::Preferences => {
                ctx.show_window(|ctx| {
                    Window::new()
                        .title("Preferences")
                        .style("popup_window")
                        .position((120.0, 120.0))
                        .size(860.0, 740.0)
                        .resizeable(true)
                        .child(Preferences::new().build(ctx))
                        .build(ctx)
                });
            }
            WindowType::TextSearch => {
                ctx.show_window(|ctx| {
                    let height = 740.0;
                    let width = 860.0;
                    Window::new()
                        .title("Text Search")
                        .style("popup_window")
                        .position((120.0, 120.0))
                        .size(width, height)
                        .resizeable(true)
                        .child(
                            TextSearch::new()
                                .width(width)
                                .height(height)
                                .code_checkbox_selected(true)
                                .build(ctx),
                        )
                        .build(ctx)
                });
            }
            WindowType::ClassSearch => {
                ctx.show_window(|ctx| {
                    let height = 740.0;
                    let width = 860.0;
                    Window::new()
                        .title("Text Search")
                        .style("popup_window")
                        .position((120.0, 120.0))
                        .size(width, height)
                        .resizeable(true)
                        .child(
                            TextSearch::new()
                                .width(width)
                                .height(height)
                                .class_checkbox_selected(true)
                                .build(ctx),
                        )
                        .build(ctx)
                });
            }
            WindowType::UsageSearch => {
                ctx.show_window(|ctx| {
                    let height = 450.0;
                    let width = 860.0;
                    Window::new()
                        .title("Usage Search")
                        .style("popup_window")
                        .position((120.0, 120.0))
                        .size(width, height)
                        .resizeable(true)
                        .child(UsageSearch::new().width(width).height(height).build(ctx))
                        .build(ctx)
                });
            }
            WindowType::RenameDialogue => {
                ctx.show_window(|ctx| {
                    Window::new()
                        .title("Rename")
                        .style("popup_window")
                        .position((120.0, 120.0))
                        .size(250, 150)
                        .resizeable(true)
                        .child(
                            RenameDialogue::new()
                                .entity_icon(select_icon(IconSet::AllIcons))
                                .entity_name(generate_name())
                                .build(ctx),
                        )
                        .build(ctx)
                });
            }
            WindowType::None => {},
        }
    }
}

widget!(Jadx<JadxState>);

impl Template for Jadx {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let width = self.width.expect("Width has to be set");
        let height = self.height.expect("Height has to be set");

        let tree_width = (0.4 * width).min(250.0);
        let editor_width = width - tree_width;
        let grid_layout = format!("{}, {}", tree_width, editor_width);

        let tabs = generate_editor_tabs();

        self.name("Jadx").child(
            Stack::new()
                .orientation("vertical")
                .child(TopMenu::new().build(ctx))
                .child(Container::new().style("rule").build(ctx))
                .child(Toolbar::new().build(ctx))
                .child(Container::new().style("rule").build(ctx))
                .child(
                    Grid::new()
                        .columns(grid_layout)
                        .margin((0, 1, 0, 0))
                        .v_align("top")
                        .h_align("start")
                        .child(
                            Container::new()
                                .attach(Grid::column(0))
                                .h_align("start")
                                .v_align("top")
                                .style("rule")
                                .width(tree_width)
                                .height(height)
                                .child(ProjectTreeWidget::new().build(ctx))
                                .build(ctx),
                        )
                        .child(
                            Container::new()
                                .attach(Grid::column(1))
                                .v_align("top")
                                .h_align("start")
                                .child(
                                    Container::new()
                                        .style("rule")
                                        .width(editor_width)
                                        .height(height)
                                        .h_align("start")
                                        .v_align("top")
                                        .child(
                                            Stack::new()
                                                .orientation("vertical")
                                                .child(EditorTabNavigationMock::new().items(tabs).build(ctx))
                                                .child(
                                                    ImageWidget::new()
                                                        .image(select_editor_screenshot())
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
                .build(ctx),
        )
    }
}
