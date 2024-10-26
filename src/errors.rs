// Returns a string with filename, current code line and column
macro_rules! line_mark {
    () => {
        format!("Marked line: {} @ {}:{}", file!(), line!(), column!())
    };
}

// Transforms an Option to Result
// If the Option contains None, a line mark will be placed along with OpErrorKind::None
macro_rules! transform {
    ($e:expr) => {{
        $e.ok_or(OpError::new(OpErrorKind::None).join_msg(&line_mark!()))?
    }};
}

pub type OpResult<T> = Result<T, OpError>;

#[derive(Debug)]
// Custom error type
pub struct OpError {
    pub kind: OpErrorKind,
    pub message: String,
}

impl OpError {
    pub fn new(kind: OpErrorKind) -> Self {
        OpError {
            kind,
            message: String::new(),
        }
    }

    // Joins the Error with a new message and returns it
    pub fn join_msg(mut self, msg: &str) -> Self {
        self.message.push_str(msg);
        OpError {
            kind: self.kind,
            message: self.message,
        }
    }
}

#[derive(Debug)]
pub enum OpErrorKind {
    None,

}