use crate::monoid::Monoid;
use nom::{error::ParseError, Err, IResult, Parser};

fn by_ref<P1>(p1: &mut P1) -> ByRef<'_, P1> { ByRef { p1 } }

struct ByRef<'a, P1> {
    p1: &'a mut P1,
}

impl<P1, I, O, E> Parser<I, O, E> for ByRef<'_, P1>
where
    P1: Parser<I, O, E>,
{
    fn parse(&mut self, input: I) -> IResult<I, O, E> { self.p1.parse(input) }
}

pub trait MonoidParser<I, O, E>: Sized
where
    O: Monoid,
{

    fn then<P2>(self, other: P2) -> Then<Self, P2> {
        Then {
            p1: self,
            p2: other,
        }
    }

    fn maybe(self) -> Maybe<Self> { Maybe { p1: self } }

    fn many0(self) -> Many0<Self> { Many0 { p1: self } }

    fn many1(self) -> Many1<Self> { Many1 { p1: self } }
}

impl<P, I, O, E> MonoidParser<I, O, E> for P
where
    P: Parser<I, O, E>,
    O: Monoid,
{
}

#[derive(Debug)]
pub struct Then<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<P1, P2, I, O, E> Parser<I, O, E> for Then<P1, P2>
where
    P1: Parser<I, O, E>,
    P2: Parser<I, O, E>,
    O: Monoid,
{
    fn parse(&mut self, input: I) -> IResult<I, O, E> {
        let (input, output1) = self.p1.parse(input)?;
        let (input, output2) = self.p2.parse(input)?;
        Ok((input, output1.combine(output2)))
    }
}

#[derive(Debug)]
pub struct Maybe<P1> {
    p1: P1,
}

impl<P1, I, O, E> Parser<I, O, E> for Maybe<P1>
where
    P1: Parser<I, O, E>,
    I: Clone,
    O: Monoid,
{
    fn parse(&mut self, input: I) -> IResult<I, O, E> {
        match self.p1.parse(input.clone()) {
            Ok(x) => Ok(x),
            Err(Err::Error(_)) => Ok((input, O::unit())),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub struct Many0<P1> {
    p1: P1,
}

impl<P1, I, O, E> Parser<I, O, E> for Many0<P1>
where
    P1: Parser<I, O, E>,
    I: Clone + PartialEq,
    O: Monoid,
    E: ParseError<I>,
{
    fn parse(&mut self, input: I) -> IResult<I, O, E> {
        nom::multi::many0(by_ref(&mut self.p1))
            .map(O::concat)
            .parse(input)
    }
}

#[derive(Debug)]
pub struct Many1<P1> {
    p1: P1,
}

impl<P1, I, O, E> Parser<I, O, E> for Many1<P1>
where
    P1: Parser<I, O, E>,
    I: Clone + PartialEq,
    O: Monoid,
    E: ParseError<I>,
{
    fn parse(&mut self, input: I) -> IResult<I, O, E> {
        nom::multi::many1(by_ref(&mut self.p1))
            .map(O::concat)
            .parse(input)
    }
}
