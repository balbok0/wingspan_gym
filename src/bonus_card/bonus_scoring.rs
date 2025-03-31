use pyo3::{prelude::*, types::PyTuple};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoringRule {
    Each(u8),
    Ladder(Box<[(u8, u8)]>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[pyclass(eq, eq_int)]
pub enum PyScoringRuleType {
    Each,
    Ladder
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyScoringRule {
    rule_type: PyScoringRuleType,
    value: ScoringRule,
}

impl <'py> IntoPyObject<'py> for ScoringRule {
    type Target = PyTuple;

    type Output = Bound<'py, Self::Target>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            ScoringRule::Each(val) => (PyScoringRuleType::Each, val).into_pyobject(py),
            ScoringRule::Ladder(items) => (PyScoringRuleType::Ladder, items.to_vec()).into_pyobject(py)
        }
    }
}
