use catalytic::scylla;
use catalytic_macro::query;

mod generated {
    pub use example_project::generated::TestTable;
}

fn main() -> Result<(), scylla::frame::value::SerializeValuesError> {
    let b = "";

    query!("select * from test_table where b = ? and c = ?", b, b);

    Ok(())
}