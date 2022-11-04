#[derive(Eq)]
pub struct ScoredWord {
    pub word: String,
    pub score: usize,
}

impl std::cmp::Ord for ScoredWord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl std::cmp::PartialOrd for ScoredWord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for ScoredWord {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

