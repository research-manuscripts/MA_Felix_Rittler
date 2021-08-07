use crate::{
    elements::ProjectNodeDescription,
    // jadx::{EditorTabItem, EditorTabItems},
};
use orbtk::prelude::*;

type Items = EditorTabItems;

widget!(EditorTabNavigationMock { items: Items });

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
                Container::new()
                    .style("rule")
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
                                    .image("src/assets/icons-16/grey_background/cross.png")
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