use std::io;
fn main() {

    println!("Input new word");   
    let mut new_word=String::new();
    io::stdin().read_line(&mut new_word).expect("Failed to read");
    new_word=new_word.trim().to_string();
    let pig_latin = pig_latin(&new_word);
    //println!("{}", first_char);
    //println!("{}", new_word);
    //println!("{}", chars.as_str());
    println!("{}", pig_latin);
}
fn pig_latin(new_word: &String)->String {
    let mut pig_latin = String::new();
    //let first_char: char =new_word.chars().take(1).last().unwrap();
    let mut chars = new_word.chars();
    let first_char: char = chars.next().unwrap();
    let vowels: Vec<char> = vec!['a','e','i','o','u'];
    for i in vowels {
        if first_char==i {
            pig_latin.push_str(new_word);
            pig_latin.push_str("hay");
            return pig_latin;
        };
    };
    pig_latin.push_str(chars.as_str());
    pig_latin.push(first_char);
    pig_latin.push_str("ay");
    pig_latin
}
