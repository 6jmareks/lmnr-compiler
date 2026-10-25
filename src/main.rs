use std::env; 


use std::collections::HashMap; //to use hashaps

use std::process::exit; //to exit on exception

use std::fs; //file reading

//use std::fmt::Display; might need this later for vectors
use std::sync::{LazyLock}; //for global

use lmnr_main_lib; //main library for language

//global error struct

struct ErrorTypes{

}




//types

//this one is arbitrary, on my first rendition of this i tried fitting all types into one hashmap, and now i've
//split it up.
#[derive(Clone, Debug)]
enum ContentValue {
    Text(String),
    Float(f64),
    Bool(bool),
    StructString(StringComposition),
    StructNum(NumComposition),
    StructFunc(FuncComposition),
    StructLexed(Lexed)
}
//ignore ignore ignore^^^

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

//function composition

#[derive(Clone, Debug)]
struct FuncComposition{
    name: String,
    params: Vec<String>,
    return_type: String,
    execute: String //Execution code to be stored as a string and called later in the program


}

//MAIN FLOWWW

fn main(){
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut char_number: usize = 0;
    

    //creating the hashmaps to store variables and functions
    let mut str_trid_map: HashMap<String, StringComposition> = HashMap::new();
    let mut num_trid_map: HashMap<String, NumComposition> = HashMap::new();
    let mut func_trid_map: HashMap<String, FuncComposition> = HashMap::new();

    //layout for storing local values as well as libs:
    /*

     _______________   __________
    |local variables| |given libs| -surface level access, call these without parents
     ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾   ‾‾‾‾‾‾‾‾‾‾
           |                |
           v                ‾‾‾‾‾‾|
     ____________________         v
    |local var attributes|       ______________
     ‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾      |lib attributes|
           |                     ‾‾‾‾‾‾‾‾‾|‾‾‾‾
           v                              v
    (further attributes)           (further attributes)
    */

    let mut lexed_lines: Vec<Lexed> = Vec::new();
    

    //reads the file and stores it in a vector
    let mut file_content: String = String::new();
    file_content = fs::read_to_string(file_path).expect("Unable to read file");

    let lexified: Vec<Lexed> = lexer(file_content, char_number);

    map_builder(&lexified, &mut str_trid_map, &mut num_trid_map, &mut func_trid_map);

    //test code here
    
    println!("FUNCMAP:    {:?}", func_trid_map);
    
    
    //test code ends

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

fn ammend_func_trid_map(func_trid_map: &mut HashMap<String, FuncComposition>, name: String, parameters: Vec<String>, return_type: String, content: String){
    let mut new_composition;
    println!("Called");
    new_composition = FuncComposition {
        name: name.clone(),
        params: parameters.clone(),
        return_type: return_type.clone(),
        execute: content.clone(),
    };
    func_trid_map.insert(name, new_composition);
    
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
            exception("Variable not found"); //Change to attribute of ErrorTypes
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
            exception("Variable not found"); //Change to attribute of ErrorTypes
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

#[derive(Clone, Debug)]
struct Lexed{
    type_of: String,
    //between:
    //variable_name_declaration, variable_value_declaration, variable_type_declaration func_call, func_params, func_def_name,
    //func_def_params, func_def_content, 

    text: String

}


/*Info for lexer


if vec_file_content[char_number]=='#'{

            let mut current_instruction_builder: String = String::new(); //dyanmic instruction builder that builds up names of functions before pushing it to instructions_vec and clearing
            
            while vec_file_content[char_number]!='('{ //while the found letter isnt the start of parameters
                current_instruction_builder.push(vec_file_content[char_number]);
                char_number = char_number+1;
            }

            //creates a struct so that later in the program we know how to interpret each bit of data without having to pile in
            //if statements
            let instruction_struct_builder = Lexed{
                type_of: "func_call".to_string(),
                text: current_instruction_builder.clone()
            };
            instructions_vec.push(instruction_struct_builder.clone());
            current_instruction_builder.clear();


The above code is just some boilerplate I wrote to basically:
Detect A
Read Everything from A until B
Push the information of everything read to instructions_vec


This is in iterate_for_lex
*/



fn iterate_for_lex(vec_file_content: &[char], char_number: &mut usize, current_instruction_builder: &mut String, instructions_vec: &mut Vec<Lexed>, type_of_lexed: String, close_on: char, skip_delim: bool){

    while vec_file_content[*char_number]!=close_on{ //while the found letter isnt the start of parameters
                current_instruction_builder.push(vec_file_content[*char_number]);
                *char_number = *char_number+1;
    }

            //creates a struct so that later in the program we know how to interpret each bit of data without having to pile in
            //if statements
    let instruction_struct_builder = Lexed{
        type_of: type_of_lexed,
        text: current_instruction_builder.clone()
    };
    instructions_vec.push(instruction_struct_builder.clone());
    current_instruction_builder.clear();
    if skip_delim{
        *char_number=*char_number+1;
    }

}

fn skip_whitespace(vec_file_content: &Vec<char>,char_number: &mut usize){
    while vec_file_content[*char_number] == ' '{
        *char_number = *char_number+1;
    }

}

//actual lexer program

fn lexer(file_content: String, mut char_number: usize) -> Vec<Lexed> {

    //keychars for token starts and ends
    let end_line: char = ';';

    let func_definition_start: char = 'f';
    let func_name_declaration_start: char = ' ';
    let func_return_type_start: char = ':';
    let func_call_start: char = '#';
    let func_params_start: char = '(';
    let func_params_close: char = ')';
    let func_content_open: char = '{';
    let func_content_close: char = '}';
    let func_type_declaration_start: char = ':';

    let variable_name_declaration_start: char = '~';
    let variable_value_declaration_start: char = '=';
    let variable_type_declaration_start: char = ':';






    let mut lines_vec: Vec<String> = Vec::new();
    let mut instructions_vec: Vec<Lexed> = Vec::new();
    
    let mut vec_file_content: Vec<char> = Vec::new();
    //quickly get all the characters from file_content and push them to vec_file_content individually
    for i in file_content.chars(){
        vec_file_content.push(i);
    }


    //iterate_for_lex(...) params are:
    //vec_file_content: &[char] (content of the file), char_number: &mut usize (&mut for borrowing purposes, char_number is the iterator), current_instruction_builder: &mut String, 
    // instructions_vec: &mut Vec<Lexed> (to push to the vector which is later converted into executable code), type_of_lexed: String (how to interpret the token), 
    // close_on: char (which char is the delimiter), skip_delim: bool (skip the delimeter so next token is clean?)
    
    while char_number < vec_file_content.len(){
        //if the key for a function is found
        if vec_file_content[char_number]==func_call_start{

            let mut current_instruction_builder: String = String::new(); //dyanmic instruction builder that builds up names of functions before pushing it to instructions_vec and clearing
            
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "func_call".to_string(), func_params_start, true);
            skip_whitespace(&vec_file_content, &mut char_number);
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "func_params".to_string(), end_line, true);

            

        } //end of taking apart functions

        //if the key for variable declaration is found
        

        if vec_file_content[char_number]==variable_name_declaration_start{
            let mut current_instruction_builder: String = String::new(); //dyanmic instruction builder that builds up names of functions before pushing it to instructions_vec and clearing
            
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "variable_name_declaration".to_string(), variable_type_declaration_start, true);
            skip_whitespace(&vec_file_content, &mut char_number);
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "variable_type_declaration".to_string(), variable_value_declaration_start, true);
            skip_whitespace(&vec_file_content, &mut char_number);
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "variable_value_declaration".to_string(), end_line, true);
        }

        if vec_file_content[char_number]==func_definition_start && vec_file_content[char_number+1]==func_name_declaration_start{
            let mut current_instruction_builder: String = String::new(); //dyanmic instruction builder that builds up names of functions before pushing it to instructions_vec and clearing
            char_number = char_number + 1; //skips 'f'
            skip_whitespace(&vec_file_content, &mut char_number); //skips ' '
            //if "f " is found
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "func_def_name".to_string(), func_params_start, true);
            skip_whitespace(&vec_file_content, &mut char_number);
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "func_def_params".to_string(), func_params_close, true);
            skip_whitespace(&vec_file_content, &mut char_number);
            char_number = char_number + 1; //skips ':'
            skip_whitespace(&vec_file_content, &mut char_number);
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "func_def_type".to_string(), func_content_open, true); //closes on ' ' to stop trailing whitespace being including in string type 
            skip_whitespace(&vec_file_content, &mut char_number);
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "func_def_content".to_string(), func_content_close, true);
        }

        char_number = char_number+1;
        }

        
        
    instructions_vec
}




