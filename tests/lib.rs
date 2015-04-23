extern crate inflector;
use inflector::Inflector;

#[test]
fn test_uncountables() {
    let mut inflector = Inflector::new("en");

    inflector.uncountable(&["equipment", "information"]);

    assert!(inflector.singularize("equipment") == "equipment");
    assert!(inflector.pluralize("information") == "information");
}

#[test]
fn test_pluralize() {
    let mut inflector = Inflector::new("en");

    inflector
        .plural("$", "s")
        .plural("(ch|sh|ss|[sxz])$", "$1es")
        .plural("([^aeiouy])y$", "$1ies");

    assert!(inflector.pluralize("car") == "cars");
    assert!(inflector.pluralize("box") == "boxes");
    assert!(inflector.pluralize("sky") == "skies");
}

#[test]
fn test_singularize() {
    let mut inflector = Inflector::new("en");

    inflector
        .singular("s$", "")
        .singular("(ch|sh|ss|[sxz])es$", "$1")
        .singular("([^aeiouy])ies$", "$1y");

    assert!(inflector.singularize("cars") == "car");
    assert!(inflector.singularize("boxes") == "box");
    assert!(inflector.singularize("skies") == "sky");
}
