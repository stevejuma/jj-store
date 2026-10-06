# Process lifetime qualification

Every transport subprocess runs under an invocation-owned supervisor entered
before CLI configuration or credential loading. Only that supervisor owns the
Git process group. A kernel pipe ties its lifetime to the invoking CLI; EOF,
unexpected input or the fixed 60-second deadline kills the group and reaps the
leader. Ordinary future cancellation, output/disk-limit failures and hard CLI
SIGKILL therefore terminate owned transport children. The supervisor is a
short-lived helper, with no installed service, shared state or replay trigger.
Cancellation after remote acceptance remains an unknown external outcome until
explicit recovery inspects the approved destination.

Native push apply and explicit recovery merge each frozen remote-ref delta with
the current native remote target using pinned JJ ref merge semantics. The base
comes from the intent's immutable source view, verified against its exact lease.
Later observations from another approved binding are retained; ambiguous frozen
bases leave apply pending. Recovery cannot silently replace a later ref target.
Interrupted cache directories are owner-only from creation, so crash cleanup can
verify and remove them without requiring a manual purge.
