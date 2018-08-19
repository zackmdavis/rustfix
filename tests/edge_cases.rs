extern crate rustfix;
use std::collections::HashSet;
use std::fs;

use rustfix::apply_suggestions;


#[test]
fn overlapping_suggestions_not_fixable() {
    let json = fs::read_to_string("./tests/edge-cases/overlapping-suggestions-not-fixable.json").unwrap();
    let suggestions =
        rustfix::get_suggestions_from_json(&json, &HashSet::new(), rustfix::Filter::Everything)
        .unwrap();
    let code = fs::read_to_string("./tests/edge-cases/overlapping-suggestions-not-fixable.rs").unwrap();
    let fix_result = apply_suggestions(&code, &suggestions);
    let fix_err = fix_result.unwrap_err();
    assert_eq!(
        "Cannot replace slice of data that was already replaced",
        fix_err.to_string()
    );
}
