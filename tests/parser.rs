
extern crate frustum;

#[test]
fn use_items() {
    let session = frustum::Session::new_from_crate_root("src/lib.rs".to_string(),
                                                        "mod somemod;".to_string());

    session.add_file("src/somemod.rs".to_string(),
                     "use std::io; use std::ite; use std::rc;".to_string());

    let krate = frustum::parser::parse_as_crate(&session).unwrap();

    assert_eq!(1, krate.module.items.len());

    assert!(match krate.module.items[0] {
        frustum::items::Item::Mod(ref inner_mod) => 3 == inner_mod.items.len(),
        _ => false,
    });
}
