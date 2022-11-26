#[cfg(test)]
use pretty_assertions::assert_eq;

use turtle_wa::lexer::*;

#[test]
fn parse_base_turtle() {
    let base = "@base <http://example.org/> .";
    let tokens = tokenize(base, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Base("http://example.org/".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_base_sparql() {
    let base = "BASE <http://example.org/> .";
    let tokens = tokenize(base, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Base("http://example.org/".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_prefix_turtle_only_colon() {
    let tokens = tokenize("@prefix : <http://example.org/> .", 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix(":".to_string(), "http://example.org/".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_prefix_sparql_only_colon() {
    let tokens = tokenize("PREFIX : <http://example.org/> .", 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix(":".to_string(), "http://example.org/".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_prefix_turtle() {
    let tokens = tokenize("@prefix foo: <http://example.org/> .", 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix("foo:".to_string(), "http://example.org/".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_prefix_sparql() {
    let tokens = tokenize("PREFIX foo: <http://example.org/> .", 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix("foo:".to_string(), "http://example.org/".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_object_list_iri() {
    let input = "<http://example.org/subject> <http://example.org/predicate> <http://example.org/object1>, <http://example.org/object2> .";

    let tokens = tokenize(&input, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://example.org/subject".to_string()),
            Lexeme::Iri("http://example.org/predicate".to_string()),
            Lexeme::Iri("http://example.org/object1".to_string()),
            Lexeme::ObjectListToken,
            Lexeme::Iri("http://example.org/object2".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_comment() {
    let input = "<http://one.example/subject1> <http://one.example/predicate1> <http://one.example/object1> . # A triple with all absolute IRIs";

    let tokens = tokenize(&input, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://one.example/subject1".to_string()),
            Lexeme::Iri("http://one.example/predicate1".to_string()),
            Lexeme::Iri("http://one.example/object1".to_string()),
            Lexeme::EndToken,
            Lexeme::Comment(" A triple with all absolute IRIs".to_string()),
        ],
    );
}

#[test]
fn parse_object_list_literal() {
    let input = "<http://example.org/#spiderman> <http://xmlns.com/foaf/0.1/name> \"Spiderman\", \"Человек-паук\"@ru .";

    let tokens = tokenize(&input, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://example.org/#spiderman".to_string()),
            Lexeme::Iri("http://xmlns.com/foaf/0.1/name".to_string()),
            Lexeme::Literal("Spiderman".to_string()),
            Lexeme::ObjectListToken,
            Lexeme::LangLiteral("Человек-паук".to_string(), "ru".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_object_list_literal_2() {
    let input = "<http://example.org/#spiderman> <http://xmlns.com/foaf/0.1/name> \"Человек-паук\"@ru, \"Spiderman\" .";

    let tokens = tokenize(&input, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://example.org/#spiderman".to_string()),
            Lexeme::Iri("http://xmlns.com/foaf/0.1/name".to_string()),
            Lexeme::LangLiteral("Человек-паук".to_string(), "ru".to_string()),
            Lexeme::ObjectListToken,
            Lexeme::Literal("Spiderman".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_object_list_literal_mix() {
    let input = "<http://example.org/#spiderman> <http://xmlns.com/foaf/0.1/name> \"Человек-паук\"@ru, <http://example.com/object>, \"Spiderman\"^^<http://www.w3.org/2001/XMLSchema#string> .";

    let tokens = tokenize(&input, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://example.org/#spiderman".to_string()),
            Lexeme::Iri("http://xmlns.com/foaf/0.1/name".to_string()),
            Lexeme::LangLiteral("Человек-паук".to_string(), "ru".to_string()),
            Lexeme::ObjectListToken,
            Lexeme::Iri("http://example.com/object".to_string()),
            Lexeme::ObjectListToken,
            Lexeme::DataTypeLiteral(
                "Spiderman".to_string(),
                "http://www.w3.org/2001/XMLSchema#string".to_string()
            ),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_prefixed_uris() {
    let input = vec![
        "@prefix : <http://example.org/> .",
        ":subject :predicate :object .",
        "@prefix foaf: <http://xmlns.com/foaf/0.1/> .",
        ":subject foaf:name \"Alice\" .",
    ]
    .join("\n");

    let tokens = tokenize(&input, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Prefix(":".to_string(), "http://example.org/".to_string()),
            Lexeme::EndToken,
            Lexeme::PrefixedIri(":subject".to_string()),
            Lexeme::PrefixedIri(":predicate".to_string()),
            Lexeme::PrefixedIri(":object".to_string()),
            Lexeme::EndToken,
            Lexeme::Prefix(
                "foaf:".to_string(),
                "http://xmlns.com/foaf/0.1/".to_string()
            ),
            Lexeme::EndToken,
            Lexeme::PrefixedIri(":subject".to_string()),
            Lexeme::PrefixedIri("foaf:name".to_string()),
            Lexeme::Literal("Alice".to_string()),
            Lexeme::EndToken,
        ],
    );
}

#[test]
fn parse_predicate_list() {
    let input = vec![
        "<http://example.org/#spiderman> <http://www.perceive.net/schemas/relationship/enemyOf> <http://example.org/#green-goblin> ;",
        "        <http://xmlns.com/foaf/0.1/name> \"Spiderman\"@de ;",
        " 				<http://xmlns.com/foaf/0.1/name> \"Spiderman\" ."]
        .join("\n");

    let tokens = tokenize(&input, 0);

    assert_eq!(
        tokens,
        vec![
            Lexeme::Iri("http://example.org/#spiderman".to_string()),
            Lexeme::Iri("http://www.perceive.net/schemas/relationship/enemyOf".to_string()),
            Lexeme::Iri("http://example.org/#green-goblin".to_string()),
            Lexeme::PredicateListToken,
            Lexeme::Iri("http://xmlns.com/foaf/0.1/name".to_string()),
            Lexeme::LangLiteral("Spiderman".to_string(), "de".to_string()),
            Lexeme::PredicateListToken,
            Lexeme::Iri("http://xmlns.com/foaf/0.1/name".to_string()),
            Lexeme::Literal("Spiderman".to_string()),
            Lexeme::EndToken,
        ],
    );
}
