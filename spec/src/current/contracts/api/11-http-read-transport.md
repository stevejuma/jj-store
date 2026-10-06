# HTTP read transport

The Axum V1 read transport binds repository-scoped Bearer credentials to the
same native sessions used by RPC. Reads include snapshot/capabilities, objects,
log, operation/evolution graphs, commit/operation detail, directories,
differences, conflicts, verified binary files, and index-task
start/status/cancel. Native publication is absent from these routes. Events,
search, typed mutations and browser assets are advertised only after their
separate implementations pass.

Graph and directory responses expose a snapshot, owned data, and a signed
next_cursor. Log requests provide an explicit at_time_millis anchor, retained on
continuations; user_email and logical workspace are explicit query context. The
operations heads parameter selects one full operation, and contradictory at_op
and heads inputs fail. Multiple current heads require selection, never automatic
reconciliation. Operation detail includes its native view and predecessor data;
commit detail adds snapshot-relative state annotations to the lossless object.

Transport limits bound URI/headers, JSON input/output, admitted requests and
total request/response lifetime. A response owns its slot through delivery;
expiry also drops an unpolled response and verified spool. Errors use fixed JSON
codes without storage details or credentials. Invalid/unknown query fields fail
explicitly. Authorized rebuild-required responses carry the captured snapshot.

Binary responses are application/octet-stream attachments with nosniff and
no-store. A single closed, open-ended or suffix byte range is selected only
after complete native verification; bounds are clamped to EOF, and
invalid/multiple or unsatisfiable ranges return 416. Range selection cannot
reset an already-consumed stream or repeatedly extend its byte allowance. File
streams reauthorize every chunk. JSON source strings are data; no endpoint
returns trusted source HTML.
