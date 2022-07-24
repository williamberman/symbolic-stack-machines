use super::word::Word;
use log::info;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Constraint {
    Eq(Box<Word>, Box<Word>),
    Neq(Box<Constraint>),
}

impl std::ops::Not for Constraint {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Constraint::Neq(x) => *x,
            x => Constraint::Neq(Box::new(x)),
        }
    }
}

impl Constraint {
    pub fn ite(self, then: Word, xelse: Word) -> Word {
        match self {
            Constraint::Eq(l, r) => match (*l, *r) {
                (Word::Ite(inner_cond, inner_then, inner_else), equality_check) => {
                    nested_ite_term_rewrite(
                        *inner_cond,
                        *inner_then,
                        *inner_else,
                        equality_check,
                        then,
                        xelse,
                    )
                }
                (equality_check, Word::Ite(inner_cond, inner_then, inner_else)) => {
                    nested_ite_term_rewrite(
                        *inner_cond,
                        *inner_then,
                        *inner_else,
                        equality_check,
                        then,
                        xelse,
                    )
                }
                (l, r) => Word::Ite(
                    Box::new(Constraint::Eq(Box::new(l), Box::new(r))),
                    Box::new(then),
                    Box::new(xelse),
                ),
            },
            xself => Word::Ite(Box::new(xself), Box::new(then), Box::new(xelse)),
        }
    }
}

// TODO(will) - this looks like it's not applying in all necessary cases
//
// (ite (= (ite inner_cond inner_then inner_else) equality_check) outer_then outer_else)
fn nested_ite_term_rewrite(
    inner_cond: Constraint,
    inner_then: Word,
    inner_else: Word,
    equality_check: Word,
    outer_then: Word,
    outer_else: Word,
) -> Word {
    info!("checking nested ite term rewrite");

    // Term re-writing a compound expression that is effectively a nop
    //
    // (ite (= (ite inner_cond inner_then inner_else) inner_then) inner_then inner_else)
    //  ->      (ite inner_cond inner_then inner_else)
    if inner_then == equality_check && inner_then == outer_then && inner_else == outer_else {
        info!("applying nop nested ite term");
        return Word::Ite(
            Box::new(inner_cond),
            Box::new(inner_then),
            Box::new(inner_else),
        );
    }

    // Term re-writing a compound expression that flips `then` and `else`
    //
    // (ite (= (ite inner_cond inner_then inner_else) inner_then) inner_else inner_then)
    //  ->      (ite inner_cond inner_else inner_then)
    //
    // (ite (= (ite inner_cond inner_then inner_else) inner_else) inner_then inner_else )
    //  ->      (ite inner_cond inner_else inner_then)
    if (inner_then == equality_check && inner_then == outer_else && inner_else == outer_then)
        || (inner_else == equality_check && inner_then == outer_then && inner_else == outer_else)
    {
        info!("applying flip then else nested ite term");
        return Word::Ite(
            Box::new(inner_cond),
            Box::new(inner_else),
            Box::new(inner_then),
        );
    }

    Word::Ite(
        Box::new(Constraint::Eq(
            Box::new(Word::Ite(
                Box::new(inner_cond),
                Box::new(inner_then),
                Box::new(inner_else),
            )),
            Box::new(equality_check),
        )),
        Box::new(outer_then),
        Box::new(outer_else),
    )
}
