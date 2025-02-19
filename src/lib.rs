use std::{fmt::Display, str::FromStr, string::ParseError};

/// Helper for the following requirements for output:
///
/// - Any stdout output line with a required result must have OUTPUT as the first word (token).
/// - All numerical values required by an assignment must have whitespace (which includes end of line) around them.
/// - Required textual output should be surrounded by single colons (:).
/// - The order for results is assignment specific and important, they must emanate from the application in the order dictated by the assignment.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HellmanOutput(String);

impl FromStr for HellmanOutput {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::default().push_str(s))
    }
}

impl Display for HellmanOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OUTPUT {}", self.0.trim_end())
    }
}

impl HellmanOutput {
    /// Push a number to the output string
    /// Accepts anything that implements Display
    pub fn push_numeric(&self, num: impl Display) -> Self {
        let num_str = &format!("{} ", num);
        Self(self.0.clone() + num_str)
    }

    /// Push a textual element to the output string
    pub fn push_str(&self, s: &str) -> Self {
        let new_addition = &format!(":{}: ", s);
        Self(self.0.clone() + new_addition)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::HellmanOutput;

    #[test]
    fn from_string() {
        let s = "Fresh Avacado";

        assert_eq!(
            &HellmanOutput::from_str(s).unwrap().to_string(),
            "OUTPUT :Fresh Avacado:"
        );
    }

    #[test]
    fn push_string() {
        let s = "Fresh Avacado";
        let output = &HellmanOutput::default().push_str(s).push_str(s);

        assert_eq!(
            &output.to_string(),
            "OUTPUT :Fresh Avacado: :Fresh Avacado:"
        );
    }

    #[test]
    fn push_numeric() {
        let s = "Fresh Avacado";
        let output = &HellmanOutput::default()
            .push_str(s)
            .push_numeric(13)
            .push_str(s)
            .push_numeric(1.1);

        assert_eq!(
            &output.to_string(),
            "OUTPUT :Fresh Avacado: 13 :Fresh Avacado: 1.1"
        );
    }
}
