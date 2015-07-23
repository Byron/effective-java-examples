
#[derive(Debug, PartialEq, Eq)]
pub enum Phase {
    Solid,
    Liquid,
    Gas,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Transition {
    Melt,
    Freeze,
    Boil,
    Condense,
    Sublime,
    Deposit,
}

impl Phase {
    pub fn transition_to(&self, other: &Phase) -> Option<Transition> {
        use Transition::*;
        use Phase::*;

        match *self {
            Solid => Some(match *other {
                Liquid => Melt,
                Gas => Sublime,
                Solid => return None
            }),
            Liquid => Some(match *other {
                Liquid => return None,
                Gas => Boil,
                Solid => Freeze,
            }),
            Gas => Some(match *other {
                Liquid => Condense,
                Gas => return None,
                Solid => Deposit
            }),
        }
    }
}