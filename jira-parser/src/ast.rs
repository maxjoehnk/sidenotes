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
    /**
     * (Color, Text)
     */
    Color(String, String),
    UnorderedList(Vec<ListItem>),
    OrderedList(Vec<ListItem>),
    Panel(Panel),
    Icon(Icon),
    /**
     * (Text, Link)
     */
    Link(String, String),
    Image(Image),
    Newline,
    Table(Table),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    CheckMark,
    Minus,
    Warning,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListItem {
    pub level: u8,
    pub content: Vec<Tag>,
}

impl ListItem {
    pub fn text(text: &str) -> Self {
        ListItem {
            level: 1,
            content: vec![Tag::Text(text.into())],
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Image {
    pub filename: String,
    pub width: Option<String>,
    pub height: Option<String>,
    pub thumbnail: Option<bool>,
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

impl From<Table> for Tag {
    fn from(table: Table) -> Self {
        Self::Table(table)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Table {
    pub rows: Vec<TableRow>,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct TableRow(pub Vec<TableField>);

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TableField {
    Heading(Vec<Tag>),
    Plain(Vec<Tag>),
}

impl TableField {
    pub fn push(&mut self, tag: Tag) {
        match self {
            Self::Heading(ref mut tags) => tags.push(tag),
            Self::Plain(ref mut tags) => tags.push(tag),
        }
    }
}

impl From<Vec<TableField>> for TableRow {
    fn from(fields: Vec<TableField>) -> Self {
        Self(fields)
    }
}
