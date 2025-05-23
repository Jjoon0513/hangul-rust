use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::hangul::constants::{CHOSUNG, JUNGSUNG, JONGSUNG};
use crate::hangul::{Jamo, Syllable};
use crate::hangul::error::HangulError;

pub struct HangulChar {
    pub chosung: char,
    pub jungsung: char,
    pub jongsung: char,
}

impl HangulChar {
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

    pub fn tensed(&self) -> Self {
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

    pub fn compose(&self) -> Result<char, HangulError> {
        let cho = CHOSUNG.iter().position(|&c| c == self.chosung)
            .ok_or(HangulError::InvalidComponent("chosung", self.chosung))?;
        let jung = JUNGSUNG.iter().position(|&c| c == self.jungsung)
            .ok_or(HangulError::InvalidComponent("jungsung", self.jungsung))?;
        let jong = JONGSUNG.iter().position(|&c| c == self.jongsung).unwrap_or(0);

        Ok(char::from_u32(0xAC00 + ((cho * 21 + jung) * 28 + jong) as u32).unwrap())
    }

    pub fn get_syllable_structure(&self, syllable: Syllable) -> char {
        match syllable {
            Syllable::Chosung => self.chosung,
            Syllable::Jungsung => self.jungsung,
            Syllable::Jongsung => self.jongsung,
        }
    }

    pub fn set_syllable_structure(&mut self, syllable: Syllable, jamo: Jamo) -> Result<(), HangulError> {
        match syllable {
            Syllable::Chosung => {
                if matches!(jamo, Jamo::Moum(_)) {
                    return Err(HangulError::InvalidComponent("chosung", jamo.to_char()));
                }
                self.chosung = jamo.to_char();
            }
            Syllable::Jungsung => {
                if matches!(jamo, Jamo::Jaum(_)) {
                    return Err(HangulError::InvalidComponent("jungsung", jamo.to_char()));
                }
                self.jungsung = jamo.to_char();
            }
            Syllable::Jongsung => {
                if matches!(jamo, Jamo::Moum(_)) {
                    return Err(HangulError::InvalidComponent("jongsung", jamo.to_char()));
                }
                self.jongsung = jamo.to_char();
            }
        }
        Ok(())
    }
}

impl Display for HangulChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.compose().unwrap_or('?'))
    }
}
