extern crate tantivy;
extern crate core;

use tantivy::schema::{SchemaBuilder, Schema, TEXT, STORED};
use tantivy::Index;

use std::env;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    main_inner(&Path::new(&args[1])).unwrap();
}

fn create_schema() -> Schema {
  let mut schema_builder = SchemaBuilder::default();
  schema_builder.add_text_field("id", STORED);
  schema_builder.add_text_field("text", TEXT);
  schema_builder.build()
}

fn main_inner(output_dir: &Path) -> tantivy::Result<()> {

    let schema = create_schema();
    let index = Index::create_in_dir(output_dir, schema.clone()).expect("failed to create index");

    // 4 GB heap
    let mut index_writer = index.writer(200_000_000).expect("failed to create index writer");

    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let doc = schema.parse_document(&line)?;
        index_writer.add_document(doc);
    }
    
    index_writer.commit()?;
    index_writer.wait_merging_threads()?;
    Ok(())
}
