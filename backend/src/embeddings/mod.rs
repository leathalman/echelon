mod process;
mod tests;
mod text_chunk;

pub use process::{generate_embedding, process_md_file};
pub use text_chunk::TextChunk;
