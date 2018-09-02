extern crate gamedata;
#[macro_use]
extern crate lazy_static;


lazy_static! {
    pub static ref GAMEDATA:&'static str = "I am a test STrings";
}

fn main(){
   
    println!("Game Data:{}",*GAMEDATA);
    let obj = gamedata::readinginstatic::readinginstatic{structure:"working".to_string()};
    obj.print_static();

   
}

mod test_stuff{

    use ::GAMEDATA;
    fn printthing(){
        println!("herenow:{}",*GAMEDATA);
    }
    
}


 


