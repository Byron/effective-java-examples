#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Kind {
    Annual,
    Perennial,
    Biennial,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Herb {
    pub name: String,
    pub kind: Kind,
}

impl Herb {
    pub fn new<S>(name: S, kind: Kind) -> Herb 
        where S: Into<String> {
        Herb {
            name: name.into(),
            kind: kind,
        }
    }
}

impl AsRef<str> for Herb {
    fn as_ref(&self) -> &str {
        self.name.as_ref()
    }
}