fn map_builder(instructs: &Vec<Lexed>, str_trid_map: &mut HashMap<String, StringComposition>, num_trid_map: &mut HashMap<String, NumComposition>, func_trid_map: &mut HashMap<String, FuncComposition>){
    for (i, item) in instructs.iter().enumerate(){
        if item.type_of == "variable_name_declaration"{




            let var_name = item.text.clone();
            let var_content = instructs[i+2].text.clone().trim().trim_matches(&['\\', '"']).to_string();
            let var_type = instructs[i+1].text.clone();
            if var_type.trim()=="String"{
                //debugging: println!("DETECTED STRING!");
                ammend_str_trid_map(str_trid_map, var_name.to_string(), var_type.to_string(), var_content.to_string());
            }
            if var_type.trim()=="num"{
                //debugging: println!("DETECTED NUM!");
                ammend_num_trid_map(num_trid_map, var_name.to_string(), var_type.to_string(), var_content.parse().unwrap());
            }
            

        }//takes variables and adds them to the map^^

        ///     | takes functinos data and lists them in the order name, params, type, and content as listed
        //      v
        if item.type_of == "func_def_name"{
            let func_name = &item.text.clone();
            let func_params = instructs[i+1].text.clone();
            let func_type = instructs[i+2].text.clone();
            let func_content = instructs[i+3].text.clone().replace("\r\n", "").replace('\n', "").replace('\r', "");;
            //turns func_params into a Vec<String>
            //for now ill use a placeholder:
            let placeholder_vec: Vec<String> = Vec::new();
            //
            ammend_func_trid_map(func_trid_map, func_name.to_string(), placeholder_vec, func_type.to_string(), func_content.to_string());

        }



    }
}


fn if_statement(conditional: bool, todo_if_true: fn(), todo_if_false: fn()){
    if conditional{
        todo_if_true();
        return
    }else{
        todo_if_false();
    }
}