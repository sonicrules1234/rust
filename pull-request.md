Previously, musl targets on mips were dynamic-only, and targets on every other architecture were
static-only. Now musl targets in general support both types of linking, so we can remove all of the
conditions that mips targets used to avoid the forced static linking.

There was [some question](https://github.com/rust-lang/rust/pull/40113#discussion_r133852974) in
PR #40113 about libunwind support for mips. That will need to be resolved before this PR is merged.
