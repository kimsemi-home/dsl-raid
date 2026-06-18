pub(super) fn canonical_header() -> &'static str {
    "// Generated from DSLRaid Canonical IR by dslraid-codegen. Do not edit by hand.\n\n"
}

pub(super) fn go_header() -> &'static str {
    "\
// Generated from DSLRaid Canonical IR by dslraid-codegen. Do not edit by hand.

package generated

func dslraidNext[S comparable](state S, event string, transitions map[S]map[string]S) (S, bool) {
\tnext, ok := transitions[state][event]
\tif !ok {
\t\treturn state, false
\t}
\treturn next, true
}

"
}
