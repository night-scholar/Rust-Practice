fn first_word_index(s: &String) -> usize {
    //把string类型转换为bytes类型
    let bytes = s.as_bytes();
    println!("{:?}",&bytes[..]);
    //迭代，i为key，item为value，要获取其引用,enumerate相当于将迭代的结果返回
    for (i, &item) in bytes.iter().enumerate() {
        //单字节字符b'A'，将空格转为单字节
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //字符串实质字面上就是切片，只不过他是str类型而非string类型
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word_index = first_word_index(&s); // word 的值为 5

    println!("{}", word_index);

    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！

    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5

    println!("{}", word);

    s.clear(); // 这清空了字符串，使其等于 ""
}
