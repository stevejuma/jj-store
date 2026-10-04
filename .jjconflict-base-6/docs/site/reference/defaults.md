# Configuration defaults

Generated from the production Rust defaults. See [configuration](configuration.md) for units, valid contexts and override behavior.

```json
{
  "limits": {
    "concurrent_requests": 8,
    "request_seconds": 30,
    "max_blob_bytes": 33554432,
    "inventory_nodes": 100000
  },
  "replication": {
    "max_batch_origins": 128,
    "max_closure_entries": 100000,
    "max_objects_per_step": 100000,
    "max_bytes_per_step": 1073741824,
    "max_native_object_bytes": 67108864,
    "poll_millis": 5000,
    "retry_initial_millis": 500,
    "retry_max_seconds": 60,
    "circuit_failures": 8,
    "circuit_cooldown_seconds": 60,
    "timeout_seconds": 30
  },
  "agent_capture": {
    "enabled": false,
    "max_readers": 4,
    "rescan_seconds": 30,
    "max_sources": 1024,
    "max_pending_bytes": 67108864,
    "harnesses": []
  },
  "search_example": {
    "repository": "00000000000000000000000000000001",
    "enabled": true,
    "source": {
      "scopes": {},
      "index_source": true,
      "include_prefixes": [],
      "exclude_prefixes": [],
      "max_text_bytes": 1048576,
      "max_files": 10000,
      "max_work": 1000000,
      "max_output_bytes": 67108864
    },
    "poll_millis": 1000,
    "timeout_seconds": 60,
    "index_bytes": 536870912,
    "snapshot_every": 32
  }
}
```
