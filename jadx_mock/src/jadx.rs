use crate::components::EditorTabItem;
use crate::components::EditorTabItems;
use crate::components::EditorTabNavigationMock;
use crate::components::ProjectTreeWidget;
use crate::components::Toolbar;
use crate::components::TopMenu;
use orbtk::prelude::*;

widget!(Jadx);

const DEMO_CONTENT: &str =
    "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor
invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam
et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est
Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed
diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam
voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd
gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor
sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore
et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo
dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum
dolor sit amet.
Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat,
vel illum dolore eu feugiat nulla facilisis at vero eros et accumsan et iusto odio
dignissim qui blandit praesent luptatum zzril delenit augue duis dolore te feugait
nulla facilisi. Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam
nonummy nibh euismod tincidunt ut laoreet dolore magna aliquam erat volutpat.
Ut wisi enim ad minim veniam, quis nostrud exerci tation ullamcorper suscipit lobortis nisl
ut aliquip ex ea commodo consequat. Duis autem vel eum iriure dolor in hendrerit in
vulputate velit esse molestie consequat, vel illum dolore eu feugiat nulla facilisis
at vero eros et accumsan et iusto odio dignissim qui blandit praesent luptatum zzril
delenit augue duis dolore te feugait nulla facilisi.
Nam liber tempor cum soluta nobis eleifend option congue nihil imperdiet doming id quod
mazim placerat facer possim assum. Lorem ipsum dolor sit amet, consectetuer adipiscing
elit, sed diam nonummy nibh euismod tincidunt ut laoreet dolore magna aliquam erat
volutpat. Ut wisi enim ad minim veniam, quis nostrud exerci tation ullamcorper suscipit
lobortis nisl ut aliquip ex ea commodo consequat.
Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat,
vel illum dolore eu feugiat nulla facilisis.

At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no
sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur
sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam
erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita
kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor
sit amet, consetetur sadipscing elitr, At accusam aliquyam diam diam dolore dolores duo
eirmod eos erat, et nonumy sed tempor et et invidunt justo labore Stet clita ea et gubergren,
kasd magna no rebum. sanctus sea sed takimata ut vero voluptua. est Lorem ipsum dolor sit
amet. Lorem ipsum dolor sit amet, consetetur";

impl Template for Jadx {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let width = self.width.expect("Width has to be set");
        let height = self.height.expect("Height has to be set");

        let tree_width = 0.4 * width;
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
                                        .height(730)
                                        .h_align("start")
                                        .child(
                                            Stack::new()
                                                .orientation("vertical")
                                                .child(EditorTabNavigationMock::new().items(tabs).build(ctx))
                                                .child(
                                                    TextBlock::new()
                                                        .style("text")
                                                        .text(DEMO_CONTENT)
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

fn generate_editor_tabs() -> EditorTabItems {
    let item1 = EditorTabItem {
        name: "Test".to_string(),
        icon_path: "src/assets/icons-16/grey_background/class_obj.png".to_string(),
    };
    let item2 = EditorTabItem {
        name: "Test2".to_string(),
        icon_path: "src/assets/icons-16/grey_background/class_obj.png".to_string(),
    };
    let item3 = EditorTabItem {
        name: "Test3".to_string(),
        icon_path: "src/assets/icons-16/grey_background/class_obj.png".to_string(),
    };
    let item4 = EditorTabItem {
        name: "Test4".to_string(),
        icon_path: "src/assets/icons-16/grey_background/class_obj.png".to_string(),
    };
    let item5 = EditorTabItem {
        name: "Test5".to_string(),
        icon_path: "src/assets/icons-16/grey_background/class_obj.png".to_string(),
    };
    let item6 = EditorTabItem {
        name: "Test6".to_string(),
        icon_path: "src/assets/icons-16/grey_background/class_obj.png".to_string(),
    };
    EditorTabItems {
        items: vec![item1, item2, item3, item4, item5, item6],
    }
}
