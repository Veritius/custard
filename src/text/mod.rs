pub mod style;

use smallvec::SmallVec;
use smartstring::{LazyCompact, SmartString};
use self::style::Style;

pub struct TextLines {
    pub lines: SmallVec<[Text; 1]>,
}

pub struct Text {
    pub text: SmartString<LazyCompact>,
    pub style: Style,
}