# WorkStore service and transports

The native WorkStore RPC uses the same authorized service as HTTP, including
pinned collection pages, explicit query-bound opaque continuations and durable
apply/status. CLI project/task commands use the bound local native service and
do not require an upstream hub.
