// Original file: "DefTable.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data;
// use Language::C::Analysis::NameSpaceMap;
// use Language::C::Analysis::SemRep;
// use Data::Map;
// use Data::IntMap;
// use IntMap;
// use Data::IntMap;
// use Data::Generics;

pub type IdentEntry = Either<TypeDef, IdentDecl>;

pub enum TagFwdDecl {
    CompDecl(CompTypeRef),
    EnumDecl(EnumTypeRef)
}
pub use self::TagFwdDecl::*;

pub type TagEntry = Either<TagFwdDecl, TagDef>;

pub struct DefTable{
    identDecls: NameSpaceMap<Ident, IdentEntry>,
    tagDecls: NameSpaceMap<SUERef, TagEntry>,
    labelDefs: NameSpaceMap<Ident, Ident>,
    memberDecls: NameSpaceMap<Ident, MemberDecl>,
    refTable: IntMap<Name>,
    typeTable: IntMap<Type>
}
fn identDecls(a: DefTable) -> NameSpaceMap<Ident, IdentEntry> { a.identDecls }
fn tagDecls(a: DefTable) -> NameSpaceMap<SUERef, TagEntry> { a.tagDecls }
fn labelDefs(a: DefTable) -> NameSpaceMap<Ident, Ident> { a.labelDefs }
fn memberDecls(a: DefTable) -> NameSpaceMap<Ident, MemberDecl> { a.memberDecls }
fn refTable(a: DefTable) -> IntMap<Name> { a.refTable }
fn typeTable(a: DefTable) -> IntMap<Type> { a.typeTable }

#[derive(Clone, Debug)]
pub enum DeclarationStatus<t> {
    NewDecl,
    Redeclared(t),
    KeepDef(t),
    Shadowed(t),
    KindMismatch(t)
}
pub use self::DeclarationStatus::*;

#[derive(Eq, Ord)]
pub enum TagEntryKind {
    CompKind(CompTyKind),
    EnumKind
}
pub use self::TagEntryKind::*;

pub fn compatIdentEntry(_0: IdentEntry) -> bool {
    match (_0) {
        Left(_tydef) => {
            either((__TODO_const(true)), (__TODO_const(false)))
        },
        Right(def) => {
            either((__TODO_const(true)), (__TODO_const(false)))
        },
    }
}

pub fn compatTagEntry(te1: TagEntry, te2: TagEntry) -> bool {
    (tagKind(te1) == tagKind(te2))
}

pub fn declStatusDescr(_0: DeclarationStatus<t>) -> String {
    match (_0) {
        NewDecl => {
            "new".to_string()
        },
        Redeclared(_) => {
            "new".to_string()
        },
        KeepDef(_) => {
            "new".to_string()
        },
        Shadowed(_) => {
            "new".to_string()
        },
        KindMismatch(_) => {
            "new".to_string()
        },
    }
}

pub fn declareTag(sueref: SUERef, decl: TagFwdDecl, deftbl: DefTable) -> (DeclarationStatus<TagEntry>, DefTable) {
    match lookupTag(sueref, deftbl) {
        None => {
            (NewDecl, deftbl {
                tagDecls: fst(defLocal((tagDecls(deftbl)), sueref, (Left(decl))))
            })
        },
        Some(old_def) if (tagKind(old_def) == tagKind((Left(decl)))) => { (KeepDef(old_def), deftbl) }
        Some(old_def) => { (KindMismatch(old_def), deftbl) }
    }
}

pub fn defRedeclStatus(sameKind: fn(t) -> fn(t) -> bool, def: t, oldDecl: Option<t>) -> DeclarationStatus<t> {
    match oldDecl {
        Some(def_q) if sameKind(def, def_q) => { Redeclared(def_q) }
        Some(def_q) => { KindMismatch(def_q) }
        None => {
            NewDecl
        },
    }
}

