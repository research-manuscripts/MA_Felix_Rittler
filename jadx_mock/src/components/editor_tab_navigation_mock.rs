use crate::elements::ProjectNodeDescription;
use orbtk::prelude::*;

widget!(EditorTabNavigationMock {
    /// Items of the tab navigation bar
    items: EditorTabItems
});

///
/// Templating the tab navigation bar.
/// Items are lined up horizontally consisting of ProjectNodeDescription + 'Close Tab' icon
/// Slightly different coloring for selected tab
impl Template for EditorTabNavigationMock {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let items_prop = self.items.as_ref().expect("Items have to be set.");
        let items: Vec<EditorTabItem>;
        match items_prop {
            PropertySource::Source(_) => items = vec![],
            PropertySource::KeySource(_, _) => items = vec![],
            PropertySource::Value(t) => items = t.items.clone(),
        }

        let items = items
            // iterate over all items
            .iter()
            // put every item into a container
            .map(|item: &EditorTabItem| {
                let container_theme = if item.selected {
                    "tab_navi_container_selected"
                } else {
                    "tab_navi_container"
                };

                let close_icon = if item.selected {
                    "src/assets/icons-16/cross.png"
                } else {
                    "src/assets/icons-16/grey_background/cross.png"
                };

                Container::new()
                    .style(container_theme)
                    .padding(5)
                    .child(
                        Stack::new()
                            .orientation("horizontal")
                            .spacing(10)
                            .child(
                                ProjectNodeDescription::new()
                                    .text(item.name.clone())
                                    .image(item.icon_path.clone())
                                    .build(ctx),
                            )
                            .child(
                                ImageWidget::new()
                                    .image(close_icon)
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx)
            })
            // create a stack that adds every container as a child
            .fold(
                Stack::new().orientation("horizontal"),
                |stack: Stack, el: Entity| stack.child(el),
            );

        self.name("EditorTabNavigationMock").child(
            Container::new()
                .style("rule")
                .background("#F0F0F0")
                .child(items.build(ctx))
                .build(ctx),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EditorTabItem {
    pub name: String,
    pub icon_path: String,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EditorTabItems {
    pub items: Vec<EditorTabItem>,
}

into_property_source!(EditorTabItems);

impl Default for EditorTabItems {
    fn default() -> Self {
        EditorTabItems { items: vec![] }
    }
}
