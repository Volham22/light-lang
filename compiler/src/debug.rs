pub trait LineDebugInfo {
    fn line(&self) -> usize;
    fn column(&self) -> usize;
    fn file_name<'a>(&'a self) -> &'a str;
}

pub trait SpanDebugInfo {
    fn begin_line(&self) -> usize;
    fn end_line(&self) -> usize;
    fn file_name<'a>(&'a self) -> &'a str;
}
