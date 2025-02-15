// example TWO as re-exporting

mod quux;


fn usage_number_two_of_bar_baz(){
    foo::bar();
    foo::baz();
}

fn main() {
    foo::bar();
    foo::baz();
}
