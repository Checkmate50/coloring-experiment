// use pest::iterators::{Pair, Pairs};
use pest_consume::{match_nodes, Error, Parser};

#[derive(Parser)]
#[grammar = "src/coloring.pest"]
struct ColoringParser;

pub mod ast;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct UserData {}

type ParseResult<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, UserData>;

#[pest_consume::parser]
impl ColoringParser {
    fn EOI(_input: Node) -> ParseResult<()> {
        Ok(())
    }
    fn single(_input: Node) -> ParseResult<()> {
        Ok(())
    }
    fn open(_input: Node) -> ParseResult<()> {
        Ok(())
    }

    fn var(input: Node) -> ParseResult<String> {
        input.as_str().parse::<String>().map_err(|e| input.error(e))
    }

    fn n(input: Node) -> ParseResult<usize> {
        input.as_str().parse::<usize>().map_err(|e| input.error(e))
    }

    fn typ(input: Node) -> ParseResult<String> {
        input.as_str().parse::<String>().map_err(|e| input.error(e))
    }

    fn types(input: Node) -> ParseResult<HashMap<ast::Var, ast::Type>> {
        let error = input.error("Duplicate types");
        match_nodes!(input.into_children();
            [type_decl(typs)..] => {
                let mut result = HashMap::new();
                for (var, typ) in typs {
                    if result.insert(var.clone(), typ) != None {
                        return Err(error);
                    }
                }
                Ok(result)
            }
        )
    }

    fn type_decl(input: Node) -> ParseResult<(ast::Var, ast::Type)> {
        Ok(match_nodes!(input.into_children();
            [var(var), typ(typ)] => (var, typ)
        ))
    }

    fn dependencies(input: Node) -> ParseResult<HashMap<ast::Var, Vec<ast::Var>>> {
        match_nodes!(input.into_children();
            [dependency(deps)..] => {
                let mut result = HashMap::new();
                for (var1, var2) in deps {
                    let v = result.entry(var1).or_insert_with(|| Vec::new());
                    v.push(var2);
                }
                Ok(result)
            }
        )
    }

    fn dependency(input: Node) -> ParseResult<(ast::Var, ast::Var)> {
        Ok(match_nodes!(input.into_children();
            [var(var1), var(var2)] => (var1, var2)
        ))
    }

    fn operations(input: Node) -> ParseResult<Vec<ast::Operation>> {
        Ok(match_nodes!(input.into_children();
            [operation(ops)..] => ops.collect()
        ))
    }

    fn operation(input: Node) -> ParseResult<ast::Operation> {
        Ok(match_nodes!(input.into_children();
            [allocation(allocation)] => ast::Operation::Allocation(allocation),
            [branch(branch)] => ast::Operation::Branch(branch)
        ))
    }

    fn allocation(input: Node) -> ParseResult<ast::Allocation> {
        Ok(match_nodes!(input.into_children();
            [alloc(alloc)] => alloc
        ))
    }

    fn alloc(input: Node) -> ParseResult<ast::Allocation> {
        Ok(match_nodes!(input.into_children();
            [single(_)] => ast::Allocation::Single,
            [open(_)] => ast::Allocation::Open,
            [typ(typ)] => ast::Allocation::Type(typ),
            [var(var)] => ast::Allocation::Var(var)
        ))
    }

    fn branch(input: Node) -> ParseResult<ast::Branch> {
        Ok(match_nodes!(input.into_children();
            [operations(left), operations(right)] => ast::Branch {
                left,
                right
            }
        ))
    }

    fn program(input: Node) -> ParseResult<ast::Program> {
        Ok(match_nodes!(input.into_children();
            [types(t), dependencies(d), operations(a), EOI(_)] => ast::Program {
                types: t,
                dependencies: d,
                allocations: a
            },
        ))
    }
}

pub fn parse(code: &str) -> ParseResult<ast::Program> {
    let user_data = UserData {};
    let parsed = ColoringParser::parse_with_userdata(Rule::program, code, user_data)?;
    let result = ColoringParser::program(parsed.single()?);
    result
}
