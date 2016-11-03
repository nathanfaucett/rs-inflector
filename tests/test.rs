extern crate inflector;


use inflector::Inflector;


#[test]
fn test_locale() {
    let inflector = Inflector::new("en");
    assert_eq!(inflector.get_locale(), "en");
}

#[test]
fn test_uncountables() {
    let mut inflector = Inflector::new("en");

    inflector.uncountable(&["equipment", "information"]);

    assert_eq!(inflector.singularize("equipment"), "equipment");
    assert_eq!(inflector.pluralize("information"), "information");
}

#[test]
fn test_irregular() {
    let mut inflector = Inflector::new("en");

    inflector.irregular("man", "men");
    inflector.irregular("person  ", "people");

    assert_eq!(inflector.singularize("men"), "man");
    assert_eq!(inflector.pluralize("man"), "men");
}

#[test]
fn test_pluralize() {
    let mut inflector = Inflector::new("en");

    inflector
        .plural("$", "s")
        .plural("(ch|sh|ss|[sxz])$", "$1es")
        .plural("([^aeiouy])y$", "$1ies");

    assert_eq!(inflector.pluralize("car"), "cars");
    assert_eq!(inflector.pluralize("box"), "boxes");
    assert_eq!(inflector.pluralize("sky"), "skies");
}

#[test]
fn test_singularize() {
    let mut inflector = Inflector::new("en");

    inflector
        .singular("s$", "")
        .singular("(ch|sh|ss|[sxz])es$", "$1")
        .singular("([^aeiouy])ies$", "$1y");

    assert_eq!(inflector.singularize("cars"), "car");
    assert_eq!(inflector.singularize("boxes"), "box");
    assert_eq!(inflector.singularize("skies"), "sky");
}
