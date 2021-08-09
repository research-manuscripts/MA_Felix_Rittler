use crate::components::{EditorTabItem, EditorTabItems, ProjectTreeNode, SearchResult, SearchResults};

pub fn generate_project_tree() -> ProjectTreeNode {
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

pub fn generate_editor_tabs() -> EditorTabItems {
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

pub fn generate_search_results() -> SearchResults {
    let item1 = SearchResult {
        name: "Test".to_string(),
        icon_path: "src/assets/icons-16/class_obj.png".to_string(),
        code_snippet: "public static void main()".to_string(),
    };

    let item2 = SearchResult {
        name: "Test".to_string(),
        icon_path: "src/assets/icons-16/class_obj.png".to_string(),
        code_snippet: "public static void main()".to_string(),
    };

    let item3 = SearchResult {
        name: "Test".to_string(),
        icon_path: "src/assets/icons-16/class_obj.png".to_string(),
        code_snippet: "public static void main()".to_string(),
    };

    let item4 = SearchResult {
        name: "Test".to_string(),
        icon_path: "src/assets/icons-16/class_obj.png".to_string(),
        code_snippet: "public static void main()".to_string(),
    };

    let item5 = SearchResult {
        name: "Test".to_string(),
        icon_path: "src/assets/icons-16/class_obj.png".to_string(),
        code_snippet: "public static void main()".to_string(),
    };

    let item6 = SearchResult {
        name: "Test".to_string(),
        icon_path: "src/assets/icons-16/class_obj.png".to_string(),
        code_snippet: "public static void main()".to_string(),
    };

    SearchResults {
        items: vec![item1, item2, item3, item4, item5, item6],
    }
}

pub const DEMO_TEXT: &str =
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