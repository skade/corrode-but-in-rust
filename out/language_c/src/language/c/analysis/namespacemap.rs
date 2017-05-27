use haskell_support::*;

struct NameSpaceMap<k<v>>(NsMap<Map<k, v>, Vec<Vec<(k, v)>>>);

pub fn defGlobal(NsMap(gs, lss): NameSpaceMap<k, a>, ident: k, def: a) -> (NameSpaceMap<k, a>, Option<a>) {
    (NsMap((Map::insert(ident, def, gs)), lss), Map::lookup(ident, gs))
}

pub fn defLocal(__0: NameSpaceMap<k, a>, __1: k, __2: a, __3: (NameSpaceMap<k, a>, Option<a>)) -> (NameSpaceMap<k, a>, Option<a>) {
    match (__0, __1, __2, __3, __4) {
        (ns, @, NsMap(_, []), ident, def) => {
            defGlobal(ns, ident, def)
        },
        (NsMap(gs, [ls, ...lss]), ident, def) => {
            (NsMap(gs, (__op_concat((__op_concat((ident, def), ls)), lss))), Prelude::lookup(ident, ls))
        },
    }
}

pub fn enterNewScope(NsMap(gs, lss): NameSpaceMap<k, a>) -> NameSpaceMap<k, a> {
    NsMap(gs, (__op_concat(vec![], lss)))
}

pub fn globalNames(NsMap(g, _): NameSpaceMap<k, v>) -> Map<k, v> {
    g
}

pub fn hasLocalNames(NsMap(_, l): NameSpaceMap<k, v>) -> bool {
    not((null(l)))
}

pub fn leaveScope(__0: NameSpaceMap<k, a>) -> (NameSpaceMap<k, a>, Vec<(k, a)>) {
    match (__0) {
        NsMap(_, []) => {
            __error!("NsMaps.leaveScope: No local scope!".to_string())
        },
        NsMap(gs, [ls, ...lss]) => {
            (NsMap(gs, lss), ls)
        },
    }
}

pub fn localNames(NsMap(_, l): NameSpaceMap<k, v>) -> Vec<Vec<(k, v)>> {
    l
}

pub fn lookupGlobal(NsMap(gs, _): NameSpaceMap<k, a>, ident: k) -> Option<a> {
    Map::lookup(ident, gs)
}

pub fn lookupInnermostScope(nsm: NameSpaceMap<k, a>, @: k, NsMap(_gs, localDefs): Option<a>) -> Option<a> {
    match localDefs {
        [ls, ..._lss] => {
            Prelude::lookup(ident, ls)
        },
        [] => {
            lookupGlobal(nsm, ident)
        },
    }
}

pub fn lookupName(ns: NameSpaceMap<k, a>, @: k, NsMap(_, localDefs): Option<a>) -> Option<a> {
    match (lookupLocal(localDefs)) {
        None => {
            lookupGlobal(ns, ident)
        },
        Some | def => {
            Some(def)
        },
    }
}

pub fn mergeNameSpace(NsMap(global1, local1): NameSpaceMap<k, a>, NsMap(global2, local2): NameSpaceMap<k, a>) -> NameSpaceMap<k, a> {
    NsMap((Map::union(global1, global2)), (localUnion(local1, local2)))
}

pub fn nameSpaceMap() -> NameSpaceMap<k, v> {
    NsMap(Map::empty, vec![])
}

pub fn nsMapToList(NsMap(gs, lss): NameSpaceMap<k, a>) -> Vec<(k, a)> {
    __op_addadd(concat(lss), Map::toList(gs))
}
