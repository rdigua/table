#[derive(Debug)]
pub struct Table{
    pub(crate) columns: Vec<Column>,
//    style: HashMap<TableComponent, char>,
    pub(crate) header: Option<Row>,
    pub(crate) rows: Vec<Row>,
    pub(crate) arrangement: ContentArrangement,
    pub(crate) delimiter: Option<char>,
//    #[cfg(feature = "tty")]
//    no_tty: bool,
//    #[cfg(feature = "tty")]
//    use_stderr: bool,
//    width: Option<u16>,
//    #[cfg(feature = "tty")]
//    enforce_styling: bool,
    // Define whether everything in a cells should be styled, including whitespaces
    // or whether only the text should be styled.
//    #[cfg(feature = "tty")]
//    pub(crate) style_text_only: bool,
}