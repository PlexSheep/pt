use comfy_table::presets;
use comfy_table::{CellAlignment, ContentArrangement, Table};
use console::{style, Color};

/// Prints content with a simple border around it.
///
/// This function is a convenience wrapper around [blockfmt] and [println]. It automatically
/// formats the content with a border using the specified color and then prints it to the console.
///
/// # Example
///
/// ```
/// use libpt_cli::console::Color;
/// use libpt_cli::printing::blockprint;
/// # fn main() {
/// blockprint("Hello world!".to_string(), Color::Blue);
/// # }
/// ```
#[inline]
pub fn blockprint(content: impl ToString, color: Color) {
    println!("{}", blockfmt(content, color))
}

/// Formats content with a simple border around it.
///
/// This function is a convenience wrapper around [blockfmt_advanced] with preset values for
/// border style, content arrangement, and cell alignment. It automatically formats the content
/// with a border as large as possible and centers the content. The resulting cell is colored in
/// the specified color.
///
/// # Example
///
/// ```
/// use libpt_cli::console::Color;
/// use libpt_cli::printing::blockfmt;
/// # fn main() {
/// let formatted_content = blockfmt("Hello world!".to_string(), Color::Blue);
/// println!("{}", formatted_content);
/// # }
/// ```
#[inline]
pub fn blockfmt(content: impl ToString, color: Color) -> String {
    blockfmt_advanced(
        content,
        Some(color),
        presets::UTF8_BORDERS_ONLY,
        ContentArrangement::DynamicFullWidth,
        CellAlignment::Center,
    )
}

/// Formats content with a border around it.
///
/// Unless you are looking for something specific, use [blockfmt] or [blockprint].
///
/// The border can be created using box-drawing characters, and the content is formatted
/// within the border. The function allows customization of the border's color, preset,
/// content arrangement, and cell alignment.
///
/// # Example
/// ```
/// use libpt_cli::comfy_table::{presets, CellAlignment, ContentArrangement};
/// use libpt_cli::console::Color;
/// use libpt_cli::printing::blockfmt_advanced;
/// # fn main() {
/// println!(
///     "{}",
///     blockfmt_advanced(
///         "Hello world!".to_string(),
///         Some(Color::Blue),
///         presets::UTF8_FULL,
///         ContentArrangement::DynamicFullWidth,
///         CellAlignment::Center
///     )
/// );
/// # }
/// ```
/// ```text
/// ┌────────────────────────────────────────────────────────────────────────────────────────┐
/// │                                      Hello world!                                      │
/// └────────────────────────────────────────────────────────────────────────────────────────┘
/// ```
///
/// # Parameters
///
/// - `content`: The content to be formatted within the border
/// - `color`: The color of the border and text
/// - `preset`: The preset style for the border
/// - `arrangement`: The arrangement of the the border (e.g., stretch to sides, wrap around )
/// - `alignment`: The alignment of the content within the cells (e.g., left, center, right)
pub fn blockfmt_advanced(
    content: impl ToString,
    color: Option<Color>,
    preset: &str,
    arrangement: ContentArrangement,
    alignment: CellAlignment,
) -> String {
    let mut table = Table::new();
    table
        .load_preset(preset)
        .set_content_arrangement(arrangement)
        .add_row(vec![content.to_string()]);
    table.column_mut(0).unwrap().set_cell_alignment(alignment);

    match color {
        Some(c) => format!("{}", style(table).fg(c)),
        None => table.to_string(),
    }
}
