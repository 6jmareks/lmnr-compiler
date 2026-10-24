use std::env;
use std::collections::HashMap;
use std::process::exit;
use std::fs;
use std::fmt::Display;
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
    execute: String //Execution code to be stored as a string and called later in the program


}

//MAIN FLOWWW

fn main(){
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut char_number: usize = 0;
    

    //creating the vectors to store variables and functions
    let mut str_trid_map: HashMap<String, StringComposition> = HashMap::new();
    let mut num_trid_map: HashMap<String, NumComposition> = HashMap::new();
    let mut func_trid_map: HashMap<String, FuncComposition> = HashMap::new();

    let mut lexed_lines: Vec<Lexed> = Vec::new();
    

    //reads the file and stores it in a vector
    let mut file_content: String = String::new();
    file_content = fs::read_to_string(file_path).expect("Unable to read file");

    let lexified: Vec<Lexed> = lexer(file_content, char_number);

    map_builder(&lexified, &mut str_trid_map, &mut num_trid_map);

    //test code here
    println!("STRINGMAP:    {:?}", str_trid_map);
    println!("NUMMAP:    {:?}", num_trid_map);
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

#[derive(Clone, Debug)]
struct Lexed{
    type_of: String,
    //between:
    //variable_name_declaration, variable_value_declaration, variable_type_declaration func_call, func_params

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

//actual lexer program

fn lexer(file_content: String, mut char_number: usize) -> Vec<Lexed> {

    let mut lines_vec: Vec<String> = Vec::new();
    let mut instructions_vec: Vec<Lexed> = Vec::new();
    
    let mut vec_file_content: Vec<char> = Vec::new();
    //quickly get all the characters from file_content and push them to vec_file_content individually
    for i in file_content.chars(){
        vec_file_content.push(i);
    }


    while char_number < vec_file_content.len(){
        //if the key for a function is found
        if vec_file_content[char_number]=='#'{

            let mut current_instruction_builder: String = String::new(); //dyanmic instruction builder that builds up names of functions before pushing it to instructions_vec and clearing
            
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "func_call".to_string(), '(', true);
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "func_params".to_string(), ';', true);

            

        } //end of taking apart functions

        //if the key for variable declaration is found
        

        if vec_file_content[char_number]=='~'{
            let mut current_instruction_builder: String = String::new(); //dyanmic instruction builder that builds up names of functions before pushing it to instructions_vec and clearing
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "variable_name_declaration".to_string(), '=', true);
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "variable_value_declaration".to_string(), ':', true);
            iterate_for_lex(&vec_file_content, &mut char_number, &mut current_instruction_builder, &mut instructions_vec, "variable_type_declaration".to_string(), ';', true);
        }

            char_number = char_number+1;
        }
        
    instructions_vec
}




fn map_builder(instructs: &Vec<Lexed>, str_trid_map: &mut HashMap<String, StringComposition>, num_trid_map: &mut HashMap<String, NumComposition>){
    for (i, item) in instructs.iter().enumerate(){
        if item.type_of == "variable_name_declaration"{

            let var_name = &item.text;
            let var_content = instructs[i+1].text.clone().trim().trim_matches(&['\\', '"']).to_string();
            let var_type = instructs[i+2].text.clone();
            if var_type.trim()=="String"{
                ammend_str_trid_map(str_trid_map, var_name.to_string(), var_type.to_string(), var_content.to_string());
            }
            if var_type.trim()=="num"{
                ammend_num_trid_map(num_trid_map, var_name.to_string(), var_type.to_string(), var_content.parse().unwrap());
            }
            

        }//takes variables and adds them



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