
use super::util::*;

// TODO improve this to track the location of individual doc comment fields, so we can check for
// comment validity: EX: making sure 'params' match the operation's actual parameters, etc.
pub struct DocComment {
    pub overview: String,
    pub see_also: Vec<String>,
    pub params: Vec<(String, String)>,
    pub returns: Option<String>,
    pub throws: Vec<(String, String)>,
    pub deprecate_reason: Option<String>,
    pub location: Location,
}
