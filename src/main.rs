use hangul::hangul::{HangulChar, Syllable, Jamo, Jaum};

fn main() {
    let mut a = HangulChar::new('어').unwrap();

    //응 안쓸꺼야 ㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗㅗ
    //절-대-안-전-해-
    a.set_syllable_structure(Syllable::Chosung, Jamo::Jaum(Jaum::Giyeok));

    println!("{}", a)
}