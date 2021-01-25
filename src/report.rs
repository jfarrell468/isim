use cli_table::format::{Border, HorizontalLine, Separator, VerticalLine, Align};
use cli_table::{print_stdout, Cell, Row, Table, TableStruct};
use std::fmt::Debug;

pub struct Report {
    pub config: Vec<crate::config::ReportField>,
    rows: Vec<cli_table::RowStruct>,
}

impl Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unimplemented")
    }
}

impl Report {
    pub fn new(c: Vec<crate::config::ReportField>) -> Report {
        Report {
            config: c,
            rows: Vec::new(),
        }
    }
    pub fn row(&mut self, r: Vec<cli_table::CellStruct>) {
        self.rows.push(r.row());
    }
    pub fn table(&mut self) -> TableStruct {
        let mut title: Vec<cli_table::CellStruct> = Vec::new();
        for f in &self.config {
            title.push(f.title().cell().align(Align::Bottom));
        }
        std::mem::take(&mut self.rows)
            .table()
            .border(Border::builder().build())
            .separator(
                Separator::builder()
                    .column(Some(VerticalLine::default()))
                    .title(Some(HorizontalLine::default()))
                    .row(None)
                    .build(),
            )
            .title(title)
    }
    pub fn print(&mut self) {
        print_stdout(self.table()).expect("Failed to print report")
    }
}
