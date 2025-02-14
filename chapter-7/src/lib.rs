// Example of using the use keyword / super keyword
mod front_of_house {
    pub mod door {
        pub fn open_door(){

        }
    }
}
mod usage {
    // you can use super like this
    // first : bring the module in parent scope => use crate::front_of_house::door;
    // then: use super keyword like this: super::door::open_door()
    pub mod person{
        use crate::front_of_house::door;

        fn alex () {
            door::open_door();
        }
    }
}