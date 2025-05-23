use crate::hangul::error::HangulError;
use crate::hangul::HangulChar;

#[derive(Clone)]
pub struct HangulSentence {
    hangulsentence:Vec<HangulChar>,
}

impl HangulSentence {
    pub fn new(input: &str) -> Result<Self, HangulError> {
        let mut hchars = Vec::new();

        for ch in input.chars() {
            let hchar = HangulChar::new(ch)?;
            hchars.push(hchar);
        }

        Ok(Self {
            hangulsentence: hchars
        })
    }
    pub fn to_string(&self)-> String{
        let mut c = String::new();

        for i in self.hangulsentence.clone().into_iter(){
            c.push(i.to_char());
        }

        c
    }
    pub fn push(&mut self, c: HangulChar) {
        self.hangulsentence.push(c);
    }

    pub fn to_vec(&self) -> Vec<HangulChar> {
        self.hangulsentence.clone()
    }

    pub fn len(&self) -> usize {
        self.hangulsentence.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hangulsentence.is_empty()
    }

    pub fn add_josa(&mut self,)
}
