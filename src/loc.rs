///
#[derive(Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]

pub struct Loc {
    /// Begin of the `Loc` range
    pub begin: usize,

    /// End of the `Loc` range
    pub end: usize,
}

impl Loc {
    /// Converts location to a range
    pub fn to_range(&self) -> std::ops::Range<usize> {
        self.begin..self.end
    }
}

impl std::fmt::Debug for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_ /*'*/>) -> Result<(), std::fmt::Error> {
        f.write_str(&format!("{}...{}", self.begin, self.end))
    }
}