pub fn defRedeclStatusLocal(sameKind: fn(t) -> fn(t) -> bool, ident: k, def: t, oldDecl: Option<t>, nsm: NameSpaceMap<k, t>) -> DeclarationStatus<t> {
    match defRedeclStatus(sameKind, def, oldDecl) {
        NewDecl => {
            match lookupName(nsm, ident) {
                Some(shadowed) => {
                    Shadowed(shadowed)
                },
                None => {
                    NewDecl
                },
            }
        },
        redecl => {
            redecl
        },
    }
}

pub fn defineGlobalIdent(ident: Ident, def: IdentDecl, deftbl: DefTable) -> (DeclarationStatus<IdentEntry>, DefTable) {

    (defRedeclStatus(compatIdentEntry, (Right(def)), oldDecl), deftbl {
        identDecls: decls_q
    })
}

pub fn defineLabel(ident: Ident, deftbl: DefTable) -> (DeclarationStatus<Ident>, DefTable) {
    {
        let (labels_q, old_label) = defLocal((labelDefs(deftbl)), ident, ident);

    (maybe(NewDecl, Redeclared, old_label), deftbl {
            labelDefs: labels_q
        })    }
}

pub fn defineScopedIdent() -> (DeclarationStatus<IdentEntry>, DefTable) {
    defineScopedIdentWhen((__TODO_const(true)))
}

pub fn defineScopedIdentWhen(override_def: fn(IdentDecl) -> bool, ident: Ident, def: IdentDecl, deftbl: DefTable) -> (DeclarationStatus<IdentEntry>, DefTable) {

    let doOverride = |_0| {
        match (_0) {
            Left(_) => {
                false
            },
            Right(old_def) => {
                false
            },
        }
    };

    let new_decls = fst((defLocal(old_decls, ident, new_def)));

    let new_def = Right(def);

    let old_decl_opt = lookupInnermostScope(old_decls, ident);

    let old_decls = identDecls(deftbl);

    let redeclStatus_q = |overriden_decl| {
        defRedeclStatusLocal(compatIdentEntry, ident, new_def, overriden_decl, old_decls)
    };

    (redecl_status, deftbl {
        identDecls: decls_q
    })
}

pub fn defineTag(sueref: SUERef, def: TagDef, deftbl: DefTable) -> (DeclarationStatus<TagEntry>, DefTable) {

    let redeclStatus = match olddecl {
            Some(fwd_decl, __OP__, Left(_)) if (tagKind(fwd_decl) == tagKind((Right(def)))) => { NewDecl }
            Some(fwd_decl, __OP__, Left(_)) => { KindMismatch(fwd_decl) }
            _ => {
                defRedeclStatusLocal(compatTagEntry, sueref, (Right(def)), olddecl, (tagDecls(deftbl)))
            },
        };

    (redeclStatus, deftbl {
        tagDecls: decls_q
    })
}

pub fn defineTypeDef(ident: Ident, tydef: TypeDef, deftbl: DefTable) -> (DeclarationStatus<IdentEntry>, DefTable) {

    (defRedeclStatus(compatIdentEntry, (Left(tydef)), oldDecl), deftbl {
        identDecls: decls_q
    })
}

pub fn emptyDefTable() -> DefTable {
    DefTable(nameSpaceMap, nameSpaceMap, nameSpaceMap, nameSpaceMap, IntMap::empty, IntMap::empty)
}

pub fn enterBlockScope(deftbl: DefTable) -> DefTable {
    enterLocalScope(deftbl {
            labelDefs: enterNewScope((labelDefs(deftbl)))
        })
}

pub fn enterFunctionScope(deftbl: DefTable) -> DefTable {
    enterLocalScope(deftbl {
            labelDefs: enterNewScope((labelDefs(deftbl)))
        })
}

pub fn enterLocalScope(deftbl: DefTable) -> DefTable {
    deftbl {
        identDecls: enterNewScope((identDecls(deftbl))),
        tagDecls: enterNewScope((tagDecls(deftbl)))
    }
}

pub fn enterMemberDecl(deftbl: DefTable) -> DefTable {
    deftbl {
        memberDecls: enterNewScope((memberDecls(deftbl)))
    }
}

