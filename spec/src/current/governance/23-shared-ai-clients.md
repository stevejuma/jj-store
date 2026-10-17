# Shared AI client integration

E-LAPIS-AI-001 moves shared native agent/compute implementation to the owning
Lapis AI repository and links its local public Rust clients. Only server
composition and desktop resource verification consume those clients. Storage,
model and WAL have no inference/controller edge. CI admits only the two named
sibling client paths for the server, retaining fail-closed unknown-dependency
checks. Current runtime paths use Control V3; historical normalization
identities and evidence remain retained.

The duplicated local-agent and compute Cargo roots are retired. Build storage
tracks the remaining owning roots; the sibling Lapis AI repository maintains its
own outputs. Application configuration, journals, admission, credentials and
publication remain owned by jj-store. Crate-root READMEs give short orientation
and usage; the manual and canonical specification retain full documentation.

Development HMR readiness allows a bounded 60-second native startup window for
complete helper and native-artifact verification before the backend serves.
Missing resources still fail preflight before stopping an existing service.

JST-GOV-002/003/004/011 require actual shared-package and consumer validation,
including focused source regressions and cold application lifecycle evidence.
M23's historical evidence is retained, while M24/M25 adapter and packaged
platform qualification remain independently gated. Local source consumption is
temporary; publication and a future crates.io switch require separate work.

M33 additionally admits only capture-to-`lapis-agent-redaction` at its exact
versioned sibling path. CI materializes its immutable source pin independently
of Lapis AI. Core/model/WAL gain no privacy/runtime edge. The shared source
materializer retains bounded archive requests, package-version checks, atomic
installation and refusal to replace existing checkouts. This is temporary
source-library consumption, not publication.

M33 permits capture-to-`lapis-ai-capture` at its exact versioned sibling path
and immutable Lapis AI source pin. Only discovery/header acquisition crosses
this boundary; core/model/WAL and capture-to-runtime remain refused. Focused
consumer recovery and complete source-library evidence remain distinct.

Normal governance checks the headless capture dependency closure and acquisition
admission. The default application feature retains its original recovery suite;
headless source/progress tests run separately without optional publication APIs.
Read-only Cargo graph inspection creates no build target or service lifetime.

M33 shared conversation primitives are qualified as an independent crate and
through the original native-policy consumer tests. Injected fixture policies do
not establish scanner, provider or desktop acceptance. Distribution still
requires complete package and consumer qualification.

The private app consumes its owning headless SDK through built workspace public
exports. The separately publishable package retains registry metadata and no UI
or runtime dependencies. Independent runtime/type archive acceptance, existing
consumer regressions and source dependency checks qualify the boundary; source
aliases and implicit prompt retry remain forbidden.

M33 local-history governance checks the normal transitive dependency closure and
explicit integration/example feature gates. Default native keeps every original
target. Headless feature selection excludes native publication, Forge and host
runtimes; source graph inspection adds no service or build lifetime. Real-source
read acceptance and native/server consumers qualify their own boundaries.

The history bridge retains a headless default dependency closure and an explicit
optional Tauri transport. Existing native consumers select their original
profile explicitly. Normal governance guards the closure and CI package
selection; release graph regression follows actual Cargo inventory and all
internal ordering edges instead of a stale fixed package count.

Tauri's generated permission reference and schema retain their producer's exact
format and are excluded by exact path from the prose formatter. Hand-authored
permissions and source remain checked; this changes no permission grant or
native transport qualification requirement.

The executable spec-first map and tracked change map both route every history
bridge crate input to the reusable-agent architecture chapter. New crates and
generated permission descriptors remain protected, with no unmapped-file escape.

Standalone workspace discovery is qualified in the companion profile and through
the original native CLI folder provider. Feature governance keeps its discovery
test binary available without native storage. Observations grant no physical or
window authority, and source discovery does not waive application acceptance.

The contained native-history schema is generated from actual owned Rust frames
and checked for equality in the owning crate. It retains producer formatting by
exact-path exclusion. Versioned JSON text carries full-width integers through
Tauri without JavaScript rounding; no native caller or cache grant enters a
frame.

The typed history client has a schema-generation drift check and an exact
size-policy exception for its generated type file, matching the existing HTTP
generated contract rule. Authored client modules retain ordinary size checks.
Normal governance qualifies built archive imports, independent type consumption,
full-width integer handling and strict bounded frames. Its source selects web
checks; native schema equality remains the Rust producer's separate gate.
