use hangul::hangul::{한글자, 음절, 자모, 모음};

fn main() {
    let mut a = 한글자::new('어').unwrap();

    a.set_음절구조(
        음절::중성,
        자모::모음(모음::ㅏ)
    )
        .expect("안대씀");

    println!("{}", a)
}