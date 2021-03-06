// Original file: "Error.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Typeable;
// use Language::C::Data::Node;
// use Language::C::Data::Position;

#[derive(Eq, Ord)]
pub enum ErrorLevel {
    LevelWarn,
    LevelError,
    LevelFatal
}
pub use self::ErrorLevel::*;

#[derive(Debug)]
pub struct ErrorInfo(ErrorLevel, Position, Vec<String>);


#[derive(Debug)]
pub struct CError(err);


#[derive(Debug)]
pub struct UnsupportedFeature(String, Position);


#[derive(Debug)]
pub struct UserError(ErrorInfo);


pub fn errorLevel() -> ErrorLevel {
    (|ErrorInfo(lvl, _, _)| { lvl }(errorInfo))
}

pub fn errorMsgs() -> Vec<String> {
    (|ErrorInfo(_, _, msgs)| { msgs }(errorInfo))
}

pub fn errorPos() -> Position {
    (|ErrorInfo(_, pos, _)| { pos }(errorInfo))
}

pub fn indent() -> String {
    "  ".to_string()
}

pub fn indentLines() -> String {
    unlines(__map!((indent(__op_addadd)), lines))
}

pub fn internalErr<a>(msg: String) -> a {
    __error!((__op_addadd(internalErrPrefix, __op_addadd("\n".to_string(), __op_addadd(indentLines(msg), "\n".to_string())))))
}

pub fn internalErrPrefix() -> String {
    unlines(vec![
            "Language.C : Internal Error".to_string(),
            __op_addadd("This is propably a bug, and should be reported at ".to_string(), "http://www.sivity.net/projects/language.c/newticket".to_string()),
        ])
}

pub fn isHardError() -> bool {
    ((() > LevelWarn(errorLevel)))
}

pub fn mkErrorInfo(lvl: ErrorLevel, msg: String, node: NodeInfo) -> ErrorInfo {
    ErrorInfo(lvl, (posOfNode(node)), (lines(msg)))
}

pub fn showError(short_msg: String) -> String {
    showErrorInfo(short_msg, errorInfo)
}

pub fn showErrorInfo(short_msg: String, ErrorInfo(level, pos, msgs): ErrorInfo) -> String {

    let header = __op_addadd(showPos(pos), __op_addadd("[".to_string(), __op_addadd(show(level), "]".to_string())));

    let showMsgLines = |_0| {
        match (_0) {
            [] => {
                internalErr("No short message or error message provided.".to_string())
            },
            [x, xs] => {
                internalErr("No short message or error message provided.".to_string())
            },
        }
    };

    __op_addadd(header, showMsgLines((__op_concat(if null(short_msg) {         
msgs} else {
short_msg
        }, msgs))))
}

pub fn unsupportedFeature<a>(msg: String, a: a) -> UnsupportedFeature {
    UnsupportedFeature(msg, (posOf(a)))
}

pub fn unsupportedFeature_(msg: String) -> UnsupportedFeature {
    UnsupportedFeature(msg, internalPos)
}

pub fn userErr(msg: String) -> UserError {
    UserError((ErrorInfo(LevelError, internalPos, (lines(msg)))))
}



