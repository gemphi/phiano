use crate::command::Context;
use crate::config;
use crate::storage::Storage;

/// Save - persists or loads the facet to/from disk.
///
/// Subcommands:
/// - `save` - saves the facet to the chroma file
/// - `load` - loads the facet from the chroma file
pub struct Save;

impl Save {
    /// Saves the facet to disk.
    pub fn save(&self, ctx: &mut Context) -> bool {
        match Storage::save(ctx.manifold, config::CHROMA_FILE) {
            Ok(_) => {
                println!(
                    "  [saved] {} ({} words)",
                    config::CHROMA_FILE,
                    ctx.manifold.vocabulary_size(),
                );
            }
            Err(e) => eprintln!("  [error] Failed to save: {}", e),
        }
        true
    }

    /// Loads the facet from disk.
    pub fn load(&self, ctx: &mut Context) -> bool {
        match Storage::load(config::CHROMA_FILE) {
            Ok(m) => {
                *ctx.manifold = m;
                println!("  [loaded] {} words", ctx.manifold.vocabulary_size());
            }
            Err(e) => eprintln!("  [error] Failed to load: {}", e),
        }
        true
    }
}
