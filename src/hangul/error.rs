#[derive(Debug)]
pub enum HangulError {
    InvalidSyllable(char),
    InvalidComponent(&'static str, char),
}
