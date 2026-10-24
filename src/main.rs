use std::env;
use std::collections::HashMap;
use std::process::exit;
use std::fs;

//types

#[derive(Clone, Debug)]
enum ContentValue {
    Text(String),
    Float(f64),
    Bool(bool),
    StructString(StringComposition),
    StructNum(NumComposition),
}


//string compositions
#[derive(Clone, Debug)]
struct StringComposition {
name: String,
type_: String,
content: String,
}

impl StringComposition {

    fn length(&self) -> i8 {
        self.content.len() as i8
        
    }
}

//num compositions
#[derive(Clone, Debug)]
struct NumComposition {
name: String,
type_: String,
content: f64,
}

impl NumComposition {
}



fn main(){
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut char_number: usize = 0;
    

    //creating the vector to store variables
    let mut str_trid_map: HashMap<String, StringComposition> = HashMap::new();
    let mut num_trid_map: HashMap<String, NumComposition> = HashMap::new();
    let mut lexed_lines: Vec<String> = Vec::new();
    

    //reads the file and stores it in a vector
    let mut file_content: String = String::new();
    file_content = fs::read_to_string(file_path).expect("Unable to read file");

    //testing stuff
    
    //end
    //Prints "Hello world"

}

fn ammend_str_trid_map(str_trid_map: &mut HashMap<String, StringComposition>, name: String, type_: String, content: String){
    let mut new_composition;
    new_composition = StringComposition {
        name: name.clone(),
        type_: type_.clone(),
        content: content.clone(),
    };
    str_trid_map.insert(name, new_composition);
    
}

fn ammend_num_trid_map(num_trid_map: &mut HashMap<String, NumComposition>, name: String, type_: String, content: f64){
    let mut new_composition;
    new_composition = NumComposition {
        name: name.clone(),
        type_: type_.clone(),
        content: content.clone(),
    };
    num_trid_map.insert(name, new_composition);
    
}

fn grab_string_content(var_name: &str, str_trid_map: &HashMap<String, StringComposition>) ->  StringComposition {
    match str_trid_map.get(var_name) {
        Some(content) => content.clone(),
        None => {
            exception("Variable not found");
            StringComposition {
                name: String::new(),
                type_: String::new(),
                content: String::new(),
            } // this line will never be reached due to the exit in exception
        }
    }
}

fn grab_num_content(var_name: &str, num_trid_map: &HashMap<String, NumComposition>) ->  NumComposition {
    match num_trid_map.get(var_name) {
        Some(content) => content.clone(),
        None => {
            exception("Variable not found");
            NumComposition {
                name: String::new(),
                type_: String::new(),
                content: 0.0 as f64,
            } // this line will never be reached due to the exit in exception
        }
    }
}

fn exception(error_type: &str) {
    println!("{} error", error_type);
    //more to be added
    std::process::exit(1);
}


//LEXER

//LEXER!!!

fn lexer(file_content: String, mut char_number: usize) -> Vec<String> {

    let mut lines_vec: Vec<String> = Vec::new();
    let mut instructions_vec: Vec<String> = Vec::new();
    
    let mut vec_file_content: Vec<char> = Vec::new();
    //quickly get all the characters from file_content and push them to vec_file_content individually
    for i in file_content.chars(){
        vec_file_content.push(i);
    }


    while char_number < vec_file_content.len(){
        //if the key for a function is found
        if vec_file_content[char_number]=='#'{
            let mut current_instruction_builder: String = String::new(); //dyanmic instruction builder that builds up names of functions before pushing it to instructions_vec and clearing
            while vec_file_content[char_number]!=';'{ //while the found letter isnt an endline
                current_instruction_builder.push(vec_file_content[char_number]);
                char_number = char_number+1;
            }
        instructions_vec.push(current_instruction_builder);
        }

            char_number = char_number+1;
        }
    instructions_vec
}




fn interpret_to_funcs(instructs: Vec<String>){
    Vec::new();

}

fn if_statement(conditional: bool, todo_if_true: fn(), todo_if_false: fn()){
    if conditional{
        todo_if_true();
        return
    }else{
        todo_if_false();
    }
}