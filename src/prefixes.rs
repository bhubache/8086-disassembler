use crate::register::SegmentRegister;

pub struct Prefixes {
    pub sr_override: Option<SegmentRegister>,
}

impl Prefixes {
    pub fn new() -> Prefixes {
        Prefixes { sr_override: None }
    }
}
