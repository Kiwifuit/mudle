mod internal;

fn main() {
    if internal::is_dockerized() {
        println!("Running inside a docker image");
    }

    let res = ureq::get(internal::api_endpoint!(""))
}
