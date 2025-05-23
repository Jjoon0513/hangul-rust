pub mod hangulsentence;
pub mod hangulchar;
pub mod constants;
pub mod error;

pub use hangulchar::HangulChar;

pub enum Syllable{
    Chosung,
    Jungsung,
    Jongsung
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Jaum {
    Giyeok,
    SsangGiyeok,
    Nieun,
    Digeut,
    SsangDigeut,
    Rieul,
    Mieum,
    Bieup,
    SsangBieup,
    Sios,
    SsangSios,
    Ieung,
    Jieut,
    SsangJieut,
    Chieut,
    Khieukh,
    Thieuth,
    Phieuph,
    Hieuh,
}

impl Jaum {
    pub fn to_char(&self) -> char {
        match self {
            Jaum::Giyeok => 'ㄱ',
            Jaum::SsangGiyeok => 'ㄲ',
            Jaum::Nieun => 'ㄴ',
            Jaum::Digeut => 'ㄷ',
            Jaum::SsangDigeut => 'ㄸ',
            Jaum::Rieul => 'ㄹ',
            Jaum::Mieum => 'ㅁ',
            Jaum::Bieup => 'ㅂ',
            Jaum::SsangBieup => 'ㅃ',
            Jaum::Sios => 'ㅅ',
            Jaum::SsangSios => 'ㅆ',
            Jaum::Ieung => 'ㅇ',
            Jaum::Jieut => 'ㅈ',
            Jaum::SsangJieut => 'ㅉ',
            Jaum::Chieut => 'ㅊ',
            Jaum::Khieukh => 'ㅋ',
            Jaum::Thieuth => 'ㅌ',
            Jaum::Phieuph => 'ㅍ',
            Jaum::Hieuh => 'ㅎ',
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'ㄱ' => Some(Jaum::Giyeok),
            'ㄲ' => Some(Jaum::SsangGiyeok),
            'ㄴ' => Some(Jaum::Nieun),
            'ㄷ' => Some(Jaum::Digeut),
            'ㄸ' => Some(Jaum::SsangDigeut),
            'ㄹ' => Some(Jaum::Rieul),
            'ㅁ' => Some(Jaum::Mieum),
            'ㅂ' => Some(Jaum::Bieup),
            'ㅃ' => Some(Jaum::SsangBieup),
            'ㅅ' => Some(Jaum::Sios),
            'ㅆ' => Some(Jaum::SsangSios),
            'ㅇ' => Some(Jaum::Ieung),
            'ㅈ' => Some(Jaum::Jieut),
            'ㅉ' => Some(Jaum::SsangJieut),
            'ㅊ' => Some(Jaum::Chieut),
            'ㅋ' => Some(Jaum::Khieukh),
            'ㅌ' => Some(Jaum::Thieuth),
            'ㅍ' => Some(Jaum::Phieuph),
            'ㅎ' => Some(Jaum::Hieuh),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Moum {
    A,
    Ae,
    Ya,
    Yae,
    Eo,
    E,
    Yeo,
    Ye,
    O,
    Wa,
    Wae,
    Oe,
    Yo,
    U,
    Weo,
    We,
    Wi,
    Yu,
    Eu,
    Yi,
    I,
}

impl Moum {
    pub fn to_char(&self) -> char {
        match self {
            Moum::A => 'ㅏ',
            Moum::Ae => 'ㅐ',
            Moum::Ya => 'ㅑ',
            Moum::Yae => 'ㅒ',
            Moum::Eo => 'ㅓ',
            Moum::E => 'ㅔ',
            Moum::Yeo => 'ㅕ',
            Moum::Ye => 'ㅖ',
            Moum::O => 'ㅗ',
            Moum::Wa => 'ㅘ',
            Moum::Wae => 'ㅙ',
            Moum::Oe => 'ㅚ',
            Moum::Yo => 'ㅛ',
            Moum::U => 'ㅜ',
            Moum::Weo => 'ㅝ',
            Moum::We => 'ㅞ',
            Moum::Wi => 'ㅟ',
            Moum::Yu => 'ㅠ',
            Moum::Eu => 'ㅡ',
            Moum::Yi => 'ㅢ',
            Moum::I => 'ㅣ',
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'ㅏ' => Some(Moum::A),
            'ㅐ' => Some(Moum::Ae),
            'ㅑ' => Some(Moum::Ya),
            'ㅒ' => Some(Moum::Yae),
            'ㅓ' => Some(Moum::Eo),
            'ㅔ' => Some(Moum::E),
            'ㅕ' => Some(Moum::Yeo),
            'ㅖ' => Some(Moum::Ye),
            'ㅗ' => Some(Moum::O),
            'ㅘ' => Some(Moum::Wa),
            'ㅙ' => Some(Moum::Wae),
            'ㅚ' => Some(Moum::Oe),
            'ㅛ' => Some(Moum::Yo),
            'ㅜ' => Some(Moum::U),
            'ㅝ' => Some(Moum::Weo),
            'ㅞ' => Some(Moum::We),
            'ㅟ' => Some(Moum::Wi),
            'ㅠ' => Some(Moum::Yu),
            'ㅡ' => Some(Moum::Eu),
            'ㅢ' => Some(Moum::Yi),
            'ㅣ' => Some(Moum::I),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Jamo{
    Jaum(Jaum),
    Moum(Moum),
}


impl Jamo {
    pub fn to_char(&self) -> char {
        match self {
            Jamo::Jaum(j) => j.to_char(),
            Jamo::Moum(m) => m.to_char(),
        }
    }
}
