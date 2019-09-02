use simple_macro::my_macro;

my_macro!{
    fn say_hello() {
        println!("Hello World!")
    }
}

fn main() {
    say_hello();
}
