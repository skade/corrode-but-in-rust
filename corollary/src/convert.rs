use parser_haskell::ast;
use parser_haskell::ast::{Expr, Pat, Ty};

use inflector::Inflector;
use hex::*;
use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet};
use regex::Regex;

use ir::{self, PrintState};

// Expand a sequence of expression terms into a nested tree of operators
// TODO Operator precedence rules should be applied here
pub fn expr_explode(span: Vec<Expr>) -> Vec<Expr> {
    if span.len() < 3 {
        return span;
    }
    for i in 0..span.len() {
        if let &ast::Expr::Operator(ref op) = &span[i] {
            return vec![ast::Expr::Op(
                Box::new(Expr::Span(expr_explode(span[0..i].to_vec().clone()))),
                op.to_string(),
                Box::new(Expr::Span(expr_explode(span[i+1..].to_vec().clone()))),
            )];
        }
    }
    span
}

pub fn print_ident(_: PrintState, expr: String) -> String {
    // Handle keywords here
    match expr.as_str() {
        "mut" => "__mut".to_string(),
        "error" => "__error!".to_string(),
        "str" => "__str".to_string(),
        "const" => "__TODO_const".to_string(),
        "@" => "__TODO_at".to_string(),
        "ref" => "__ref".to_string(),
        "static" => "__static".to_string(),
        "enum" => "__enum".to_string(),
        "use" => "__use".to_string(),
        "mod" => "__mod".to_string(),
        "final" => "__final".to_string(),
        "fn" => "__fn".to_string(),
        "pure" => "__pure".to_string(),
        "as" => "__as".to_string(),
        "main" => "__main".to_string(),

        "map" => "__map!".to_string(),

        _ => {
            let mut expr = expr.to_string();

            // Print ..:XXX symbol sequences in idents in hex
            if expr.find(":").is_some() {
                let pos = expr.find(":").unwrap();
                expr = format!("{}__id_{}", &expr[0..pos], (&expr[pos..]).to_hex())
            }

            expr.replace("'", "_q").replace(".", "::")
        }
    }
}


pub fn print_type_ident(state: PrintState, s: &str) -> String {
    // Handle common translations for types here
    match s {
        "Int" => format!("isize"),
        "Nothing" => format!("None"),
        "Just" => format!("Some"),
        "Maybe" => format!("Option"),
        "True" => format!("true"),
        "False" => format!("false"),
        "Bool" => format!("bool"),
        _ => print_ident(state, s.to_string()),
    }
}

pub fn print_op_fn(value: &str) -> String {
    // Here you add new infix operators.
    match value {
        "/" => "__op_div".to_string(),
        "++" => "__op_addadd".to_string(),
        ":" => "__op_concat".to_string(),
        ">>=" => "__op_bind".to_string(),
        ">>" => "__op_rshift".to_string(),
        "<<" => "__op_lshift".to_string(),
        "<+>" => "__op_doc_conat".to_string(),
        "$+$" => "__op_line_concat".to_string(),
        "$$" => "__op_line_something".to_string(), // TODO what does this do
        "<$>" => "__op_dollar_arrow".to_string(), // TODO what does this do
        "$!" => "__op_TODO_dollarnot".to_string(),
        ".&." => "__op_dotted_and".to_string(),
        ".|." => "__op_dotted_or".to_string(),
        "!!" => "__op_index".to_string(),
        "/=" => "__op_assign_div".to_string(),
        "+=" => "__op_assign_add".to_string(),
        "-=" => "__op_assign_sub".to_string(),
        "*=" => "__op_assign_mul".to_string(),
        "^" => "__op_power".to_string(),
        "<>" => "__op_ne".to_string(),
        "!" => "__op_array_index".to_string(), // Returns the element of an immutable array at the specified index, Data.Array.Base
        "," => "__op_tuple2".to_string(),
        "\\\\" => "__op_forwardslash".to_string(), // TODO what does this do
        "$" => "__op_dollar".to_string(), // TODO what does this do
        _ => print_type_ident(PrintState::new(), value)
    }
}

