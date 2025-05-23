use hangul::hangul::{HangulChar, Syllable, Jamo, Moum};


fn main() {
    let mut a = HangulChar::new('어').unwrap();

    //응 안쓸꺼야 ㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗ
    //절-대-안-전-해-
    a.set_syllable_structure(
        Syllable::Chosung,
        Jamo::Moum(Moum::A)
    )
        .expect("안대씀");

    println!("{}", a)
}