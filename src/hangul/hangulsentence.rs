use crate::hangul::HangulChar;

pub struct HangulSentence {
    hangulchars:Vec<HangulChar>,
}

impl HangulSentence {
    pub fn new(hangulchars:String) -> Self{
        let hg =
        Self{
            hangulchars
        }
    }
}