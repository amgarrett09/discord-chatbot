use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum ModuleStatus {
    Enabled,
    Disabled,
}

impl ToString for ModuleStatus {
    fn to_string(&self) -> String {
        match self {
            ModuleStatus::Enabled => "enabled".to_string(),
            _ => "disabled".to_string(),
        }
    }
}

impl FromStr for ModuleStatus {
    type Err = ParseStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "enabled" => Ok(ModuleStatus::Enabled),
            "disabled" => Ok(ModuleStatus::Disabled),
            _ => Err(ParseStatusError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseStatusError;

impl fmt::Display for ParseStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "failed to parse string slice: it needs to be either \"enabled\" \
             or \"disabled\""
        )
    }
}
impl error::Error for ParseStatusError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
