use std::cmp::min;

use crate::{
    elements::SearchResultTableEntry,
    generator::{
        constants::{SEARCH_RESULTS_MAX_ITEM_COUNT, SEARCH_RESULTS_MAX_PAGE_SIZE},
        select_item,
    },
};
use orbtk::prelude::*;

widget!(SearchResultTable {
    /// Height of the table in pixel
    table_height: f64,
    /// Number of table elements
    items: SearchResults
});

///
/// Templating the search result table consisting of the table with dynamic column sizes,
/// table elements lined up vertically and the pagination buttons and text
impl Template for SearchResultTable {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        let item_count = select_item(1..=SEARCH_RESULTS_MAX_ITEM_COUNT);
        let lower_border = select_item(1..item_count);
        // calc page size (lower or equals the number of remaining items)
        let page_size = min(SEARCH_RESULTS_MAX_PAGE_SIZE, item_count - lower_border);
        let higher_border = select_item(lower_border..(lower_border + page_size));

        let pagination_text = format!(
            "Showing results {} to {} of {}",
            lower_border, higher_border, item_count
        );

        let table_height_prop = self
            .table_height
            .as_ref()
            .expect("Height of text search has to be set.");
        let table_height: f64;
        match table_height_prop {
            PropertySource::Source(_) => table_height = 0.0,
            PropertySource::KeySource(_, _) => table_height = 0.0,
            PropertySource::Value(t) => table_height = *t,
        }

        let width = self.width.expect("Width has to be set");
        let table_column_width = 0.5 * width;
        let table_grid_columns = format!("{}, {}", table_column_width, table_column_width);

        let entry_height = 20.0;

        // create table entries
        let items_prop = self.items.as_ref().expect("Items have to be set.");
        let items: Vec<SearchResult>;
        match items_prop {
            PropertySource::Source(_) => items = vec![],
            PropertySource::KeySource(_, _) => items = vec![],
            PropertySource::Value(t) => items = t.items.clone(),
        }

        let items = items
            // iterate over all items
            .iter()
            // limit elements to table height
            .take(((table_height - 35.0) / entry_height) as usize)
            // put every item into a container
            .map(|item: &SearchResult| {
                Container::new()
                    .height(entry_height)
                    .child(
                        SearchResultTableEntry::new()
                            .entry_name(item.name.clone())
                            .code_snippet(item.code_snippet.clone())
                            .image(item.icon_path.clone())
                            .width_column_1(table_column_width)
                            .width_column_2(table_column_width)
                            .build(ctx),
                    )
                    .build(ctx)
            })
            // create a stack that adds every container as a child
            .fold(
                Stack::new().orientation("vertical"),
                |stack: Stack, el: Entity| stack.child(el),
            );

        self.name("Preferences").child(
            Container::new()
                .child(
                    Stack::new()
                        .orientation("vertical")
                        .spacing(20)
                        .child(
                            Container::new()
                                .style("rule")
                                .background("#ffffff")
                                .height(table_height)
                                .child(
                                    Grid::new()
                                        .columns(table_grid_columns)
                                        .rows("30, *")
                                        .child(
                                            TextBlock::new()
                                                .style("text")
                                                .text("Node")
                                                .margin(5)
                                                .attach(Grid::column(0))
                                                .attach(Grid::row(0))
                                                .build(ctx),
                                        )
                                        .child(
                                            TextBlock::new()
                                                .style("text")
                                                .text("Code")
                                                .margin(5)
                                                .attach(Grid::column(1))
                                                .attach(Grid::row(0))
                                                .build(ctx),
                                        )
                                        .child(items.attach(Grid::row(1)).build(ctx))
                                        .build(ctx),
                                )
                                .build(ctx),
                        )
                        .child(
                            Stack::new()
                                .orientation("horizontal")
                                .spacing(5)
                                .child(Button::new().style("wide_windows_button").text("<-").build(ctx))
                                .child(Button::new().text("->").style("wide_windows_button").build(ctx))
                                .child(
                                    TextBlock::new()
                                        .style("text")
                                        .v_align("center")
                                        .text(pagination_text)
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

///
/// Domain object to describe a single search result
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub name: String,
    pub icon_path: String,
    pub code_snippet: String,
}

/// 
/// Wrapper for Search Result
/// Needed because Vec<SearchResult> is not supported by OrbTK as property for SearchResultTable
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResults {
    pub items: Vec<SearchResult>,
}

into_property_source!(SearchResults);

impl Default for SearchResults {
    fn default() -> Self {
        SearchResults { items: vec![] }
    }
}
