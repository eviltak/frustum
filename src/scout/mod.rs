
// Copyright (c) 2016 Arav Singhal.

use syntax;
use items::Item;

pub fn get_items_in_module(module: &syntax::ast::Mod, sess: &syntax::parse::ParseSess) -> Vec<Item> {
    for item in &module.items {

    }

    unimplemented!()
}
