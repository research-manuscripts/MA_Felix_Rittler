use std::sync::Mutex;

use rand::{
    distributions::{
        uniform::{SampleRange, SampleUniform},
        Alphanumeric,
    },
    thread_rng, Rng,
};

use crate::{
    components::{EditorTabItem, EditorTabItems, ProjectTreeNode, SearchResult, SearchResults},
    generator_constants::{
        IconSet, ALL_ICONS, EDITOR_SCREENSHOTS, ENTITY_ICONS, FILE_ICONS, FONTS, MAX_CHILD_COUNT,
        MAX_NAME_LENGTH, MAX_PATH_LENGTH, MAX_SEARCH_RESULT_COUNT, MAX_TAB_COUNT, MAX_TREE_ITEMS,
    },
};

pub fn fill_checkbox() -> bool {
    thread_rng().gen::<bool>()
}

pub fn select_item<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    thread_rng().gen_range(range)
}

pub fn select_font() -> String {
    FONTS[select_item(0..FONTS.len())].to_string()
}

pub fn generate_name() -> String {
    generate_string(1..=MAX_NAME_LENGTH)
}

pub fn generate_package_path() -> String {
    generate_string(1..=MAX_PATH_LENGTH)
}

pub fn generate_project_tree() -> ProjectTreeNode {
    let project_tree_size: Mutex<i32> = Mutex::new(0);

    generate_sub_tree(0, &project_tree_size)
}

pub fn select_icon(icon_set: IconSet) -> String {
    match icon_set {
        IconSet::AllIcons => return ALL_ICONS[select_item(0..ALL_ICONS.len()) as usize].to_string(),
        IconSet::FileIcons => return FILE_ICONS[select_item(0..FILE_ICONS.len()) as usize].to_string(),
        IconSet::EntityIcons => return ENTITY_ICONS[select_item(0..ENTITY_ICONS.len()) as usize].to_string(),
    }
}

pub fn select_editor_screenshot() -> String {
    EDITOR_SCREENSHOTS[select_item(0..EDITOR_SCREENSHOTS.len()) as usize].to_string()
}

pub fn generate_editor_tabs() -> EditorTabItems {
    let tab_count: i32 = select_item(1..=MAX_TAB_COUNT);
    let selected_index: i32 = select_item(0..tab_count);

    let mut items: Vec<EditorTabItem> = vec![];
    for i in 0..tab_count {
        let name = generate_string(1..=MAX_NAME_LENGTH);
        let icon_path = select_icon(IconSet::FileIcons);
        let selected = selected_index == i;

        items.push(EditorTabItem {
            name,
            icon_path,
            selected,
        });
    }

    EditorTabItems { items }
}

pub fn generate_search_results() -> SearchResults {
    let search_result_count: i32 = select_item(0..=MAX_SEARCH_RESULT_COUNT);

    let mut items: Vec<SearchResult> = vec![];
    for _ in 0..search_result_count {
        items.push(SearchResult {
            name: generate_string(1..=MAX_NAME_LENGTH),
            icon_path: select_icon(IconSet::EntityIcons),
            code_snippet: generate_string(0..=MAX_NAME_LENGTH),
        });
    }

    SearchResults { items }
}

fn generate_string<R>(length_range: R) -> String
where
    R: SampleRange<i32>,
{
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(select_item(length_range) as usize)
        .map(char::from)
        .collect()
}

fn generate_sub_tree(level: i32, project_tree_size: &Mutex<i32>) -> ProjectTreeNode {
    let tree_size: i32 = select_item(0..=MAX_CHILD_COUNT);

    // sample random values for node
    let image: String = select_icon(IconSet::AllIcons);
    let text = generate_string(1..=MAX_NAME_LENGTH);

    // only 5 levels and max 100 items
    if level >= 5 || *project_tree_size.lock().unwrap() >= MAX_TREE_ITEMS {
        return ProjectTreeNode {
            image,
            text,
            nodes: vec![],
        };
    }

    *project_tree_size.lock().unwrap() += tree_size;

    // sample sub tree
    let mut children: Vec<ProjectTreeNode> = vec![];
    for _i in 0..tree_size {
        children.push(generate_sub_tree(level + 1, project_tree_size));
    }

    let size = *project_tree_size.lock().unwrap();
    if size > MAX_TREE_ITEMS {
        println!("Tree size: {}", size);
    }

    ProjectTreeNode {
        image,
        text,
        nodes: children,
    }
}
