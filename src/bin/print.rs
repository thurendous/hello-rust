#![allow(unused)]

#[derive(Debug)]

struct Lang {
    language: String,
    version: String,
}

fn main() {
    let lang: &str = "Rust";
    let lang2 = "Solidity";
    println!("Hello {}", lang);
    println!("Hello {} {}", lang, lang);
    println!("Hello {lang}");
    println!("Hello {lang} {lang2}");

    let x = 2;
    println!("{0} x {0} = {1}", x, x * x); // 2 x 2 = 4, 0 is the index of the first argument, and 1 is the index of the second argument

    let lang = Lang {
        language: "Rust".to_string(),
        version: "1.0.0".to_string(),
    };
    println!("{} {}", lang.language, lang.version);

    println!("{:?}", lang);
    println!("{:#?}", lang);
}
