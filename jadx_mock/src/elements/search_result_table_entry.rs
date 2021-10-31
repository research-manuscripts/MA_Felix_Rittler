use crate::elements::ProjectNodeDescription;
use orbtk::prelude::*;

widget!(
    /// Defines one entry in the search results table
    SearchResultTableEntry {
        // Name of the node
        entry_name: String,
        /// Content of the second column (e.g. code)
        code_snippet: String,
        /// Sets or shares the image property
        image: Image,
        /// Width of the first column
        width_column_1: f64,
        /// Width of the second column
        width_column_2: f64
    }
);

///
/// Templating a single SearchResultTableEntry
impl Template for SearchResultTableEntry {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let width_column_1_prop = self.width_column_1.as_ref().expect("Width of column 1 has to be set");
        let width_column_2_prop = self.width_column_2.as_ref().expect("Width of column 2 has to be set");

        // determine the width of the two columns
        let width_column_1: f64;
        match width_column_1_prop {
            PropertySource::Source(_) => width_column_1 = 0.0,
            PropertySource::KeySource(_, _) => width_column_1 = 0.0,
            PropertySource::Value(t) => width_column_1 = *t,
        }

        let width_column_2: f64;
        match width_column_2_prop {
            PropertySource::Source(_) => width_column_2 = 0.0,
            PropertySource::KeySource(_, _) => width_column_2 = 0.0,
            PropertySource::Value(t) => width_column_2 = *t,
        }

        // create grid layout
        let layout = format!("{}, {}", width_column_1, width_column_2);

        self.name("SearchResultTableEntry").child(
            Grid::new()
                .columns(layout)
                .rows("30")
                .child(
                    ProjectNodeDescription::new()
                        .margin((5, 0, 10, 0))
                        .attach(Grid::column(0))
                        .attach(Grid::row(0))
                        .text(("entry_name", id))
                        .image(id)
                        .build(ctx),
                )
                .child(
                    TextBlock::new()
                        .margin(5)
                        .attach(Grid::column(1))
                        .attach(Grid::row(0))
                        .style("text")
                        .v_align("center")
                        .text(("code_snippet", id))
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}
