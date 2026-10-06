# Exact tree-changing checkout preparation

Before tree-changing completion can write physical files, its private provider
MUST capture an exact bounded checkout plan under the original foreground lease.
The plan binds the retained preparation, archive and recovery witness, source
and candidate workspace commits/trees, native checkout and tree-state bytes with
their original fingerprints, complete physical selection and every affected
path's original file and ancestor-directory identity. Inputs come from the
retained native operation, never client patch text, inferred latest history or
replacement command input. Preparation performs no physical or metadata writes
and returns no checkout-completion proof.

The initial preparation profile supports full sparse coverage and resolved
regular/executable files, including binary bytes, additions and deletions. It
MUST reject conflicts, symlinks/submodules, unsafe or filesystem-ambiguous
paths, file/directory transitions, non-ASCII paths pending filesystem-specific
alias proof, dirty affected source files and existing untracked or ignored
addition targets. Unrelated physical edits remain intact. Parent directories are
opened descriptor-relative without following links; missing parents are explicit
planned absences. Reads verify object identities and bound per-file, tree and
total payload sizes before allocation. Complete fresh physical guards and
unchanged metadata/policy checks bracket capture.

Only canonical versioned plans with matching original identities can be decoded
or revalidated. Revalidation compares both retained tree effects and current
file/directory fingerprints; matching bytes after replacement do not make a
stale input valid. The plan is data, not permission to publish, initialize,
snapshot or modify a checkout. Durable plan/journal selection, per-path
interrupted installation, native tree-state completion and verified after-state
remain required before tree-changing execution or writable capability.
