use crate::components::{
    About, EditorTabNavigationMock, Preferences, ProjectTreeWidget, RenameDialogue, TextSearch, Toolbar,
    TopMenu, UsageSearch, PREFERENCES_WINDOW_HEIGHT, PREFERENCES_WINDOW_WIDTH, RENAME_DIALOGUE_HEIGHT,
    RENAME_DIALOGUE_WIDTH,
};
use crate::generator::constants::IconSet;
use crate::generator::*;
use orbtk::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowType {
    Preferences,
    TextSearch(Size),
    ClassSearch(Size),
    UsageSearch(Size),
    RenameDialogue,
    About(Size),
    None,
}

impl Default for WindowType {
    fn default() -> Self {
        return WindowType::None;
    }
}

into_property_source!(WindowType);

#[derive(AsAny, Default)]
struct JadxState {
    additional_window: WindowType,
    size: Size,
}

impl State for JadxState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        // read window to render from state and open it
        match self.additional_window {
            WindowType::Preferences => {
                let window_position = generate_window_position(
                    Size::new(PREFERENCES_WINDOW_WIDTH, PREFERENCES_WINDOW_HEIGHT),
                    self.size,
                );

                ctx.show_window(move |ctx| {
                    Window::new()
                        .title("Preferences")
                        .style("popup_window")
                        .position(window_position)
                        .size(PREFERENCES_WINDOW_WIDTH, PREFERENCES_WINDOW_HEIGHT)
                        .resizeable(true)
                        .child(Preferences::new().build(ctx))
                        .build(ctx)
                });
            }
            WindowType::TextSearch(size) => {
                let width = size.width();
                let height = size.height();

                let window_position = generate_window_position(size, self.size);

                ctx.show_window(move |ctx| {
                    Window::new()
                        .title("Text Search")
                        .style("popup_window")
                        .position(window_position)
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
            WindowType::ClassSearch(size) => {
                let width = size.width();
                let height = size.height();

                let window_position = generate_window_position(size, self.size);

                ctx.show_window(move |ctx| {
                    Window::new()
                        .title("Text Search")
                        .style("popup_window")
                        .position(window_position)
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
            WindowType::UsageSearch(size) => {
                let width = size.width();
                let height = size.height();

                let window_position = generate_window_position(size, self.size);

                ctx.show_window(move |ctx| {
                    Window::new()
                        .title("Usage Search")
                        .style("popup_window")
                        .position(window_position)
                        .size(width, height)
                        .resizeable(true)
                        .child(UsageSearch::new().width(width).height(height).build(ctx))
                        .build(ctx)
                });
            }
            WindowType::RenameDialogue => {
                let window_position = generate_window_position(
                    Size::new(RENAME_DIALOGUE_WIDTH, RENAME_DIALOGUE_HEIGHT),
                    self.size,
                );

                ctx.show_window(move |ctx| {
                    Window::new()
                        .title("Rename")
                        .style("popup_window")
                        .position(window_position)
                        .size(RENAME_DIALOGUE_WIDTH, RENAME_DIALOGUE_HEIGHT)
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
            WindowType::About(size) => {
                let width = size.width();
                let height = size.height();

                let window_position = generate_window_position(size, self.size);

                ctx.show_window(move |ctx| {
                    Window::new()
                        .title("About")
                        .style("popup_window")
                        .position(window_position)
                        .size(width, height)
                        .resizeable(true)
                        .child(About::new().height(height).build(ctx))
                        .build(ctx)
                });
            }
            WindowType::None => {}
        }
    }
}

widget!(Jadx<JadxState> {
    additional_window: WindowType,
    /// Width of the window. has to differ from "width" attribute because of the buggy layouting.
    /// "width" < 1500 might not work.
    /// This attribute can be smaller
    window_width: f64,
    /// Height of the window. has to differ from "height" attribute because of the buggy layouting.
    /// "height" < 1500 might not work.
    /// This attribute can be smaller
    window_height: f64
});

impl Template for Jadx {
    fn template(mut self, _id: Entity, ctx: &mut BuildContext) -> Self {

        // init additional_window attribute of state
        match self
            .additional_window
            .as_ref()
            .unwrap_or(&PropertySource::Value(WindowType::None))
        {
            PropertySource::Source(_) => self.state.additional_window = WindowType::None,
            PropertySource::KeySource(_, _) => self.state.additional_window = WindowType::None,
            PropertySource::Value(t) => {
                self.state.additional_window = t.clone();
            }
        }

        let width = self.width.expect("Width has to be set");
        let height = self.height.expect("Height has to be set");

        // init window_size attribute of state
        let mut window_size = Size::new(width, height);
        match self
            .window_width
            .as_ref()
            .unwrap_or(&PropertySource::Value(width))
        {
            PropertySource::Source(_) => {}
            PropertySource::KeySource(_, _) => {}
            PropertySource::Value(t) => {
                window_size.set_width(*t);
            }
        }
        match self
            .window_height
            .as_ref()
            .unwrap_or(&PropertySource::Value(height))
        {
            PropertySource::Source(_) => {}
            PropertySource::KeySource(_, _) => {}
            PropertySource::Value(t) => {
                window_size.set_height(*t);
            }
        }

        self.state.size = window_size;

        // template
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