/// Backwards compatibility.
pub fn print_expr(state: PrintState, expr: &ast::Expr) -> String {
    let expr = convert_expr(state, expr);
    format!("{}", ir::Printer { state: state, out: expr })
}

/// Converts several Haskell expresions to a vector of Rust expressions.
pub fn convert_exprs<'a, I>(state: PrintState, exprs: I) -> Vec<ir::Expr>
where
    I: IntoIterator<Item = &'a ast::Expr>
{
    exprs.into_iter().map(|e| convert_expr(state, e)).collect()
}

/// Converts a Haskell expression to Rust.
pub fn convert_expr(state: PrintState, expr: &ast::Expr) -> ir::Expr {
    use parser_haskell::ast::Expr::*;

    let freeform = match *expr {
        Parens(ref r) => {
            let mut res = None;
            if r.len() == 1 {
                if let &Expr::Span(ref inner) = &r[0] {
                    if inner.len() == 1 && matches!(&inner[0], &Expr::Ref(..)) {
                        // HACK: Print singly-wrapped idents as functions
                        res = Some(format!("{}()", print_expr(state, &inner[0])))
                    }
                }
            }

            if res.is_none() {
                let mut out = vec![];
                for item in r {
                    out.push(print_expr(state, item));
                }
                res = Some(format!("({})", out.join(", ")))
            }

            res.unwrap()
        }
        Vector(ref r) => {
            let exprs = convert_exprs(state, r);
            // memoize line length
            let line_length = exprs.iter().fold(Some(2), |sum, e| {
                sum.and_then(|n| e.line_length().map(|len| n + len + 2))
            });
            return ir::Expr::VecLiteral { exprs, line_length };
        }
        Do(ref stmts, ref whence) => {
            print_do(state.tab(), stmts, whence)
        }
        Let(ref assigns, ref expr) => {
            let mut out = assigns.iter().map(|a| print_let(state.tab(), a)).collect::<Vec<_>>();
            out.push(format!("{}{}", state.indent(), print_expr(state.tab(), expr)));
            // todo: should be possible to inline this extra {} in most scenarios
            format!("{{\n{}{}}}", out.join("\n"), state.indent())
        }
        Ref(ast::Ident(ref i)) => {
            print_type_ident(state, i)
        }
        Number(n) => return ir::Expr::Number(n),
        Op(ref l, ref op, ref r) => {
            if op == "&&"
                || op == "=="
                || op == "*"
                || op == "+"
                || op == "-"
                || op == "||"
                || op == ">"
                || op == "<"
                || op == ">="
                || op == "<=" {
                format!("({} {} {})", print_expr(state, l), op, print_expr(state, r))
            } else if op == "$" {
                if let &Expr::Span(ref left) = &**l {
                    let mut left_inner = left.clone();
                    left_inner.push((**r).clone());
                    print_expr(state, &Expr::Span(left_inner))
                } else {
                    format!("{}({})", print_expr(state, l), print_expr(state, r))
                }
            } else if op == "." {
                let l: Expr = (**l).clone();
                let r: Expr = (**r).clone();

                // Dot operator. (f . g) x = f(g(x)
                //TODO this is conditional on overcomplicated AST of spans and parens but easily
                // might change in the future
                if let &Expr::Span(ref left) = &l {
                    if let &Expr::Parens(ref span) = &left[0] {
                        if let &Expr::Span(ref innerspan) = &span[0] {
                            // --> (a b c) ...
                            let mut innerspan = innerspan.clone();
                            //TODO what about span[1..]
                            innerspan.push(r);
                            format!("{}", print_expr(state, &Expr::Span(vec![Expr::Parens(vec![Expr::Span(innerspan)])])))
                        } else {
                            panic!("WHAT {:?}", l);
                            //format!("({} . {})", print_expr(state, &l), print_expr(state, &r))
                        }
                    } else if let &Expr::Ref(..) = &left[0] {
                        // --> a ...
                        let mut outer = left.clone();
                        outer.push(r);
                        format!("{}", print_expr(state, &Expr::Span(outer)))
                    } else {
                        panic!("WHAT {:?}", l);
                    }
                } else {
                    panic!("WHAT {:?}", l);
                }

            } else if op == "<-" {
                format!("let {} = {}", print_expr(state, l), print_expr(state, r))
            } else {
                let new_op = print_op_fn(op);
                format!("{}({}, {})", new_op, print_expr(state, l), print_expr(state, r))
            }
        }
        Record(ref base, ref items) => {
            let mut out = vec![];
            for &(ast::Ident(ref i), ref v) in items {
                out.push(format!("{}{}: {}",
                    state.tab().indent(),
                    i,
                    print_expr(state.tab().tab(), v)));
            }
            match **base {
                Expr::Ref(..) => {
                    format!("{} {{\n{}\n{}}}",
                        print_expr(state.tab(), base),
                        out.join(",\n"), state.indent())   
                }
                _ => {
                    // For non-refs, create a macro that will augment keys onto original object.
                    format!("__assign!({}, {{\n{}\n{}}})",
                        print_expr(state.tab(), base),
                        out.join(",\n"), state.indent())   
                }
            }
        }
        Str(ref s) => return ir::Expr::StrLiteral(s.clone()),
        Char(ref s) => {
            assert!(s.len() == 1, "char lit {:?}", s);
            format!("{:?}", s.chars().next().unwrap())
        }
        Span(ref span) => {
            let span = expr_explode(span.clone());

            // TODO this first clause is a highly specific accident the parser now relies on
            // this should be removed asap
            if span.len() == 2 && ({
                if let ast::Expr::Record(..) = span[1] {
                    true
                } else { false }
            }) && print_expr(state, &span[0].clone()) == "in" {
                format!("{} {}",
                    print_expr(state, &span[0].clone()),
                    print_expr(state, &span[1].clone()))
            } else if span.len() == 1 {
                print_expr(state, &span[0])
            } else if span.len() == 0 {
                format!("()") //TODO not sure what this would be?
            } else {
                // Check for return() here, for now
                if print_expr(state, &span[0]) == "return" {
                    //TODO handle return more intelligently
                    print_expr(state, &Expr::Span(span[1..].to_vec()))
                } else {
                    // Check for `if` statements
                    let first_word = print_expr(PrintState::new(), &span[0]);
                    if first_word == "if" {
                        let mut span = span.clone();
                        span.remove(0);
                        let mut then = vec![];
                        let mut els = vec![]; // TODO
                        if let Some(pos) = span.iter()
                            .position(|x| print_expr(PrintState::new(), x) == "else") {
                            els = span.split_off(pos + 1);
                            span.split_off(pos);
                        }
                        if let Some(pos) = span.iter()
                            .position(|x| print_expr(PrintState::new(), x) == "then") {
                            then = span.split_off(pos + 1);
                            span.split_off(pos);
                        }

                        // Condition
                        let cond = print_expr(state, &Expr::Span(span));
                        if els.len() > 0 {
                            format!("if {} {{ {}\n{}}} else {{\n{}\n{}}}",
                                cond,
                                state.indent(),
                                print_expr(state, &Expr::Span(then)),
                                print_expr(state, &Expr::Span(els)),
                                state.indent())
                        } else {
                            format!("if {} {{ {} }}",
                                cond,
                                print_expr(state, &Expr::Span(then)))
                        }
                    } else if first_word == "unless" || first_word == "when" {
                        let mut span = span.clone();
                        span.remove(0);

                        // Condition
                        let cond = if first_word == "unless" {
                            format!("!({})", print_expr(state, &span.remove(0)))
                        } else {
                            format!("{}", print_expr(state, &span.remove(0)))
                        };
                        format!("if {} {{ {} }}",
                            cond,
                            print_expr(state, &Expr::Span(span)))
                    } else {
                        // Print normal function invocation a(...)
                        let mut span = span.clone();
                        let start = print_expr(state, &span.remove(0));
                        let mut end = "".to_string();
                        if span.len() > 0 {
                            let mut out = vec![];
                            for item in &span {
                                out.push(print_expr(state.tab(), item));
                            }
                            end = format!("({})", out.join(", "));
                        }
                        format!("{}{}", start, end)
                    }
                }
            }
        }
        Case(ref cond, ref rest) => {
            let mut out = vec![];
            for item in rest {
                match item.clone() {
                    ast::CaseCond::Matching(label, arms) => {
                        for (cond, arm) in arms {
                            let label_str = print_case_patterns(state, vec![Pat::Span(label.clone())]);
                            let cond_str = cond.iter().map(|x| print_expr(state, x)).collect::<Vec<_>>().join(" && ");
                            if cond_str == "otherwise" {
                                out.push(format!("{}{} => {{ {} }}",
                                    state.tab().indent(),
                                    label_str,
                                    print_expr(state.tab(), &arm),
                                ));
                            } else {
                                out.push(format!("{}{} if {} => {{ {} }}",
                                    state.tab().indent(),
                                    label_str,
                                    cond_str,
                                    print_expr(state.tab(), &arm),
                                ));
                            }
                        }
                    }
                    ast::CaseCond::Direct(labels, arms) => {
                        out.push(format!("{}{} => {{\n{}{}\n{}}},",
                            state.tab().indent(),
                            print_case_patterns(state, labels),
                            state.tab().tab().indent(),
                            arms.iter()
                                .map(|x| print_expr(state.tab().tab(), x))
                                .collect::<Vec<_>>()
                                .join("; "),
                            state.tab().indent(),
                        ));
                    }
                }
            }
            format!("match {} {{\n{}\n{}}}", print_expr(state.tab(), cond), out.join("\n"), state.indent())
        }
        Lambda(ref pats, ref body) => {
            format!("|{}| {{ {} }}",
                print_patterns(state.tab(), pats),
                print_expr(state.tab(), body))
        }
        Operator(ref value) => {
            print_op_fn(value)
        }
        Generator(..) => {
            format!("/* Expr::Generator */ Generator")
        }
        RecordArgs(..) | Error => {
            format!("/* Expr::Error */ Error")
        }
    };
    ir::Expr::Free(freeform)
}

