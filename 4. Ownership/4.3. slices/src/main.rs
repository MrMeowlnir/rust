fn main() {
    let my_string = String::from("hello world");

    // `first_word` рабботает со слайсами типа `String`, с частями и целыми
    let word1 = first_word(&my_string[0..6]);
    let word2 = first_word(&my_string[..]);
    // `first_word` также работает с сылкой на сам `String`,
    // который эквивалентен целому куску `String`
    let word3 = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` работает с слайсами строковых литералов, с частями и целыми
    let word4 = first_word(&my_string_literal[0..6]);
    let word5 = first_word(&my_string_literal[..]);

    // Так как строковые литералы это по факту строковые слайсы,
    // работать функция будет также с литералом без синтаксиса слайсов и ссылок
    let word6 = first_word(my_string_literal);
    println!("{word1}, {word2}, {word3}, {word4}, {word5}, {word6}")
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