pub fn globalDefs(deftbl: DefTable) -> GlobalDecls {

    let e = Map::empty;

    let insertDecl = |_0, _1, _2| {
        match (_0, _1, _2) {
            (ident, Left(tydef), ds) => {
                ds {
                    gTypeDefs: Map::insert(ident, tydef, (gTypeDefs(ds)))
                }
            },
            (ident, Right(obj), ds) => {
                ds {
                    gTypeDefs: Map::insert(ident, tydef, (gTypeDefs(ds)))
                }
            },
        }
    };

    Map::foldWithKey(insertDecl, (GlobalDecls(e, gtags, e)), (globalNames(identDecls(deftbl))))
}

pub fn identOfTyDecl() -> Ident {
    either(identOfTypeDef, declIdent)
}

pub fn inFileScope(dt: DefTable) -> bool {
    not(((hasLocalNames((identDecls(dt))) || hasLocalNames((labelDefs(dt))))))
}

pub fn insertType(dt: DefTable, n: Name, t: Type) -> DefTable {
    dt {
        typeTable: IntMap::insert((nameId(n)), t, (typeTable(dt)))
    }
}

pub fn leaveBlockScope(deftbl: DefTable) -> DefTable {
    leaveLocalScope(deftbl {
            labelDefs: leaveScope_((labelDefs(deftbl)))
        })
}

pub fn leaveFunctionScope(deftbl: DefTable) -> DefTable {
    leaveLocalScope(deftbl {
            labelDefs: leaveScope_((labelDefs(deftbl)))
        })
}

pub fn leaveLocalScope(deftbl: DefTable) -> DefTable {
    deftbl {
        identDecls: leaveScope_((identDecls(deftbl))),
        tagDecls: leaveScope_((tagDecls(deftbl)))
    }
}

pub fn leaveMemberDecl(deftbl: DefTable) -> (Vec<MemberDecl>, DefTable) {
    {
        let (decls_q, members) = leaveScope((memberDecls(deftbl)));

    __op_tuple2((), (__map!(snd, members))((deftbl {
                memberDecls: decls_q
            })))    }
}

pub fn leaveScope_<a>() -> NameSpaceMap<k, a> {
    fst(leaveScope)
}

pub fn lookupIdent(ident: Ident, deftbl: DefTable) -> Option<IdentEntry> {
    lookupName((identDecls(deftbl)), ident)
}

pub fn lookupIdentInner(ident: Ident, deftbl: DefTable) -> Option<IdentEntry> {
    lookupInnermostScope((identDecls(deftbl)), ident)
}

pub fn lookupLabel(ident: Ident, deftbl: DefTable) -> Option<Ident> {
    lookupName((labelDefs(deftbl)), ident)
}

pub fn lookupTag(sue_ref: SUERef, deftbl: DefTable) -> Option<TagEntry> {
    lookupName((tagDecls(deftbl)), sue_ref)
}

pub fn lookupTagInner(sue_ref: SUERef, deftbl: DefTable) -> Option<TagEntry> {
    lookupInnermostScope((tagDecls(deftbl)), sue_ref)
}

pub fn lookupType(dt: DefTable, n: Name) -> Option<Type> {
    IntMap::lookup((nameId(n)), (typeTable(dt)))
}

pub fn mergeDefTable(DefTable(i1, t1, l1, m1, r1, tt1): DefTable, DefTable(i2, t2, l2, m2, r2, tt2): DefTable) -> DefTable {
    DefTable((mergeNameSpace(i1, i2)), (mergeNameSpace(t1, t2)), (mergeNameSpace(l1, l2)), (mergeNameSpace(m1, m2)), (union(r1, r2)), (union(tt1, tt2)))
}

pub fn tagKind(_0: TagEntry) -> TagEntryKind {
    match (_0) {
        Left(CompDecl(cd)) => {
            CompKind((compTag(cd)))
        },
        Left(EnumDecl(_)) => {
            CompKind((compTag(cd)))
        },
        Right(CompDef(cd)) => {
            CompKind((compTag(cd)))
        },
        Right(EnumDef(_)) => {
            CompKind((compTag(cd)))
        },
    }
}



