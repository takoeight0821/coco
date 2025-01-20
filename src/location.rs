use ariadne::Span;

#[derive(Debug, Clone)]
pub struct Location {
    pub file: String,
    pub start: usize,
    pub end: usize,
}

impl Location {
    pub fn to(&self, other: &Self) -> Self {
        Self {
            file: self.file.clone(),
            start: self.start,
            end: other.end,
        }
    }
}

impl Span for Location {
    type SourceId = String;

    fn source(&self) -> &Self::SourceId {
        &self.file
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }
}
