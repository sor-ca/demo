use std::io;
fn main() {
println!("Your string, please");
let mut our_string=String::new();
io::stdin().read_line(&mut our_string).
        expect("Failed to read");
let word_number: usize;
println!("Insert the word number");
loop {
    let mut number=String::new();
    io::stdin().read_line(&mut number).
        expect("Failed to read");
    let x: usize = match number.trim().parse() {
         Ok(y)=> y,
         Err(_)=> {  
             println!("A number!");
             continue;
             },
        };
    word_number=x;
    break;
};
let without_spaces: &str = our_string.trim();
let length = without_spaces.len();
if length !=0 {
    let word = number_the_word(&without_spaces, word_number);
    println!("The word number {} is {}", word_number, word)    ;
} else {
    println!("There is no word at all");
}   
 //   let test_strings: [&str; 6] = [
 //       "  ",
 //       "single",
 //       "two words",
 //       "some more words",
 //       "very long string with correct structure",
 //       "double  spaces  in  string",
 //   ];

  //  for str in test_strings.iter() {
  //      println!("'{}' => '{}'", str, second_word(&str));
   // }
}

fn number_the_word(s: &str, word_number: usize) -> &str {
    let bytes = s.as_bytes();
    let mut begin: usize =0;
    let mut end: usize=0;
    let mut space_marker: usize=0;
    let mut word_count = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item != b' ' {
            if space_marker==0 {
                word_count = 1;
                continue;
            }
             else if i==(space_marker+1)  {
                     word_count=word_count+1;
                     if word_count==word_number {
                         begin=i;
                     }
                 }
             }
       else {
            space_marker = i;
            if word_count == word_number {
                end = i;
                break;
            }
        }
    }
    if end !=0 {
        &s[begin..end]
    } else if begin !=0 && end==0 {
        &s[begin..]
    } else if word_number==1 {
        &s[..]
    } else  {
        println!("There is no such  word");
        &s[0..0]
        }
    }
