use coloring::parser;

fn main() {
    let input_string = std::fs::read_to_string("src/test.color").expect("couldn't read input");
    dbg!(parser::parse(&input_string).expect("Parsing failed"));
}