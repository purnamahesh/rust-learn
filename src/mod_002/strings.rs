fn main () {


    // static str: stored in binary
    let s = "Init String";


    // String: stored in heap
    let s = String::new();

    let s = String::from("Init String");

    let mut s = "Init".to_string();

    let hello = String::from(" اﻟﺴﻼمﻋﻠﻴﻜﻢ ");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from(" ָׁשלֹום ");
    let hello = String::from("नम�ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    s.push(' ');

    s.push_str("String");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    // indexing

    let hello = String::from("Здравствуйте");

    println!("{} {}", hello, hello.len());

    // let a = &hello[0]; // error: rust don't allow string indexing 

    let a = &hello[0..2]; 

    println!("{}", a);

    let a = &hello[0..4]; 

    println!("{}", a);

    let ns = String::from("Mahesh");

    println!("{}", &ns[0..1].len());

    for c in hello.chars() {
        println!("{c}")
    }

    for b in hello.bytes() {
        println!("{b}")
    }

}