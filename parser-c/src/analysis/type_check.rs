// Original file: "TypeCheck.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Control::Monad;
// use Data::Maybe;
// use Language::C::Data::Ident;
// use Language::C::Data::Node;
// use Language::C::Data::Position;
// use Language::C::Pretty;
// use Language::C::Syntax::AST;
// use Language::C::Syntax::Constants;
// use Language::C::Syntax::Ops;
// use Language::C::Analysis::DefTable;
// use Language::C::Analysis::SemRep;
// use Language::C::Analysis::TravMonad;
// use Language::C::Analysis::TypeConversions;
// use Language::C::Analysis::TypeUtils;
// use Language::C::Analysis::Debug;
// use Text::PrettyPrint::HughesPJ;

pub fn assignCompatible(_0: CAssignOp, _1: Type, _2: Type) -> Either<String, ()> {
    match (_0, _1, _2) {
        (CAssignOp, t1, t2) => {
            match (canonicalType(t1), canonicalType(t2)) {
                (DirectType(TyBuiltin(TyAny), _, _), _) => {
                    ()
                },
                (_, DirectType(TyBuiltin(TyAny), _, _)) => {
                    ()
                },
                (PtrType(DirectType(TyVoid, _, _), _, _), t2_q) if isPointerType(t2_q) => { () }
                (t1_q, PtrType(DirectType(TyVoid, _, _), _, _)) if isPointerType(t1_q) => { () }
                (PtrType(_, _, _), t2_q) if isIntegralType(t2_q) => { () }
                (t1_q, t2_q) if (isPointerType(t1_q) && isPointerType(t2_q)) => { compatible((baseType(t1_q)), (baseType(t2_q))) }
                (DirectType(TyComp(c1), _, _), DirectType(TyComp(c2), _, _)) if (sueRef(c1) == sueRef(c2)) => { () }
                (DirectType(TyComp(c1), _, _), DirectType(TyComp(c2), _, _)) => { fail(__op_addadd("incompatible compound types in assignment: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))) }
                (DirectType(TyBuiltin(TyVaList), _, _), DirectType(TyBuiltin(TyVaList), _, _)) => {
                    ()
                },
                (DirectType(tn1, _, _), DirectType(tn2, _, _)) if isJust((arithmeticConversion(tn1, tn2))) => { () }
                (DirectType(tn1, _, _), DirectType(tn2, _, _)) => { fail(__op_addadd("incompatible direct types in assignment: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))) }
                (t1_q, t2_q) => {
                    compatible(t1_q, t2_q)
                },
            }
        },
        (op, t1, t2) => {
            match (canonicalType(t1), canonicalType(t2)) {
                (DirectType(TyBuiltin(TyAny), _, _), _) => {
                    ()
                },
                (_, DirectType(TyBuiltin(TyAny), _, _)) => {
                    ()
                },
                (PtrType(DirectType(TyVoid, _, _), _, _), t2_q) if isPointerType(t2_q) => { () }
                (t1_q, PtrType(DirectType(TyVoid, _, _), _, _)) if isPointerType(t1_q) => { () }
                (PtrType(_, _, _), t2_q) if isIntegralType(t2_q) => { () }
                (t1_q, t2_q) if (isPointerType(t1_q) && isPointerType(t2_q)) => { compatible((baseType(t1_q)), (baseType(t2_q))) }
                (DirectType(TyComp(c1), _, _), DirectType(TyComp(c2), _, _)) if (sueRef(c1) == sueRef(c2)) => { () }
                (DirectType(TyComp(c1), _, _), DirectType(TyComp(c2), _, _)) => { fail(__op_addadd("incompatible compound types in assignment: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))) }
                (DirectType(TyBuiltin(TyVaList), _, _), DirectType(TyBuiltin(TyVaList), _, _)) => {
                    ()
                },
                (DirectType(tn1, _, _), DirectType(tn2, _, _)) if isJust((arithmeticConversion(tn1, tn2))) => { () }
                (DirectType(tn1, _, _), DirectType(tn2, _, _)) => { fail(__op_addadd("incompatible direct types in assignment: ".to_string(), __op_addadd(pType(t1), __op_addadd(", ".to_string(), pType(t2))))) }
                (t1_q, t2_q) => {
                    compatible(t1_q, t2_q)
                },
            }
        },
    }
}

pub fn assignCompatible_q(ni: NodeInfo, op: CAssignOp, t1: Type, t2: Type) -> m<()> {
    typeErrorOnLeft(ni, (assignCompatible(op, t1, t2)))
}

pub fn binopType(op: CBinaryOp, t1: Type, t2: Type) -> Either<String, Type> {
    match (op, canonicalType(t1), canonicalType(t2)) {
        (_, t1_q, t2_q) if isLogicOp(op) => { __op_rshift(checkScalar(t1_q), __op_rshift(checkScalar(t2_q), boolType)) }
        (_, t1_q, t2_q) if isCmpOp(op) => { match (t1_q, t2_q) {
            (DirectType(tn1, _, _), DirectType(tn2, _, _)) => {
                match arithmeticConversion(tn1, tn2) {
                    Some(_) => {
                        boolType
                    },
                    None => {
                        fail(render(__op_doc_conat(text("incompatible arithmetic types in comparison: ".to_string()), __op_doc_conat(pretty(t1), __op_doc_conat(text("and".to_string()), pretty(t2))))))
                    },
                }
            },
            (PtrType(DirectType(TyVoid, _, _), _, _), _) if isPointerType(t2_q) => { boolType }
            (_, PtrType(DirectType(TyVoid, _, _), _, _)) if isPointerType(t1_q) => { boolType }
            (_, _) if (isPointerType(t1_q) && isIntegralType(t2_q)) => { boolType }
            (_, _) if (isIntegralType(t1_q) && isPointerType(t2_q)) => { boolType }
            (_, _) if (isPointerType(t1_q) && isPointerType(t2_q)) => { __op_rshift(compatible(t1_q, t2_q), boolType) }
            (_, _) => {
                fail("incompatible types in comparison".to_string())
            },
        } }
        (CSubOp, ArrayType(t1_q, _, _, _), ArrayType(t2_q, _, _, _)) => {
            __op_rshift(compatible(t1_q, t2_q), ptrDiffType)
        },
        (CSubOp, ArrayType(t1_q, _, _, _), PtrType(t2_q, _, _)) => {
            __op_rshift(compatible(t1_q, t2_q), ptrDiffType)
        },
        (CSubOp, PtrType(t1_q, _, _), ArrayType(t2_q, _, _, _)) => {
            __op_rshift(compatible(t1_q, t2_q), ptrDiffType)
        },
        (CSubOp, PtrType(t1_q, _, _), PtrType(t2_q, _, _)) => {
            __op_rshift(compatible(t1_q, t2_q), ptrDiffType)
        },
        (_, PtrType(_, _, _), t2_q) if (isPtrOp(op) && isIntegralType(t2_q)) => { t1 }
        (_, PtrType(_, _, _), t2_q) => { fail(__op_addadd("invalid pointer operation: ".to_string(), render((pretty(op))))) }
        (CAddOp, t1_q, PtrType(_, _, _)) if isIntegralType(t1_q) => { t2 }
        (_, ArrayType(_, _, _, _), t2_q) if (isPtrOp(op) && isIntegralType(t2_q)) => { t1 }
        (_, ArrayType(_, _, _, _), t2_q) => { fail(__op_addadd("invalid pointer operation: ".to_string(), render((pretty(op))))) }
        (CAddOp, t1_q, ArrayType(_, _, _, _)) if isIntegralType(t1_q) => { t2 }
        (_, DirectType(tn1, q1, a1), DirectType(tn2, q2, a2)) => {
            /*do*/ {
                if (isBitOp(op)) { (__op_rshift(checkIntegral(t1), checkIntegral(t2))) };
                match arithmeticConversion(tn1, tn2) {
                    Some(tn) => {
                        DirectType(tn, (mergeTypeQuals(q1, q2)), (mergeAttributes(a1, a2)))
                    },
                    None => {
                        fail(render(__op_doc_conat(text("invalid binary operation:".to_string()), __op_doc_conat(pretty(t1), __op_doc_conat(pretty(op), pretty(t2))))))
                    },
                }
            }
        },
        (_, _, _) => {
            fail(render(__op_doc_conat(text("unhandled binary operation:".to_string()), __op_doc_conat(pretty(t1), __op_doc_conat(pretty(op), pretty(t2))))))
        },
    }
}

pub fn binopType_q(ni: NodeInfo, op: CBinaryOp, t1: Type, t2: Type) -> m<Type> {
    typeErrorOnLeft(ni, (binopType(op, t1, t2)))
}

pub fn castCompatible(t1: Type, t2: Type) -> Either<String, ()> {
    match (canonicalType(t1), canonicalType(t2)) {
        (DirectType(TyVoid, _, _), _) => {
            ()
        },
        (_, _) => {
            __op_rshift(checkScalar(t1), checkScalar(t2))
        },
    }
}

pub fn checkIntegral_q(ni: NodeInfo) -> m<()> {
    typeErrorOnLeft(ni, checkIntegral)
}

pub fn checkScalar(t: Type) -> Either<String, ()> {
    match canonicalType(t) {
        DirectType(_, _, _) => {
            ()
        },
        PtrType(_, _, _) => {
            ()
        },
        ArrayType(_, _, _, _) => {
            ()
        },
        t_q => {
            fail(__op_addadd("expected scalar type, got: ".to_string(), __op_addadd(pType(t), __op_addadd(" (".to_string(), __op_addadd(pType(t_q), ")".to_string())))))
        },
    }
}

pub fn checkScalar_q(ni: NodeInfo) -> m<()> {
    typeErrorOnLeft(ni, checkScalar)
}

pub fn compatible(t1: Type, t2: Type) -> Either<String, ()> {
    void(compositeType(t1, t2))
}

pub fn compositeDeclAttrs(DeclAttrs(inl, stor, attrs1): DeclAttrs, DeclAttrs(_, _, attrs2): DeclAttrs) -> DeclAttrs {
    DeclAttrs(inl, stor, (mergeAttrs(attrs1, attrs2)))
}

pub fn compositeParamDecl(_0: ParamDecl, _1: ParamDecl) -> Either<String, ParamDecl> {
    match (_0, _1) {
        (ParamDecl(vd1, ni1), ParamDecl(vd2, _)) => {
            compositeParamDecl_q(ParamDecl, vd1, vd2, ni1)
        },
        (AbstractParamDecl(vd1, _), ParamDecl(vd2, ni2)) => {
            compositeParamDecl_q(ParamDecl, vd1, vd2, ni1)
        },
        (ParamDecl(vd1, ni1), AbstractParamDecl(vd2, _)) => {
            compositeParamDecl_q(ParamDecl, vd1, vd2, ni1)
        },
        (AbstractParamDecl(vd1, ni1), AbstractParamDecl(vd2, _)) => {
            compositeParamDecl_q(ParamDecl, vd1, vd2, ni1)
        },
    }
}

pub fn compositeParamDecl_q(f: fn(VarDecl) -> fn(NodeInfo) -> ParamDecl, VarDecl(n1, attrs1, t1): VarDecl, VarDecl(n2, attrs2, t2): VarDecl, dni: NodeInfo) -> Either<String, ParamDecl> {

    let t1_q = canonicalType(t1);

    let t2_q = canonicalType(t2);

    /*do*/ {
        let vd = compositeVarDecl((VarDecl(n1, attrs1, t1_q)), (VarDecl(n2, attrs2, t2_q)));

        f(vd, dni)
    }
}

pub fn compositeSize(_0: ArraySize, _1: ArraySize) -> Either<String, ArraySize> {
    match (_0, _1) {
        (UnknownArraySize(_), s2) => {
            s2
        },
        (s1, UnknownArraySize(_)) => {
            s2
        },
        (ArraySize(s1, e1), ArraySize(s2, e2)) => {
            s2
        },
    }
}

pub fn compositeType(_0: Type, _1: Type) -> Either<String, Type> {
    match (_0, _1) {
        (t1, DirectType(TyBuiltin(TyAny), _, _)) => {
            t1
        },
        (DirectType(TyBuiltin(TyAny), _, _), t2) => {
            t1
        },
        (t1, __OP__, DirectType(tn1, q1, a1), t2, __OP__, DirectType(tn2, q2, a2)) => {
            t1
        },
        (PtrType(t1, q1, a1), PtrType(DirectType(TyVoid, _, _), q2, _)) => {
            t1
        },
        (PtrType(DirectType(TyVoid, _, _), q1, _), PtrType(t2, q2, a2)) => {
            t1
        },
        (PtrType(t1, q1, a1), t2) => {
            t1
        },
        (t1, PtrType(t2, q2, a2)) => {
            t1
        },
        (ArrayType(t1, _sz1, q1, a1), t2) => {
            t1
        },
        (t1, ArrayType(t2, _sz2, q2, a2)) => {
            t1
        },
        (ArrayType(t1, s1, q1, a1), ArrayType(t2, s2, q2, a2)) => {
            t1
        },
        (t1, t2) => {
            t1
        },
        (TypeDefType(tdr1, _q1, _a1), TypeDefType(tdr2, _q2, _a2)) => {
            t1
        },
        (FunctionType(ft1, attrs1), FunctionType(ft2, attrs2)) => {
            t1
        },
        (t1, t2) => {
            t1
        },
    }
}

pub fn compositeVarDecl(VarDecl(n1, attrs1, t1): VarDecl, VarDecl(_, attrs2, t2): VarDecl) -> Either<String, VarDecl> {
    /*do*/ {
        let t = compositeType(t1, t2);

        (VarDecl(n1, (compositeDeclAttrs(attrs1, attrs2)), t))
    }
}

pub fn conditionalType(t1: Type, t2: Type) -> Either<String, Type> {
    match (canonicalType(t1), canonicalType(t2)) {
        (PtrType(DirectType(TyVoid, _, _), _, _), t2_q) if isPointerType(t2_q) => { t2 }
        (t1_q, PtrType(DirectType(TyVoid, _, _), _, _)) if isPointerType(t1_q) => { t1 }
        (ArrayType(t1_q, _, q1, a1), ArrayType(t2_q, _, q2, a2)) => {
            /*do*/ {
                let t = compositeType(t1_q, t2_q);

                ArrayType(t, (UnknownArraySize(false)), (mergeTypeQuals(q1, q2)), (mergeAttrs(a1, a2)))
            }
        },
        (t1_q(__OP__, DirectType(tn1, q1, a1)), t2_q(__OP__, DirectType(tn2, q2, a2))) => {
            match arithmeticConversion(tn1, tn2) {
                Some(tn) => {
                    DirectType(tn, (mergeTypeQuals(q1, q2)), (mergeAttributes(a1, a2)))
                },
                None => {
                    compositeType(t1_q, t2_q)
                },
            }
        },
        (t1_q, t2_q) => {
            compositeType(t1_q, t2_q)
        },
    }
}

pub fn conditionalType_q(ni: NodeInfo, t1: Type, t2: Type) -> m<Type> {
    typeErrorOnLeft(ni, conditionalType(t1, t2))
}

pub fn constType(_0: CConst) -> m<Type> {
    match (_0) {
        CIntConst(CInteger(_, _, flags), _) => {
            DirectType((TyIntegral((getIntType(flags)))), noTypeQuals, noAttributes)
        },
        CCharConst(CChar(_, true), _) => {
            DirectType((TyIntegral((getIntType(flags)))), noTypeQuals, noAttributes)
        },
        CCharConst(CChar(_, false), _) => {
            DirectType((TyIntegral((getIntType(flags)))), noTypeQuals, noAttributes)
        },
        CCharConst(CChars(_, _), _) => {
            DirectType((TyIntegral((getIntType(flags)))), noTypeQuals, noAttributes)
        },
        CFloatConst(CFloat(fs), _) => {
            DirectType((TyIntegral((getIntType(flags)))), noTypeQuals, noAttributes)
        },
        CStrConst(CString(chars, wide), ni) => {
            DirectType((TyIntegral((getIntType(flags)))), noTypeQuals, noAttributes)
        },
    }
}

pub fn deepTypeAttrs(_0: Type) -> m<Attributes> {
    match (_0) {
        DirectType(TyComp(CompTypeRef(sue, _, ni)), _, attrs) => {
            liftM((attrs(__op_addadd)), sueAttrs(ni, sue))
        },
        DirectType(TyEnum(EnumTypeRef(sue, ni)), _, attrs) => {
            liftM((attrs(__op_addadd)), sueAttrs(ni, sue))
        },
        DirectType(_, _, attrs) => {
            liftM((attrs(__op_addadd)), sueAttrs(ni, sue))
        },
        PtrType(t, _, attrs) => {
            liftM((attrs(__op_addadd)), sueAttrs(ni, sue))
        },
        ArrayType(t, _, _, attrs) => {
            liftM((attrs(__op_addadd)), sueAttrs(ni, sue))
        },
        FunctionType(FunType(t, _, _), attrs) => {
            liftM((attrs(__op_addadd)), sueAttrs(ni, sue))
        },
        FunctionType(FunTypeIncomplete(t), attrs) => {
            liftM((attrs(__op_addadd)), sueAttrs(ni, sue))
        },
        TypeDefType(TypeDefRef(i, _, ni), _, attrs) => {
            liftM((attrs(__op_addadd)), sueAttrs(ni, sue))
        },
    }
}

pub fn derefType(_0: Type) -> Either<String, Type> {
    match (_0) {
        PtrType(t, _, _) => {
            t
        },
        ArrayType(t, _, _, _) => {
            t
        },
        t => {
            t
        },
    }
}

pub fn expandAnonymous(_0: NodeInfo, _1: (VarName, Type)) -> m<Vec<(Ident, Type)>> {
    match (_0, _1) {
        (ni, (NoName, DirectType(TyComp(ctr), _, _))) => {
            __op_bind(lookupSUE(ni, (sueRef(ctr))), tagMembers(ni))
        },
        (_, (NoName, _)) => {
            __op_bind(lookupSUE(ni, (sueRef(ctr))), tagMembers(ni))
        },
        (_, (VarName(n, _), t)) => {
            __op_bind(lookupSUE(ni, (sueRef(ctr))), tagMembers(ni))
        },
    }
}

pub fn fieldType(ni: NodeInfo, m: Ident, t: Type) -> m<Type> {
    match canonicalType(t) {
        DirectType(TyComp(ctr), _, _) => {
            /*do*/ {
                let td = lookupSUE(ni, (sueRef(ctr)));

                let ms = tagMembers(ni, td);

                match lookup(m, ms) {
                    Some(ft) => {
                        ft
                    },
                    None => {
                        typeError(ni, __op_addadd("field not found: ".to_string(), identToString(m)))
                    },
                }
            }
        },
        _t_q => {
            astError(ni, __op_addadd("field of non-composite type: ".to_string(), __op_addadd(identToString(m), __op_addadd(", ".to_string(), pType(t)))))
        },
    }
}

pub fn lookupSUE(ni: NodeInfo, sue: SUERef) -> m<TagDef> {
    /*do*/ {
        let dt = getDefTable;

        match lookupTag(sue, dt) {
            Some(Right(td)) => {
                td
            },
            _ => {
                typeError(ni, __op_addadd("unknown composite type: ".to_string(), (render(pretty))(sue)))
            },
        }
    }
}

pub fn mergeAttrs() -> Attributes {
    (__op_addadd)
}

pub fn notFound<a>(i: Ident) -> Either<String, a> {
    Left(__op_addadd("not found: ".to_string(), identToString(i)))
}

pub fn pType() -> String {
    render(pretty)
}

pub fn sizeEqual(_0: CExpr, _1: CExpr) -> bool {
    match (_0, _1) {
        (CConst(CIntConst(i1, _)), CConst(CIntConst(i2, _))) => {
            (i1 == i2)
        },
        (e1, e2) => {
            (i1 == i2)
        },
    }
}

pub fn sueAttrs(ni: NodeInfo, sue: SUERef) -> m<Attributes> {
    /*do*/ {
        let dt = getDefTable;

        match lookupTag(sue, dt) {
            None => {
                astError(ni, __op_addadd("SUE not found: ".to_string(), render((pretty(sue)))))
            },
            Some(Left(_)) => {
                vec![]
            },
            Some(Right(CompDef(CompType(_, _, _, attrs, _)))) => {
                attrs
            },
            Some(Right(EnumDef(EnumType(_, _, attrs, _)))) => {
                attrs
            },
        }
    }
}

pub fn tagMembers(ni: NodeInfo, td: TagDef) -> m<Vec<(Ident, Type)>> {

    let getMembers = |ds| {
        /*do*/ {
            let ts = __map!(declType, ds);

            let ns = __map!(declName, ds);

            liftM(concat, mapM((expandAnonymous(ni)), (zip(ns, ts))))
        }
    };

    match td {
        CompDef(CompType(_, _, ms, _, _)) => {
            getMembers(ms)
        },
        EnumDef(EnumType(_, es, _, _)) => {
            getMembers(es)
        },
    }
}

pub fn typeDefAttrs(ni: NodeInfo, i: Ident) -> m<Attributes> {
    /*do*/ {
        let dt = getDefTable;

        match lookupIdent(i, dt) {
            None => {
                astError(ni, __op_addadd("can\'t find typedef name: ".to_string(), identToString(i)))
            },
            Some(Left(TypeDef(_, t, attrs, _))) => {
                liftM((attrs(__op_addadd)), deepTypeAttrs(t))
            },
            Some(Right(_)) => {
                astError(ni, __op_addadd("not a typedef name: ".to_string(), identToString(i)))
            },
        }
    }
}

pub fn typeError<a>() -> m<a> {
    astError
}

pub fn typeErrorOnLeft<a>(_0: NodeInfo, _1: Either<String, a>) -> m<a> {
    match (_0, _1) {
        (ni, Left(err)) => {
            typeError(ni, err)
        },
        (_, Right(v)) => {
            typeError(ni, err)
        },
    }
}

pub fn varAddrType(d: IdentDecl) -> Either<String, Type> {

    let t = declType(d);

    /*do*/ {
        match declStorage(d) {
            Auto(true) => {
                fail("address of register variable".to_string())
            },
            _ => {
                ()
            },
        };
        match t {
            ArrayType(_, _, q, a) => {
                PtrType(t, q, a)
            },
            _ => {
                simplePtr(t)
            },
        }
    }
}