pub fn unpack_fndef(t: Ty) -> Vec<Ty> {
    match t {
        Ty::Pair(a, b) => {
            let mut v = vec![*a];
            v.extend(unpack_fndef(*b));
            v
        }
        _ => {
            vec![t]
        }
    }
}



// Expand a sequence of expression terms into a nested tree of operators
// TODO Operator precedence rules should be applied here
pub fn pat_explode(span: Vec<Pat>) -> Vec<Pat> {
    if span.len() < 3 {
        return span;
    }
    for i in 0..span.len() {
        if let &ast::Pat::Operator(ref op) = &span[i] {
            if op == ":" {
                return vec![ast::Pat::Concat(
                    Box::new(Pat::Span(pat_explode(span[0..i].to_vec().clone()))),
                    Box::new(Pat::Span(pat_explode(span[i+1..].to_vec().clone()))),
                )];
            } else {
                //TODO exhaustiveness check
                continue;
                //panic!("dont know how to translate {:?}", op);
            }
        }
    }
    span
}

pub fn print_pattern(state: PrintState, pat: &Pat) -> String {
    match *pat {
        Pat::Ref(ast::Ident(ref s)) =>{
            print_type_ident(state, s)
        }
        Pat::Span(ref span) => {
            let span = pat_explode(span.to_vec());
            let mut out_span = print_pattern(state.tab(), &span[0]);
            if span.len() > 1 {
                out_span.push_str(&format!("({})", print_patterns(state.tab(), &span[1..])));
            }
            out_span
        }
        Pat::Str(ref s) => {
            format!("{:?}", s)
        }
        Pat::Char(ref s) => {
            assert!(s.len() == 1, "char lit {:?}", s);
            format!("{:?}", s.chars().next().unwrap())
        }
        Pat::Num(n) => format!("{}", n),
        Pat::Tuple(ref pats) => {
            if pats.len() == 1 {
                print_pattern(state.tab(), &pats[0])
            } else {
                format!("({})", print_patterns(state.tab(), pats))
            }
        }
        Pat::Brackets(ref pats) => {
            format!("[{}]", print_patterns(state.tab(), pats))
        }
        Pat::Record(ref id, ref items) => {
            let mut out = vec![];
            for &(ast::Ident(ref i), ref v) in items {
                out.push(format!("{}{}: {}",
                    state.tab().indent(),
                    i,
                    print_pattern(state.tab().tab(), v)));
            }
            format!("{} {{\n{}\n{}}}",
                print_type_ident(state, &id.0),
                out.join(",\n"), state.indent())
        }
        Pat::ViewPattern(ast::Ident(ref s), ref p) => {
            format!("/* TODO ViewPattern */ {}", s) // print_pattern(state.tab(), &**p))
        }
        Pat::Not(ref s) => print_pattern(state, &**s),
        Pat::EmptyParen => format!("()"),
        Pat::Concat(ref a, ref b) => {
            format!("[{}, {}]", // TODO expand second argument
                print_pattern(state.tab(), &**a),
                print_pattern(state.tab(), &**b),
            )
        }
        Pat::Operator(_) => {
            // Should only be @
            format!("__OP__")
        }
        Pat::Infix(ref ident) => {
            errln!("Infix pattern `{}` was not rearranged", ident.0);
            format!("/* TODO(INFIX) */")
        }
    }
}


