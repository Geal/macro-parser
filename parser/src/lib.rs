use nom::{
    bytes::complete::{tag, take_while1},
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
pub struct Hello<'a> {
    pub name: &'a str,
}

pub fn parser(i: &str) -> IResult<&str, &str> {
    delimited(tag("Hello "), take_while1(|c| c != '!'), tag("!"))(i)
}
