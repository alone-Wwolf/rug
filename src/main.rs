use std::env;
use std::process;



fn main() {
    let command : Vec<String> = env::args().collect();

    let config = rug::TypeConfig::new(&command).unwrap_or_else(|err|{
        eprintln!("There is problem with : {}",err);
        process::exit(3)
    });



    match rug::run(config){
        Ok(o) => o ,
    	Err(err) => eprintln!("{}",err)
    }


}