
// this an example similar to page 165
mod restaurant {
    pub mod chef {
        pub mod table {
            pub fn serving_food(){}
    }
    }
}
fn another_scope() {
    crate::restaurant::chef::table::serving_food();
}

fn main () {
}