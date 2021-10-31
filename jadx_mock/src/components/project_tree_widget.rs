use crate::{elements::ProjectTreeElement, generator::generate_project_tree};
use orbtk::prelude::*;

widget!(ProjectTreeWidget {});

///
/// Templating the project tree widget, which contains a ProjectTreeElement
impl Template for ProjectTreeWidget {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let tree = generate_project_tree();
        self.name("Test").child(
            Container::new()
                .margin((-10, 5, 5, 5))
                .child(
                    ProjectTreeElement::new()
                        .text("app-release.apk")
                        .image("src/assets/icons-16/grey_background/report.png")
                        .tree(tree)
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

/// Domain object of a node within the project tree
#[derive(AsAny, Debug, Clone, PartialEq)]
pub struct ProjectTreeNode {
    /// Icon of the node
    pub image: String,
    /// Name of the node
    pub text: String,
    /// Subnodes of the node
    pub nodes: Vec<ProjectTreeNode>,
}

impl Default for ProjectTreeNode {
    fn default() -> Self {
        ProjectTreeNode {
            image: "src/assets/icons-16/package_obj.png".to_string(),
            text: "Package".to_string(),
            nodes: vec![],
        }
    }
}

into_property_source!(ProjectTreeNode);
