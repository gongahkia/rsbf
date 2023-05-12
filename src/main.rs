// OTHER LEARNING 
    // -- learn how rust lifetimes work
// TO DO
    // -- reads a local .bf file
        // -- implement check to panic if there is no local brainfuck file found
    // -- write this in haskell

fn char_to_u8(character:char) -> u8 {
    character as u8
}

fn u8_to_bf(number:u8) -> String {
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

// &str literal to brainf
fn str_to_bfstring(input:&str) -> String {
    let mut final_string:String = String::from("");
    for character in input.chars() {
        final_string.push_str(bf_to_bfstring(u8_to_bf(char_to_u8(character))).as_str());
    }
    final_string.truncate(final_string.len() - 1);
    final_string
}

fn bfstring_to_string(brainfuck:&str) -> Vec<String> {
    let instruction_collection:Vec<String> = brainfuck.split(".>").into_iter().map(|x| x.to_string()).collect();
    instruction_collection
}

fn u8_to_char(collection_u8:Vec<u8>) -> String {
    let mut output:String = String::from("");
    for value in collection_u8 {
        output.push(value as char);     
    }
    output
}

fn collection_destructurer(collection:Vec<String>) -> Vec<u8> {
    let mut output:Vec<u8> = vec![];
    for instruction in collection {
        let mut counter:u8 = 0;
        let instruction_vector:Vec<char> = instruction.chars().collect();
        for character in instruction_vector {
            if character == '+' {
                counter += 1
            }
        }
        output.push(counter);
    }
    output
}

// bf to dynamic String
fn bfstring_to_str(input:&str) -> String {
    let output:String = u8_to_char(collection_destructurer(bfstring_to_string(input)));
    output
}

fn main() {
    println!("{}", bfstring_to_str(str_to_bfstring("yes").as_str()));
}
