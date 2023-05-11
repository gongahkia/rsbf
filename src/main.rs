// OTHER LEARNING 
    // -- learn how rust lifetimes work
// TO DO
    // -- reads a local .bf file
    // -- base-level interpreter with direct translation
    // -- implement check to panic if there is no local brainfuck file found
    // -- write an interpreter and decryptor
    // -- write this in haskell

// convert ASCII character to u32 value
fn char_to_u32(character:char) -> u32 {
    character as u32
}

fn u32_to_bf(number:u32) -> String {
    let mut output:String = String::from("");
    for _ in 0..number {
        output.push_str("+");
    };
    output.push_str(".");
    output
}

fn bf_to_bfstring(mut bf:String) -> String {
    bf.push_str(">");
    bf
}

fn str_to_bfstring(input:&str) -> String {
    let mut final_string:String = String::from("");
    for character in input.chars() {
        final_string.push_str(bf_to_bfstring(u32_to_bf(char_to_u32(character))).as_str());
    }
    final_string.truncate(final_string.len() - 1);
    final_string
}

fn main() {
    println!("{}", str_to_bfstring("monster hunter"));
}