pub fn print_type<T: Borrow<Ty>>(state: PrintState, t: T) -> String {
    match *t.borrow() {
        Ty::Ref(ast::Ident(ref s)) => {
            print_type_ident(state, s)
        }
        Ty::Not(ref s) => {
            print_type(state, &**s)
        }
        Ty::Span(ref span) => {
            let mut out_span = print_type(state.tab(), &span[0]);
            if span.len() > 1 {
                out_span.push_str(&format!("<{}>", print_types(state.tab(), &span[1..])));
            }
            out_span
        }
        Ty::Tuple(ref spans) => {
            if spans.len() == 1 {
                print_type(state.tab(), &spans[0])
            } else {
                format!("({})", print_types(state.tab(), spans))
            }
        }
        Ty::Brackets(ref t) => {
            format!("Vec<{}>", print_type(state.tab(), &**t))
        }
        Ty::Pair(ref a, ref b) => {
            format!("fn({}) -> {}", print_type(state, &**a), print_type(state, &**b))
        }
        Ty::Record(ref items) => {
            let mut out = vec![];
            for &(ast::Ident(ref i), ref v) in items {
                out.push(format!("{}{}: {}",
                    state.tab().indent(),
                    i,
                    print_type(state.tab().tab(), v)));
            }
            format!("{{\n{}\n{}}}",
                out.join(",\n"), state.indent())
        }
        Ty::EmptyParen => "()".to_string(),
        Ty::RangeOp => ".. /* todo range */".to_string(),
        Ty::Dummy => "()".to_string(),
    }
}

