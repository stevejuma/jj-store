# Qualified primitive probe

The initial single-node candidate passed
[seven probes](../../../../../docs/evidence/m0-rustfs-single-node.json). The
probe discards one acknowledgement deliberately; it does not claim to inject a
failure inside RustFS. Production publication, IAM enforcement, streamed upload
limits, and multi-node routes require their own later acceptance evidence.

Verified spools support bounded half-open range readers using local seek and
limited reads. Opening a range never exposes its private path. This primitive
does not establish remote integrity; native callers verify the complete object
before creating a range reader.

A [second cold run](../../../../../docs/evidence/m0-rustfs-cold.json) confirms
incomplete request bodies never become readable objects. This evidence
supplements the first report without replacing its recorded outcomes.
