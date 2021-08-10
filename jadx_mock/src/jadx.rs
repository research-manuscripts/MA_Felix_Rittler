use std::cmp::min;

use crate::components::EditorTabNavigationMock;
use crate::components::ProjectTreeWidget;
use crate::components::Toolbar;
use crate::components::TopMenu;
use crate::mock_data_generator::*;
use orbtk::prelude::*;

widget!(Jadx);

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
                                // .margin((20, 0, 0, 0))
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
                                                    ImageWidget::new().image("src/assets/editor_screenshots/enum.png").build(ctx)
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
