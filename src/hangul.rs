pub mod hangulsentence;
pub mod hangulchar;
pub mod constants;
pub mod error;

use std::fmt;
pub use hangulchar::한글자;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 순우리말수사 {
    영, // actualy 영 is not pure-korean
    하나, // 1
    둘, // 2
    셋, // 3
    넷, // 4
    다섯, // 5
    여섯, // 6
    일곱, // 7
    여덟, // 8
    아홉, // 9
    열, // 10
    수물, // 20
    서른, // 30
    마흔, // 40
    쉰, // 50
    예순, // 60
    일흔, // 70
    여든, // 80
    아흔, // 90
    온, // 100 (온 is not used in korean often)
    즈믄, // 1000 (즈믄 is not used in korean often)
}
impl fmt::Display for 순우리말수사 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl 순우리말수사 {
    pub fn from_char(s: &str) -> Result<Self, String> {
        match s {
            "영" => Ok(Self::영),
            "하나" => Ok(Self::하나),
            "둘" => Ok(Self::둘),
            "셋" => Ok(Self::셋),
            "넷" => Ok(Self::넷),
            "다섯" => Ok(Self::다섯),
            "여섯" => Ok(Self::여섯),
            "일곱" => Ok(Self::일곱),
            "여덟" => Ok(Self::여덟),
            "아홉" => Ok(Self::아홉),
            "열" => Ok(Self::열),
            "수물" => Ok(Self::수물),
            "서른" => Ok(Self::서른),
            "마흔" => Ok(Self::마흔),
            "쉰" => Ok(Self::쉰),
            "예순" => Ok(Self::예순),
            "일흔" => Ok(Self::일흔),
            "여든" => Ok(Self::여든),
            "아흔" => Ok(Self::아흔),
            "온" => Ok(Self::온),
            "즈믄" => Ok(Self::즈믄),
            _ => Err(format!("‘{}’은(는) 순우리말 수사가 아닙니다.", s)),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::영 => "영",
            Self::하나 => "하나",
            Self::둘 => "둘",
            Self::셋 => "셋",
            Self::넷 => "넷",
            Self::다섯 => "다섯",
            Self::여섯 => "여섯",
            Self::일곱 => "일곱",
            Self::여덟 => "여덟",
            Self::아홉 => "아홉",
            Self::열 => "열",
            Self::수물 => "수물",
            Self::서른 => "서른",
            Self::마흔 => "마흔",
            Self::쉰 => "쉰",
            Self::예순 => "예순",
            Self::일흔 => "일흔",
            Self::여든 => "여든",
            Self::아흔 => "아흔",
            Self::온 => "온",
            Self::즈믄 => "즈믄",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 한문수사 {
    공, // 0
    일, // 1
    이, // 2
    삼, // 3
    사, // 4
    오, // 5
    육, // 6
    칠, // 7
    팔, // 8
    구, // 9
    십, // 10
    백, // 100
    천, // 1000
    만, // 10000,
    억, // 10000,0000
    조, // 10000,0000,0000
    경, // 10000,0000,0000,0000
}

pub enum 수사 {
    한문수사(한문수사),
    순우리말수사(순우리말수사)
}

impl fmt::Display for 한문수사 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl 한문수사 {
    pub fn from_char(c: char) -> Result<한문수사, String>{
        match c{
            '영' => Ok(한문수사::공),
            '일' => Ok(한문수사::일),
            '이' => Ok(한문수사::이),
            '심' => Ok(한문수사::삼),
            '사' => Ok(한문수사::사),
            '오' => Ok(한문수사::오),
            '육' => Ok(한문수사::육),
            '칠' => Ok(한문수사::칠),
            '팔' => Ok(한문수사::팔),
            '구' => Ok(한문수사::구),
            '십' => Ok(한문수사::십),
            '백' => Ok(한문수사::백),
            '천' => Ok(한문수사::천),
            '만' => Ok(한문수사::만),
            '억' => Ok(한문수사::억),
            '조' => Ok(한문수사::조),
            '경' => Ok(한문수사::경),
            _ => Err(format!("{}은(는) 단일수사가 아닙니다. (백+만 같은 합성수사)", c))
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            한문수사::공 => "영",
            한문수사::일 => "일",
            한문수사::이 => "이",
            한문수사::삼 => "삼",
            한문수사::사 => "사",
            한문수사::오 => "오",
            한문수사::육 => "육",
            한문수사::칠 => "칠",
            한문수사::팔 => "팔",
            한문수사::구 => "구",
            한문수사::십 => "십",
            한문수사::백 => "백",
            한문수사::천 => "천",
            한문수사::만 => "만",
            한문수사::억 => "억",
            한문수사::조 => "조",
            한문수사::경 => "경",
        }

    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 조사 {
    // 격조사
    이,      // 주격조사: 이/가
    을,      // 목적격조사: 을/를
    에게,    // 간접 목적격조사: 에게/한테
    의,      // 소유격조사
    에,      // 부사격조사: 장소, 시간 등
    에서,    // 부사격조사: 장소, 출발점 등
    로,      // 부사격조사: 방향, 수단 (로/으로 통합)

    // 보조사
    은,      // 보조사: 주제 (은/는)
    도,      // 보조사: 또한
    만,      // 보조사: 한정
    까지,    // 보조사: 범위
    마저,    // 보조사: 심지어
    조차,    // 보조사: 심지어
    라도,    // 보조사: 양보
    야,      // 보조사: 강조
}

impl fmt::Display for 조사 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            조사::이 => "이",
            조사::을 => "을",
            조사::에게 => "에게",
            조사::의 => "의",
            조사::에 => "에",
            조사::에서 => "에서",
            조사::로 => "로",
            조사::은 => "은",
            조사::도 => "도",
            조사::만 => "만",
            조사::까지 => "까지",
            조사::마저 => "마저",
            조사::조차 => "조차",
            조사::라도 => "라도",
            조사::야 => "야",
        };
        write!(f, "{}", s)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 음절 {
    초성,
    중성,
    종성
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 자음 {
    ㄱ,
    ㄲ,
    ㄴ,
    ㄷ,
    ㄸ,
    ㄹ,
    ㅁ,
    ㅂ,
    ㅃ,
    ㅅ,
    ㅆ,
    ㅇ,
    ㅈ,
    ㅉ,
    ㅊ,
    ㅋ,
    ㅌ,
    ㅍ,
    ㅎ,
}

impl 자음 {
    pub fn to_char(&self) -> char {
        match self {
            자음::ㄱ => 'ㄱ',
            자음::ㄲ => 'ㄲ',
            자음::ㄴ => 'ㄴ',
            자음::ㄷ => 'ㄷ',
            자음::ㄸ => 'ㄸ',
            자음::ㄹ => 'ㄹ',
            자음::ㅁ => 'ㅁ',
            자음::ㅂ => 'ㅂ',
            자음::ㅃ => 'ㅃ',
            자음::ㅅ => 'ㅅ',
            자음::ㅆ => 'ㅆ',
            자음::ㅇ => 'ㅇ',
            자음::ㅈ => 'ㅈ',
            자음::ㅉ => 'ㅉ',
            자음::ㅊ => 'ㅊ',
            자음::ㅋ => 'ㅋ',
            자음::ㅌ => 'ㅌ',
            자음::ㅍ => 'ㅍ',
            자음::ㅎ => 'ㅎ',
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'ㄱ' => Some(자음::ㄱ),
            'ㄲ' => Some(자음::ㄲ),
            'ㄴ' => Some(자음::ㄲ),
            'ㄷ' => Some(자음::ㄷ),
            'ㄸ' => Some(자음::ㄸ),
            'ㄹ' => Some(자음::ㄹ),
            'ㅁ' => Some(자음::ㅁ),
            'ㅂ' => Some(자음::ㅂ),
            'ㅃ' => Some(자음::ㅃ),
            'ㅅ' => Some(자음::ㅅ),
            'ㅆ' => Some(자음::ㅆ),
            'ㅇ' => Some(자음::ㅇ),
            'ㅈ' => Some(자음::ㅈ),
            'ㅉ' => Some(자음::ㅉ),
            'ㅊ' => Some(자음::ㅊ),
            'ㅋ' => Some(자음::ㅋ),
            'ㅌ' => Some(자음::ㅌ),
            'ㅍ' => Some(자음::ㅍ),
            'ㅎ' => Some(자음::ㅎ),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 모음 {
    ㅏ,
    ㅐ,
    ㅑ,
    ㅒ,
    ㅓ,
    ㅔ,
    ㅕ,
    ㅖ,
    ㅗ,
    ㅘ,
    ㅙ,
    ㅚ,
    ㅛ,
    ㅜ,
    ㅝ,
    ㅞ,
    ㅟ,
    ㅠ,
    ㅡ,
    ㅢ,
    ㅣ,
}

impl 모음 {
    pub fn to_char(&self) -> char {
        match self {
            모음::ㅏ => 'ㅏ',
            모음::ㅐ => 'ㅐ',
            모음::ㅑ => 'ㅑ',
            모음::ㅒ => 'ㅒ',
            모음::ㅓ => 'ㅓ',
            모음::ㅔ => 'ㅔ',
            모음::ㅕ => 'ㅕ',
            모음::ㅖ => 'ㅖ',
            모음::ㅗ => 'ㅗ',
            모음::ㅘ => 'ㅘ',
            모음::ㅙ => 'ㅙ',
            모음::ㅚ => 'ㅚ',
            모음::ㅛ => 'ㅛ',
            모음::ㅜ => 'ㅜ',
            모음::ㅝ => 'ㅝ',
            모음::ㅞ => 'ㅞ',
            모음::ㅟ => 'ㅟ',
            모음::ㅠ => 'ㅠ',
            모음::ㅡ => 'ㅡ',
            모음::ㅢ => 'ㅢ',
            모음::ㅣ => 'ㅣ',
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'ㅏ' => Some(모음::ㅏ),
            'ㅐ' => Some(모음::ㅐ),
            'ㅑ' => Some(모음::ㅑ),
            'ㅒ' => Some(모음::ㅒ),
            'ㅓ' => Some(모음::ㅓ),
            'ㅔ' => Some(모음::ㅔ),
            'ㅕ' => Some(모음::ㅕ),
            'ㅖ' => Some(모음::ㅖ),
            'ㅗ' => Some(모음::ㅗ),
            'ㅘ' => Some(모음::ㅘ),
            'ㅙ' => Some(모음::ㅙ),
            'ㅚ' => Some(모음::ㅚ),
            'ㅛ' => Some(모음::ㅛ),
            'ㅜ' => Some(모음::ㅜ),
            'ㅝ' => Some(모음::ㅝ),
            'ㅞ' => Some(모음::ㅞ),
            'ㅟ' => Some(모음::ㅟ),
            'ㅠ' => Some(모음::ㅠ),
            'ㅡ' => Some(모음::ㅡ),
            'ㅢ' => Some(모음::ㅢ),
            'ㅣ' => Some(모음::ㅣ),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 자모 {
    자음(자음),
    모음(모음),
}


impl 자모 {
    pub fn to_char(&self) -> char {
        match self {
            자모::자음(j) => j.to_char(),
            자모::모음(m) => m.to_char(),
        }
    }
}
