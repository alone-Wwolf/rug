use std::fs;
use std::error::Error;
use colored::*;


pub struct TypeConfig{
    pub query : String,
    pub filename : String,
    pub case_sen : String,

}


impl TypeConfig{
    pub fn new(args : &[String]) -> Result<Self,&'static str>{
        if args.len() < 3 {
            return Err("query and file must supplie");
            
        }

        let q  = args[1].clone();
        let f  = args[2].clone();
        // optional args
        let s1 = match args.get(3).clone() {
            None => "None".to_string(),
            Some(i) =>i.to_string()
        };


        Ok(TypeConfig{
            query  : q,
            filename : f,
            case_sen : s1,
        })
    }
}


pub fn run(config : TypeConfig)->Result<(),Box<dyn Error>>{
	
    let content = fs::read_to_string(config.filename)?;

    if config.case_sen.eq("None"){
        for line in search(&config.query , &content){
            let new = line.replace(&config.query, &format!("{}",&config.query.red()));
            println!("{}.",new);
        }
    }

    if config.case_sen.eq("case"){
        for line in search(&config.query.to_lowercase() , &content.to_lowercase()){
            let new = line.replace(&config.query.to_lowercase(), &format!("{}",&config.query.red()));
           println!("{}.",new);
        }

    }

    Ok(())
}


pub fn search<'a>(query : &'a str , contents : &'a str)-> Vec<String>{

    let mut results = Vec::new();

    for (mut index, line)in contents.lines().enumerate(){
        index+=1;
        if line.contains(&query){
            results.push(index.to_string() + ". " + &line.to_string());
        }
    }

    results
}


#[cfg(test)]

mod tests {
    #[test]
    // can you test the program here
    fn search_test(){
        unimplemented!();
    }

}