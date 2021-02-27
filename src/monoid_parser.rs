use crate::monoid::Monoid;
use nom::{error::ParseError, IResult, Parser};
use std::marker::PhantomData;

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

/// Provides helper methods on parsers that produce [`Monoid`]s
pub trait MonoidParser<I, O, E>: Sized
where
    O: Monoid,
{
    /// Parse with `self`, then flatten the result with
    /// [`concat`](`Monoid::concat`)
    fn flatten<T>(self) -> Flatten<Self, T> {
        Flatten {
            p1: self,
            phantom: PhantomData,
        }
    }

    /// Parse with `self`, then with `p2`, then
    /// [`combine`](`Monoid::combine`) their results
    fn then<P2>(self, p2: P2) -> Then<Self, P2> { Then { p1: self, p2 } }

    /// The same as [`nom::combinator::opt`], but flattens the result
    fn opt(self) -> Opt<Self> { Opt { p1: self } }

    /// The same as [`nom::multi::many0`], but flattens the result
    fn many0(self) -> Many0<Self> { Many0 { p1: self } }

    /// The same as [`nom::multi::many1`], but flattens the result
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
pub struct Flatten<P1, T> {
    p1: P1,
    phantom: PhantomData<T>,
}

impl<P1, I, T, O, E> Parser<I, O, E> for Flatten<P1, T>
where
    P1: Parser<I, T, E>,
    T: IntoIterator<Item = O>,
    O: Monoid,
    E: ParseError<I>,
{
    fn parse(&mut self, input: I) -> IResult<I, O, E> {
        by_ref(&mut self.p1).map(O::concat).parse(input)
    }
}

#[derive(Debug)]
pub struct Opt<P1> {
    p1: P1,
}

impl<P1, I, O, E> Parser<I, O, E> for Opt<P1>
where
    P1: Parser<I, O, E>,
    I: Clone,
    O: Monoid,
    E: ParseError<I>,
{
    fn parse(&mut self, input: I) -> IResult<I, O, E> {
        nom::combinator::opt(by_ref(&mut self.p1))
            .flatten()
            .parse(input)
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
            .flatten()
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
            .flatten()
            .parse(input)
    }
}
