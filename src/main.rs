use std::env;
use std::collections::HashMap;
use std::process::exit;

//types

#[derive(Clone)]
enum ContentValue {
    Text(String),
    Float(f64),
    Bool(bool),
    StructString(StringComposition),
    StructNum(NumComposition),
}


//string compositions
#[derive(Clone)]
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
#[derive(Clone)]
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
    let mut line_number = 1;
    

    //creating the vector to store variables
    let mut trid_map: HashMap<String, ContentValue> = HashMap::new();


}

fn ammend_trid_map(trid_map: &mut HashMap<String, ContentValue>, name: String, type_: String, content: String){
    let mut new_composition;
    if type_ == "String" {
        new_composition = StringComposition {
            name: name.clone(),
            type_: type_.clone(),
            content: content.clone(),
        };
        
    }else{
        exception("Invalid type");
        return;
    }
    trid_map.insert(name.clone(), ContentValue::StructString(new_composition));
}

fn grab_content(var_name: &str, trid_map: &HashMap<String, ContentValue>) ->  ContentValue {
    match trid_map.get(var_name) {
        Some(content) => content.clone(), // Found it! Clone the ContentValue to return it
        None => {
            exception("Variable not found");
            panic!("Compilation halted due to missing variable.");
        }
    }
}

fn exception(error_type: &str) {
    println!("{} error", error_type);
    //more to be added
    std::process::exit(1);
}