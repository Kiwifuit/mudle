/**
Expands to the endpoint to GET to. This is really just a
complex `concat!` macro when you supply a "part"

# Examples
```rust
api_endpoint!(); // Expands to the API endpoint, or the webpage to scrape
api_endpoint!("/my/"); // Expands to the endpoint with `/my/` postfixed
```
*/
#[macro_export]
macro_rules! api_endpoint {
    () => {
        "https://basic-ed.cit.edu"
    };

    ($part:literal) => {
        concat!("https://basic-ed.cit.edu", $part)
    };
}
