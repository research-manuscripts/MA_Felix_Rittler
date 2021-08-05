use crate::elements::ProjectTreeElement;
use orbtk::prelude::*;

widget!(ProjectTreeWidget {});

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

fn generate_project_tree() -> ProjectTreeNode {
    let node111 = ProjectTreeNode {
        image: "src/assets/icons-16/int_obj.png".to_string(),
        text: "INotificationSideChannel".to_string(),
        nodes: vec![],
    };

    let node112 = ProjectTreeNode {
        image: "src/assets/icons-16/class_obj.png".to_string(),
        text: "RemoteActionCompatParcelizer".to_string(),
        nodes: vec![],
    };

    let node11 = ProjectTreeNode {
        image: "src/assets/icons-16/package_obj.png".to_string(),
        text: "android.support.v4.app".to_string(),
        nodes: vec![node111, node112],
    };

    let node12 = ProjectTreeNode {
        image: "src/assets/icons-16/package_obj.png".to_string(),
        text: "android.support.v4.graphics.drawable".to_string(),
        nodes: vec![],
    };

    let node13 = ProjectTreeNode {
        image: "src/assets/icons-16/package_obj.png".to_string(),
        text: "android.support.v4.media".to_string(),
        nodes: vec![],
    };

    let parent_node1 = ProjectTreeNode {
        image: "src/assets/icons-16/packagefolder_obj.png".to_string(),
        text: "Source code".to_string(),
        nodes: vec![node11, node12, node13],
    };

    let node21 = ProjectTreeNode {
        image: "src/assets/icons-16/folder.png".to_string(),
        text: "assets".to_string(),
        nodes: vec![],
    };

    let node22 = ProjectTreeNode {
        image: "src/assets/icons-16/folder.png".to_string(),
        text: "kotlin".to_string(),
        nodes: vec![],
    };

    let node23 = ProjectTreeNode {
        image: "src/assets/icons-16/folder.png".to_string(),
        text: "META-INF".to_string(),
        nodes: vec![],
    };

    let node24 = ProjectTreeNode {
        image: "src/assets/icons-16/template_obj.png".to_string(),
        text: "AndroidManifest.xml".to_string(),
        nodes: vec![],
    };

    let parent_node2 = ProjectTreeNode {
        image: "src/assets/icons-16/packagefolder_obj.png".to_string(),
        text: "Resources".to_string(),
        nodes: vec![node21, node22, node23, node24],
    };

    let root = ProjectTreeNode {
        image: "src/assets/icons-16/java_model_obj.png".to_string(),
        text: "app-release.apk".to_string(),
        nodes: vec![parent_node1, parent_node2],
    };

    root
}

#[derive(AsAny, Debug, Clone, PartialEq)]
pub struct ProjectTreeNode {
    pub image: String,
    pub text: String,
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
