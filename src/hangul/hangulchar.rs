use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::hangul::constants::{CHOSUNG, JUNGSUNG, JONGSUNG};
use crate::hangul::{자모, 음절};
use crate::hangul::error::HangulError;

#[derive(Clone)]
pub struct 한글자 {
    pub chosung: char,
    pub jungsung: char,
    pub jongsung: char,
}

impl 한글자 {
    pub fn new(hangul: char) -> Result<Self, HangulError> {
        let code = hangul as u32;
        if !(0xAC00..=0xD7A3).contains(&code) {
            return Err(HangulError::InvalidSyllable(hangul));
        }

        let syllable_index = code - 0xAC00;
        let cho = (syllable_index / (21 * 28)) as usize;
        let jung = ((syllable_index % (21 * 28)) / 28) as usize;
        let jong = (syllable_index % 28) as usize;

        Ok(Self {
            chosung: CHOSUNG[cho],
            jungsung: JUNGSUNG[jung],
            jongsung: JONGSUNG[jong],
        })
    }
    pub fn 된소리화(&self) -> Self {
        let chosung_tensed = match self.chosung {
            'ㄱ' => 'ㄲ',
            'ㅂ' => 'ㅃ',
            'ㅈ' => 'ㅉ',
            'ㄷ' => 'ㄸ',
            'ㅅ' => 'ㅆ',
            _ => self.chosung,
        };

        Self {
            chosung: chosung_tensed,
            jungsung: self.jungsung,
            jongsung: self.jongsung,
        }
    }
    pub fn 한글로결합(&self) -> Result<char, HangulError> {
        let cho = CHOSUNG.iter().position(|&c| c == self.chosung)
            .ok_or(HangulError::InvalidComponent("chosung", self.chosung))?;
        let jung = JUNGSUNG.iter().position(|&c| c == self.jungsung)
            .ok_or(HangulError::InvalidComponent("jungsung", self.jungsung))?;
        let jong = JONGSUNG.iter().position(|&c| c == self.jongsung).unwrap_or(0);

        Ok(char::from_u32(0xAC00 + ((cho * 21 + jung) * 28 + jong) as u32).unwrap())
    }
    pub fn get_음절구조(&self, 음절: 음절) -> char {
        match 음절 {
            음절::초성 => self.chosung,
            음절::중성 => self.jungsung,
            음절::종성 => self.jongsung,
        }
    }

    pub fn 종성이없는가(&self) -> bool {
        self.jongsung == '\0'
    }

    pub fn to_char(&self) -> char {
        self.한글로결합().unwrap_or('?')
    }


    pub fn set_음절구조(&mut self, 음절: 음절, 자모: 자모) -> Result<(), HangulError> {
        match 음절 {
            음절::초성 => {
                if matches!(자모, 자모::모음(_)) {
                    return Err(HangulError::InvalidComponent("chosung", 자모.to_char()));
                }
                self.chosung = 자모.to_char();
            }
            음절::중성 => {
                if matches!(자모, 자모::자음(_)) {
                    return Err(HangulError::InvalidComponent("jungsung", 자모.to_char()));
                }
                self.jungsung = 자모.to_char();
            }
            음절::종성 => {
                if matches!(자모, 자모::모음(_)) {
                    return Err(HangulError::InvalidComponent("jongsung", 자모.to_char()));
                }
                self.jongsung = 자모.to_char();
            }
        }
        Ok(())
    }
}

impl Display for 한글자 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.한글로결합().unwrap_or('?'))
    }
}
