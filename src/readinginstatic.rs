
use ::GAMEDATA;

pub struct readinginstatic{
 pub structure:String,

}

impl readinginstatic{
    pub fn print_static(&self){
             println!("In Print static width:{}",*::GAMEDATA);
    }

}
