extern crate frustum;

fn main() {
    println!("Hello, frustum!");

    // TODO: Move tests to tests/*.rs
    let session = frustum::Session::new();
    session.add_file("lib.rs".to_string(), "mod module;".to_string());

    println!("{:?}",
             session.parse_sess.codemap().files.borrow_mut().len());
}
