// Our First Example Of Chapter 7
/*
mod chapter_seven {
    enum Pages {
        One,
        Two,
    }
}
fn main () {
}
// usage of the keyword use in rust to bring a module

mod front_of_house {
    pub mod door {

    }
}
use crate::front_of_house::door;
fn main () {
}
*/
// usage of the keyword super
mod front_of_house {
    pub mod door {
        pub fn open_door(){

        }
    }
}
mod usage {
    pub mod person{
       super::door::open_door();
    }
}