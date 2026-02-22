use std::path::Path;

use anyhow::Result;

use crate::cli::Format;
use crate::config::Config;
use crate::context::{build_rich_context, resolve_location};
use crate::model;
use crate::output::print_context;

use super::do_scan;

pub fn cmd_context(
    root: &Path,
    config: &Config,
    format: &Format,
    location: &str,
    n: usize,
    no_cache: bool,
) -> Result<()> {
    // Scan first so we have items available for ID-based resolution
    let scan = do_scan(root, config, no_cache)?;
    let (file, line) = resolve_location(location, &scan.items)?;

    let todos_in_file: Vec<&model::TodoItem> =
        scan.items.iter().filter(|i| i.file == file).collect();

    let rich = build_rich_context(root, &file, line, n, &todos_in_file)?;
    print_context(&rich, format);
    Ok(())
}
