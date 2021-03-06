use super::*;

/// Examples: Attr, AttrEq
pub trait XFind<'l> {
    /// Usually must be [Copy] so it's compatible with any multi-matchers.
    /// The Out must be copied in the event of "cartesian" product scenarios where multi-matchers
    /// return multiple combinations of their inner matchers' Out.
    type Out: Copy;
    fn find<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, LLSelection)>
    where
        M: XDirection<'l>;
}

pub struct AttrEq<'a, Attr> {
    attr: &'a Attr,
}

pub fn attr_eq<A>(attr: &A) -> AttrEq<'_, A> {
    AttrEq { attr }
}

pub struct TokenHasAny<'a, Attr: PartialEq> {
    one_of: &'a [Attr],
}

pub fn token_has_any<A: PartialEq>(attrs: &[A]) -> TokenHasAny<'_, A> {
    TokenHasAny { one_of: attrs }
}

pub struct Attr<Attr>(std::marker::PhantomData<Attr>);

pub fn attr<A>() -> Attr<A> {
    Attr(Default::default())
}

#[derive(PartialEq, Clone, Copy)]
pub struct NextIdx(pub(crate) usize);

impl<'l, Attr: PartialEq + 'static> XFind<'l> for AttrEq<'_, Attr> {
    type Out = ();

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, NextIdx)>
    where
        M: XDirection<'l>,
    {
        direction.next_attr_eq(self.attr, ll_line)
    }
}

impl<'l, Attr: PartialEq + 'static> XFind<'l> for TokenHasAny<'_, Attr> {
    type Out = &'l Attr;

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, NextIdx)>
    where
        M: XDirection<'l>,
    {
        direction.next_token_attr_one_of(self.one_of, ll_line)
    }
}

impl<'l, A: 'static> XFind<'l> for Attr<A> {
    type Out = &'l A;

    fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, NextIdx)>
    where
        M: XDirection<'l>,
    {
        direction.next_attr::<A>(ll_line)
    }
}

// impl<'l, A: XMatchNext<'l>, B: XMatchNext<'l>> XMatchNext<'l> for (A, B) {
//     type Out = (A::Out, B::Out);

//     fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, NextIdx)>
//     where
//         M: XDirection<'l>,
//     {
//         // ╰─╯A(1)
//         // ╰─╯A(2)
//         // ╰──╯A(3)
//         // ╰─╯B(1)
//         // ╰─╯B(2)
//         // ╰──╯B(3)
//         let bs = self.1.go(direction, ll_line);
//         self.0
//             .go(direction, ll_line)
//             .into_iter()
//             .flat_map(|(a, a_end_idx)| {
//                 bs.iter().filter_map(move |(b, b_end_idx)| {
//                     if a_end_idx == *b_end_idx {
//                         Some(((a, *b), a_end_idx.clone()))
//                     } else {
//                         None
//                     }
//                 })
//             })
//             .collect()
//         // .0 = (&'m A(1), EndIdx(3)); (&'m A(2), EndIdx(3)); (&'m A(3), EndIdx(4))
//         // .1 = (&'m B(1), EndIdx(3)); (&'m B(2), EndIdx(3)); (&'m B(3), EndIdx(4))

//         // Out[0] = (&'m A(1), &'m B(1)), EndIdx(3)
//         // Out[1] = (&'m A(2), &'m B(1)), EndIdx(3)
//         // Out[2] = (&'m A(1), &'m B(2)), EndIdx(3)
//         // Out[3] = (&'m A(2), &'m B(2)), EndIdx(3)

//         // Out[4] = (&'m A(3), &'m B(3)), EndIdx(4)

//         // match up the EndIdx values...
//     }
// }

// impl<'l, A: XMatchNext<'l>, B: XMatchNext<'l>, C: XMatchNext<'l>> XMatchNext<'l> for (A, B, C)
// where
//     A::Out: Copy,
//     B::Out: Copy,
//     C::Out: Copy,
// {
//     type Out = (A::Out, B::Out, C::Out);

//     fn go<M>(&self, direction: &M, ll_line: &'l LLLine) -> Vec<(Self::Out, NextIdx)>
//     where
//         M: XDirection<'l>,
//     {
//         // match up the EndIdx values...
//         let bs = self.1.go(direction, ll_line);
//         let cs = self.2.go(direction, ll_line);
//         self.0
//             .go(direction, ll_line)
//             .into_iter()
//             .flat_map(|(a, a_end_idx)| {
//                 let cs_iter = cs.iter();
//                 bs.iter().flat_map(move |(b, b_end_idx)| {
//                     cs_iter.clone().filter_map(move |(c, c_end_idx)| {
//                         if &a_end_idx == b_end_idx && &a_end_idx == c_end_idx {
//                             Some(((a, *b, *c), a_end_idx))
//                         } else {
//                             None
//                         }
//                     })
//                 })
//             })
//             .collect()
//     }
// }

pub trait XDirection<'l>
where
    Self: Sized,
{
    fn attr<T: 'static>(&self, ll_line: &'l LLLine) -> Vec<(&'l T, LLSelection)>;
    fn attr_eq<T: 'static + PartialEq>(
        &self,
        equals: &T,
        ll_line: &'l LLLine,
    ) -> Vec<((), LLSelection)>;
    fn token_attr_one_of<T: 'static + PartialEq>(
        &self,
        set: &[T],
        ll_line: &'l LLLine,
    ) -> Vec<(&'l T, LLSelection)>;
}

