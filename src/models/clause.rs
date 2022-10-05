use std::fmt;

pub struct DisjunctiveClause {
    pub selectors: Vec<Selector>,
}

#[derive(PartialEq)]
pub enum Selector {
    Eq(String, String),
    Leq(String, f64),
    Geq(String, f64),
}

impl fmt::Display for DisjunctiveClause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( ")?;
        for selector in &self.selectors {
            write!(f, "{}", selector)?;
            if selector != self.selectors.last().unwrap() {
                write!(f, " ∨ ")?;
            }
        }
        write!(f, " )")
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Selector::Eq(attr, val) => write!(f, "[{}={}]", attr, val),
            Selector::Leq(attr, val) => write!(f, "[{}<={}]", attr, val),
            Selector::Geq(attr, val) => write!(f, "[{}>={}]", attr, val),
        }
    }
}

impl DisjunctiveClause {
    pub fn new(selectors: Vec<Selector>) -> DisjunctiveClause {
        DisjunctiveClause { selectors }
    }
}

impl Selector {
    pub fn new_eq(attr: String, value: String) -> Selector {
        Selector::Eq(attr, value)
    }

    pub fn new_leq(attr: String, value: f64) -> Selector {
        Selector::Leq(attr, value)
    }

    pub fn new_geq(attr: String, value: f64) -> Selector {
        Selector::Geq(attr, value)
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let selectors = vec![
            Selector::new_eq("a".to_string(), "b".to_string()),
            Selector::new_leq("c".to_string(), 1.0),
            Selector::new_geq("d".to_string(), 2.0),
        ];
        let clause = DisjunctiveClause::new(selectors);

        assert_eq!(
            format!("{}", clause),
            "( [a=b] ∨ [c<=1] ∨ [d>=2] )".to_string()
        );
    }
}
