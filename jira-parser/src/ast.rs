#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Tag {
    Text(String),
    Heading(u8, Vec<Tag>),
    Strong(String),
    Emphasis(String),
    Citation(String),
    Deleted(String),
    Inserted(String),
    Superscript(String),
    Subscript(String),
    Monospaced(String),
    InlineQuote(String),
    Quote(String),
    Color(String, String),
    UnorderedList(Vec<Tag>),
    OrderedList(Vec<Tag>),
    Panel(Panel),
    Newline
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Panel {
    pub content: Vec<Tag>,
    pub title: Option<String>,
    pub border_style: Option<String>,
    pub border_color: Option<String>,
    pub border_width: Option<String>,
    pub background_color: Option<String>,
    pub title_background_color: Option<String>,
}

impl From<Tag> for Vec<Tag> {
    fn from(tag: Tag) -> Self {
        vec![tag]
    }
}
