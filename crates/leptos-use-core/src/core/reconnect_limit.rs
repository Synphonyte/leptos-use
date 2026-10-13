#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReconnectLimit {
    Infinite,
    Limited(u64),
}

impl Default for ReconnectLimit {
    fn default() -> Self {
        ReconnectLimit::Limited(3)
    }
}

impl ReconnectLimit {
    pub fn is_exceeded_by(self, times: u64) -> bool {
        match self {
            ReconnectLimit::Infinite => false,
            ReconnectLimit::Limited(limit) => times >= limit,
        }
    }
}
