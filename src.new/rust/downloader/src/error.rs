
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FakeUserAgentError;

impl std::fmt::Display for FakeUserAgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FakeUserAgentError: Unknown error")
    }
}

impl std::error::Error for FakeUserAgentError {}

pub type UserAgentError = FakeUserAgentError;