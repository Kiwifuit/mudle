mod internal;

fn main() {
    if internal::is_dockerized() {
        println!("Running inside a docker image");
    }

    println!("Hello world!")
}
