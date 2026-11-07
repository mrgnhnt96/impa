#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Block,
    Warn,
}

impl Level {
    pub fn name(self) -> &'static str {
        match self {
            Level::Block => "block",
            Level::Warn => "warn",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sure {
    Certain,
    Candidate,
}

impl Sure {
    pub fn name(self) -> &'static str {
        match self {
            Sure::Certain => "certain",
            Sure::Candidate => "candidate",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rule {
    pub id: &'static str,
    pub level: Level,
    pub sure: Sure,
}

const fn block(id: &'static str) -> Rule {
    Rule {
        id,
        level: Level::Block,
        sure: Sure::Certain,
    }
}

const fn warn(id: &'static str) -> Rule {
    Rule {
        id,
        level: Level::Warn,
        sure: Sure::Certain,
    }
}

const fn flag(id: &'static str) -> Rule {
    Rule {
        id,
        level: Level::Warn,
        sure: Sure::Candidate,
    }
}

pub const T2: Rule = block("T2");
pub const T4: Rule = block("T4");
pub const T15: Rule = block("T15");
pub const S2: Rule = block("S2");
pub const S3: Rule = block("S3");
pub const S6: Rule = block("S6");
pub const S9: Rule = block("S9");
pub const S12: Rule = block("S12");
pub const S14: Rule = block("S14");
pub const S15: Rule = block("S15");
pub const S16: Rule = block("S16");
pub const S20: Rule = block("S20");
pub const S25: Rule = block("S25");
pub const S30: Rule = block("S30");
pub const S32: Rule = block("S32");
pub const S33: Rule = block("S33");
pub const S34: Rule = block("S34");
pub const E1: Rule = block("E1");
pub const E9: Rule = block("E9");
pub const E10: Rule = block("E10");
pub const A4: Rule = block("A4");
pub const A6: Rule = block("A6");
pub const A7: Rule = warn("A7");
pub const A8: Rule = flag("A8");
pub const A10: Rule = flag("A10");
pub const A13: Rule = warn("A13");
pub const G2: Rule = block("G2");
pub const G3: Rule = block("G3");
pub const G3_PROGRAM: Rule = flag("G3");
pub const G14: Rule = block("G14");
pub const G15: Rule = block("G15");
pub const G18: Rule = block("G18");
pub const G20: Rule = block("G20");
pub const G38: Rule = block("G38");
pub const G38_COMMENT: Rule = flag("G38");
pub const G39: Rule = block("G39");

pub const RULE_IDS: &[&str] = &[
    T2.id, T4.id, T15.id, S2.id, S3.id, S6.id, S9.id, S12.id, S14.id, S15.id, S16.id, S20.id,
    S25.id, S30.id, S32.id, S33.id, S34.id, E1.id, E9.id, E10.id, A4.id, A6.id, A7.id, A8.id,
    A10.id, A13.id, G2.id, G3.id, G14.id, G15.id, G18.id, G20.id, G38.id, G39.id,
];
