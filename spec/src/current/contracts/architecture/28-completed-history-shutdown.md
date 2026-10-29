# Completed history shutdown

JST-ARCH-007's history lifetime applies when an embedding host explicitly awaits
shutdown before destroying its window, and the optional Tauri adapter then
receives window-destroyed, application-exit or plugin-drop callbacks. Successful
shutdown retires the drained caller scopes. Later revocation of those scopes and
repeated shutdown succeed without reporting an active worker. Admission remains
closed; cleanup cannot reopen a scope or reuse an old handle.

Scope retirement occurs only after the serialized lifecycle owner has canceled
and joined its exact jobs and drained in-flight readers. A draining scope cannot
be treated as absent early. Failed cleanup remains an error. This clarification
changes no wire format, source pin, cache authority or publication behavior.
E-HISTORY-CLOSE-001 records the source regression and cold desktop consumer
check.
