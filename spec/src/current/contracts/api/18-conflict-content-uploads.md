# Conflict content uploads

`POST /v1/repos/{repo}/conflict-uploads` takes application/octet-stream and
strict query parameters epoch, position, operation, commit, path and
fingerprint. The operation is the single expected head. Write authorization
precedes parsing and body consumption; content encodings, wrong media types,
malformed/duplicate headers, unknown query fields, excessive bodies and short
declared transfers fail explicitly. Both streaming bytes and request duration
are bounded. A 200 response supplies the server-computed native file ID, byte
length, context and scoped upload handle; it is not a mutation receipt. Retry
identical bytes and binding after a lost response. Uploads never publish native
history.

resolve_conflicts now also accepts uploaded_file with upload, executable and
optional copy_from_term. It validates the complete authorized binding and file
integrity before atomic native rewriting. Invalid handles return 409
invalid_upload; stale snapshots and conflicts retain their existing errors.
Capabilities expose the upload byte cap and supported resolution actions.
Generated OpenAPI and TypeScript include binary upload and resolution contracts;
the browser stages bounded bytes separately and retains the returned scoped
handle until preview or apply.

ResolveConflicts previews and retained results expose an optional
conflict_resolution report: exact target before/after IDs, sorted selected_paths
and complete remaining native conflict paths for every changed result commit.
Empty lists explicitly mean no remaining conflicts on that result; absent
reports are legacy or non-resolution responses. Preview IDs are provisional. The
generated OpenAPI and TypeScript contracts describe this extension; the M4.17
editor renders the report through the existing durable mutation workflow.
