use crate::hangul::error::HangulError;
use crate::hangul::{한글자, 조사};

#[derive(Clone)]
pub struct 한글문장 {
    hangulsentence:Vec<한글자>,
}

impl 한글문장 {
    pub fn new(input: &str) -> Result<Self, HangulError> {
        let mut hchars = Vec::new();

        for ch in input.chars() {
            let hchar = 한글자::new(ch)?;
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
    pub fn push(&mut self, c: 한글자) {
        self.hangulsentence.push(c);
    }

    pub fn to_vec(&self) -> Vec<한글자> {
        self.hangulsentence.clone()
    }

    pub fn len(&self) -> usize {
        self.hangulsentence.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hangulsentence.is_empty()
    }

    pub fn 조사붙히기(&mut self, 조사: 조사){
        match 조사 {
            조사::은 => {
                if self.hangulsentence.last().is_none() {
                    return;
                }
                else if self.hangulsentence.last().unwrap().종성이없는가() {
                    self.hangulsentence.push(한글자::new('는').unwrap());
                }
                else {
                    self.hangulsentence.push(한글자::new('은').unwrap());
                }
            }
            조사::이 => {
                if self.hangulsentence.last().is_none() {
                    return;
                }
                else if self.hangulsentence.last().unwrap().종성이없는가() {
                    self.hangulsentence.push(한글자::new('가').unwrap());
                }
                else {
                    self.hangulsentence.push(한글자::new('이').unwrap());
                }
            }
            조사::을 => {
                if self.hangulsentence.last().is_none() {
                    return;
                }
                else if self.hangulsentence.last().unwrap().종성이없는가() {
                    self.hangulsentence.push(한글자::new('를').unwrap());
                }
                else {
                    self.hangulsentence.push(한글자::new('을').unwrap());
                }
            }
            other => {
                if self.hangulsentence.last().is_none() {
                    return;
                }
                else {
                    for ch in other.to_string().chars() {
                        self.hangulsentence.push(한글자::new(ch).unwrap());
                    }
                }
            }

        }
    }

}
