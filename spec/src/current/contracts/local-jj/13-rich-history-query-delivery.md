# Rich history query delivery

Prepared graph visibility and divergence flags use exact visible change groups
from the selected admitted projection, with logarithmic point membership. They
MUST NOT enumerate hidden incarnations merely to describe one selected commit.
Cold readers retain upstream resolution; full prefix/offset contracts are
unchanged.

Prepared evolution retains only bounded, evictable predecessor-walk state on its
exact operation generation. Only proven unique known anchors can retain a
checkpoint; unknown/repeated metadata keeps first-occurrence cursor semantics
through bounded replay. Format1 histories remain readable without checkpoint
reuse. Fresh scoped readers validate cached anchors, source operation
fingerprints and selected bodies on every request. Preserve upstream ordering,
duplicate/cycle behavior, legacy metadata boundaries and unknown versus
known-empty predecessors. Source binding/authorization and ordinary work, byte,
visit and time limits remain unchanged; checkpoint loss cannot authorize a wider
read or a partial result.

Prepared evolution shares the exact ranked predecessor occurrence lookup.
Unrelated bookmark-only operations need no body hydration. Lookup considers all
pending commit IDs and stops at the first legacy metadata boundary, retaining
known-empty versus unknown records and repeated-ID semantics. Disposable history
format3 remains checksum-bound and admitted before decoding/ranking; formats1/2
retain bounded replay. Relevant source operations still pass fresh fingerprint,
membership and authorization checks; no source capability survives a request.

Prepared `all()` log pages reuse the exact visible JJ order captured during
preparation, with compact lexical rank lookup for exclusive deep anchors. Source
hydration follows the selected window; a continuation probe loads no commit
body. Current binding/authorization, visible-anchor membership, direct parent
order and terminal-page behavior remain exact. Arrays and sorting share existing
resident/work admission. Arbitrary filtered revsets retain their separate
bounded upstream path; prepared evolution follows the checkpoint policy above.

The registered-folder `query_info` request MUST share the existing local
operation pin and worker admission. Its catalog excludes unqualified content and
cross-operation predicates. Standard JJ 0.45.1 aliases MUST evaluate against
server defaults with explicit email/workspace/time context, regardless of
user/repository aliases. The additive `operations_filtered` graph variant uses
the same bounded metadata grammar as native repositories and preserves sparse
pagination, actual operation parents and exact continuation bindings. Reads MUST
preserve operation-head files, physical dirty files and companion selections.
Custom aliases and local content/cross-operation predicate qualification remain
deferred.

Registered-folder query metadata MAY default author email from host JJ
configuration (JJ_EMAIL and normal JJ user paths), then the registered
repository's JJ configuration, then Git configuration. JJ 0.45 secure repository
config is read only when its metadata binds the registered repository path;
legacy config remains a read-only fallback. Reads MUST NOT migrate config,
snapshot dirty files or alter heads. Only bounded email is returned; custom
aliases remain isolated from query evaluation. Missing/malformed email leaves
explicit session input available.
