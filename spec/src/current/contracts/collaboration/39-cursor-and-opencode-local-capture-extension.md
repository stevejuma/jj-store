# Cursor and OpenCode local capture extension

JST-COLLAB-017,JST-COLLAB-018,JST-COLLAB-019 and JST-COLLAB-030 extend the
configured local preview to Cursor Desktop 3.22.7, Cursor CLI 2026.09.26-dd393fe
and the existing OpenCode 1.18.30 read-only API reader. Product surfaces qualify
independently. Cursor setup explicitly opts into transcript capture; owned hooks
supply exact conversation, product, workspace and transcript binding plus
lifecycle. Encoded project names alone grant no admission. Approved roots
default to CURSOR_DATA_DIR/projects or HOME/.cursor/projects, with explicit
configuration taking precedence. Default project-root discovery MUST traverse
only `<project>/agent-transcripts`, excluding generated `mcps`, `canvases` and
`node_modules` trees and transcripts without regular hook binding hints.
Explicit transcript roots retain bounded discovery. Transcript mode has one
message producer, bounded JSONL reads and exact durable prefix recovery; hooks
do not duplicate transcript messages. Missing native identities, timestamps,
disabled transcripts and unsupported versions remain explicit reduced coverage.
Rewrites and partial flushes retain acknowledged progress and original durable
intents. Current linked native workspace and Write admission are required before
reading or publishing. All harnesses share existing sanitation, spool, receipts,
controls and runtime bounds. OpenCode uses an explicitly configured
credential-free loopback API; unavailable acquisition yields to other harnesses.
Configured API startup qualifies independently from inactive import/export.
Current AgentStore only; legacy migration is excluded. Inactive history,
database readers, export, cloud sessions and additional platform/version
qualification remain deferred. Previously admitted sources may recover and
finish flushing without admitting new inactive history.
