# Repository text file edits

EditFile names an exact local bookmark target, existing source path, destination
path, UTF-8 replacement text and new-change description. The bookmark MUST be a
single resolved target matching the command expectation. Source and destination
paths use the canonical repository-path rules, the source MUST resolve to an
ordinary non-conflicted file, and a rename MUST NOT replace an existing entry.
The replacement text is capped at the native text-read limit.

The command preserves the source executable bit and copy identity, writes a new
file object, creates one child change and moves the selected bookmark to that
child in the same unpublished transaction. It does not rewrite the selected
commit or its descendants and never touches a physical working copy. Preview,
apply, unknown-outcome recovery and replay use the existing stable request and
durable receipt contract. A protected bookmark or stale target fails before
publication.
