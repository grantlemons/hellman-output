use std::{fmt::Display, str::FromStr, string::ParseError};

/// Helper for the following requirements for output:
///
/// - Any stdout output line with a required result must have OUTPUT as the first word (token).
/// - All numerical values required by an assignment must have whitespace (which includes end of line) around them.
/// - Required textual output should be surrounded by single colons (:).
/// - The order for results is assignment specific and important, they must emanate from the application in the order dictated by the assignment.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HellmanOutput(String);

impl Default for HellmanOutput {
    fn default() -> Self {
        Self("OUTPUT".to_string())
    }
}

impl FromStr for HellmanOutput {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default().push_str(s))
    }
}

impl Display for HellmanOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl HellmanOutput {
    /// Push a number to the output string
    pub fn push_numeric(&self, num: isize) -> Self {
        let num_str = &format!(" {} ", num);
        Self(self.0.clone() + num_str)
    }

    /// Push a textual element to the output string
    pub fn push_str(&self, s: &str) -> Self {
        let new_addition = &format!(" :{}: ", s);
        Self(self.0.clone() + new_addition)
    }
}
