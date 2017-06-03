// Original file: "GCC.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data::RList;
// use Language::C::System::Preprocess;
// use Data::Maybe;
// use System::Process;
// use System::Directory;
// use Data::List;

pub struct GCC{
    gccPath: FilePath
}
fn gccPath(a: GCC) -> FilePath { a.gccPath }

pub type ParseArgsState = ((Option<FilePath>, Option<FilePath>, RList<CppOption>), (RList<String>, RList<String>));

pub fn buildCppArgs(CppArgs(options, extra_args, _tmpdir, input_file, output_file_opt): CppArgs) -> Vec<String> {
    __op_addadd((concatMap(tOption, options)), __op_addadd(outputFileOpt, __op_addadd(vec!["-E".to_string(), input_file], extra_args)))
}

pub fn gccParseCPPArgs(args: Vec<String>) -> Either<String, (CppArgs, Vec<String>)> {
    match mungeArgs(((None, None, RList::empty), (RList::empty, RList::empty)), args) {
        Left(err) => {
            Left(err)
        },
        Right(((None, _, _), _)) => {
            Left("No .c / .hc / .h source file given".to_string())
        },
        Right(((Some(input_file), output_file_opt, cpp_opts), (extra_args, other_args))) => {
            Right((__assign!((rawCppArgs((RList::reverse(extra_args)), input_file)), {
                    outputFile: output_file_opt,
                    cppOptions: RList::reverse(cpp_opts)
                }), RList::reverse(other_args)))
        },
    }
}

pub fn newGCC() -> GCC {
    GCC
}


