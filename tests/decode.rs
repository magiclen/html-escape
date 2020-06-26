extern crate html_escape;

#[test]
fn decode() {
    println!("{}", html_escape::decode("&gxt;123&gt;2&#65;&#x65;"));
}