pub fn print_types<I, T>(state: PrintState, iter: I) -> String
where
    I: IntoIterator<Item = T>,
    T: Borrow<Ty>,
{
    iter.into_iter().map(|t| print_type(state, t)).collect::<Vec<_>>().join(", ")
}

pub fn print_patterns<I, T>(state: PrintState, iter: I) -> String
where
    I: IntoIterator<Item = T>,
    T: Borrow<Pat>,
{
    iter.into_iter().map(|p| print_pattern(state, p.borrow())).collect::<Vec<_>>().join(", ")
}

pub fn print_case_patterns<I, T>(state: PrintState, iter: I) -> String
where
    I: IntoIterator<Item = T>,
    T: Borrow<Pat>,
{
    iter.into_iter().map(|p| print_pattern(state, p.borrow())).collect::<Vec<_>>().join(" | ")
}

pub fn print_item_list(state: PrintState, stats: &[ast::Item], toplevel: bool) -> String {
    let mut types = btreemap![];
    for item in stats {
        //errln!("{:?}", item);

        // println!("well {:?}", item);
        if let &ast::Item::Prototype(ref idents, ref d) = item {
            for &ast::Ident(ref s) in idents {
                if types.contains_key(&s) {
                    panic!("that shouldn't happen {:?}", s);
                }
                types.insert(s, d.clone());
            }
        }
    }

    // Output
    let mut out = vec![];

    // Print out imports.
    let mut first = true;
    for item in stats {
        if let ast::Item::Import(ref imports) = *item {
            if first {
                out.push(format!("// NOTE: These imports are advisory. You probably need to change them to support Rust."))
            }
            first = false;

            for import in imports {
                if import.len() > 0 {
                    out.push(format!("// use {};", import[0].0.replace(".", "::")));
                }
            }
        }
    }
    out.push(format!(""));

    // Print out data structures & type aliases.
    for item in stats {
        let mut item = item.clone();

        if let ast::Item::Newtype(name, ty, derives, args) = item.clone() {
            item = ast::Item::Data(name, vec![vec![ty]], derives, args);
        }

        if let ast::Item::Data(name, data, derives, args) = item {
            let derive_rust = derives.iter()
                .map(|x| {
                    // Convert common Haskell "derive" terms into Rust's
                    if x.0 == "Data" {
                        format!("Clone")
                    } else if x.0 == "Typeable" || x.0 == "Show" {
                        format!("Debug")
                    } else {
                        x.0.to_string()
                    }
                })
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            // Enum
            if data.len() > 1 {
                out.push(format!("{}{}pub enum {} {{\n{}{}\n{}}}\n{}{}",
                    state.indent(),
                    if derive_rust.len() > 0 {
                        format!("#[derive({})]\n{}", derive_rust.join(", "), state.indent())
                    } else {
                        format!("")
                    },
                    print_type(state, Ty::Span({
                        let mut v = vec![Ty::Ref(name.clone())];
                        v.extend(args);
                        v
                    })),
                    state.tab().indent(),
                    data.iter().map(|tyset| {
                        let mut tyset = tyset.clone();
                        if tyset.len() == 1 {
                            if let Ty::Span(values) = tyset[0].clone() {
                                tyset = values;
                            }
                        }
                        format!("{}{}",
                            print_type(state.tab(), tyset[0].clone()),
                            if tyset.len() > 2 {
                                format!("{}", print_type(state, Ty::Tuple(tyset.clone()[1..].to_vec())))
                            } else if tyset.len() == 2 && matches!(&tyset[1], &Ty::Record(..)) {
                                format!("{}", print_type(state, Ty::Tuple(tyset.clone()[1..].to_vec())))
                            } else if tyset.len() > 1 {
                                format!("({})", print_type(state, Ty::Tuple(tyset.clone()[1..].to_vec())))
                            } else {
                                "".to_string()
                            },
                        )
                    }).collect::<Vec<_>>().join(&format!(",\n{}", state.tab().indent())),
                    state.indent(),
                    state.indent(),
                    format!("pub use self::{}::*;", print_ident(state, name.0)),
                ));
            } else {
                let props = if data.len() == 0 {
                    format!(";")
                } else {
                    let tyset = data[0].clone(); // 0th in pipe sequence
                    if let &Ty::Span(ref inner) = &tyset[0] {
                        if let &Ty::Record(..) = &inner[1] {
                            assert!(inner.len() == 2);
                            print_type(state, &inner[1])
                        } else {
                            format!("({});", print_types(state, &inner[1..]))
                        }
                    } else {
                        unreachable!();
                    }
                };

                let mut accessors = vec![];
                if data.len() > 0 {
                    let tyset = data[0].clone(); // 0th in pipe sequence
                    if let &Ty::Span(ref inner) = &tyset[0] {
                        if let &Ty::Record(ref args) = &inner[1] {
                            for arg in args {
                                accessors.push(format!("{}fn {}(a: {}) -> {} {{ a.{} }}",
                                    state.indent(),
                                    print_type_ident(state, &(arg.0).0),
                                    print_type_ident(state, &name.0),
                                    print_type(state, &arg.1),
                                    print_type_ident(state, &(arg.0).0),
                                ));
                            }
                        } else {
                        }
                    } else {
                        unreachable!();
                    }
                }

                out.push(format!("{}{}pub struct {}{}\n{}",
                    state.indent(),
                    if derive_rust.len() > 0 {
                        format!("#[derive({})]\n{}", derive_rust.join(", "), state.indent())
                    } else {
                        format!("")
                    },
                    print_type(state, Ty::Span({
                        let mut v = vec![Ty::Ref(name)];
                        v.extend(args);
                        v
                    })),
                    props,
                    accessors.join("\n"),
                ));
            }
            out.push("".to_string())
        } else if let ast::Item::Type(name, data, args) = item.clone() {
            assert!(data.len() > 0);
            let tyset = data[0].clone(); // 0th in pipe sequence
            let props = format!(" = {};", print_type(state, &tyset));
            out.push(format!("{}pub type {}{}\n",
                state.indent(),
                print_type(state, Ty::Span({
                    let mut v = vec![Ty::Ref(name)];
                    v.extend(args);
                    v
                })),
                props,
            ));
        }
    }

    // Print out assignments as fns
    let mut cache: BTreeMap<String, Vec<(ast::Assignment, Vec<ast::Item>)>> = btreemap![];
    for item in stats {
        if let ast::Item::Assign(assign, where_) = item.clone() {
            if !where_.is_empty() {
                // TODO
                //println!("// push {:?} into fn", where_)
                // out.push(print_item_list(state.tab(), &where_, false));
            }

            let mut assign = *assign;

            // If hte AST is refactored to break out the first Ident
            // for the issigned, this whole check should be deleted
            let ident = match assign {
              ast::Assignment::Assign { ref mut pats, .. }
              | ast::Assignment::Case { ref mut pats, .. } =>
                    match pats.remove(0) {
                        Pat::Ref(ast::Ident(s)) => s,
                        span => {
                            errln!("Expected ident, got {:?}\n\nin: {:?}\n", span, item);
                            continue;
                        }
                    }
            };

            cache.entry(ident)
                .or_insert(vec![])
                .push((assign, where_));
        }
    }

    // Convert guards into basically case statements
    let mut new_cache: BTreeMap<String, Vec<(ast::Assignment, Vec<ast::Item>)>> = btreemap![];
    for (key, mut fnset) in cache {
        let fnset2 = fnset.clone();
        if fnset.len() > 1 {
            // There are multiple impls of this function, so expand this into a
            // case statement.
            if let (ast::Assignment::Assign { ref mut pats, ref mut expr }, ..) = fnset[0] {
                let args = (0..pats.len())
                    .map(|x| format!("_{}", x))
                    .collect::<Vec<_>>();

                // Convert args into case options.
                let src_pats = args
                    .iter()
                    .map(|x| Pat::Ref(ast::Ident(x.to_string())))
                    .collect::<Vec<_>>();

                // Generate case statements.
                let expr = ast::Expr::Case(
                    Box::new(ast::Expr::Parens(args.iter()
                        .map(|x| ast::Expr::Ref(ast::Ident(x.to_string())))
                        .collect::<Vec<_>>())),
                    fnset2.iter().map(|&(ref x, _)| {
                        match x {
                            &ast::Assignment::Assign { ref pats, .. } => {
                                ast::CaseCond::Direct(
                                    vec![Pat::Tuple(pats.clone())],
                                    vec![expr.clone()])
                            }
                            &ast::Assignment::Case { ref pats, .. } => {
                                ast::CaseCond::Direct(
                                    vec![Pat::Tuple(pats.clone())],
                                    vec![expr.clone()])
                            }
                        }
                    }).collect::<Vec<_>>(),
                );
                let res = vec![(ast::Assignment::Assign { pats: src_pats, expr }, vec![])];
                new_cache.insert(key, res);
            } else {
                // unreachable!();
            }
        } else {
            new_cache.insert(key, fnset);
        }
    }

    for (key, fnset) in new_cache {
        for (assign, where_) in fnset {
            if let ast::Assignment::Assign { pats: args, expr } = assign {
                // For type-less functions,
                if !types.contains_key(&key) {
                    // TODO Unless we can infer top-level types, we just bail.
                    if toplevel {
                        panic!("Cannot print untyped fn {:?}", key);
                    } else {
                        let let_str = print_let(state, &ast::Assignment::Assign { pats: {
                            let mut out = vec![ast::Pat::Ref(ast::Ident(key.to_string()))];
                            out.extend(args);
                            out
                        }, expr });
                        out.push(format!("{}", let_str));
                        continue;
                    }
                }

                let d = types[&key].clone();
                //assert!(d.len() == 1);
                //TODO what did this assert do
                let t = unpack_fndef(d[0].clone());
                assert!(t.len() >= 1);

                let mut args_span = vec![];
                for (arg, ty) in args.iter().zip(t.iter()) {
                    args_span.push(format!("{}: {}", print_pattern(state, arg), print_type(state.tab(), ty)));
                }
                let args_str = args_span.join(", ");

                let ret_str = print_type(state.tab(), t.last().unwrap());

                let re = Regex::new(r"\b(a|b)\b").unwrap();
                let mut type_args = re.captures_iter(&args_str)
                    .map(|x| x[1].to_string())
                    .collect::<::std::collections::HashSet<_>>();
                
                for item in re.captures_iter(&ret_str) {
                    type_args.insert(item[1].to_string());
                }

                let mut type_args = type_args.into_iter().collect::<Vec<_>>();
                type_args.sort();

                // Format where clause
                let mut where_str = format!("");
                if where_.len() > 0 {
                    where_str = format!("{}\n",
                        print_item_list(state.tab(), &where_, false));
                }

                let trans_name = print_type_ident(state, &key);
                out.push(
                    format!("{}pub fn {}{}({}) -> {} {{\n{}{}{}\n{}}}\n",
                        state.indent(),
                        trans_name,
                        if type_args.len() > 0 {
                            format!("<{}>", type_args.join(", "))
                        } else {
                            format!("")
                        },
                        args_str,
                        ret_str,
                        where_str,
                        state.tab().indent(),
                        print_expr(state.tab(), &expr),
                        state.indent()));
            } else {
                // TODO
            }
        }
    }

    out.join("\n")
}

