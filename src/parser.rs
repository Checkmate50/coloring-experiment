// use pest::iterators::{Pair, Pairs};
use pest_consume::{match_nodes, Error, Parser};

#[derive(Parser)]
#[grammar = "src/coloring.pest"]
struct ColoringParser;

use crate::ast;

#[derive(Clone, Debug)]
struct UserData {}

type ParseResult<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, UserData>;

#[pest_consume::parser]
impl ColoringParser {
    fn EOI(_input: Node) -> ParseResult<()> {
        Ok(())
    }

    fn types(input: Node) -> ParseResult<String> {
        input.as_str().parse::<String>().map_err(|e| input.error(e))
    }

    fn dependencies(input: Node) -> ParseResult<String> {
        input.as_str().parse::<String>().map_err(|e| input.error(e))
    }

    fn allocations(input: Node) -> ParseResult<String> {
        input.as_str().parse::<String>().map_err(|e| input.error(e))
    }

    fn program(input : Node) -> ParseResult<ast::Program> {
        Ok(match_nodes!(input.into_children();
            [types(t), dependencies(d), allocations(a), EOI(_)] => ast::Program {
                types: t,
                dependencies: d,
                allocations: a
            },
        ))
    }
}

pub fn parse(code: &str) -> ParseResult<ast::Program> {
    dbg!(code);
    let user_data = UserData {};
    let parsed = ColoringParser::parse_with_userdata(Rule::program, code, user_data)?;
    let result = ColoringParser::program(parsed.single()?);
    result
}