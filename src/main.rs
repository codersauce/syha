use tree_sitter::{Parser, Query, QueryCursor};
use tree_sitter_rust::HIGHLIGHT_QUERY;

fn main() {
    let mut parser = Parser::new();
    let language = tree_sitter_rust::language();
    parser
        .set_language(language)
        .expect("Error loading Rust grammar");

    let code = r#"fn main() {
        println!("Hello, world!");
    }"#;

    let tree = parser.parse(code, None).unwrap();

    let query = Query::new(language, HIGHLIGHT_QUERY).expect("Failed to create query");

    print_syntax_highlighted_code(&tree, code, &query);
}

fn print_syntax_highlighted_code(tree: &tree_sitter::Tree, source_code: &str, query: &Query) {
    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(query, tree.root_node(), source_code.as_bytes());

    for mat in matches {
        for capture in mat.captures {
            let node = capture.node;
            let start_byte = node.start_byte();
            let end_byte = node.end_byte();
            let text = &source_code[start_byte..end_byte];

            match query.capture_names()[capture.index as usize].as_str() {
                "function" => print!("\x1b[1;34m{}\x1b[0m", text),
                "string" => print!("\x1b[0;32m{}\x1b[0m", text),
                _ => print!("{}", text),
            }
        }
    }
}