pub fn print_let(state: PrintState, assign: &ast::Assignment) -> String {
    // With no type signature, we print a lambda.
    // If there are no arguments, we compute it now (non-lazily).
    let (pats, expr) = match assign {
      &ast::Assignment::Assign { ref pats, ref expr } => (pats, expr),
      _ => unreachable!(),
    };
    if pats.len() == 0 {
        // TODO why does this case occur?
        format!("")
    } else if pats.len() == 1 {
        format!("{}let {} = {};\n",
            state.indent(),
            print_pattern(state, &pats[0]),
            print_expr(state.tab(), expr))
    } else {
        format!("{}let {} = |{}| {{\n{}{}\n{}}};\n",
            state.indent(),
            print_pattern(state, &pats[0]),
            print_patterns(state, &pats[1..]),
            state.tab().indent(),
            print_expr(state.tab(), expr),
            state.indent())
    }
}

pub fn print_do(state: PrintState, stmts: &[ast::DoItem], items: &[ast::Item]) -> String {
    let mut whence = print_item_list(state, items, false);
    if whence.len() > 0 {
        whence.push_str("\n\n");
    }

    let mut body = vec![];
    for (i, stmt) in stmts.iter().enumerate() {
        match *stmt {
            ast::DoItem::Let(ref assigns) => {
                for assign in assigns {
                    body.push(print_let(state, assign));
                }
            }
            ast::DoItem::Bind(ref pats, ref expr) => {
                // good enough for now
                let assign = ast::Assignment::Assign { pats: pats.clone(), expr: *expr.clone() };
                body.push(print_let(state, &assign));
            }
            ast::DoItem::Expression(ref e) => {
                let mut expr = print_expr(state, &*e);
                if i + 1 < stmts.len() {
                    expr.push(';');
                }
                body.push(format!("{}{}", state.indent(), expr));
            }
        }
    }
    format!("/*do*/ {{\n{}{}\n{}}}", whence, body.join("\n"), state.untab().indent())
}
