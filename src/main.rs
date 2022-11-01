use serde::{Serialize,Deserialize};
use jwt_simple::prelude::*;
fn main(){
   
  let key = HS256Key::generate();
  println!(key)
}