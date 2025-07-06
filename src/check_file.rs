use std::fs;
use std::io::Read;
use clap::builder::TypedValueParser;
use cssparser::{ParseError, Parser, ParserInput, Token };
use crate::error::PrettystrictError;
//this is the main fn that will execute to lint the css file. 
//it will also bring in "invalid_property" to do the initial linting after parsing.


fn check(file: Option<String>, watch: bool) -> Result<(), PrettystrictError> {
    let mut path = file.unwrap_or_else(|| "src/styles.css".to_string());
    
    
    if watch { 
        println!("Watching for changes...");
        Ok(())
    } else {
        println!("Checking {}", path);
        let mut css_file = fs::File::open(&path)?;
        let mut contents = String::new();
        css_file.read_to_string(&mut contents)?;

        let mut input = ParserInput::new(contents.as_str());
        let mut parser_output = Parser::new(&mut input);
        
        
        
        Ok(())
    }
}
//READ:
//implement invalid property fn after parsing
//implement watching file fn from memory stores