pub(crate) struct XForwards {
    pub(super) idx: usize,
}

// TODO:
//        ╰─╯Amount(1000.25)
//  ╰───╯Person(A)
//                 ╰───╯Person(B)
//           ╰───╯VerbPhrase
// selection
//  .find_by(attr::<Clause>())
//  .iter ... .map
//      .contains_in_any_order(&(multiple::<Person, 2>(), attr::<Amount>(), attr::<VerbPhrase>()))
//        -> Vec of 2 -> Full original selection
//             - ([&Person(A), &Person(B)], &Amount, &VerbPhrase)
// pub(crate) struct XContains {
//     pub(super) start_idx: usize,
//     pub(super) end_idx: usize,
// }

impl<'l> XDirection<'l> for XForwards {
    fn next_attr_eq<T: 'static + PartialEq>(
        &self,
        equals: &T,
        ll_line: &'l LLLine,
    ) -> Vec<((), NextIdx)> {
        // [ ... ] - Current Cursor
        //        [ ... ] - Trying to match Attr
        if self.idx + 1 == ll_line.ll_tokens.len() {
            return Vec::new();
        }

        ll_line
            .attrs
            .starts_at
            .get(self.idx + 1)
            .expect("Huh... match_forwards was at the end")
            .get::<T>()
            .iter()
            .flat_map(|range| {
                ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<T>()
                    .iter()
                    .filter_map(move |val| {
                        if val == equals {
                            Some(((), NextIdx(range.1)))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }

    fn next_attr<T: 'static>(&self, ll_line: &'l LLLine) -> Vec<(&'l T, NextIdx)> {
        // [ ... ] - Current Cursor
        //        [ ... ] - Trying to match Attr
        if self.idx + 1 == ll_line.ll_tokens.len() {
            return Vec::new();
        }

        ll_line
            .attrs
            .starts_at
            .get(self.idx + 1)
            .expect("Huh... match_forwards was at the end")
            .get::<T>()
            .iter()
            .flat_map(|range| {
                ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<T>()
                    .iter()
                    .map(move |val| (val, NextIdx(range.1)))
            })
            .collect()
    }

    fn next_token_attr_one_of<T: 'static + PartialEq>(
        &self,
        set: &[T],
        ll_line: &'l LLLine,
    ) -> Vec<(&'l T, NextIdx)> {
        // [ ... ] - Current Cursor
        //        [ ... ] - Trying to match Attr
        let next_token_idx = self.idx + 1;
        if next_token_idx == ll_line.ll_tokens.len() {
            return Vec::new();
        }

        ll_line
            .attrs
            .values
            .get(&(next_token_idx, next_token_idx))
            .expect("Huh... match_forwards was at the end")
            .get::<T>()
            .iter()
            .filter_map(|value| {
                if set.contains(&value) {
                    Some((value, NextIdx(next_token_idx)))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub(crate) struct XBackwards {
    pub(super) idx: usize,
}

impl<'l> XDirection<'l> for XBackwards {
    fn next_attr_eq<T: 'static + PartialEq>(
        &self,
        equals: &T,
        ll_line: &'l LLLine,
    ) -> Vec<((), NextIdx)> {
        //        [ ... ] - Current Cursor
        // [ ... ] - Trying to match Attr
        //   [...] - Trying to match Attr
        if self.idx == 0 {
            return Vec::new();
        }

        let next_token_idx = self.idx - 1;
        ll_line
            .attrs
            .ends_at
            .get(next_token_idx)
            .expect("Huh... match_backwards was at the end")
            .get::<T>()
            .iter()
            .flat_map(|range| {
                ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<T>()
                    .iter()
                    .filter_map(move |val| {
                        if val == equals {
                            Some(((), NextIdx(range.0)))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }

    fn next_attr<T: 'static>(&self, ll_line: &'l LLLine) -> Vec<(&'l T, NextIdx)> {
        //        [ ... ] - Current Cursor
        // [ ... ] - Trying to match Attr
        //   [...] - Trying to match Attr
        if self.idx == 0 {
            return Vec::new();
        }

        let next_token_idx = self.idx - 1;
        ll_line
            .attrs
            .ends_at
            .get(next_token_idx)
            .expect("Huh... Backwards::next_attr was at the start")
            .get::<T>()
            .iter()
            .flat_map(|range| {
                ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<T>()
                    .iter()
                    .map(move |val| (val, NextIdx(range.0)))
            })
            .collect()
    }

    fn next_token_attr_one_of<T: 'static + PartialEq>(
        &self,
        set: &[T],
        ll_line: &'l LLLine,
    ) -> Vec<(&'l T, NextIdx)> {
        //        [ ... ] - Current Cursor
        // [ ... ] - Trying to match Attr
        //   [...] - Trying to match Attr
        if self.idx == 0 {
            return Vec::new();
        }

        let next_token_idx = self.idx - 1;
        ll_line
            .attrs
            .ends_at
            .get(next_token_idx)
            .expect("Huh... Backwards::next_attr was at the start")
            .get::<T>()
            .iter()
            .flat_map(|range| {
                ll_line
                    .attrs
                    .values
                    .get(&range)
                    .unwrap()
                    .get::<T>()
                    .iter()
                    .filter_map(move |val| {
                        if set.contains(&val) {
                            Some((val, NextIdx(range.0)))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }
}
