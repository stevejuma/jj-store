# API data types

Generated from the application OpenAPI schemas. Exact identity widths, required fields and variants are preserved. Do not coerce large JSON integers into JavaScript numbers. See [transport semantics](../transport.md).

## ErrorBody

```text
{
  "type": "object",
  "properties": {
    "diagnostic": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/Diagnostic"
        },
        {
          "type": "null"
        }
      ]
    },
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "code": {
      "type": "string"
    },
    "snapshot": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReadSnapshot"
        },
        {
          "type": "null"
        }
      ]
    },
    "local_snapshot": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/LocalCodeSnapshot"
        },
        {
          "type": "null"
        }
      ]
    },
    "heads": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/HeadSnapshot"
        },
        {
          "type": "null"
        }
      ]
    },
    "search_status": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/SearchStatus"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "code"
  ]
}
```

## Diagnostic

```text
{
  "type": "object",
  "properties": {
    "kind": {
      "type": "string"
    },
    "message": {
      "type": "string"
    }
  },
  "required": [
    "kind",
    "message"
  ]
}
```

## ReadSnapshot

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "repo_id": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "published_seq": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "manifest_revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "code_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "heads": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "operation_id": {
      "$ref": "#/components/schemas/OperationId"
    }
  },
  "required": [
    "version",
    "repo_id",
    "epoch",
    "published_seq",
    "manifest_revision",
    "code_state_version",
    "heads",
    "operation_id"
  ]
}
```

## RepositoryId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## EpochId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## WalPosition

```text
{
  "type": "integer",
  "format": "uint64",
  "minimum": 0,
  "description": "Publication order is distinct from native operation ancestry."
}
```

## OperationId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{128}$",
  "minLength": 128,
  "maxLength": 128
}
```

## LocalCodeSnapshot

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "profile": {
      "type": "string"
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "generation": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    }
  },
  "required": [
    "version",
    "profile",
    "repository",
    "generation",
    "operation"
  ]
}
```

## HeadSnapshot

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "repo_id": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "published_seq": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "manifest_revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "code_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "heads": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    }
  },
  "required": [
    "version",
    "repo_id",
    "epoch",
    "published_seq",
    "manifest_revision",
    "code_state_version",
    "heads"
  ]
}
```

## SearchStatus

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "repository_position": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "durable_position": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/WalPosition"
        },
        {
          "type": "null"
        }
      ]
    },
    "indexed_position": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/WalPosition"
        },
        {
          "type": "null"
        }
      ]
    },
    "index_generation": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RecordDigest"
        },
        {
          "type": "null"
        }
      ]
    },
    "is_stale": {
      "type": "boolean"
    },
    "degraded": {
      "type": "boolean"
    }
  },
  "required": [
    "version",
    "repository",
    "epoch",
    "repository_position",
    "durable_position",
    "indexed_position",
    "index_generation",
    "is_stale",
    "degraded"
  ]
}
```

## RecordDigest

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{64}$",
  "minLength": 64,
  "maxLength": 64
}
```

## Bootstrap

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "access_mode": {
      "$ref": "#/components/schemas/AccessMode"
    },
    "capabilities": {
      "$ref": "#/components/schemas/Capabilities"
    },
    "storage": {
      "$ref": "#/components/schemas/StorageState"
    },
    "backups": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/BackupStatus"
      }
    }
  },
  "required": [
    "version",
    "access_mode",
    "capabilities",
    "storage",
    "backups"
  ]
}
```

## AccessMode

```text
{
  "type": "string",
  "enum": [
    "authenticated",
    "local_trusted"
  ]
}
```

## Capabilities

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "native_authority": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "native_profile": {
      "type": "string"
    },
    "authentication": {
      "type": "string"
    },
    "read_api": {
      "type": "boolean"
    },
    "browsing_windows": {
      "type": "boolean"
    },
    "index_tasks": {
      "type": "boolean"
    },
    "automatic_code_index": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "events": {
      "type": "boolean"
    },
    "application_events": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "search": {
      "type": "boolean"
    },
    "local_workspace_links": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "local_folder_picker": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "local_repository_management": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "local_native_repository_creation": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "local_repository_conversion": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "local_search_management": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "local_sync_management": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "local_storage_management": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "local_capture_management": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "local_agent_execution": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "local_settings_management": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "typed_mutations": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "mutation_request_bytes": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "conflict_upload_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "conflict_resolution_actions": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "event_limits": {
      "$ref": "#/components/schemas/EventLimits"
    },
    "limits": {
      "$ref": "#/components/schemas/ReadLimits"
    },
    "log_context": {
      "$ref": "#/components/schemas/LogContext"
    }
  },
  "required": [
    "version",
    "native_profile",
    "authentication",
    "read_api",
    "browsing_windows",
    "index_tasks",
    "events",
    "search",
    "typed_mutations",
    "mutation_request_bytes",
    "conflict_upload_bytes",
    "conflict_resolution_actions",
    "event_limits",
    "limits",
    "log_context"
  ]
}
```

## EventLimits

```text
{
  "type": "object",
  "properties": {
    "streams": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "replay_window": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "page": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "poll_millis": {
      "type": "integer",
      "format": "uint128",
      "minimum": 0
    },
    "stream_lifetime_millis": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint128",
      "minimum": 0
    },
    "heartbeat_millis": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint128",
      "minimum": 0
    }
  },
  "required": [
    "streams",
    "replay_window",
    "page",
    "poll_millis"
  ]
}
```

## ReadLimits

```text
{
  "type": "object",
  "properties": {
    "requests": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "workers": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "deadline_millis": {
      "type": "integer",
      "format": "uint128",
      "minimum": 0
    },
    "json_bytes": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "page": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "index_commits": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "inline_index_commits": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "file_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "requests",
    "workers",
    "deadline_millis",
    "json_bytes",
    "page",
    "index_commits",
    "inline_index_commits",
    "file_bytes"
  ]
}
```

## LogContext

```text
{
  "type": "object",
  "properties": {
    "at_time_millis": {
      "type": "string"
    },
    "workspace": {
      "type": "string"
    },
    "user_email": {
      "type": "string"
    }
  },
  "required": [
    "at_time_millis",
    "workspace",
    "user_email"
  ]
}
```

## StorageState

```text
{
  "type": "object",
  "properties": {
    "ready": {
      "type": "boolean"
    }
  },
  "required": [
    "ready"
  ]
}
```

## BackupStatus

```text
{
  "type": "object",
  "properties": {
    "target": {
      "type": "string"
    },
    "driver": {
      "type": "string"
    },
    "transfer": {
      "$ref": "#/components/schemas/BackupTransfer"
    },
    "last_verified_restore_point": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/VerifiedRestorePoint"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "target",
    "driver",
    "transfer",
    "last_verified_restore_point"
  ]
}
```

## BackupTransfer

```text
{
  "type": "object",
  "properties": {
    "state": {
      "type": "string"
    },
    "completed_objects": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "total_objects": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    },
    "completed_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "total_bytes": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "state",
    "completed_objects",
    "total_objects",
    "completed_bytes",
    "total_bytes"
  ]
}
```

## VerifiedRestorePoint

```text
{
  "type": "object",
  "properties": {
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "position": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "verified_at_millis": {
      "type": "integer",
      "format": "int64"
    }
  },
  "required": [
    "repository",
    "epoch",
    "position",
    "operation",
    "verified_at_millis"
  ]
}
```

## RepositoryPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "generation": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "repositories": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/RepositorySummary"
      }
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "generation",
    "repositories"
  ]
}
```

## RepositorySummary

```text
{
  "type": "object",
  "properties": {
    "profile": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CodeProfileV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "availability": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/Availability"
        },
        {
          "type": "null"
        }
      ]
    },
    "id": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "name": {
      "type": "string"
    },
    "permission": {
      "$ref": "#/components/schemas/EffectivePermission"
    },
    "principal": {
      "$ref": "#/components/schemas/PrincipalId"
    }
  },
  "required": [
    "id",
    "name",
    "permission",
    "principal"
  ]
}
```

## CodeProfileV1

```text
{
  "type": "string",
  "enum": [
    "native_v1",
    "git_sha1_jj045"
  ]
}
```

## Availability

```text
{
  "type": "string",
  "enum": [
    "ready",
    "unavailable",
    "identity_changed"
  ]
}
```

## EffectivePermission

```text
{
  "type": "string",
  "enum": [
    "read",
    "write",
    "admin"
  ]
}
```

## PrincipalId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## Envelope_QueryInfo

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/QueryInfo"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "next_cursor"
  ]
}
```

## QueryInfo

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "jj_version": {
      "type": "string"
    },
    "max_query_bytes": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "callables": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/Callable"
      }
    },
    "aliases": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "operators": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "operation_fields": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "unsupported": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "default_user_email": {
      "type": [
        "string",
        "null"
      ],
      "description": "Device identity hint, available only in LocalTrusted query metadata."
    }
  },
  "required": [
    "version",
    "jj_version",
    "max_query_bytes",
    "callables",
    "aliases",
    "operators",
    "operation_fields",
    "unsupported"
  ]
}
```

## Callable

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "signature": {
      "type": "string"
    },
    "insert_text": {
      "type": "string"
    },
    "description": {
      "type": "string"
    },
    "kind": {
      "type": "string"
    }
  },
  "required": [
    "name",
    "signature",
    "insert_text",
    "description",
    "kind"
  ]
}
```

## LocalSearchSettings

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "revision": {
      "type": "string"
    },
    "desired": {
      "$ref": "#/components/schemas/SearchConfiguration"
    },
    "effective": {
      "$ref": "#/components/schemas/SearchConfiguration"
    },
    "operation": {
      "$ref": "#/components/schemas/SearchOperation"
    },
    "error": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "revision",
    "desired",
    "effective",
    "operation",
    "error"
  ]
}
```

## SearchConfiguration

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "enabled": {
      "type": "boolean"
    },
    "index_source": {
      "type": "boolean"
    },
    "scopes": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/SearchScope"
      }
    },
    "include_prefixes": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "type": "string"
      }
    },
    "exclude_prefixes": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "type": "string"
      }
    },
    "index_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "poll_millis": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0,
      "default": 1000
    },
    "timeout_seconds": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0,
      "default": 60
    },
    "snapshot_every": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0,
      "default": 32
    },
    "max_text_bytes": {
      "type": "integer",
      "format": "uint",
      "minimum": 0,
      "default": 1048576
    },
    "max_files": {
      "type": "integer",
      "format": "uint",
      "minimum": 0,
      "default": 10000
    },
    "max_work": {
      "type": "integer",
      "format": "uint",
      "minimum": 0,
      "default": 1000000
    },
    "max_output_bytes": {
      "type": "integer",
      "format": "uint",
      "minimum": 0,
      "default": 67108864
    }
  },
  "required": [
    "enabled",
    "index_source",
    "scopes",
    "include_prefixes",
    "exclude_prefixes",
    "index_bytes",
    "poll_millis",
    "timeout_seconds",
    "snapshot_every",
    "max_text_bytes",
    "max_files",
    "max_work",
    "max_output_bytes"
  ]
}
```

## SearchScope

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "name": {
          "type": "string"
        },
        "kind": {
          "type": "string",
          "const": "bookmark"
        }
      },
      "required": [
        "kind",
        "name"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "name": {
          "type": "string"
        },
        "kind": {
          "type": "string",
          "const": "workspace"
        }
      },
      "required": [
        "kind",
        "name"
      ]
    }
  ]
}
```

## SearchOperation

```text
{
  "type": "string",
  "enum": [
    "idle",
    "applying",
    "rebuilding",
    "retrying",
    "failed"
  ]
}
```

## StorageOverview

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "ready": {
      "type": "boolean"
    },
    "current": {
      "$ref": "#/components/schemas/StoragePosition"
    },
    "checkpoint": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/StoragePosition"
        },
        {
          "type": "null"
        }
      ]
    },
    "limits": {
      "$ref": "#/components/schemas/StorageLimits"
    },
    "latest": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/StorageOperation"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "ready",
    "current",
    "checkpoint",
    "limits",
    "latest"
  ]
}
```

## StoragePosition

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "position": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "digest": {
      "$ref": "#/components/schemas/RecordDigest"
    }
  },
  "required": [
    "repository",
    "epoch",
    "position",
    "revision",
    "digest"
  ]
}
```

## StorageLimits

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "entries": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "timeout_millis": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "entries",
    "bytes",
    "timeout_millis"
  ]
}
```

## StorageOperation

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "kind": {
      "$ref": "#/components/schemas/StorageKind"
    },
    "phase": {
      "$ref": "#/components/schemas/StoragePhase"
    },
    "target": {
      "$ref": "#/components/schemas/StoragePosition"
    },
    "limits": {
      "$ref": "#/components/schemas/StorageLimits"
    },
    "selection": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CheckpointSelection"
        },
        {
          "type": "null"
        }
      ]
    },
    "message": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "request_id",
    "kind",
    "phase",
    "target",
    "limits",
    "selection",
    "message"
  ]
}
```

## RequestId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## StorageKind

```text
{
  "type": "string",
  "enum": [
    "check",
    "checkpoint"
  ]
}
```

## StoragePhase

```text
{
  "type": "string",
  "enum": [
    "prepared",
    "running",
    "complete",
    "incomplete",
    "failed",
    "interrupted",
    "unknown",
    "canceled"
  ]
}
```

## CheckpointSelection

```text
{
  "type": "string",
  "enum": [
    "not_covered",
    "selected",
    "covered"
  ]
}
```

## LocalSyncStatus

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "configured": {
      "type": "boolean"
    },
    "worker": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/SyncStatus"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "configured",
    "worker"
  ]
}
```

## SyncStatus

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "local_ready": {
      "type": "boolean"
    },
    "offline_ready": {
      "type": "boolean"
    },
    "phase": {
      "$ref": "#/components/schemas/SyncPhase"
    },
    "local_replica": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReplicaId"
        },
        {
          "type": "null"
        }
      ]
    },
    "peer_replica": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReplicaId"
        },
        {
          "type": "null"
        }
      ]
    },
    "local_frontier": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/SyncFrontier"
        },
        {
          "type": "null"
        }
      ]
    },
    "peer_frontier": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/SyncFrontier"
        },
        {
          "type": "null"
        }
      ]
    },
    "pushed": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/SyncFrontier"
        },
        {
          "type": "null"
        }
      ]
    },
    "pulled": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/SyncFrontier"
        },
        {
          "type": "null"
        }
      ]
    },
    "local_pending_positions": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    },
    "peer_pending_positions": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    },
    "checkpoint_revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "operation_heads": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "reconciliation_required": {
      "type": "boolean"
    },
    "pull": {
      "$ref": "#/components/schemas/SyncDirectionStatus"
    },
    "push": {
      "$ref": "#/components/schemas/SyncDirectionStatus"
    },
    "search": {
      "$ref": "#/components/schemas/SyncSearchStatus"
    }
  },
  "required": [
    "version",
    "repository",
    "local_ready",
    "offline_ready",
    "phase",
    "local_replica",
    "peer_replica",
    "local_frontier",
    "peer_frontier",
    "pushed",
    "pulled",
    "local_pending_positions",
    "peer_pending_positions",
    "checkpoint_revision",
    "operation_heads",
    "reconciliation_required",
    "pull",
    "push",
    "search"
  ]
}
```

## SyncPhase

```text
{
  "type": "string",
  "enum": [
    "connecting",
    "running",
    "paused",
    "degraded",
    "stopped"
  ]
}
```

## ReplicaId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## SyncFrontier

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "replica": {
      "$ref": "#/components/schemas/ReplicaId"
    },
    "log_epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "through": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "observed_upper_bound": {
      "$ref": "#/components/schemas/WalPosition"
    }
  },
  "required": [
    "replica",
    "log_epoch",
    "through",
    "observed_upper_bound"
  ]
}
```

## SyncDirectionStatus

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "consecutive_failures": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "circuit_open": {
      "type": "boolean"
    },
    "last_fault": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/SyncFault"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "consecutive_failures",
    "circuit_open",
    "last_fault"
  ]
}
```

## SyncFault

```text
{
  "type": "string",
  "enum": [
    "transient_endpoint",
    "permanent_endpoint",
    "integrity",
    "limit",
    "cursor_conflict",
    "outcome_unknown"
  ]
}
```

## SyncSearchStatus

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "configured": {
      "type": "boolean"
    },
    "repository_position": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/WalPosition"
        },
        {
          "type": "null"
        }
      ]
    },
    "indexed_position": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/WalPosition"
        },
        {
          "type": "null"
        }
      ]
    },
    "degraded": {
      "type": "boolean"
    }
  },
  "required": [
    "configured",
    "repository_position",
    "indexed_position",
    "degraded"
  ]
}
```

## SyncBarrier

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "local_replica": {
      "$ref": "#/components/schemas/ReplicaId"
    },
    "peer_replica": {
      "$ref": "#/components/schemas/ReplicaId"
    },
    "push_epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "push_through": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "pull_epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "pull_through": {
      "$ref": "#/components/schemas/WalPosition"
    }
  },
  "required": [
    "version",
    "repository",
    "local_replica",
    "peer_replica",
    "push_epoch",
    "push_through",
    "pull_epoch",
    "pull_through"
  ]
}
```

## LocalSyncWait

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "barrier": {
      "$ref": "#/components/schemas/SyncBarrier"
    },
    "reached": {
      "type": "boolean"
    },
    "status": {
      "$ref": "#/components/schemas/SyncStatus"
    }
  },
  "required": [
    "version",
    "barrier",
    "reached",
    "status"
  ]
}
```

## WorkspaceLinks

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "links": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/WorkspaceLink"
      }
    }
  },
  "required": [
    "version",
    "revision",
    "links"
  ]
}
```

## WorkspaceLink

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "id": {
      "type": "string"
    },
    "path": {
      "type": "string"
    },
    "profile": {
      "$ref": "#/components/schemas/WorkspaceProfile"
    },
    "availability": {
      "$ref": "#/components/schemas/WorkspaceAvailability"
    }
  },
  "required": [
    "id",
    "path",
    "profile",
    "availability"
  ]
}
```

## WorkspaceProfile

```text
{
  "type": "string",
  "enum": [
    "native",
    "local_jj_git"
  ]
}
```

## WorkspaceAvailability

```text
{
  "type": "string",
  "enum": [
    "ready",
    "unavailable"
  ]
}
```

## FolderSelection

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "path": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "path"
  ]
}
```

## SearchPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "repository_position": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "indexed_position": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "index_generation": {
      "$ref": "#/components/schemas/RecordDigest"
    },
    "selected_view": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReadSnapshot"
        },
        {
          "type": "null"
        }
      ]
    },
    "scope_snapshots": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/SearchScopeSnapshot"
      }
    },
    "results": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/SearchDocument"
      }
    },
    "is_stale": {
      "type": "boolean"
    },
    "degraded": {
      "type": "boolean"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    },
    "truncated": {
      "type": "boolean"
    }
  },
  "required": [
    "version",
    "repository",
    "epoch",
    "repository_position",
    "indexed_position",
    "index_generation",
    "selected_view",
    "scope_snapshots",
    "results",
    "is_stale",
    "degraded",
    "next_cursor",
    "truncated"
  ]
}
```

## SearchScopeSnapshot

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "state": {
      "$ref": "#/components/schemas/SearchScopeState"
    },
    "operation": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/OperationId"
        },
        {
          "type": "null"
        }
      ]
    },
    "commits": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    }
  },
  "required": [
    "state",
    "operation",
    "commits"
  ]
}
```

## SearchScopeState

```text
{
  "type": "string",
  "enum": [
    "ready",
    "conflicted",
    "missing",
    "ambiguous_heads",
    "disabled"
  ]
}
```

## CommitId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{128}$",
  "minLength": 128,
  "maxLength": 128
}
```

## SearchDocument

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "key": {
      "$ref": "#/components/schemas/SearchDocumentKey"
    },
    "title": {
      "type": "string"
    },
    "body": {
      "type": "string"
    },
    "actor": {
      "type": [
        "string",
        "null"
      ]
    },
    "author": {
      "type": "string"
    },
    "workspace": {
      "type": [
        "string",
        "null"
      ]
    },
    "operation": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/OperationId"
        },
        {
          "type": "null"
        }
      ]
    },
    "commit": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CommitId"
        },
        {
          "type": "null"
        }
      ]
    },
    "change": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ChangeId"
        },
        {
          "type": "null"
        }
      ]
    },
    "file": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/FileId"
        },
        {
          "type": "null"
        }
      ]
    },
    "parents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "predecessors": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "attributes": {
      "type": "object",
      "additionalProperties": {
        "type": "string"
      }
    },
    "language": {
      "type": [
        "string",
        "null"
      ]
    },
    "has_conflict": {
      "type": "boolean"
    },
    "timestamp_millis": {
      "type": "integer",
      "format": "int64"
    }
  },
  "required": [
    "key",
    "title",
    "body",
    "actor",
    "author",
    "workspace",
    "operation",
    "commit",
    "change",
    "file",
    "parents",
    "predecessors",
    "attributes",
    "language",
    "has_conflict",
    "timestamp_millis"
  ],
  "description": "Owned extraction output. Native identity fields are stored exactly; the body\nis indexed but never stored in query hits. Fetch source via an authorized ID."
}
```

## SearchDocumentKey

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/components/schemas/OperationId"
        },
        "kind": {
          "type": "string",
          "const": "operation"
        }
      },
      "required": [
        "kind",
        "id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/components/schemas/CommitId"
        },
        "kind": {
          "type": "string",
          "const": "commit"
        }
      },
      "required": [
        "kind",
        "id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "scope": {
          "type": "string"
        },
        "path": {
          "type": "string"
        },
        "kind": {
          "type": "string",
          "const": "file"
        }
      },
      "required": [
        "kind",
        "scope",
        "path"
      ]
    }
  ]
}
```

## ChangeId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## FileId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{128}$",
  "minLength": 128,
  "maxLength": 128
}
```

## AgentSessionPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/AgentReadSnapshot"
    },
    "sessions": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentSessionSummary"
      }
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "sessions",
    "next_cursor"
  ]
}
```

## AgentReadSnapshot

```text
{
  "type": "object",
  "properties": {
    "repo_id": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "published_seq": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "manifest_revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "agent_root": {
      "$ref": "#/components/schemas/RecordDigest"
    },
    "agent_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "repo_id",
    "epoch",
    "published_seq",
    "manifest_revision",
    "agent_root",
    "agent_state_version"
  ]
}
```

## AgentSessionSummary

```text
{
  "type": "object",
  "properties": {
    "session_id": {
      "$ref": "#/components/schemas/AgentSessionId"
    },
    "conversation_title": {
      "type": [
        "string",
        "null"
      ]
    },
    "discovery_text": {
      "type": [
        "string",
        "null"
      ]
    },
    "snapshot_id": {
      "$ref": "#/components/schemas/AgentObjectId"
    },
    "integration": {
      "$ref": "#/components/schemas/IntegrationId"
    },
    "state": {
      "$ref": "#/components/schemas/SessionState"
    },
    "parent_session_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentSessionId"
        },
        {
          "type": "null"
        }
      ]
    },
    "first_activity_at_millis": {
      "type": [
        "integer",
        "null"
      ],
      "format": "int64"
    },
    "last_activity_at_millis": {
      "type": [
        "integer",
        "null"
      ],
      "format": "int64"
    },
    "coverage": {
      "$ref": "#/components/schemas/AgentCoverage"
    },
    "evidence_count": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "event_batch_count": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "transcript_count": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "tool_count": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    }
  },
  "required": [
    "session_id",
    "snapshot_id",
    "integration",
    "state",
    "parent_session_id",
    "first_activity_at_millis",
    "last_activity_at_millis",
    "coverage",
    "evidence_count",
    "event_batch_count",
    "transcript_count",
    "tool_count"
  ]
}
```

## AgentSessionId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## AgentObjectId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{64}$",
  "minLength": 64,
  "maxLength": 64
}
```

## IntegrationId

```text
{
  "type": "string",
  "enum": [
    "codex",
    "cursor",
    "open_code",
    "owned_controller"
  ]
}
```

## SessionState

```text
{
  "type": "string",
  "enum": [
    "discovered",
    "active",
    "idle",
    "ended",
    "unknown"
  ]
}
```

## AgentCoverage

```text
{
  "type": "object",
  "properties": {
    "completeness": {
      "$ref": "#/components/schemas/CompletenessV1"
    },
    "gaps": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "redacted_fields": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "truncated": {
      "type": "boolean"
    }
  },
  "required": [
    "completeness",
    "gaps",
    "redacted_fields",
    "truncated"
  ]
}
```

## CompletenessV1

```text
{
  "type": "string",
  "enum": [
    "complete",
    "partial",
    "unknown"
  ]
}
```

## AgentSessionDetail

```text
{
  "type": "object",
  "properties": {
    "conversation_feedback": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentExecutionFeedback"
        },
        {
          "type": "null"
        }
      ]
    },
    "conversation_unavailable": {
      "type": [
        "string",
        "null"
      ]
    },
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/AgentReadSnapshot"
    },
    "session": {
      "$ref": "#/components/schemas/AgentSessionSummary"
    },
    "evidence": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentObjectId"
      }
    },
    "event_batches": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentObjectId"
      }
    },
    "transcripts": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentTranscriptDescriptor"
      }
    },
    "tools": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentToolLink"
      }
    },
    "activity": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentActivity"
      }
    },
    "activity_truncated": {
      "type": "boolean"
    },
    "code": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentCodeAssociation"
      }
    },
    "code_truncated": {
      "type": "boolean"
    }
  },
  "required": [
    "version",
    "snapshot",
    "session",
    "evidence",
    "event_batches",
    "transcripts",
    "tools",
    "activity",
    "activity_truncated",
    "code",
    "code_truncated"
  ]
}
```

## AgentExecutionFeedback

```text
{
  "type": "object",
  "properties": {
    "messages": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentExecutionFeedbackMessage"
      }
    },
    "profile": {
      "type": [
        "string",
        "null"
      ]
    },
    "write_unavailable": {
      "type": [
        "string",
        "null"
      ],
      "description": "Current browser feedback write capability; does not suppress readable history."
    }
  },
  "required": [
    "messages"
  ],
  "description": "Repository-selected messages and ordinary discussion revisions, never journal authority."
}
```

## AgentExecutionFeedbackMessage

```text
{
  "type": "object",
  "properties": {
    "row_id": {
      "type": "string"
    },
    "target": {
      "$ref": "#/components/schemas/ReactionTargetV1"
    },
    "threads": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ThreadId"
      }
    },
    "replies": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentExecutionFeedbackReply"
      }
    },
    "text_parts": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentExecutionFeedbackTextPart"
      },
      "default": []
    },
    "answers": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommentReplyLinkV1"
      },
      "default": []
    }
  },
  "required": [
    "row_id",
    "target",
    "threads",
    "replies",
    "text_parts",
    "answers"
  ]
}
```

## ReactionTargetV1

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "thread_id": {
          "$ref": "#/components/schemas/ThreadId"
        },
        "type": {
          "type": "string",
          "const": "thread"
        }
      },
      "required": [
        "type",
        "thread_id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "thread_id": {
          "$ref": "#/components/schemas/ThreadId"
        },
        "message_id": {
          "$ref": "#/components/schemas/HumanMessageId"
        },
        "revision": {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        "type": {
          "type": "string",
          "const": "human_message"
        }
      },
      "required": [
        "type",
        "thread_id",
        "message_id",
        "revision"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "session_id": {
          "$ref": "#/components/schemas/AgentSessionId"
        },
        "message_id": {
          "$ref": "#/components/schemas/MessageId"
        },
        "event_id": {
          "$ref": "#/components/schemas/AgentEventId"
        },
        "batch": {
          "$ref": "#/components/schemas/AgentObjectRef"
        },
        "content": {
          "$ref": "#/components/schemas/AgentObjectRef"
        },
        "type": {
          "type": "string",
          "const": "agent_message"
        }
      },
      "required": [
        "type",
        "session_id",
        "message_id",
        "event_id",
        "batch",
        "content"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "session_id": {
          "$ref": "#/components/schemas/AgentSessionId"
        },
        "tool_id": {
          "$ref": "#/components/schemas/ToolCallId"
        },
        "snapshot": {
          "$ref": "#/components/schemas/AgentObjectRef"
        },
        "type": {
          "type": "string",
          "const": "tool_invocation"
        }
      },
      "required": [
        "type",
        "session_id",
        "tool_id",
        "snapshot"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "session_id": {
          "$ref": "#/components/schemas/AgentSessionId"
        },
        "tool_id": {
          "$ref": "#/components/schemas/ToolCallId"
        },
        "snapshot": {
          "$ref": "#/components/schemas/AgentObjectRef"
        },
        "content": {
          "$ref": "#/components/schemas/AgentObjectRef"
        },
        "type": {
          "type": "string",
          "const": "tool_output"
        }
      },
      "required": [
        "type",
        "session_id",
        "tool_id",
        "snapshot",
        "content"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "thread_id": {
          "$ref": "#/components/schemas/ReviewThreadId"
        },
        "type": {
          "type": "string",
          "const": "review_thread"
        }
      },
      "required": [
        "type",
        "thread_id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "thread_id": {
          "$ref": "#/components/schemas/ReviewThreadId"
        },
        "comment_id": {
          "$ref": "#/components/schemas/CommentId"
        },
        "revision": {
          "$ref": "#/components/schemas/ReviewObjectRef"
        },
        "type": {
          "type": "string",
          "const": "review_comment"
        }
      },
      "required": [
        "type",
        "thread_id",
        "comment_id",
        "revision"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "repository": {
          "$ref": "#/components/schemas/RepositoryId"
        },
        "thread_id": {
          "$ref": "#/components/schemas/ReviewThreadId"
        },
        "comment_id": {
          "$ref": "#/components/schemas/CommentId"
        },
        "revision": {
          "$ref": "#/components/schemas/ReviewObjectRef"
        },
        "type": {
          "type": "string",
          "const": "repository_review_comment"
        }
      },
      "required": [
        "type",
        "repository",
        "thread_id",
        "comment_id",
        "revision"
      ],
      "description": "An independently authorized ReviewStore. Its objects are not dependencies\nin the conversation repository's local review graph."
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "task_id": {
          "$ref": "#/components/schemas/TaskId"
        },
        "comment_id": {
          "$ref": "#/components/schemas/WorkCommentId"
        },
        "version": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0
        },
        "type": {
          "type": "string",
          "const": "work_comment"
        }
      },
      "required": [
        "type",
        "task_id",
        "comment_id",
        "version"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "session_id": {
          "$ref": "#/components/schemas/AgentSessionId"
        },
        "checkpoint_id": {
          "$ref": "#/components/schemas/AgentCheckpointId"
        },
        "version": {
          "$ref": "#/components/schemas/AgentObjectRef"
        },
        "type": {
          "type": "string",
          "const": "agent_checkpoint"
        }
      },
      "required": [
        "type",
        "session_id",
        "checkpoint_id",
        "version"
      ]
    }
  ],
  "description": "Identity, immutable content and validation witnesses are explicit, never a UI position."
}
```

## ThreadId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## HumanMessageId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## CollabObjectRef

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "kind": {
      "$ref": "#/components/schemas/CollabObjectKind"
    },
    "id": {
      "$ref": "#/components/schemas/CollabObjectId"
    }
  },
  "required": [
    "kind",
    "id"
  ]
}
```

## CollabObjectKind

```text
{
  "type": "string",
  "enum": [
    "index_page",
    "thread_version",
    "conversation_metadata",
    "thread_event",
    "human_message_revision",
    "thread_link",
    "reaction",
    "reaction_retraction",
    "reaction_slot",
    "custom_emoji_version",
    "image_asset"
  ]
}
```

## CollabObjectId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{64}$",
  "minLength": 64,
  "maxLength": 64
}
```

## MessageId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## AgentEventId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## AgentObjectRef

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "kind": {
      "$ref": "#/components/schemas/AgentObjectKind"
    },
    "id": {
      "$ref": "#/components/schemas/AgentObjectId"
    }
  },
  "required": [
    "kind",
    "id"
  ]
}
```

## AgentObjectKind

```text
{
  "type": "string",
  "enum": [
    "index_page",
    "session_snapshot",
    "turn_snapshot",
    "message_snapshot",
    "tool_snapshot",
    "intent",
    "observation_batch",
    "event_batch",
    "transcript_manifest",
    "transcript_page",
    "transcript_chunk",
    "content",
    "checkpoint_version",
    "association",
    "alias",
    "relation_page",
    "source_state",
    "projection_catalog",
    "catalog_conflict"
  ]
}
```

## ToolCallId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## ReviewThreadId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## CommentId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## ReviewObjectRef

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "kind": {
      "$ref": "#/components/schemas/ReviewObjectKind"
    },
    "id": {
      "$ref": "#/components/schemas/ReviewObjectId"
    }
  },
  "required": [
    "kind",
    "id"
  ]
}
```

## ReviewObjectKind

```text
{
  "type": "string",
  "enum": [
    "index_page",
    "anchor",
    "comment_body",
    "comment_revision",
    "thread_event",
    "event_page",
    "thread_snapshot",
    "relation_page",
    "status_page"
  ]
}
```

## ReviewObjectId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{64}$",
  "minLength": 64,
  "maxLength": 64
}
```

## TaskId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## WorkCommentId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## AgentCheckpointId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## AgentExecutionFeedbackReply

```text
{
  "type": "object",
  "properties": {
    "historical": {
      "type": "boolean",
      "default": false
    },
    "batches": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/RequestId"
      },
      "default": []
    },
    "thread_id": {
      "$ref": "#/components/schemas/ThreadId"
    },
    "revision": {
      "$ref": "#/components/schemas/CollabObjectRef"
    },
    "message": {
      "$ref": "#/components/schemas/HumanMessageRevisionV1"
    },
    "thread_revision": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ],
      "default": null
    },
    "source": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ConversationSourceV1"
        },
        {
          "type": "null"
        }
      ],
      "default": null
    },
    "answers": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommentReplyLinkV1"
      },
      "default": []
    }
  },
  "required": [
    "historical",
    "batches",
    "thread_id",
    "revision",
    "message",
    "thread_revision",
    "source",
    "answers"
  ]
}
```

## HumanMessageRevisionV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "schema_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "message_id": {
      "$ref": "#/components/schemas/HumanMessageId"
    },
    "thread_id": {
      "$ref": "#/components/schemas/ThreadId"
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "accepted_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "previous": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ]
    },
    "reply_to": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ]
    },
    "body": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "schema_version",
    "message_id",
    "thread_id",
    "actor",
    "accepted_at_millis",
    "previous",
    "body"
  ]
}
```

## ConversationSourceV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "target": {
      "$ref": "#/components/schemas/ReactionTargetV1"
    },
    "part": {
      "type": [
        "string",
        "null"
      ],
      "description": "JSON pointer to an exact string in captured JSON; None selects plain text."
    },
    "selection": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/TextSelectionV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "quote": {
      "type": "string",
      "description": "Retained display evidence, validated against the selected bytes."
    }
  },
  "required": [
    "target",
    "part",
    "selection",
    "quote"
  ]
}
```

## TextSelectionV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "encoding": {
      "$ref": "#/components/schemas/TextEncoding"
    },
    "start_byte": {
      "$ref": "#/components/schemas/ByteOffset"
    },
    "end_byte": {
      "$ref": "#/components/schemas/ByteOffset"
    },
    "selected_sha256": {
      "$ref": "#/components/schemas/ReviewDigest"
    }
  },
  "required": [
    "encoding",
    "start_byte",
    "end_byte",
    "selected_sha256"
  ]
}
```

## TextEncoding

```text
{
  "type": "string",
  "enum": [
    "utf8"
  ]
}
```

## ByteOffset

```text
{
  "type": "string",
  "pattern": "^(0|[1-9][0-9]*)$"
}
```

## ReviewDigest

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{64}$",
  "minLength": 64,
  "maxLength": 64
}
```

## CommentReplyLinkV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "prompt": {
      "description": "Exact user message capture containing a successful context delivery.",
      "allOf": [
        {
          "$ref": "#/components/schemas/ReactionTargetV1"
        }
      ]
    },
    "answer": {
      "$ref": "#/components/schemas/ConversationSourceV1"
    },
    "comments": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReactionTargetV1"
      }
    }
  },
  "required": [
    "prompt",
    "answer",
    "comments"
  ]
}
```

## AgentExecutionFeedbackTextPart

```text
{
  "type": "object",
  "properties": {
    "part": {
      "type": [
        "string",
        "null"
      ]
    },
    "text": {
      "type": "string"
    }
  },
  "required": [
    "part",
    "text"
  ]
}
```

## AgentTranscriptDescriptor

```text
{
  "type": "object",
  "properties": {
    "transcript_id": {
      "$ref": "#/components/schemas/AgentObjectId"
    },
    "source_instance_id": {
      "$ref": "#/components/schemas/SourceInstanceId"
    },
    "source_generation": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "source_format": {
      "type": "string"
    },
    "decoder_version": {
      "type": "string"
    },
    "sanitizer_version": {
      "type": "string"
    },
    "representation": {
      "$ref": "#/components/schemas/SanitizedRepresentation"
    },
    "logical_byte_length": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "record_count": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "gaps": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    }
  },
  "required": [
    "transcript_id",
    "source_instance_id",
    "source_generation",
    "source_format",
    "decoder_version",
    "sanitizer_version",
    "representation",
    "logical_byte_length",
    "record_count",
    "gaps"
  ]
}
```

## SourceInstanceId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## SanitizedRepresentation

```text
{
  "type": "string",
  "enum": [
    "sanitized_native",
    "canonical_producer"
  ]
}
```

## AgentToolLink

```text
{
  "type": "object",
  "properties": {
    "tool_call_id": {
      "$ref": "#/components/schemas/ToolCallId"
    },
    "snapshot_id": {
      "$ref": "#/components/schemas/AgentObjectId"
    }
  },
  "required": [
    "tool_call_id",
    "snapshot_id"
  ]
}
```

## AgentActivity

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "sequence": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0
        },
        "event_id": {
          "$ref": "#/components/schemas/AgentEventId"
        },
        "state": {
          "$ref": "#/components/schemas/SessionState"
        },
        "type": {
          "type": "string",
          "const": "session_state"
        }
      },
      "required": [
        "type",
        "sequence",
        "event_id",
        "state"
      ]
    },
    {
      "type": "object",
      "properties": {
        "reaction_target": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/ReactionTargetV1"
            },
            {
              "type": "null"
            }
          ]
        },
        "sequence": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0
        },
        "event_id": {
          "$ref": "#/components/schemas/AgentEventId"
        },
        "turn_id": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/TurnId"
            },
            {
              "type": "null"
            }
          ]
        },
        "message_id": {
          "$ref": "#/components/schemas/MessageId"
        },
        "occurred_at_millis": {
          "type": [
            "integer",
            "null"
          ],
          "format": "int64"
        },
        "role": {
          "$ref": "#/components/schemas/MessageRole"
        },
        "content": {
          "$ref": "#/components/schemas/AgentContent"
        },
        "type": {
          "type": "string",
          "const": "message"
        }
      },
      "required": [
        "type",
        "sequence",
        "event_id",
        "turn_id",
        "message_id",
        "role",
        "content"
      ]
    },
    {
      "type": "object",
      "properties": {
        "sequence": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0
        },
        "event_id": {
          "$ref": "#/components/schemas/AgentEventId"
        },
        "tool": {
          "$ref": "#/components/schemas/AgentToolActivity"
        },
        "type": {
          "type": "string",
          "const": "tool"
        }
      },
      "required": [
        "type",
        "sequence",
        "event_id",
        "tool"
      ]
    },
    {
      "type": "object",
      "properties": {
        "sequence": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0
        },
        "event_id": {
          "$ref": "#/components/schemas/AgentEventId"
        },
        "tool_call_id": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/ToolCallId"
            },
            {
              "type": "null"
            }
          ]
        },
        "state": {
          "$ref": "#/components/schemas/PermissionState"
        },
        "type": {
          "type": "string",
          "const": "permission"
        }
      },
      "required": [
        "type",
        "sequence",
        "event_id",
        "tool_call_id",
        "state"
      ]
    },
    {
      "type": "object",
      "properties": {
        "sequence": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0
        },
        "event_id": {
          "$ref": "#/components/schemas/AgentEventId"
        },
        "path": {
          "type": "string"
        },
        "type": {
          "type": "string",
          "const": "file_observed"
        }
      },
      "required": [
        "type",
        "sequence",
        "event_id",
        "path"
      ]
    },
    {
      "type": "object",
      "properties": {
        "sequence": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0
        },
        "event_id": {
          "$ref": "#/components/schemas/AgentEventId"
        },
        "missing_from": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0
        },
        "missing_to": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0
        },
        "type": {
          "type": "string",
          "const": "capture_gap"
        }
      },
      "required": [
        "type",
        "sequence",
        "event_id",
        "missing_from",
        "missing_to"
      ]
    }
  ]
}
```

## TurnId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## MessageRole

```text
{
  "type": "string",
  "enum": [
    "user",
    "assistant",
    "system",
    "tool"
  ]
}
```

## AgentContent

```text
{
  "type": "object",
  "properties": {
    "media_type": {
      "$ref": "#/components/schemas/SanitizedMediaType"
    },
    "content": {
      "type": "string"
    }
  },
  "required": [
    "media_type",
    "content"
  ]
}
```

## SanitizedMediaType

```text
{
  "type": "string",
  "enum": [
    "json",
    "text"
  ]
}
```

## AgentToolActivity

```text
{
  "type": "object",
  "properties": {
    "reaction_target": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReactionTargetV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "tool_call_id": {
      "$ref": "#/components/schemas/ToolCallId"
    },
    "snapshot_id": {
      "$ref": "#/components/schemas/AgentObjectId"
    },
    "turn_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/TurnId"
        },
        {
          "type": "null"
        }
      ]
    },
    "native_name": {
      "type": "string"
    },
    "kind": {
      "$ref": "#/components/schemas/ToolKind"
    },
    "state": {
      "$ref": "#/components/schemas/ToolState"
    },
    "duration_millis": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    },
    "input": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentContent"
        },
        {
          "type": "null"
        }
      ]
    },
    "output": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentContent"
        },
        {
          "type": "null"
        }
      ]
    },
    "observed_paths": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "code": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentCodeAssociation"
      }
    }
  },
  "required": [
    "tool_call_id",
    "snapshot_id",
    "turn_id",
    "native_name",
    "kind",
    "state",
    "input",
    "output",
    "observed_paths",
    "code"
  ]
}
```

## ToolKind

```text
{
  "type": "string",
  "enum": [
    "read",
    "edit",
    "delete",
    "move",
    "search",
    "execute",
    "fetch",
    "plan",
    "delegate",
    "other"
  ]
}
```

## ToolState

```text
{
  "type": "string",
  "enum": [
    "pending",
    "running",
    "succeeded",
    "failed",
    "cancelled",
    "interrupted",
    "skipped",
    "unknown"
  ]
}
```

## AgentCodeAssociation

```text
{
  "type": "object",
  "properties": {
    "association_id": {
      "$ref": "#/components/schemas/AssociationId"
    },
    "object_id": {
      "$ref": "#/components/schemas/AgentObjectId"
    },
    "session_id": {
      "$ref": "#/components/schemas/AgentSessionId"
    },
    "tool_call_ids": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ToolCallId"
      }
    },
    "relation": {
      "$ref": "#/components/schemas/AssociationRelation"
    },
    "before_operations": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "after_operations": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "before_commits": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "after_commits": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "changes": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ChangeId"
      }
    },
    "paths": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "created_at_millis": {
      "type": "integer",
      "format": "int64"
    }
  },
  "required": [
    "association_id",
    "object_id",
    "session_id",
    "tool_call_ids",
    "relation",
    "before_operations",
    "after_operations",
    "before_commits",
    "after_commits",
    "changes",
    "paths",
    "created_at_millis"
  ]
}
```

## AssociationId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## AssociationRelation

```text
{
  "type": "string",
  "enum": [
    "initiated_revision",
    "rewrote_revision",
    "contributed_to_snapshot",
    "observed_during_interval",
    "read_revision",
    "validated_revision",
    "manual_assertion"
  ]
}
```

## PermissionState

```text
{
  "type": "string",
  "enum": [
    "not_observed",
    "requested",
    "granted",
    "denied"
  ]
}
```

## AgentTranscriptPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/AgentReadSnapshot"
    },
    "session_id": {
      "$ref": "#/components/schemas/AgentSessionId"
    },
    "transcript_id": {
      "$ref": "#/components/schemas/AgentObjectId"
    },
    "source_instance_id": {
      "$ref": "#/components/schemas/SourceInstanceId"
    },
    "source_generation": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "chunks": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentTranscriptChunk"
      }
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "session_id",
    "transcript_id",
    "source_instance_id",
    "source_generation",
    "chunks",
    "next_cursor"
  ]
}
```

## AgentTranscriptChunk

```text
{
  "type": "object",
  "properties": {
    "sequence": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "occurred_at_millis": {
      "type": [
        "integer",
        "null"
      ],
      "format": "int64"
    },
    "media_type": {
      "$ref": "#/components/schemas/SanitizedMediaType"
    },
    "content": {
      "type": "string"
    }
  },
  "required": [
    "sequence",
    "media_type",
    "content"
  ]
}
```

## AgentRelationPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/AgentReadSnapshot"
    },
    "subject": {
      "$ref": "#/components/schemas/AgentRelationSubject"
    },
    "relations": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentRelationLink"
      }
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "subject",
    "relations",
    "next_cursor"
  ]
}
```

## AgentRelationSubject

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "session_id": {
          "$ref": "#/components/schemas/AgentSessionId"
        },
        "subject": {
          "type": "string",
          "const": "session"
        }
      },
      "required": [
        "subject",
        "session_id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "session_id": {
          "$ref": "#/components/schemas/AgentSessionId"
        },
        "tool_call_id": {
          "$ref": "#/components/schemas/ToolCallId"
        },
        "subject": {
          "type": "string",
          "const": "tool"
        }
      },
      "required": [
        "subject",
        "session_id",
        "tool_call_id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "operation_id": {
          "$ref": "#/components/schemas/OperationId"
        },
        "subject": {
          "type": "string",
          "const": "operation"
        }
      },
      "required": [
        "subject",
        "operation_id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "commit_id": {
          "$ref": "#/components/schemas/CommitId"
        },
        "subject": {
          "type": "string",
          "const": "revision"
        }
      },
      "required": [
        "subject",
        "commit_id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "checkpoint_id": {
          "$ref": "#/components/schemas/AgentCheckpointId"
        },
        "subject": {
          "type": "string",
          "const": "checkpoint"
        }
      },
      "required": [
        "subject",
        "checkpoint_id"
      ]
    }
  ]
}
```

## AgentRelationLink

```text
{
  "type": "object",
  "properties": {
    "kind": {
      "$ref": "#/components/schemas/AgentRelationKind"
    },
    "object_id": {
      "$ref": "#/components/schemas/AgentObjectId"
    },
    "association": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentCodeAssociation"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "kind",
    "object_id",
    "association"
  ]
}
```

## AgentRelationKind

```text
{
  "type": "string",
  "enum": [
    "checkpoint",
    "association"
  ]
}
```

## AgentCheckpointPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/AgentReadSnapshot"
    },
    "checkpoints": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentCheckpointSummary"
      }
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "checkpoints",
    "next_cursor"
  ]
}
```

## AgentCheckpointSummary

```text
{
  "type": "object",
  "properties": {
    "checkpoint_id": {
      "$ref": "#/components/schemas/AgentCheckpointId"
    },
    "version_id": {
      "$ref": "#/components/schemas/AgentObjectId"
    },
    "session_id": {
      "$ref": "#/components/schemas/AgentSessionId"
    },
    "turn_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/TurnId"
        },
        {
          "type": "null"
        }
      ]
    },
    "scope_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ProvenanceScopeId"
        },
        {
          "type": "null"
        }
      ]
    },
    "before": {
      "$ref": "#/components/schemas/AgentCodeAnchor"
    },
    "after": {
      "$ref": "#/components/schemas/AgentCodeAnchor"
    },
    "code_relation": {
      "$ref": "#/components/schemas/CodeRelation"
    },
    "evidence_method": {
      "$ref": "#/components/schemas/EvidenceMethod"
    },
    "source_trust": {
      "$ref": "#/components/schemas/SourceTrust"
    },
    "exclusive_write_scope": {
      "type": "boolean"
    },
    "coverage": {
      "$ref": "#/components/schemas/AgentCoverage"
    },
    "capture_status": {
      "$ref": "#/components/schemas/CaptureStatusV1"
    },
    "execution_outcome": {
      "$ref": "#/components/schemas/ExecutionOutcomeV1"
    },
    "event_batch_count": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "tool_count": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "association_count": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "supersedes": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentObjectId"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "checkpoint_id",
    "version_id",
    "session_id",
    "turn_id",
    "scope_id",
    "before",
    "after",
    "code_relation",
    "evidence_method",
    "source_trust",
    "exclusive_write_scope",
    "coverage",
    "capture_status",
    "execution_outcome",
    "event_batch_count",
    "tool_count",
    "association_count",
    "supersedes"
  ]
}
```

## ProvenanceScopeId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## AgentCodeAnchor

```text
{
  "type": "object",
  "properties": {
    "operations": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "commits": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    }
  },
  "required": [
    "operations",
    "commits"
  ]
}
```

## CodeRelation

```text
{
  "type": "string",
  "enum": [
    "changed",
    "no_change",
    "not_observed"
  ]
}
```

## EvidenceMethod

```text
{
  "type": "string",
  "enum": [
    "controlled_scope",
    "hook_boundary",
    "transcript_recovery",
    "historical_import",
    "manual_assertion"
  ]
}
```

## SourceTrust

```text
{
  "type": "string",
  "enum": [
    "authenticated_controller",
    "local_user_hook",
    "locally_trusted_plugin",
    "approved_harness_api",
    "local_transcript",
    "imported_archive",
    "untrusted_assertion"
  ]
}
```

## CaptureStatusV1

```text
{
  "type": "string",
  "enum": [
    "complete",
    "partial",
    "missing"
  ]
}
```

## ExecutionOutcomeV1

```text
{
  "type": "string",
  "enum": [
    "succeeded",
    "failed",
    "cancelled",
    "interrupted",
    "unknown"
  ]
}
```

## AgentCheckpointDetail

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/AgentReadSnapshot"
    },
    "checkpoint": {
      "$ref": "#/components/schemas/AgentCheckpointSummary"
    },
    "session_snapshot_id": {
      "$ref": "#/components/schemas/AgentObjectId"
    },
    "transcript_id": {
      "$ref": "#/components/schemas/AgentObjectId"
    },
    "event_batches": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentObjectId"
      }
    },
    "tools": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentToolLink"
      }
    },
    "associations": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentObjectId"
      }
    }
  },
  "required": [
    "version",
    "snapshot",
    "checkpoint",
    "session_snapshot_id",
    "transcript_id",
    "event_batches",
    "tools",
    "associations"
  ]
}
```

## ReviewThreadPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/ReviewReadSnapshot"
    },
    "threads": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewThreadSummary"
      }
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "threads",
    "next_cursor"
  ]
}
```

## ReviewReadSnapshot

```text
{
  "type": "object",
  "properties": {
    "repo_id": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "published_seq": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "manifest_revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "review_root": {
      "$ref": "#/components/schemas/RecordDigest"
    },
    "review_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "repo_id",
    "epoch",
    "published_seq",
    "manifest_revision",
    "review_root",
    "review_state_version"
  ]
}
```

## ReviewThreadSummary

```text
{
  "type": "object",
  "properties": {
    "thread_id": {
      "$ref": "#/components/schemas/ReviewThreadId"
    },
    "snapshot_id": {
      "$ref": "#/components/schemas/ReviewObjectId"
    },
    "original_anchor_id": {
      "$ref": "#/components/schemas/ReviewObjectId"
    },
    "anchor": {
      "$ref": "#/components/schemas/ArtifactAnchorV1"
    },
    "event_page_root_id": {
      "$ref": "#/components/schemas/ReviewObjectId"
    },
    "status": {
      "$ref": "#/components/schemas/ThreadStatusV1"
    },
    "selected_workflow_action": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReviewActionId"
        },
        {
          "type": "null"
        }
      ]
    },
    "selected_workflow_actions": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewActionId"
      }
    },
    "thread_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "content_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "workflow_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "comment_count": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "comment_conflict_count": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "last_event_sequence": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "presentation": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReviewSummaryPresentation"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "thread_id",
    "snapshot_id",
    "original_anchor_id",
    "anchor",
    "event_page_root_id",
    "status",
    "selected_workflow_action",
    "selected_workflow_actions",
    "thread_version",
    "content_version",
    "workflow_version",
    "comment_count",
    "comment_conflict_count",
    "last_event_sequence"
  ]
}
```

## ArtifactAnchorV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "schema_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "repository_id": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "target": {
      "$ref": "#/components/schemas/ArtifactTargetV1"
    },
    "created_context": {
      "$ref": "#/components/schemas/CreatedContextV1"
    },
    "sensitivity": {
      "$ref": "#/components/schemas/SensitivityV1"
    }
  },
  "required": [
    "schema_version",
    "repository_id",
    "target",
    "created_context",
    "sensitivity"
  ]
}
```

## ArtifactTargetV1

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "target": {
          "type": "string",
          "const": "code_text"
        },
        "value": {
          "$ref": "#/components/schemas/CodeTextAnchorV1"
        }
      },
      "required": [
        "target",
        "value"
      ]
    },
    {
      "type": "object",
      "properties": {
        "target": {
          "type": "string",
          "const": "portable_code_text"
        },
        "value": {
          "$ref": "#/components/schemas/PortableCodeTextV2"
        }
      },
      "required": [
        "target",
        "value"
      ]
    },
    {
      "type": "object",
      "properties": {
        "target": {
          "type": "string",
          "const": "portable_code_artifact"
        },
        "value": {
          "$ref": "#/components/schemas/PortableCodeArtifactV2"
        }
      },
      "required": [
        "target",
        "value"
      ]
    },
    {
      "type": "object",
      "properties": {
        "target": {
          "type": "string",
          "const": "code_artifact"
        },
        "value": {
          "$ref": "#/components/schemas/CodeArtifactAnchorV1"
        }
      },
      "required": [
        "target",
        "value"
      ]
    },
    {
      "type": "object",
      "properties": {
        "target": {
          "type": "string",
          "const": "agent_text"
        },
        "value": {
          "$ref": "#/components/schemas/AgentTextAnchorV1"
        }
      },
      "required": [
        "target",
        "value"
      ]
    },
    {
      "type": "object",
      "properties": {
        "target": {
          "type": "string",
          "const": "agent_artifact"
        },
        "value": {
          "$ref": "#/components/schemas/AgentArtifactAnchorV1"
        }
      },
      "required": [
        "target",
        "value"
      ]
    },
    {
      "type": "object",
      "properties": {
        "target": {
          "type": "string",
          "const": "agent_session"
        },
        "value": {
          "type": "object",
          "properties": {
            "session_id": {
              "$ref": "#/components/schemas/AgentSessionId"
            },
            "session_snapshot": {
              "$ref": "#/components/schemas/AgentObjectRef"
            }
          },
          "required": [
            "session_id",
            "session_snapshot"
          ]
        }
      },
      "required": [
        "target",
        "value"
      ]
    },
    {
      "type": "object",
      "properties": {
        "target": {
          "type": "string",
          "const": "revision"
        },
        "value": {
          "type": "object",
          "properties": {
            "operation_id": {
              "$ref": "#/components/schemas/OperationId"
            },
            "commit_id": {
              "$ref": "#/components/schemas/CommitId"
            },
            "change_id": {
              "anyOf": [
                {
                  "$ref": "#/components/schemas/ChangeId"
                },
                {
                  "type": "null"
                }
              ]
            }
          },
          "required": [
            "operation_id",
            "commit_id",
            "change_id"
          ]
        }
      },
      "required": [
        "target",
        "value"
      ]
    },
    {
      "type": "object",
      "properties": {
        "target": {
          "type": "string",
          "const": "operation"
        },
        "value": {
          "type": "object",
          "properties": {
            "operation_id": {
              "$ref": "#/components/schemas/OperationId"
            }
          },
          "required": [
            "operation_id"
          ]
        }
      },
      "required": [
        "target",
        "value"
      ]
    },
    {
      "type": "object",
      "properties": {
        "target": {
          "type": "string",
          "const": "repository"
        }
      },
      "required": [
        "target"
      ]
    }
  ]
}
```

## CodeTextAnchorV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "operation_id": {
      "$ref": "#/components/schemas/OperationId"
    },
    "commit_id": {
      "$ref": "#/components/schemas/CommitId"
    },
    "change_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ChangeId"
        },
        {
          "type": "null"
        }
      ]
    },
    "path": {
      "type": "string"
    },
    "file_id": {
      "$ref": "#/components/schemas/FileId"
    },
    "value_kind": {
      "$ref": "#/components/schemas/CodeValueKind"
    },
    "text": {
      "$ref": "#/components/schemas/TextSelectionV1"
    },
    "diff_context": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/DiffContextV1"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "operation_id",
    "commit_id",
    "change_id",
    "path",
    "file_id",
    "value_kind",
    "text",
    "diff_context"
  ]
}
```

## CodeValueKind

```text
{
  "type": "string",
  "enum": [
    "file",
    "binary_file",
    "empty_file",
    "symlink",
    "gitlink",
    "conflict_term",
    "conflict_artifact"
  ]
}
```

## DiffContextV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "before_operation_id": {
      "$ref": "#/components/schemas/OperationId"
    },
    "before_commit_id": {
      "$ref": "#/components/schemas/CommitId"
    },
    "before_path": {
      "type": [
        "string",
        "null"
      ]
    },
    "after_operation_id": {
      "$ref": "#/components/schemas/OperationId"
    },
    "after_commit_id": {
      "$ref": "#/components/schemas/CommitId"
    },
    "after_path": {
      "type": [
        "string",
        "null"
      ]
    },
    "side": {
      "$ref": "#/components/schemas/DiffSide"
    },
    "comparison": {
      "$ref": "#/components/schemas/ComparisonKind"
    }
  },
  "required": [
    "before_operation_id",
    "before_commit_id",
    "before_path",
    "after_operation_id",
    "after_commit_id",
    "after_path",
    "side",
    "comparison"
  ]
}
```

## DiffSide

```text
{
  "type": "string",
  "enum": [
    "before",
    "after"
  ]
}
```

## ComparisonKind

```text
{
  "type": "string",
  "enum": [
    "explicit_pair"
  ]
}
```

## PortableCodeTextV2

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "artifact": {
      "$ref": "#/components/schemas/PortableCodeArtifactV2"
    },
    "text": {
      "$ref": "#/components/schemas/TextSelectionV1"
    },
    "diff_context": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/PortableDiffContextV2"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "artifact",
    "text",
    "diff_context"
  ]
}
```

## PortableCodeArtifactV2

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "revision": {
      "$ref": "#/components/schemas/PortableRevisionV2"
    },
    "path": {
      "type": "string"
    },
    "object": {
      "$ref": "#/components/schemas/CodeObjectIdV1"
    },
    "value_kind": {
      "$ref": "#/components/schemas/CodeValueKind"
    }
  },
  "required": [
    "revision",
    "path",
    "object",
    "value_kind"
  ]
}
```

## PortableRevisionV2

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "evidence": {
      "$ref": "#/components/schemas/CodeEvidenceRefV1"
    },
    "commit": {
      "$ref": "#/components/schemas/CodeCommitIdV1"
    },
    "change": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ChangeId"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "evidence",
    "commit",
    "change"
  ]
}
```

## CodeEvidenceRefV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "profile": {
      "$ref": "#/components/schemas/CodeProfileV1"
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "capsule": {
      "$ref": "#/components/schemas/CodeCapsuleId"
    }
  },
  "required": [
    "repository",
    "profile",
    "operation",
    "capsule"
  ]
}
```

## CodeCapsuleId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{64}$",
  "minLength": 64,
  "maxLength": 64
}
```

## CodeCommitIdV1

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "profile": {
          "type": "string",
          "const": "native_v1"
        },
        "id": {
          "$ref": "#/components/schemas/CommitId"
        }
      },
      "required": [
        "profile",
        "id"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "profile": {
          "type": "string",
          "const": "git_sha1_jj045"
        },
        "id": {
          "$ref": "#/components/schemas/GitSha1Id"
        }
      },
      "required": [
        "profile",
        "id"
      ],
      "additionalProperties": false
    }
  ]
}
```

## GitSha1Id

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{40}$",
  "minLength": 40,
  "maxLength": 40
}
```

## CodeObjectIdV1

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "object": {
          "$ref": "#/components/schemas/ObjectKey"
        },
        "profile": {
          "type": "string",
          "const": "native_v1"
        }
      },
      "required": [
        "profile",
        "object"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "kind": {
          "$ref": "#/components/schemas/GitObjectKindV1"
        },
        "id": {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        "profile": {
          "type": "string",
          "const": "git_sha1_jj045"
        }
      },
      "required": [
        "profile",
        "kind",
        "id"
      ]
    }
  ]
}
```

## ObjectKey

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "commit"
        },
        "id": {
          "$ref": "#/components/schemas/CommitId"
        }
      },
      "required": [
        "kind",
        "id"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "tree"
        },
        "id": {
          "$ref": "#/components/schemas/TreeId"
        }
      },
      "required": [
        "kind",
        "id"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "file"
        },
        "id": {
          "$ref": "#/components/schemas/FileId"
        }
      },
      "required": [
        "kind",
        "id"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "symlink"
        },
        "id": {
          "$ref": "#/components/schemas/SymlinkId"
        }
      },
      "required": [
        "kind",
        "id"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "copy"
        },
        "id": {
          "$ref": "#/components/schemas/CopyId"
        }
      },
      "required": [
        "kind",
        "id"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "view"
        },
        "id": {
          "$ref": "#/components/schemas/ViewId"
        }
      },
      "required": [
        "kind",
        "id"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "operation"
        },
        "id": {
          "$ref": "#/components/schemas/OperationId"
        }
      },
      "required": [
        "kind",
        "id"
      ],
      "additionalProperties": false
    }
  ]
}
```

## TreeId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{128}$",
  "minLength": 128,
  "maxLength": 128
}
```

## SymlinkId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{128}$",
  "minLength": 128,
  "maxLength": 128
}
```

## CopyId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{128}$",
  "minLength": 128,
  "maxLength": 128
}
```

## ViewId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{128}$",
  "minLength": 128,
  "maxLength": 128
}
```

## GitObjectKindV1

```text
{
  "type": "string",
  "enum": [
    "commit",
    "tree",
    "blob"
  ]
}
```

## PortableDiffContextV2

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "before": {
      "$ref": "#/components/schemas/PortableRevisionV2"
    },
    "before_path": {
      "type": [
        "string",
        "null"
      ]
    },
    "after": {
      "$ref": "#/components/schemas/PortableRevisionV2"
    },
    "after_path": {
      "type": [
        "string",
        "null"
      ]
    },
    "side": {
      "$ref": "#/components/schemas/DiffSide"
    }
  },
  "required": [
    "before",
    "before_path",
    "after",
    "after_path",
    "side"
  ]
}
```

## CodeArtifactAnchorV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "operation_id": {
      "$ref": "#/components/schemas/OperationId"
    },
    "commit_id": {
      "$ref": "#/components/schemas/CommitId"
    },
    "change_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ChangeId"
        },
        {
          "type": "null"
        }
      ]
    },
    "path": {
      "type": "string"
    },
    "object": {
      "$ref": "#/components/schemas/ObjectKey"
    },
    "value_kind": {
      "$ref": "#/components/schemas/CodeValueKind"
    }
  },
  "required": [
    "operation_id",
    "commit_id",
    "change_id",
    "path",
    "object",
    "value_kind"
  ]
}
```

## AgentTextAnchorV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "session_id": {
      "$ref": "#/components/schemas/AgentSessionId"
    },
    "session_snapshot": {
      "$ref": "#/components/schemas/AgentObjectRef"
    },
    "owner": {
      "$ref": "#/components/schemas/AgentContentOwnerV1"
    },
    "block_id": {
      "$ref": "#/components/schemas/BlockId"
    },
    "block_version": {
      "$ref": "#/components/schemas/AgentObjectRef"
    },
    "part": {
      "$ref": "#/components/schemas/AgentContentPart"
    },
    "content": {
      "$ref": "#/components/schemas/AgentObjectRef"
    },
    "representation_version": {
      "type": "string"
    },
    "sanitizer_version": {
      "type": "string"
    },
    "normalizer_version": {
      "type": "string"
    },
    "source_coverage": {
      "$ref": "#/components/schemas/SourceCoverageV1"
    },
    "text": {
      "$ref": "#/components/schemas/TextSelectionV1"
    }
  },
  "required": [
    "session_id",
    "session_snapshot",
    "owner",
    "block_id",
    "block_version",
    "part",
    "content",
    "representation_version",
    "sanitizer_version",
    "normalizer_version",
    "source_coverage",
    "text"
  ]
}
```

## AgentContentOwnerV1

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "tool_call_id": {
          "$ref": "#/components/schemas/ToolCallId"
        },
        "snapshot": {
          "$ref": "#/components/schemas/AgentObjectRef"
        },
        "attempt_id": {
          "$ref": "#/components/schemas/ToolAttemptId"
        },
        "kind": {
          "type": "string",
          "const": "tool"
        }
      },
      "required": [
        "kind",
        "tool_call_id",
        "snapshot",
        "attempt_id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "message_id": {
          "$ref": "#/components/schemas/MessageId"
        },
        "snapshot": {
          "$ref": "#/components/schemas/AgentObjectRef"
        },
        "kind": {
          "type": "string",
          "const": "message"
        }
      },
      "required": [
        "kind",
        "message_id",
        "snapshot"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "block_id": {
          "$ref": "#/components/schemas/BlockId"
        },
        "snapshot": {
          "$ref": "#/components/schemas/AgentObjectRef"
        },
        "kind": {
          "type": "string",
          "const": "plan"
        }
      },
      "required": [
        "kind",
        "block_id",
        "snapshot"
      ]
    }
  ]
}
```

## ToolAttemptId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## BlockId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## AgentContentPart

```text
{
  "type": "string",
  "enum": [
    "input",
    "output",
    "message",
    "plan",
    "exposed_summary"
  ]
}
```

## SourceCoverageV1

```text
{
  "type": "string",
  "enum": [
    "complete",
    "partial",
    "redacted",
    "truncated",
    "unknown"
  ]
}
```

## AgentArtifactAnchorV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "session_id": {
      "$ref": "#/components/schemas/AgentSessionId"
    },
    "session_snapshot": {
      "$ref": "#/components/schemas/AgentObjectRef"
    },
    "owner": {
      "$ref": "#/components/schemas/AgentContentOwnerV1"
    }
  },
  "required": [
    "session_id",
    "session_snapshot",
    "owner"
  ]
}
```

## CreatedContextV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "publication_position": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    },
    "code_state_version": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    },
    "agent_root": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RecordDigest"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "publication_position",
    "code_state_version",
    "agent_root"
  ]
}
```

## SensitivityV1

```text
{
  "type": "string",
  "enum": [
    "public",
    "repository",
    "restricted"
  ]
}
```

## ThreadStatusV1

```text
{
  "type": "string",
  "enum": [
    "open",
    "resolved"
  ]
}
```

## ReviewActionId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## ReviewSummaryPresentation

```text
{
  "type": "object",
  "properties": {
    "created_by": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "created_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "updated_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "excerpt": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "created_by",
    "created_at_millis",
    "updated_at_millis",
    "excerpt"
  ]
}
```

## ReviewActivitySummary

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/ReviewReadSnapshot"
    },
    "activity": {
      "$ref": "#/components/schemas/ReviewActivity"
    }
  },
  "required": [
    "version",
    "snapshot",
    "activity"
  ]
}
```

## ReviewActivity

```text
{
  "type": "object",
  "properties": {
    "threads": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "threads_partial": {
      "type": "boolean"
    },
    "updated_at_millis": {
      "type": [
        "integer",
        "null"
      ],
      "format": "int64"
    }
  },
  "required": [
    "threads",
    "threads_partial",
    "updated_at_millis"
  ],
  "description": "Facts about a bounded first catalog window, never an inferred complete count."
}
```

## ReviewThreadDetail

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/ReviewReadSnapshot"
    },
    "thread": {
      "$ref": "#/components/schemas/ReviewThreadSummary"
    },
    "anchor": {
      "$ref": "#/components/schemas/ReviewAnchor"
    },
    "comments": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewComment"
      }
    },
    "comment_conflicts": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewCommentConflict"
      }
    },
    "events": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewEvent"
      }
    }
  },
  "required": [
    "version",
    "snapshot",
    "thread",
    "anchor",
    "comments",
    "comment_conflicts",
    "events"
  ]
}
```

## ReviewAnchor

```text
{
  "type": "object",
  "properties": {
    "object_id": {
      "$ref": "#/components/schemas/ReviewObjectId"
    },
    "value": {
      "$ref": "#/components/schemas/ArtifactAnchorV1"
    }
  },
  "required": [
    "object_id",
    "value"
  ]
}
```

## ReviewComment

```text
{
  "type": "object",
  "properties": {
    "comment_id": {
      "$ref": "#/components/schemas/CommentId"
    },
    "revision_id": {
      "$ref": "#/components/schemas/ReviewObjectId"
    },
    "parent_comment_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CommentId"
        },
        {
          "type": "null"
        }
      ]
    },
    "author": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "created_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "previous_revision_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReviewObjectId"
        },
        {
          "type": "null"
        }
      ]
    },
    "superseded_revision_ids": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewObjectId"
      }
    },
    "content": {
      "$ref": "#/components/schemas/ReviewCommentContent"
    }
  },
  "required": [
    "comment_id",
    "revision_id",
    "parent_comment_id",
    "author",
    "actor",
    "created_at_millis",
    "previous_revision_id",
    "superseded_revision_ids",
    "content"
  ]
}
```

## ReviewCommentContent

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "body_id": {
          "$ref": "#/components/schemas/ReviewObjectId"
        },
        "format": {
          "$ref": "#/components/schemas/ReviewBodyFormatV1"
        },
        "text": {
          "type": "string"
        },
        "sanitizer_version": {
          "type": "string"
        },
        "state": {
          "type": "string",
          "const": "visible"
        }
      },
      "required": [
        "state",
        "body_id",
        "format",
        "text",
        "sanitizer_version"
      ]
    },
    {
      "type": "object",
      "properties": {
        "reason": {
          "type": [
            "string",
            "null"
          ]
        },
        "state": {
          "type": "string",
          "const": "tombstoned"
        }
      },
      "required": [
        "state",
        "reason"
      ]
    },
    {
      "type": "object",
      "properties": {
        "reason": {
          "type": "string"
        },
        "state": {
          "type": "string",
          "const": "hidden"
        }
      },
      "required": [
        "state",
        "reason"
      ]
    }
  ]
}
```

## ReviewBodyFormatV1

```text
{
  "type": "string",
  "enum": [
    "plain_text",
    "safe_markdown"
  ]
}
```

## ReviewCommentConflict

```text
{
  "type": "object",
  "properties": {
    "comment_id": {
      "$ref": "#/components/schemas/CommentId"
    },
    "parent_comment_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CommentId"
        },
        {
          "type": "null"
        }
      ]
    },
    "alternatives": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewCommentVersion"
      }
    }
  },
  "required": [
    "comment_id",
    "parent_comment_id",
    "alternatives"
  ]
}
```

## ReviewCommentVersion

```text
{
  "type": "object",
  "properties": {
    "revision_id": {
      "$ref": "#/components/schemas/ReviewObjectId"
    },
    "author": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "created_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "previous_revision_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReviewObjectId"
        },
        {
          "type": "null"
        }
      ]
    },
    "superseded_revision_ids": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewObjectId"
      }
    },
    "content": {
      "$ref": "#/components/schemas/ReviewCommentContent"
    }
  },
  "required": [
    "revision_id",
    "author",
    "actor",
    "created_at_millis",
    "previous_revision_id",
    "superseded_revision_ids",
    "content"
  ]
}
```

## ReviewEvent

```text
{
  "type": "object",
  "properties": {
    "event_id": {
      "$ref": "#/components/schemas/ReviewObjectId"
    },
    "causal_parent_ids": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewObjectId"
      }
    },
    "sequence": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "accepted_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "event": {
      "$ref": "#/components/schemas/ReviewEventKind"
    }
  },
  "required": [
    "event_id",
    "causal_parent_ids",
    "sequence",
    "actor",
    "accepted_at_millis",
    "event"
  ]
}
```

## ReviewEventKind

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "anchor_id": {
          "$ref": "#/components/schemas/ReviewObjectId"
        },
        "initial_comment_id": {
          "$ref": "#/components/schemas/CommentId"
        },
        "initial_revision_id": {
          "$ref": "#/components/schemas/ReviewObjectId"
        },
        "action": {
          "type": "string",
          "const": "created"
        }
      },
      "required": [
        "action",
        "anchor_id",
        "initial_comment_id",
        "initial_revision_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "comment_id": {
          "$ref": "#/components/schemas/CommentId"
        },
        "parent_comment_id": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/CommentId"
            },
            {
              "type": "null"
            }
          ]
        },
        "revision_id": {
          "$ref": "#/components/schemas/ReviewObjectId"
        },
        "action": {
          "type": "string",
          "const": "reply_added"
        }
      },
      "required": [
        "action",
        "comment_id",
        "parent_comment_id",
        "revision_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "comment_id": {
          "$ref": "#/components/schemas/CommentId"
        },
        "previous_revision_id": {
          "$ref": "#/components/schemas/ReviewObjectId"
        },
        "revision_id": {
          "$ref": "#/components/schemas/ReviewObjectId"
        },
        "action": {
          "type": "string",
          "const": "comment_revised"
        }
      },
      "required": [
        "action",
        "comment_id",
        "previous_revision_id",
        "revision_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "comment_id": {
          "$ref": "#/components/schemas/CommentId"
        },
        "superseded_revision_ids": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/ReviewObjectId"
          }
        },
        "revision_id": {
          "$ref": "#/components/schemas/ReviewObjectId"
        },
        "action": {
          "type": "string",
          "const": "comment_conflict_resolved"
        }
      },
      "required": [
        "action",
        "comment_id",
        "superseded_revision_ids",
        "revision_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "action_id": {
          "$ref": "#/components/schemas/ReviewActionId"
        },
        "reason": {
          "type": [
            "string",
            "null"
          ]
        },
        "action": {
          "type": "string",
          "const": "resolved"
        }
      },
      "required": [
        "action",
        "action_id",
        "reason"
      ]
    },
    {
      "type": "object",
      "properties": {
        "action_id": {
          "$ref": "#/components/schemas/ReviewActionId"
        },
        "reason": {
          "type": [
            "string",
            "null"
          ]
        },
        "action": {
          "type": "string",
          "const": "reopened"
        }
      },
      "required": [
        "action",
        "action_id",
        "reason"
      ]
    }
  ]
}
```

## ReviewCommandOutcome

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "receipt": {
          "$ref": "#/components/schemas/ReviewCommandReceipt"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "receipt"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## ReviewCommandReceipt

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "publication_id": {
      "$ref": "#/components/schemas/PublicationId"
    },
    "request_digest": {
      "$ref": "#/components/schemas/IntentDigest"
    },
    "accepted": {
      "$ref": "#/components/schemas/Snapshot"
    },
    "result": {
      "$ref": "#/components/schemas/ReviewCommandResultV1"
    }
  },
  "required": [
    "version",
    "request_id",
    "publication_id",
    "request_digest",
    "accepted",
    "result"
  ]
}
```

## PublicationId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## IntentDigest

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{64}$",
  "minLength": 64,
  "maxLength": 64
}
```

## Snapshot

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "position": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "code_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "heads": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    }
  },
  "required": [
    "epoch",
    "position",
    "heads"
  ]
}
```

## ReviewCommandResultV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "schema_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "thread_id": {
      "$ref": "#/components/schemas/ReviewThreadId"
    },
    "comment_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CommentId"
        },
        {
          "type": "null"
        }
      ]
    },
    "action_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReviewActionId"
        },
        {
          "type": "null"
        }
      ]
    },
    "thread_snapshot": {
      "$ref": "#/components/schemas/ReviewObjectRef"
    },
    "comment_revision": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReviewObjectRef"
        },
        {
          "type": "null"
        }
      ]
    },
    "event": {
      "$ref": "#/components/schemas/ReviewObjectRef"
    }
  },
  "required": [
    "schema_version",
    "thread_id",
    "comment_id",
    "action_id",
    "thread_snapshot",
    "comment_revision",
    "event"
  ]
}
```

## ReviewCommandStatus

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "receipt": {
          "$ref": "#/components/schemas/ReviewCommandReceipt"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "receipt"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "not_known"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## CollabPage

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/CollabReadSnapshotV1"
    },
    "items": {
      "$ref": "#/components/schemas/CollabPageItems"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "items",
    "next_cursor"
  ]
}
```

## CollabReadSnapshotV1

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "repo_id": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "manifest_digest": {
      "$ref": "#/components/schemas/RecordDigest"
    },
    "published_seq": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "code_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "operation_heads": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "thread_root": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RecordDigest"
        },
        {
          "type": "null"
        }
      ]
    },
    "thread_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "reaction_root": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RecordDigest"
        },
        {
          "type": "null"
        }
      ]
    },
    "reaction_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "agent_root": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RecordDigest"
        },
        {
          "type": "null"
        }
      ]
    },
    "review_root": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RecordDigest"
        },
        {
          "type": "null"
        }
      ]
    },
    "work_root": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RecordDigest"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "repo_id",
    "epoch",
    "manifest_digest",
    "published_seq",
    "code_state_version",
    "operation_heads",
    "thread_root",
    "thread_state_version",
    "reaction_root",
    "reaction_state_version",
    "agent_root",
    "review_root"
  ]
}
```

## CollabPageItems

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "threads": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/ThreadSummaryV1"
          }
        },
        "type": {
          "type": "string",
          "const": "threads"
        }
      },
      "required": [
        "type",
        "threads"
      ]
    },
    {
      "type": "object",
      "properties": {
        "thread_id": {
          "$ref": "#/components/schemas/ThreadId"
        },
        "events": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/ThreadEventItemV1"
          }
        },
        "type": {
          "type": "string",
          "const": "events"
        }
      },
      "required": [
        "type",
        "thread_id",
        "events"
      ]
    },
    {
      "type": "object",
      "properties": {
        "target": {
          "$ref": "#/components/schemas/ReactionTargetV1"
        },
        "revision": {
          "$ref": "#/components/schemas/TargetRevisionV1"
        },
        "reactions": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/ReactionAggregateV1"
          }
        },
        "conflict_count": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0
        },
        "actors": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/ReactionActorRowV1"
          }
        },
        "type": {
          "type": "string",
          "const": "reactions"
        }
      },
      "required": [
        "type",
        "target",
        "revision",
        "reactions",
        "conflict_count",
        "actors"
      ]
    },
    {
      "type": "object",
      "properties": {
        "versions": {
          "type": "object",
          "additionalProperties": false,
          "patternProperties": {
            "^[0-9a-f]{32}$": {
              "$ref": "#/components/schemas/CollabObjectRef"
            }
          }
        },
        "definitions": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/CustomEmojiVersionV1"
          }
        },
        "type": {
          "type": "string",
          "const": "emojis"
        }
      },
      "required": [
        "type",
        "versions",
        "definitions"
      ]
    }
  ]
}
```

## ThreadSummaryV1

```text
{
  "type": "object",
  "properties": {
    "reference": {
      "$ref": "#/components/schemas/CollabObjectRef"
    },
    "value": {
      "$ref": "#/components/schemas/ThreadVersionV1"
    },
    "directory": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ThreadDirectoryMetadata"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "reference",
    "value"
  ]
}
```

## ThreadVersionV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "schema_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "thread_id": {
      "$ref": "#/components/schemas/ThreadId"
    },
    "title": {
      "type": "string"
    },
    "status": {
      "$ref": "#/components/schemas/ThreadStatusV12"
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "accepted_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "previous": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ]
    },
    "event": {
      "$ref": "#/components/schemas/CollabObjectRef"
    },
    "baseline": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ThreadCodeAnchorV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "agent_message": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReactionTargetV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "comment_source": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ConversationSourceV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "purpose": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ThreadPurposeV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "codebase": {
      "type": [
        "array",
        "null"
      ],
      "items": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    }
  },
  "required": [
    "schema_version",
    "thread_id",
    "title",
    "status",
    "actor",
    "accepted_at_millis",
    "previous",
    "event",
    "baseline"
  ]
}
```

## ThreadStatusV12

```text
{
  "type": "string",
  "enum": [
    "created",
    "active",
    "idle",
    "archived",
    "tombstoned"
  ]
}
```

## ThreadCodeAnchorV1

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "profile": {
          "type": "string",
          "const": "native"
        },
        "anchor": {
          "$ref": "#/components/schemas/CodeAnchorV1"
        }
      },
      "required": [
        "profile",
        "anchor"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "profile": {
          "type": "string",
          "const": "portable"
        },
        "anchor": {
          "$ref": "#/components/schemas/CodeAnchorV2"
        }
      },
      "required": [
        "profile",
        "anchor"
      ],
      "additionalProperties": false
    }
  ],
  "description": "Native IDs and portable evidence remain distinct; neither is a device path."
}
```

## CodeAnchorV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "operations": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "commits": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    }
  },
  "required": [
    "operations",
    "commits"
  ]
}
```

## CodeAnchorV2

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "operations": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/CodeEvidenceRefV1"
      }
    },
    "commits": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/CodeCommitEvidenceV2"
      }
    }
  },
  "required": [
    "operations",
    "commits"
  ]
}
```

## CodeCommitEvidenceV2

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "evidence": {
      "$ref": "#/components/schemas/CodeEvidenceRefV1"
    },
    "commit": {
      "$ref": "#/components/schemas/CodeCommitIdV1"
    }
  },
  "required": [
    "evidence",
    "commit"
  ]
}
```

## ThreadPurposeV1

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "target": {
          "$ref": "#/components/schemas/ConversationMetadataTargetV1"
        },
        "type": {
          "type": "string",
          "const": "conversation"
        }
      },
      "required": [
        "type",
        "target"
      ]
    },
    {
      "type": "object",
      "properties": {
        "type": {
          "type": "string",
          "const": "annotation"
        }
      },
      "required": [
        "type"
      ],
      "additionalProperties": false
    }
  ]
}
```

## ConversationMetadataTargetV1

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "type": "string"
        },
        "type": {
          "type": "string",
          "const": "conversation"
        }
      },
      "required": [
        "type",
        "id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "session": {
          "$ref": "#/components/schemas/AgentSessionId"
        },
        "type": {
          "type": "string",
          "const": "recorded"
        }
      },
      "required": [
        "type",
        "session"
      ]
    }
  ]
}
```

## ThreadDirectoryMetadata

```text
{
  "type": "object",
  "properties": {
    "message_count": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "reply_count": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "excerpt": {
      "type": "string"
    },
    "participants": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/PrincipalId"
      }
    },
    "last_activity_at_millis": {
      "type": "integer",
      "format": "int64"
    }
  },
  "required": [
    "message_count",
    "reply_count",
    "excerpt",
    "participants",
    "last_activity_at_millis"
  ]
}
```

## ThreadEventItemV1

```text
{
  "type": "object",
  "properties": {
    "link": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ThreadLinkV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "message": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/HumanMessageRevisionV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "reference": {
      "$ref": "#/components/schemas/CollabObjectRef"
    },
    "value": {
      "$ref": "#/components/schemas/ThreadEventV1"
    }
  },
  "required": [
    "message",
    "reference",
    "value"
  ]
}
```

## ThreadLinkV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "schema_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "id": {
      "$ref": "#/components/schemas/ThreadLinkId"
    },
    "thread_id": {
      "$ref": "#/components/schemas/ThreadId"
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "accepted_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "previous": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ]
    },
    "active": {
      "type": "boolean"
    },
    "target": {
      "$ref": "#/components/schemas/ThreadLinkTargetV1"
    }
  },
  "required": [
    "schema_version",
    "id",
    "thread_id",
    "actor",
    "accepted_at_millis",
    "previous",
    "active",
    "target"
  ]
}
```

## ThreadLinkId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## ThreadLinkTargetV1

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "reply": {
          "$ref": "#/components/schemas/CommentReplyLinkV1"
        },
        "type": {
          "type": "string",
          "const": "comment_reply"
        }
      },
      "required": [
        "type",
        "reply"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "comments": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/ReactionTargetV1"
          }
        },
        "message": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/CollabObjectRef"
            },
            {
              "type": "null"
            }
          ]
        },
        "reply_to": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/CollabObjectRef"
            },
            {
              "type": "null"
            }
          ]
        },
        "type": {
          "type": "string",
          "const": "comment_batch"
        }
      },
      "required": [
        "type",
        "request_id",
        "comments",
        "message"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "source_state": {
          "$ref": "#/components/schemas/AgentObjectRef"
        },
        "event_batch": {
          "$ref": "#/components/schemas/AgentObjectRef"
        },
        "binding": {
          "$ref": "#/components/schemas/ThreadBindingV1"
        },
        "type": {
          "type": "string",
          "const": "capture_gap"
        }
      },
      "required": [
        "type",
        "source_state",
        "event_batch",
        "binding"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "session_id": {
          "$ref": "#/components/schemas/AgentSessionId"
        },
        "snapshot": {
          "$ref": "#/components/schemas/AgentObjectRef"
        },
        "primary": {
          "type": "boolean"
        },
        "type": {
          "type": "string",
          "const": "session"
        }
      },
      "required": [
        "type",
        "session_id",
        "snapshot",
        "primary"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "anchor": {
          "$ref": "#/components/schemas/ThreadCodeAnchorV1"
        },
        "evidence_method": {
          "$ref": "#/components/schemas/EvidenceMethod"
        },
        "evidence": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/AgentObjectRef"
            },
            {
              "type": "null"
            }
          ]
        },
        "type": {
          "type": "string",
          "const": "code"
        }
      },
      "required": [
        "type",
        "anchor",
        "evidence_method",
        "evidence"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "review_id": {
          "$ref": "#/components/schemas/ReviewThreadId"
        },
        "snapshot": {
          "$ref": "#/components/schemas/ReviewObjectRef"
        },
        "type": {
          "type": "string",
          "const": "review"
        }
      },
      "required": [
        "type",
        "review_id",
        "snapshot"
      ]
    }
  ]
}
```

## ThreadBindingV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "thread_id": {
      "$ref": "#/components/schemas/ThreadId"
    },
    "observed_thread": {
      "$ref": "#/components/schemas/CollabObjectRef"
    },
    "observed_assignment": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "thread_id",
    "observed_thread",
    "observed_assignment"
  ],
  "description": "Capture adapters may bind explicitly. Absence is the supported unlinked state."
}
```

## ThreadEventV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "schema_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "id": {
      "$ref": "#/components/schemas/ThreadEventId"
    },
    "thread_id": {
      "$ref": "#/components/schemas/ThreadId"
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "accepted_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "previous": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ]
    },
    "event": {
      "$ref": "#/components/schemas/ThreadEventKindV1"
    }
  },
  "required": [
    "schema_version",
    "id",
    "thread_id",
    "actor",
    "accepted_at_millis",
    "previous",
    "event"
  ]
}
```

## ThreadEventId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## ThreadEventKindV1

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "type": {
          "type": "string",
          "const": "codebase_changed"
        }
      },
      "required": [
        "type"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "version": {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        "type": {
          "type": "string",
          "const": "metadata"
        }
      },
      "required": [
        "type",
        "version"
      ]
    },
    {
      "type": "object",
      "properties": {
        "type": {
          "type": "string",
          "const": "created"
        }
      },
      "required": [
        "type"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "previous_title": {
          "type": "string"
        },
        "type": {
          "type": "string",
          "const": "renamed"
        }
      },
      "required": [
        "type",
        "previous_title"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "previous_status": {
          "$ref": "#/components/schemas/ThreadStatusV12"
        },
        "action": {
          "$ref": "#/components/schemas/ThreadActionV1"
        },
        "type": {
          "type": "string",
          "const": "lifecycle"
        }
      },
      "required": [
        "type",
        "previous_status",
        "action"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "revision": {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        "type": {
          "type": "string",
          "const": "message"
        }
      },
      "required": [
        "type",
        "revision"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "version": {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        "type": {
          "type": "string",
          "const": "link"
        }
      },
      "required": [
        "type",
        "version"
      ]
    }
  ]
}
```

## ThreadActionV1

```text
{
  "type": "string",
  "enum": [
    "activate",
    "idle",
    "archive",
    "tombstone",
    "restore"
  ]
}
```

## TargetRevisionV1

```text
{
  "type": "string",
  "enum": [
    "current",
    "earlier"
  ]
}
```

## ReactionAggregateV1

```text
{
  "type": "object",
  "properties": {
    "value": {
      "$ref": "#/components/schemas/ReactionValueV1"
    },
    "count": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "viewer_reacted": {
      "type": "boolean"
    }
  },
  "required": [
    "value",
    "count",
    "viewer_reacted"
  ]
}
```

## ReactionValueV1

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "type": {
          "type": "string",
          "const": "like"
        }
      },
      "required": [
        "type"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "type": {
          "type": "string",
          "const": "dislike"
        }
      },
      "required": [
        "type"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "sequence": {
          "type": "string"
        },
        "type": {
          "type": "string",
          "const": "emoji"
        }
      },
      "required": [
        "type",
        "sequence"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "emoji_id": {
          "$ref": "#/components/schemas/CustomEmojiId"
        },
        "version": {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        "type": {
          "type": "string",
          "const": "custom"
        }
      },
      "required": [
        "type",
        "emoji_id",
        "version"
      ]
    }
  ]
}
```

## CustomEmojiId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## ReactionActorRowV1

```text
{
  "type": "object",
  "properties": {
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "active": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/CollabObjectRef"
      }
    },
    "value": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReactionValueV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "conflict": {
      "type": "boolean"
    }
  },
  "required": [
    "actor",
    "active",
    "value",
    "conflict"
  ]
}
```

## CustomEmojiVersionV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "schema_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "emoji_id": {
      "$ref": "#/components/schemas/CustomEmojiId"
    },
    "shortcode": {
      "type": "string"
    },
    "label": {
      "type": "string"
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "accepted_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "previous": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ]
    },
    "asset": {
      "$ref": "#/components/schemas/CollabObjectRef"
    },
    "preview": {
      "$ref": "#/components/schemas/CollabObjectRef"
    },
    "retired": {
      "type": "boolean"
    }
  },
  "required": [
    "schema_version",
    "emoji_id",
    "shortcode",
    "label",
    "actor",
    "accepted_at_millis",
    "previous",
    "asset",
    "preview",
    "retired"
  ]
}
```

## ThreadDetailV1

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/CollabReadSnapshotV1"
    },
    "thread": {
      "$ref": "#/components/schemas/ThreadSummaryV1"
    }
  },
  "required": [
    "snapshot",
    "thread"
  ]
}
```

## HumanMessageDetailV1

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/CollabReadSnapshotV1"
    },
    "target": {
      "$ref": "#/components/schemas/ReactionTargetV1"
    },
    "message": {
      "$ref": "#/components/schemas/HumanMessageRevisionV1"
    }
  },
  "required": [
    "snapshot",
    "target",
    "message"
  ]
}
```

## CollabCommandOutcome

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "receipt": {
          "$ref": "#/components/schemas/CollabCommandReceipt"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "receipt"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## CollabCommandReceipt

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "publication_id": {
      "$ref": "#/components/schemas/PublicationId"
    },
    "request_digest": {
      "$ref": "#/components/schemas/IntentDigest"
    },
    "accepted": {
      "$ref": "#/components/schemas/Snapshot"
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "accepted_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "result": {
      "$ref": "#/components/schemas/CollabCommandResultV1"
    }
  },
  "required": [
    "version",
    "request_id",
    "publication_id",
    "request_digest",
    "accepted",
    "actor",
    "accepted_at_millis",
    "result"
  ]
}
```

## CollabCommandResultV1

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "result": {
          "$ref": "#/components/schemas/ThreadCommandResultV1"
        },
        "domain": {
          "type": "string",
          "const": "thread"
        }
      },
      "required": [
        "domain",
        "result"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "result": {
          "$ref": "#/components/schemas/ReactionCommandResultV1"
        },
        "domain": {
          "type": "string",
          "const": "reaction"
        }
      },
      "required": [
        "domain",
        "result"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "version": {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        "domain": {
          "type": "string",
          "const": "emoji"
        }
      },
      "required": [
        "domain",
        "version"
      ]
    }
  ]
}
```

## ThreadCommandResultV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "thread_id": {
      "$ref": "#/components/schemas/ThreadId"
    },
    "version": {
      "$ref": "#/components/schemas/CollabObjectRef"
    },
    "event": {
      "$ref": "#/components/schemas/CollabObjectRef"
    },
    "message": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ]
    },
    "link": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ]
    },
    "reassigned_from": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ThreadId"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "thread_id",
    "version",
    "event",
    "message",
    "link",
    "reassigned_from"
  ]
}
```

## ReactionCommandResultV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "schema_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "slot": {
      "$ref": "#/components/schemas/CollabObjectRef"
    },
    "active": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/CollabObjectRef"
      }
    },
    "value": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReactionValueV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "conflict": {
      "type": "boolean"
    }
  },
  "required": [
    "schema_version",
    "slot",
    "active",
    "value",
    "conflict"
  ]
}
```

## CollabCommandStatus

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "receipt": {
          "$ref": "#/components/schemas/CollabCommandReceipt"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "receipt"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "not_known"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## ContentPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/ContentSnapshot"
    },
    "results": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ContentHit"
      }
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "results",
    "next_cursor"
  ]
}
```

## ContentSnapshot

```text
{
  "type": "object",
  "properties": {
    "repo_id": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "manifest_digest": {
      "$ref": "#/components/schemas/RecordDigest"
    },
    "published_seq": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "thread_root": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RecordDigest"
        },
        {
          "type": "null"
        }
      ]
    },
    "work_root": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RecordDigest"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "repo_id",
    "epoch",
    "manifest_digest",
    "published_seq",
    "thread_root",
    "work_root"
  ]
}
```

## ContentHit

```text
{
  "type": "object",
  "properties": {
    "kind": {
      "$ref": "#/components/schemas/ContentKind"
    },
    "id": {
      "type": "string"
    },
    "title": {
      "type": "string"
    },
    "excerpt": {
      "type": "string"
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "revision": {
      "type": "string"
    },
    "thread": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ThreadId"
        },
        {
          "type": "null"
        }
      ]
    },
    "message": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/HumanMessageId"
        },
        {
          "type": "null"
        }
      ]
    },
    "project": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ProjectId"
        },
        {
          "type": "null"
        }
      ]
    },
    "task": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/TaskId"
        },
        {
          "type": "null"
        }
      ]
    },
    "comment": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/WorkCommentId"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "kind",
    "id",
    "title",
    "excerpt",
    "actor",
    "at_millis",
    "revision",
    "thread",
    "message",
    "project",
    "task",
    "comment"
  ]
}
```

## ContentKind

```text
{
  "type": "string",
  "enum": [
    "thread",
    "message",
    "project",
    "task",
    "comment"
  ]
}
```

## ProjectId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{32}$",
  "minLength": 32,
  "maxLength": 32
}
```

## WorkBrowsePage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/WorkReadSnapshot"
    },
    "projects": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/WorkProjectSummary"
      }
    },
    "tasks": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/WorkTaskSummary"
      }
    },
    "total_matches": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "projects",
    "tasks",
    "total_matches",
    "next_cursor"
  ]
}
```

## WorkReadSnapshot

```text
{
  "type": "object",
  "properties": {
    "repo_id": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "published_seq": {
      "$ref": "#/components/schemas/WalPosition"
    },
    "manifest_revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "work_root": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RecordDigest"
        },
        {
          "type": "null"
        }
      ]
    },
    "work_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "repo_id",
    "epoch",
    "published_seq",
    "manifest_revision",
    "work_root",
    "work_state_version"
  ]
}
```

## WorkProjectSummary

```text
{
  "type": "object",
  "properties": {
    "id": {
      "$ref": "#/components/schemas/ProjectId"
    },
    "title": {
      "type": "string"
    },
    "excerpt": {
      "type": "string"
    },
    "version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "repositories": {
      "type": "object",
      "additionalProperties": false,
      "patternProperties": {
        "^[0-9a-f]{32}$": {
          "type": "boolean"
        }
      }
    },
    "task_counts": {
      "$ref": "#/components/schemas/WorkTaskCounts"
    },
    "actor": {
      "$ref": "#/components/schemas/WorkSummaryActor"
    }
  },
  "required": [
    "id",
    "title",
    "excerpt",
    "version",
    "repositories",
    "task_counts",
    "actor"
  ]
}
```

## WorkTaskCounts

```text
{
  "type": "object",
  "properties": {
    "active": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "done": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "total": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    }
  },
  "required": [
    "active",
    "done",
    "total"
  ]
}
```

## WorkSummaryActor

```text
{
  "type": "object",
  "properties": {
    "created_by": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "updated_by": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "created_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "updated_at_millis": {
      "type": "integer",
      "format": "int64"
    }
  },
  "required": [
    "created_by",
    "updated_by",
    "created_at_millis",
    "updated_at_millis"
  ]
}
```

## WorkTaskSummary

```text
{
  "type": "object",
  "properties": {
    "id": {
      "$ref": "#/components/schemas/TaskId"
    },
    "project": {
      "$ref": "#/components/schemas/ProjectId"
    },
    "project_title": {
      "type": "string"
    },
    "title": {
      "type": "string"
    },
    "state": {
      "$ref": "#/components/schemas/TaskStateV1"
    },
    "repository": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RepositoryId"
        },
        {
          "type": "null"
        }
      ]
    },
    "parent": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/TaskId"
        },
        {
          "type": "null"
        }
      ]
    },
    "parent_title": {
      "type": [
        "string",
        "null"
      ]
    },
    "comment_count": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "subtask_count": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "actor": {
      "$ref": "#/components/schemas/WorkSummaryActor"
    }
  },
  "required": [
    "id",
    "project",
    "project_title",
    "title",
    "state",
    "repository",
    "parent",
    "parent_title",
    "comment_count",
    "subtask_count",
    "version",
    "actor"
  ]
}
```

## TaskStateV1

```text
{
  "type": "string",
  "enum": [
    "open",
    "in_progress",
    "blocked",
    "done"
  ]
}
```

## WorkActivityPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/WorkReadSnapshot"
    },
    "groups": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/WorkActivityGroup"
      },
      "maxItems": 256
    },
    "total_groups": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "groups",
    "total_groups",
    "next_cursor"
  ]
}
```

## WorkActivityGroup

```text
{
  "type": "object",
  "properties": {
    "repository": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RepositoryId"
        },
        {
          "type": "null"
        }
      ]
    },
    "project": {
      "$ref": "#/components/schemas/ProjectId"
    },
    "tasks": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "updated_at_millis": {
      "type": "integer",
      "format": "int64"
    }
  },
  "required": [
    "repository",
    "project",
    "tasks",
    "updated_at_millis"
  ]
}
```

## WorkPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/WorkReadSnapshot"
    },
    "projects": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ProjectV1"
      }
    },
    "tasks": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/TaskV1"
      }
    },
    "comments": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/WorkCommentV1"
      }
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "projects",
    "tasks",
    "comments",
    "next_cursor"
  ]
}
```

## ProjectV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "schema_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "id": {
      "$ref": "#/components/schemas/ProjectId"
    },
    "title": {
      "type": "string"
    },
    "body": {
      "type": "string"
    },
    "version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "repositories": {
      "type": "object",
      "additionalProperties": false,
      "patternProperties": {
        "^[0-9a-f]{32}$": {
          "type": "boolean"
        }
      },
      "description": "False preserves a retired membership and its historical task targets."
    },
    "actor": {
      "$ref": "#/components/schemas/WorkActorV1"
    }
  },
  "required": [
    "schema_version",
    "id",
    "title",
    "body",
    "version",
    "repositories",
    "actor"
  ]
}
```

## WorkActorV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "created_by": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "updated_by": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "created_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "updated_at_millis": {
      "type": "integer",
      "format": "int64"
    }
  },
  "required": [
    "created_by",
    "updated_by",
    "created_at_millis",
    "updated_at_millis"
  ]
}
```

## TaskV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "schema_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "id": {
      "$ref": "#/components/schemas/TaskId"
    },
    "project": {
      "$ref": "#/components/schemas/ProjectId"
    },
    "title": {
      "type": "string"
    },
    "body": {
      "type": "string"
    },
    "state": {
      "$ref": "#/components/schemas/TaskStateV1"
    },
    "repository": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RepositoryId"
        },
        {
          "type": "null"
        }
      ]
    },
    "parent": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/TaskId"
        },
        {
          "type": "null"
        }
      ]
    },
    "version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "actor": {
      "$ref": "#/components/schemas/WorkActorV1"
    }
  },
  "required": [
    "schema_version",
    "id",
    "project",
    "title",
    "body",
    "state",
    "repository",
    "parent",
    "version",
    "actor"
  ]
}
```

## WorkCommentV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "schema_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "id": {
      "$ref": "#/components/schemas/WorkCommentId"
    },
    "task": {
      "$ref": "#/components/schemas/TaskId"
    },
    "reply_to": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/WorkCommentId"
        },
        {
          "type": "null"
        }
      ]
    },
    "body": {
      "type": [
        "string",
        "null"
      ],
      "description": "None is an explicit tombstone; historical bodies stay in selected WAL roots."
    },
    "version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "actor": {
      "$ref": "#/components/schemas/WorkActorV1"
    }
  },
  "required": [
    "schema_version",
    "id",
    "task",
    "reply_to",
    "body",
    "version",
    "actor"
  ]
}
```

## WorkCommandOutcome

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "receipt": {
          "$ref": "#/components/schemas/WorkCommandReceipt"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "receipt"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## WorkCommandReceipt

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "publication_id": {
      "$ref": "#/components/schemas/PublicationId"
    },
    "request_digest": {
      "$ref": "#/components/schemas/IntentDigest"
    },
    "accepted": {
      "$ref": "#/components/schemas/Snapshot"
    },
    "result": {
      "$ref": "#/components/schemas/WorkCommandResultV1"
    }
  },
  "required": [
    "version",
    "request_id",
    "publication_id",
    "request_digest",
    "accepted",
    "result"
  ]
}
```

## WorkCommandResultV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "project": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ProjectV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "task": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/TaskV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "comment": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/WorkCommentV1"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "project",
    "task",
    "comment"
  ]
}
```

## WorkCommandStatus

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "receipt": {
          "$ref": "#/components/schemas/WorkCommandReceipt"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "receipt"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "not_known"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## CommittedEvent

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "cursor": {
      "$ref": "#/components/schemas/EventCursor"
    }
  },
  "required": [
    "version",
    "cursor"
  ],
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "snapshot": {
          "$ref": "#/components/schemas/HeadSnapshot"
        },
        "type": {
          "type": "string",
          "const": "ready"
        }
      },
      "required": [
        "type",
        "snapshot"
      ]
    },
    {
      "type": "object",
      "properties": {
        "type": {
          "type": "string",
          "const": "changed"
        }
      },
      "required": [
        "type"
      ]
    },
    {
      "type": "object",
      "properties": {
        "reason": {
          "$ref": "#/components/schemas/EventResetReason"
        },
        "snapshot": {
          "$ref": "#/components/schemas/HeadSnapshot"
        },
        "type": {
          "type": "string",
          "const": "reset"
        }
      },
      "required": [
        "type",
        "reason",
        "snapshot"
      ]
    }
  ]
}
```

## EventCursor

```text
{
  "type": "object",
  "properties": {
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "epoch": {
      "$ref": "#/components/schemas/EpochId"
    },
    "position": {
      "$ref": "#/components/schemas/WalPosition"
    }
  },
  "required": [
    "repository",
    "epoch",
    "position"
  ]
}
```

## EventResetReason

```text
{
  "type": "string",
  "enum": [
    "epoch_changed",
    "future_position",
    "replay_window"
  ]
}
```

## Envelope_ContentObject

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/ContentObject"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "next_cursor"
  ]
}
```

## ContentObject

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "profile": {
      "type": "string"
    },
    "key": {
      "$ref": "#/components/schemas/ObjectKey"
    },
    "virtual_object": {
      "type": "boolean"
    },
    "data": {
      "$ref": "#/components/schemas/ContentObjectData"
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "profile",
    "key",
    "virtual_object",
    "data",
    "stats"
  ]
}
```

## ContentObjectData

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "commit"
        },
        "data": {
          "$ref": "#/components/schemas/NativeCommit"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "tree"
        },
        "data": {
          "$ref": "#/components/schemas/NativeTree"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "copy"
        },
        "data": {
          "$ref": "#/components/schemas/NativeCopyHistory"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "view"
        },
        "data": {
          "$ref": "#/components/schemas/NativeView"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "operation"
        },
        "data": {
          "$ref": "#/components/schemas/NativeOperation"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "symlink"
        },
        "data": {
          "type": "object",
          "properties": {
            "target": {
              "type": "string"
            }
          },
          "required": [
            "target"
          ]
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "file"
        },
        "data": {
          "type": "object",
          "properties": {
            "bytes": {
              "type": "integer",
              "format": "uint64",
              "minimum": 0
            }
          },
          "required": [
            "bytes"
          ]
        }
      },
      "required": [
        "kind",
        "data"
      ]
    }
  ]
}
```

## NativeCommit

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "parents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "predecessors": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "root_tree": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/TreeId"
      }
    },
    "conflict_labels": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "change_id": {
      "$ref": "#/components/schemas/ChangeId"
    },
    "description": {
      "type": "string"
    },
    "author": {
      "$ref": "#/components/schemas/NativeSignature"
    },
    "committer": {
      "$ref": "#/components/schemas/NativeSignature"
    },
    "secure_sig": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/NativeSecureSig"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "parents",
    "predecessors",
    "root_tree",
    "conflict_labels",
    "change_id",
    "description",
    "author",
    "committer",
    "secure_sig"
  ]
}
```

## NativeSignature

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "name": {
      "type": "string"
    },
    "email": {
      "type": "string"
    },
    "timestamp": {
      "$ref": "#/components/schemas/NativeTimestamp"
    }
  },
  "required": [
    "name",
    "email",
    "timestamp"
  ]
}
```

## NativeTimestamp

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "millis": {
      "type": "integer",
      "format": "int64"
    },
    "offset_minutes": {
      "type": "integer",
      "format": "int32"
    }
  },
  "required": [
    "millis",
    "offset_minutes"
  ]
}
```

## NativeSecureSig

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "data": {
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0,
        "maximum": 255
      }
    },
    "sig": {
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0,
        "maximum": 255
      }
    }
  },
  "required": [
    "data",
    "sig"
  ]
}
```

## NativeTree

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "entries": {
      "type": "array",
      "items": {
        "type": "array",
        "prefixItems": [
          {
            "type": "string"
          },
          {
            "$ref": "#/components/schemas/NativeTreeValue"
          }
        ],
        "minItems": 2,
        "maxItems": 2
      }
    }
  },
  "required": [
    "entries"
  ]
}
```

## NativeTreeValue

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/components/schemas/FileId"
        },
        "executable": {
          "type": "boolean"
        },
        "copy": {
          "$ref": "#/components/schemas/CopyReference"
        },
        "type": {
          "type": "string",
          "const": "file"
        }
      },
      "required": [
        "type",
        "id",
        "executable",
        "copy"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/components/schemas/SymlinkId"
        },
        "type": {
          "type": "string",
          "const": "symlink"
        }
      },
      "required": [
        "type",
        "id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "$ref": "#/components/schemas/TreeId"
        },
        "type": {
          "type": "string",
          "const": "tree"
        }
      },
      "required": [
        "type",
        "id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "id": {
          "type": "array",
          "items": {
            "type": "integer",
            "format": "uint8",
            "minimum": 0,
            "maximum": 255
          }
        },
        "type": {
          "type": "string",
          "const": "submodule"
        }
      },
      "required": [
        "type",
        "id"
      ]
    }
  ]
}
```

## CopyReference

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "type": {
          "type": "string",
          "const": "placeholder"
        }
      },
      "required": [
        "type"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "type": {
          "type": "string",
          "const": "object"
        },
        "id": {
          "$ref": "#/components/schemas/CopyId"
        }
      },
      "required": [
        "type",
        "id"
      ],
      "additionalProperties": false
    }
  ],
  "description": "A copy placeholder is not a content-addressed object or dependency."
}
```

## NativeCopyHistory

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "current_path": {
      "type": "string"
    },
    "parents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CopyReference"
      }
    },
    "salt": {
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0,
        "maximum": 255
      }
    }
  },
  "required": [
    "current_path",
    "parents",
    "salt"
  ]
}
```

## NativeView

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "head_ids": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "local_bookmarks": {
      "type": "object",
      "additionalProperties": {
        "type": "array",
        "items": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/CommitId"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "local_tags": {
      "type": "object",
      "additionalProperties": {
        "type": "array",
        "items": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/CommitId"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "remote_views": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/NativeRemoteView"
      }
    },
    "git_refs": {
      "type": "object",
      "additionalProperties": {
        "type": "array",
        "items": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/CommitId"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "git_heads": {
      "type": "object",
      "additionalProperties": {
        "type": "array",
        "items": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/CommitId"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "wc_commit_ids": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/CommitId"
      }
    }
  },
  "required": [
    "head_ids",
    "local_bookmarks",
    "local_tags",
    "remote_views",
    "git_refs",
    "git_heads",
    "wc_commit_ids"
  ]
}
```

## NativeRemoteView

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "bookmarks": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/NativeRemoteRef"
      }
    },
    "tags": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/NativeRemoteRef"
      }
    }
  },
  "required": [
    "bookmarks",
    "tags"
  ]
}
```

## NativeRemoteRef

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "target": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/CommitId"
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "state": {
      "$ref": "#/components/schemas/NativeRemoteState"
    }
  },
  "required": [
    "target",
    "state"
  ]
}
```

## NativeRemoteState

```text
{
  "type": "string",
  "enum": [
    "new",
    "tracked"
  ]
}
```

## NativeOperation

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "view_id": {
      "$ref": "#/components/schemas/ViewId"
    },
    "parents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "metadata": {
      "$ref": "#/components/schemas/NativeMetadata"
    },
    "commit_predecessors": {
      "type": [
        "object",
        "null"
      ],
      "additionalProperties": false,
      "patternProperties": {
        "^[0-9a-f]{128}$": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/CommitId"
          }
        }
      }
    }
  },
  "required": [
    "view_id",
    "parents",
    "metadata",
    "commit_predecessors"
  ]
}
```

## NativeMetadata

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "start": {
      "$ref": "#/components/schemas/NativeTimestamp"
    },
    "end": {
      "$ref": "#/components/schemas/NativeTimestamp"
    },
    "description": {
      "type": "string"
    },
    "hostname": {
      "type": "string"
    },
    "username": {
      "type": "string"
    },
    "is_snapshot": {
      "type": "boolean"
    },
    "workspace_name": {
      "type": [
        "string",
        "null"
      ]
    },
    "attributes": {
      "type": "object",
      "additionalProperties": {
        "type": "string"
      }
    }
  },
  "required": [
    "start",
    "end",
    "description",
    "hostname",
    "username",
    "is_snapshot",
    "workspace_name",
    "attributes"
  ]
}
```

## GraphStats

```text
{
  "type": "object",
  "properties": {
    "metadata_nodes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "predicate_file_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "predicate_lines": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "object_reads": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "object_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "work": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "metadata_nodes",
    "predicate_file_bytes",
    "predicate_lines",
    "object_reads",
    "object_bytes",
    "work"
  ]
}
```

## Envelope_GraphPage

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/GraphPage"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "next_cursor"
  ]
}
```

## GraphPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "cache": {
      "$ref": "#/components/schemas/CacheReport"
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "data": {
      "$ref": "#/components/schemas/GraphNodes"
    },
    "next": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ObjectKey"
        },
        {
          "type": "null"
        }
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "cache",
    "operation",
    "data",
    "next",
    "stats"
  ]
}
```

## CacheReport

```text
{
  "type": "object",
  "properties": {
    "state": {
      "$ref": "#/components/schemas/CacheState"
    },
    "evicted": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "retained_entries": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "retained_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "state",
    "evicted",
    "retained_entries",
    "retained_bytes"
  ]
}
```

## CacheState

```text
{
  "type": "string",
  "enum": [
    "not_used",
    "built",
    "reused",
    "rebuilt"
  ]
}
```

## GraphNodes

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "graph": {
          "type": "string",
          "const": "log"
        },
        "nodes": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/GraphCommitNode"
          }
        }
      },
      "required": [
        "graph",
        "nodes"
      ]
    },
    {
      "type": "object",
      "properties": {
        "graph": {
          "type": "string",
          "const": "operations"
        },
        "nodes": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/GraphOperationNode"
          }
        }
      },
      "required": [
        "graph",
        "nodes"
      ]
    },
    {
      "type": "object",
      "properties": {
        "graph": {
          "type": "string",
          "const": "evolution"
        },
        "nodes": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/GraphEvolutionNode"
          }
        }
      },
      "required": [
        "graph",
        "nodes"
      ]
    }
  ]
}
```

## GraphCommitNode

```text
{
  "type": "object",
  "properties": {
    "commit": {
      "$ref": "#/components/schemas/GraphCommit"
    },
    "edges": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GraphEdge"
      }
    }
  },
  "required": [
    "commit",
    "edges"
  ]
}
```

## GraphCommit

```text
{
  "type": "object",
  "properties": {
    "id": {
      "$ref": "#/components/schemas/CommitId"
    },
    "change_id": {
      "$ref": "#/components/schemas/ChangeId"
    },
    "parents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "description": {
      "type": "string"
    },
    "author": {
      "$ref": "#/components/schemas/GraphSignature"
    },
    "committer": {
      "$ref": "#/components/schemas/GraphSignature"
    },
    "tree_terms": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/TreeId"
      }
    },
    "conflicted": {
      "type": "boolean"
    },
    "bookmarks": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GraphBookmark"
      }
    },
    "workspaces": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "visible": {
      "type": "boolean"
    },
    "divergent": {
      "type": "boolean"
    }
  },
  "required": [
    "id",
    "change_id",
    "parents",
    "description",
    "author",
    "committer",
    "tree_terms",
    "conflicted",
    "bookmarks",
    "workspaces",
    "visible",
    "divergent"
  ]
}
```

## GraphSignature

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "email": {
      "type": "string"
    },
    "timestamp": {
      "$ref": "#/components/schemas/GraphTimestamp"
    }
  },
  "required": [
    "name",
    "email",
    "timestamp"
  ]
}
```

## GraphTimestamp

```text
{
  "type": "object",
  "properties": {
    "millis": {
      "type": "integer",
      "format": "int64"
    },
    "offset_minutes": {
      "type": "integer",
      "format": "int32"
    }
  },
  "required": [
    "millis",
    "offset_minutes"
  ]
}
```

## GraphBookmark

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "remote": {
      "type": [
        "string",
        "null"
      ]
    },
    "added": {
      "type": "boolean"
    },
    "removed": {
      "type": "boolean"
    },
    "conflicted": {
      "type": "boolean"
    }
  },
  "required": [
    "name",
    "remote",
    "added",
    "removed",
    "conflicted"
  ]
}
```

## GraphEdge

```text
{
  "type": "object",
  "properties": {
    "target": {
      "$ref": "#/components/schemas/CommitId"
    },
    "kind": {
      "$ref": "#/components/schemas/GraphEdgeKind"
    }
  },
  "required": [
    "target",
    "kind"
  ]
}
```

## GraphEdgeKind

```text
{
  "type": "string",
  "enum": [
    "direct",
    "indirect",
    "missing"
  ]
}
```

## GraphOperationNode

```text
{
  "type": "object",
  "properties": {
    "id": {
      "$ref": "#/components/schemas/OperationId"
    },
    "parents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "view": {
      "$ref": "#/components/schemas/ViewId"
    },
    "description": {
      "type": "string"
    },
    "user": {
      "type": "string"
    },
    "host": {
      "type": "string"
    },
    "start": {
      "$ref": "#/components/schemas/GraphTimestamp"
    },
    "end": {
      "$ref": "#/components/schemas/GraphTimestamp"
    },
    "is_snapshot": {
      "type": "boolean"
    },
    "workspace": {
      "type": [
        "string",
        "null"
      ]
    },
    "attributes": {
      "type": "object",
      "additionalProperties": {
        "type": "string"
      }
    },
    "predecessor_data_present": {
      "type": "boolean"
    }
  },
  "required": [
    "id",
    "parents",
    "view",
    "description",
    "user",
    "host",
    "start",
    "end",
    "is_snapshot",
    "workspace",
    "attributes",
    "predecessor_data_present"
  ]
}
```

## GraphEvolutionNode

```text
{
  "type": "object",
  "properties": {
    "commit": {
      "$ref": "#/components/schemas/GraphCommit"
    },
    "operation": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/OperationId"
        },
        {
          "type": "null"
        }
      ]
    },
    "predecessors": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "predecessors_known": {
      "type": "boolean"
    },
    "legacy_predecessors": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    }
  },
  "required": [
    "commit",
    "operation",
    "predecessors",
    "predecessors_known",
    "legacy_predecessors"
  ]
}
```

## Envelope_NativeCodeSummary

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/NativeCodeSummary"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "next_cursor"
  ]
}
```

## NativeCodeSummary

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "commits": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "commits_partial": {
      "type": "boolean"
    },
    "latest": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/NativeCodeSummaryLatest"
        },
        {
          "type": "null"
        }
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "commits",
    "commits_partial",
    "latest",
    "stats"
  ]
}
```

## NativeCodeSummaryLatest

```text
{
  "type": "object",
  "properties": {
    "id": {
      "$ref": "#/components/schemas/CommitId"
    },
    "description": {
      "type": "string"
    },
    "author": {
      "$ref": "#/components/schemas/NativeSignature"
    },
    "committer": {
      "$ref": "#/components/schemas/NativeSignature"
    }
  },
  "required": [
    "id",
    "description",
    "author",
    "committer"
  ]
}
```

## Window

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/Page"
    },
    "previous_cursor": {
      "type": [
        "string",
        "null"
      ]
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "previous_cursor",
    "next_cursor"
  ]
}
```

## Page

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "references": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/Reference"
      }
    },
    "previous": {
      "type": [
        "string",
        "null"
      ]
    },
    "next": {
      "type": [
        "string",
        "null"
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "operation",
    "references",
    "previous",
    "next",
    "stats"
  ]
}
```

## Reference

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "targets": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/Target"
          }
        },
        "kind": {
          "type": "string",
          "const": "workspace"
        }
      },
      "required": [
        "kind",
        "name",
        "targets"
      ]
    },
    {
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "terms": {
          "type": "array",
          "items": {
            "anyOf": [
              {
                "$ref": "#/components/schemas/CommitId"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "conflicted": {
          "type": "boolean"
        },
        "workspaces": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "targets": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/Target"
          }
        },
        "kind": {
          "type": "string",
          "const": "bookmark"
        }
      },
      "required": [
        "kind",
        "name",
        "terms",
        "conflicted",
        "workspaces",
        "targets"
      ]
    },
    {
      "type": "object",
      "properties": {
        "change": {
          "$ref": "#/components/schemas/ChangeId"
        },
        "divergent": {
          "type": "boolean"
        },
        "targets": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/Target"
          }
        },
        "kind": {
          "type": "string",
          "const": "change"
        }
      },
      "required": [
        "kind",
        "change",
        "divergent",
        "targets"
      ]
    }
  ]
}
```

## Target

```text
{
  "type": "object",
  "properties": {
    "commit": {
      "$ref": "#/components/schemas/CommitId"
    },
    "change": {
      "$ref": "#/components/schemas/ChangeId"
    },
    "parents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "description": {
      "type": "string"
    },
    "author": {
      "$ref": "#/components/schemas/NativeSignature"
    },
    "committer": {
      "$ref": "#/components/schemas/NativeSignature"
    },
    "conflicted": {
      "type": "boolean"
    },
    "workspaces": {
      "type": "array",
      "items": {
        "type": "string"
      }
    }
  },
  "required": [
    "commit",
    "change",
    "parents",
    "description",
    "author",
    "committer",
    "conflicted",
    "workspaces"
  ]
}
```

## Envelope_OperationDetail

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/OperationDetail"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "next_cursor"
  ]
}
```

## OperationDetail

```text
{
  "type": "object",
  "properties": {
    "operation": {
      "$ref": "#/components/schemas/ContentObject"
    },
    "view": {
      "$ref": "#/components/schemas/ContentObject"
    }
  },
  "required": [
    "operation",
    "view"
  ]
}
```

## Envelope_OperationDiffResponse

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/OperationDiffResponse"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "next_cursor"
  ]
}
```

## OperationDiffResponse

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "data": {
      "$ref": "#/components/schemas/OperationDiffData"
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "data",
    "stats"
  ]
}
```

## OperationDiffData

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "operation": {
          "$ref": "#/components/schemas/OperationId"
        },
        "view": {
          "$ref": "#/components/schemas/ViewId"
        },
        "status": {
          "type": "string",
          "const": "root"
        }
      },
      "required": [
        "status",
        "operation",
        "view"
      ]
    },
    {
      "type": "object",
      "properties": {
        "operation": {
          "$ref": "#/components/schemas/OperationId"
        },
        "parents": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/OperationId"
          }
        },
        "status": {
          "type": "string",
          "const": "select_parent"
        }
      },
      "required": [
        "status",
        "operation",
        "parents"
      ]
    },
    {
      "required": [
        "status"
      ],
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "difference"
        }
      },
      "allOf": [
        {
          "$ref": "#/components/schemas/OperationDiffDifference"
        }
      ]
    }
  ]
}
```

## OperationDiffViewDelta

```text
{
  "type": "object",
  "properties": {
    "heads": {
      "$ref": "#/components/schemas/SetDelta_CommitId"
    },
    "local_bookmarks": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/NamedChange_Array_of_Nullable_CommitId"
      }
    },
    "local_tags": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/NamedChange_Array_of_Nullable_CommitId"
      }
    },
    "remote_views": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/NamedChange_NativeRemoteView"
      }
    },
    "git_refs": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/NamedChange_Array_of_Nullable_CommitId"
      }
    },
    "git_heads": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/NamedChange_Array_of_Nullable_CommitId"
      }
    },
    "workspaces": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/NamedChange_CommitId"
      }
    }
  },
  "required": [
    "heads",
    "local_bookmarks",
    "local_tags",
    "remote_views",
    "git_refs",
    "git_heads",
    "workspaces"
  ]
}
```

## SetDelta_CommitId

```text
{
  "type": "object",
  "properties": {
    "added": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "removed": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    }
  },
  "required": [
    "added",
    "removed"
  ]
}
```

## NamedChange_Array_of_Nullable_CommitId

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "before": {
      "type": [
        "array",
        "null"
      ],
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/CommitId"
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "after": {
      "type": [
        "array",
        "null"
      ],
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/CommitId"
          },
          {
            "type": "null"
          }
        ]
      }
    }
  },
  "required": [
    "name",
    "before",
    "after"
  ]
}
```

## NamedChange_NativeRemoteView

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "before": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/NativeRemoteView"
        },
        {
          "type": "null"
        }
      ]
    },
    "after": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/NativeRemoteView"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "name",
    "before",
    "after"
  ]
}
```

## NamedChange_CommitId

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "before": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CommitId"
        },
        {
          "type": "null"
        }
      ]
    },
    "after": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CommitId"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "name",
    "before",
    "after"
  ]
}
```

## OperationDiffDifference

```text
{
  "type": "object",
  "properties": {
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "parent": {
      "$ref": "#/components/schemas/OperationId"
    },
    "before_view": {
      "$ref": "#/components/schemas/ViewId"
    },
    "after_view": {
      "$ref": "#/components/schemas/ViewId"
    },
    "view": {
      "$ref": "#/components/schemas/OperationDiffViewDelta"
    },
    "visible_commits": {
      "$ref": "#/components/schemas/SetDelta_CommitId"
    },
    "commit_predecessors": {
      "type": [
        "object",
        "null"
      ],
      "additionalProperties": false,
      "patternProperties": {
        "^[0-9a-f]{128}$": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/CommitId"
          }
        }
      }
    }
  },
  "required": [
    "operation",
    "parent",
    "before_view",
    "after_view",
    "view",
    "visible_commits",
    "commit_predecessors"
  ]
}
```

## Window2

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/Page2"
    },
    "previous_cursor": {
      "type": [
        "string",
        "null"
      ]
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "previous_cursor",
    "next_cursor"
  ]
}
```

## Page2

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "parent": {
      "$ref": "#/components/schemas/OperationId"
    },
    "groups": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/Group"
      }
    },
    "previous": {
      "type": [
        "string",
        "null"
      ]
    },
    "next": {
      "type": [
        "string",
        "null"
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "operation",
    "parent",
    "groups",
    "previous",
    "next",
    "stats"
  ]
}
```

## Group

```text
{
  "type": "object",
  "properties": {
    "change": {
      "$ref": "#/components/schemas/ChangeId"
    },
    "before": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "after": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "comparison": {
      "$ref": "#/components/schemas/Comparison"
    }
  },
  "required": [
    "change",
    "before",
    "after",
    "comparison"
  ]
}
```

## Comparison

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "from": {
          "$ref": "#/components/schemas/CommitId"
        },
        "to": {
          "$ref": "#/components/schemas/CommitId"
        },
        "status": {
          "type": "string",
          "const": "exact"
        }
      },
      "required": [
        "status",
        "from",
        "to"
      ]
    },
    {
      "type": "object",
      "properties": {
        "before": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/CommitId"
          }
        },
        "after": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/CommitId"
          }
        },
        "status": {
          "type": "string",
          "const": "select"
        }
      },
      "required": [
        "status",
        "before",
        "after"
      ]
    },
    {
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "unavailable"
        }
      },
      "required": [
        "status"
      ]
    }
  ]
}
```

## Envelope_CommitDetail

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/CommitDetail"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "next_cursor"
  ]
}
```

## CommitDetail

```text
{
  "type": "object",
  "properties": {
    "object": {
      "$ref": "#/components/schemas/ContentObject"
    },
    "commit": {
      "$ref": "#/components/schemas/GraphCommit"
    }
  },
  "required": [
    "object",
    "commit"
  ]
}
```

## Envelope_TreePage

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/TreePage"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "next_cursor"
  ]
}
```

## TreePage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "commit": {
      "$ref": "#/components/schemas/CommitId"
    },
    "path": {
      "type": "string"
    },
    "entries": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/TreeEntry"
      }
    },
    "next": {
      "type": [
        "string",
        "null"
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "commit",
    "path",
    "entries",
    "next",
    "stats"
  ]
}
```

## TreeEntry

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "terms": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/NativeTreeValue"
          },
          {
            "type": "null"
          }
        ]
      },
      "description": "Native alternating add/remove terms. A sole None means absence and is\nexcluded from directory listings; unresolved deletions remain explicit."
    },
    "conflicted": {
      "type": "boolean"
    }
  },
  "required": [
    "name",
    "terms",
    "conflicted"
  ]
}
```

## BlamePage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "commit": {
      "type": "string"
    },
    "path": {
      "type": "string"
    },
    "file_id": {
      "type": "string"
    },
    "line_count": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "ranges": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/BlameRange"
      }
    },
    "commits": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/BlameCommit"
      }
    },
    "complete": {
      "type": "boolean"
    },
    "continuation": {
      "type": [
        "string",
        "null"
      ]
    },
    "history_commits": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "repository",
    "operation",
    "commit",
    "path",
    "file_id",
    "line_count",
    "ranges",
    "commits",
    "complete",
    "continuation",
    "history_commits",
    "stats"
  ]
}
```

## BlameRange

```text
{
  "type": "object",
  "properties": {
    "start": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "end": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "commit_id": {
      "type": "string"
    }
  },
  "required": [
    "start",
    "end",
    "commit_id"
  ]
}
```

## BlameCommit

```text
{
  "type": "object",
  "properties": {
    "id": {
      "type": "string"
    },
    "change_id": {
      "type": "string"
    },
    "description": {
      "type": "string"
    },
    "author": {
      "$ref": "#/components/schemas/GraphSignature"
    },
    "committer": {
      "$ref": "#/components/schemas/GraphSignature"
    }
  },
  "required": [
    "id",
    "change_id",
    "description",
    "author",
    "committer"
  ]
}
```

## Window3

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/Page3"
    },
    "previous_cursor": {
      "type": [
        "string",
        "null"
      ]
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "previous_cursor",
    "next_cursor"
  ]
}
```

## Page3

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "commit": {
      "$ref": "#/components/schemas/CommitId"
    },
    "path": {
      "type": "string"
    },
    "recursive": {
      "type": "boolean"
    },
    "entries": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/Entry"
      }
    },
    "previous": {
      "type": [
        "string",
        "null"
      ]
    },
    "next": {
      "type": [
        "string",
        "null"
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "commit",
    "path",
    "recursive",
    "entries",
    "previous",
    "next",
    "stats"
  ]
}
```

## Entry

```text
{
  "type": "object",
  "properties": {
    "path": {
      "type": "string"
    },
    "terms": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/NativeTreeValue"
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "conflicted": {
      "type": "boolean"
    },
    "last_change": {
      "$ref": "#/components/schemas/LastChange"
    }
  },
  "required": [
    "path",
    "terms",
    "conflicted",
    "last_change"
  ]
}
```

## LastChange

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "attribution": {
          "$ref": "#/components/schemas/Attribution"
        },
        "commit": {
          "$ref": "#/components/schemas/CommitId"
        },
        "change": {
          "$ref": "#/components/schemas/ChangeId"
        },
        "description": {
          "type": "string"
        },
        "author": {
          "$ref": "#/components/schemas/NativeSignature"
        },
        "committer": {
          "$ref": "#/components/schemas/NativeSignature"
        },
        "status": {
          "type": "string",
          "const": "available"
        }
      },
      "required": [
        "status",
        "attribution",
        "commit",
        "change",
        "description",
        "author",
        "committer"
      ]
    },
    {
      "type": "object",
      "properties": {
        "reason": {
          "$ref": "#/components/schemas/LastChangeUnavailable"
        },
        "status": {
          "type": "string",
          "const": "unavailable"
        }
      },
      "required": [
        "status",
        "reason"
      ]
    }
  ]
}
```

## Attribution

```text
{
  "type": "string",
  "enum": [
    "path_change",
    "snapshot"
  ],
  "description": "Provenance of the displayed commit, not a claim about all-branch history."
}
```

## LastChangeUnavailable

```text
{
  "type": "string",
  "enum": [
    "merge",
    "budget",
    "not_requested",
    "pending",
    "not_found"
  ]
}
```

## Envelope_ConflictResponse

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/ConflictResponse"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "next_cursor"
  ]
}
```

## ConflictResponse

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "commit": {
      "$ref": "#/components/schemas/CommitId"
    },
    "change": {
      "$ref": "#/components/schemas/ChangeId"
    },
    "conflicts": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ConflictDescriptor"
      }
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "commit",
    "change",
    "conflicts",
    "stats"
  ]
}
```

## ConflictDescriptor

```text
{
  "type": "object",
  "properties": {
    "path": {
      "type": "string"
    },
    "fingerprint": {
      "$ref": "#/components/schemas/ConflictFingerprint"
    },
    "terms": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ConflictTerm"
      }
    },
    "editor": {
      "$ref": "#/components/schemas/ConflictEditor"
    }
  },
  "required": [
    "path",
    "fingerprint",
    "terms",
    "editor"
  ]
}
```

## ConflictFingerprint

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{128}$",
  "minLength": 128,
  "maxLength": 128
}
```

## ConflictTerm

```text
{
  "type": "object",
  "properties": {
    "id": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "role": {
      "$ref": "#/components/schemas/ConflictRole"
    },
    "label": {
      "type": [
        "string",
        "null"
      ]
    },
    "value": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/NativeTreeValue"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "id",
    "role",
    "label",
    "value"
  ]
}
```

## ConflictRole

```text
{
  "type": "string",
  "enum": [
    "add",
    "remove"
  ]
}
```

## ConflictEditor

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "contents": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "minItems": 3,
          "maxItems": 3
        },
        "executable": {
          "type": "boolean"
        },
        "status": {
          "type": "string",
          "const": "text"
        }
      },
      "required": [
        "status",
        "contents",
        "executable"
      ]
    },
    {
      "type": "object",
      "properties": {
        "reason": {
          "$ref": "#/components/schemas/ConflictUnsupported"
        },
        "status": {
          "type": "string",
          "const": "unsupported"
        }
      },
      "required": [
        "status",
        "reason"
      ]
    }
  ]
}
```

## ConflictUnsupported

```text
{
  "type": "string",
  "enum": [
    "multiple_sides",
    "delete_modify",
    "structural",
    "executable_mode",
    "copy_history",
    "binary",
    "too_large",
    "invalid_path"
  ]
}
```

## Envelope_FileDiffResponse

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/FileDiffResponse"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "next_cursor"
  ]
}
```

## FileDiffResponse

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "from": {
      "$ref": "#/components/schemas/CommitId"
    },
    "to": {
      "$ref": "#/components/schemas/CommitId"
    },
    "path": {
      "type": "string"
    },
    "changes": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/FileDiffChange"
      }
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "from",
    "to",
    "path",
    "changes",
    "stats"
  ]
}
```

## FileDiffChange

```text
{
  "type": "object",
  "properties": {
    "path": {
      "type": "string"
    },
    "before": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/NativeTreeValue"
          },
          {
            "type": "null"
          }
        ]
      },
      "description": "Native values at the target; source annotations never replace these."
    },
    "after": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/NativeTreeValue"
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "sources": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/FileDiffSource"
      }
    },
    "content": {
      "$ref": "#/components/schemas/FileDiffContent"
    }
  },
  "required": [
    "path",
    "before",
    "after",
    "sources",
    "content"
  ]
}
```

## FileDiffSource

```text
{
  "type": "object",
  "properties": {
    "kind": {
      "$ref": "#/components/schemas/FileDiffSourceKind"
    },
    "path": {
      "type": "string"
    },
    "terms": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/NativeTreeValue"
          },
          {
            "type": "null"
          }
        ]
      }
    }
  },
  "required": [
    "kind",
    "path",
    "terms"
  ]
}
```

## FileDiffSourceKind

```text
{
  "type": "string",
  "enum": [
    "normal",
    "copy",
    "rename"
  ]
}
```

## FileDiffContent

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "hunks": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/FileDiffHunk"
          }
        },
        "status": {
          "type": "string",
          "const": "text"
        }
      },
      "required": [
        "status",
        "hunks"
      ]
    },
    {
      "type": "object",
      "properties": {
        "reason": {
          "$ref": "#/components/schemas/FileDiffUnavailable"
        },
        "status": {
          "type": "string",
          "const": "unavailable"
        }
      },
      "required": [
        "status",
        "reason"
      ]
    }
  ]
}
```

## FileDiffHunk

```text
{
  "type": "object",
  "properties": {
    "kind": {
      "$ref": "#/components/schemas/FileDiffHunkKind"
    },
    "before_start": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "after_start": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "before": {
      "type": "string"
    },
    "after": {
      "type": "string"
    }
  },
  "required": [
    "kind",
    "before_start",
    "after_start",
    "before",
    "after"
  ]
}
```

## FileDiffHunkKind

```text
{
  "type": "string",
  "enum": [
    "matching",
    "different"
  ]
}
```

## FileDiffUnavailable

```text
{
  "type": "string",
  "enum": [
    "not_requested",
    "conflict",
    "structural",
    "multiple_sources",
    "binary",
    "too_large",
    "complexity"
  ]
}
```

## Window4

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/Page4"
    },
    "previous_cursor": {
      "type": [
        "string",
        "null"
      ]
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "previous_cursor",
    "next_cursor"
  ]
}
```

## Page4

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "from": {
      "$ref": "#/components/schemas/CommitId"
    },
    "to": {
      "$ref": "#/components/schemas/CommitId"
    },
    "path": {
      "type": "string"
    },
    "changes": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/FileDiffChange"
      }
    },
    "previous": {
      "type": [
        "string",
        "null"
      ]
    },
    "next": {
      "type": [
        "string",
        "null"
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "from",
    "to",
    "path",
    "changes",
    "previous",
    "next",
    "stats"
  ]
}
```

## KindPreparationTask

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "id": {
      "type": "string"
    },
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    },
    "kind": {
      "$ref": "#/components/schemas/KindPreparationKind"
    },
    "progress": {
      "$ref": "#/components/schemas/KindPreparationProgress"
    }
  },
  "required": [
    "version",
    "id",
    "snapshot",
    "kind",
    "progress"
  ],
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "running"
        }
      },
      "required": [
        "status"
      ]
    },
    {
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "canceling"
        }
      },
      "required": [
        "status"
      ]
    },
    {
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "ready"
        }
      },
      "required": [
        "status"
      ]
    },
    {
      "type": "object",
      "properties": {
        "error": {
          "$ref": "#/components/schemas/GraphError"
        },
        "status": {
          "type": "string",
          "const": "failed"
        }
      },
      "required": [
        "status",
        "error"
      ]
    },
    {
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "canceled"
        }
      },
      "required": [
        "status"
      ]
    }
  ]
}
```

## KindPreparationKind

```text
{
  "type": "string",
  "enum": [
    "operations",
    "copies"
  ]
}
```

## KindPreparationProgress

```text
{
  "type": "object",
  "properties": {
    "complete": {
      "type": "boolean"
    },
    "matching_ids": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "stats": {
      "$ref": "#/components/schemas/KindPreparationStats"
    }
  },
  "required": [
    "complete",
    "matching_ids",
    "stats"
  ]
}
```

## KindPreparationStats

```text
{
  "type": "object",
  "properties": {
    "windows": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "scanned": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "descriptor_work": {
      "type": "integer",
      "format": "uint",
      "minimum": 0,
      "description": "Successful descriptors plus conservative full-window charges after an\nunknown/failed/canceled read; retrying cannot hide that spent work."
    },
    "record_reads": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "record_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "windows",
    "scanned",
    "descriptor_work",
    "record_reads",
    "record_bytes"
  ]
}
```

## GraphError

```text
{
  "oneOf": [
    {
      "type": "string",
      "enum": [
        "canceled",
        "blame_reset",
        "denied",
        "not_committed",
        "epoch_changed",
        "limit",
        "busy",
        "index_budget",
        "index_rebuild_required",
        "invalid",
        "unavailable",
        "corrupt",
        "incomplete",
        "not_found",
        "not_directory",
        "range"
      ]
    },
    {
      "type": "object",
      "properties": {
        "invalid_query": {
          "$ref": "#/components/schemas/Diagnostic"
        }
      },
      "required": [
        "invalid_query"
      ],
      "additionalProperties": false
    }
  ]
}
```

## IndexTask

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "id": {
      "type": "string"
    },
    "snapshot": {
      "$ref": "#/components/schemas/ReadSnapshot"
    }
  },
  "required": [
    "version",
    "id",
    "snapshot"
  ],
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "running"
        }
      },
      "required": [
        "status"
      ]
    },
    {
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "canceling"
        }
      },
      "required": [
        "status"
      ]
    },
    {
      "type": "object",
      "properties": {
        "index": {
          "$ref": "#/components/schemas/IndexReport"
        },
        "status": {
          "type": "string",
          "const": "ready"
        }
      },
      "required": [
        "status",
        "index"
      ]
    },
    {
      "type": "object",
      "properties": {
        "error": {
          "$ref": "#/components/schemas/GraphError"
        },
        "status": {
          "type": "string",
          "const": "failed"
        }
      },
      "required": [
        "status",
        "error"
      ]
    },
    {
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "canceled"
        }
      },
      "required": [
        "status"
      ]
    }
  ]
}
```

## IndexReport

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "cache": {
      "$ref": "#/components/schemas/CacheReport"
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    }
  },
  "required": [
    "version",
    "operation",
    "cache",
    "stats"
  ]
}
```

## Preview

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "preview": {
          "$ref": "#/components/schemas/PreviewReport"
        },
        "status": {
          "type": "string",
          "const": "prepared"
        }
      },
      "required": [
        "status",
        "preview"
      ]
    },
    {
      "type": "object",
      "properties": {
        "receipt": {
          "$ref": "#/components/schemas/Receipt"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "receipt"
      ]
    }
  ]
}
```

## PreviewReport

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "request_digest": {
      "$ref": "#/components/schemas/IntentDigest"
    },
    "expected": {
      "$ref": "#/components/schemas/Snapshot"
    },
    "result": {
      "description": "Provisional native IDs; apply revalidates and may produce different IDs.",
      "allOf": [
        {
          "$ref": "#/components/schemas/Prepared"
        }
      ]
    }
  },
  "required": [
    "version",
    "request_id",
    "request_digest",
    "expected",
    "result"
  ]
}
```

## Prepared

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "created": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "rewrites": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/Rewrite"
      }
    },
    "bookmarks": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/Change"
      }
    },
    "workspaces": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/Change2"
      }
    },
    "stats": {
      "$ref": "#/components/schemas/GraphStats"
    },
    "staged_objects": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "staged_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "abandoned": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "skipped_hidden": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    "conflict_resolution": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ConflictResolutionReport"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "operation",
    "created",
    "rewrites",
    "bookmarks",
    "workspaces",
    "stats",
    "staged_objects",
    "staged_bytes"
  ]
}
```

## Rewrite

```text
{
  "type": "object",
  "properties": {
    "commit": {
      "$ref": "#/components/schemas/CommitId"
    },
    "change": {
      "$ref": "#/components/schemas/ChangeId"
    },
    "predecessors": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    }
  },
  "required": [
    "commit",
    "change",
    "predecessors"
  ]
}
```

## Change

```text
{
  "type": "object",
  "properties": {
    "before": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/CommitId"
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "after": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/CommitId"
          },
          {
            "type": "null"
          }
        ]
      }
    }
  },
  "required": [
    "before",
    "after"
  ]
}
```

## Change2

```text
{
  "type": "object",
  "properties": {
    "before": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CommitId"
        },
        {
          "type": "null"
        }
      ]
    },
    "after": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CommitId"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "before",
    "after"
  ]
}
```

## ConflictResolutionReport

```text
{
  "type": "object",
  "properties": {
    "target": {
      "$ref": "#/components/schemas/Change3"
    },
    "selected_paths": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "remaining": {
      "type": "object",
      "additionalProperties": false,
      "patternProperties": {
        "^[0-9a-f]{128}$": {
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    }
  },
  "required": [
    "target",
    "selected_paths",
    "remaining"
  ],
  "description": "Complete remaining native paths on the target and every changed result."
}
```

## Change3

```text
{
  "type": "object",
  "properties": {
    "before": {
      "$ref": "#/components/schemas/CommitId"
    },
    "after": {
      "$ref": "#/components/schemas/CommitId"
    }
  },
  "required": [
    "before",
    "after"
  ]
}
```

## Receipt

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "publication_id": {
      "$ref": "#/components/schemas/PublicationId"
    },
    "request_digest": {
      "$ref": "#/components/schemas/IntentDigest"
    },
    "accepted": {
      "$ref": "#/components/schemas/Snapshot"
    },
    "result": {
      "$ref": "#/components/schemas/Prepared"
    }
  },
  "required": [
    "version",
    "request_id",
    "publication_id",
    "request_digest",
    "accepted",
    "result"
  ]
}
```

## Outcome

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "receipt": {
          "$ref": "#/components/schemas/Receipt"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "receipt"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## ConflictUpload

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "id": {
      "$ref": "#/components/schemas/ConflictUploadId"
    },
    "file": {
      "$ref": "#/components/schemas/FileId"
    },
    "bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "binding": {
      "$ref": "#/components/schemas/Binding"
    }
  },
  "required": [
    "version",
    "id",
    "file",
    "bytes",
    "binding"
  ]
}
```

## ConflictUploadId

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{64}$",
  "minLength": 64,
  "maxLength": 64
}
```

## Binding

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "expected": {
      "$ref": "#/components/schemas/Snapshot"
    },
    "commit": {
      "$ref": "#/components/schemas/CommitId"
    },
    "path": {
      "type": "string"
    },
    "fingerprint": {
      "$ref": "#/components/schemas/ConflictFingerprint"
    }
  },
  "required": [
    "expected",
    "commit",
    "path",
    "fingerprint"
  ]
}
```

## Status

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "receipt": {
          "$ref": "#/components/schemas/Receipt"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "receipt"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "not_known"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## ApplicationFreshnessEvent

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "version": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0
        },
        "witnesses": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/Change4"
          }
        },
        "type": {
          "type": "string",
          "const": "ready"
        }
      },
      "required": [
        "type",
        "version",
        "witnesses"
      ]
    },
    {
      "type": "object",
      "properties": {
        "version": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0
        },
        "change": {
          "$ref": "#/components/schemas/Change4"
        },
        "type": {
          "type": "string",
          "const": "changed"
        }
      },
      "required": [
        "type",
        "version",
        "change"
      ]
    },
    {
      "type": "object",
      "properties": {
        "version": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0
        },
        "reason": {
          "type": "string"
        },
        "witnesses": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/Change4"
          }
        },
        "type": {
          "type": "string",
          "const": "reset"
        }
      },
      "required": [
        "type",
        "version",
        "reason",
        "witnesses"
      ]
    }
  ]
}
```

## Change4

```text
{
  "type": "object",
  "properties": {
    "repository": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RepositoryId"
        },
        {
          "type": "null"
        }
      ]
    },
    "domain": {
      "$ref": "#/components/schemas/Domain"
    },
    "witness": {
      "type": "string"
    },
    "identity": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "repository",
    "domain",
    "witness"
  ]
}
```

## Domain

```text
{
  "type": "string",
  "enum": [
    "repository",
    "code",
    "threads",
    "agents",
    "agent_execution",
    "agent_settings",
    "reviews",
    "work",
    "storage",
    "sync",
    "search",
    "index_tasks",
    "capture",
    "configuration",
    "discovery",
    "workspaces",
    "conversion"
  ]
}
```

## ApplicationFreshnessFrame

```text
{
  "type": "object",
  "properties": {
    "id": {
      "type": "string"
    },
    "event": {
      "$ref": "#/components/schemas/ApplicationFreshnessEvent"
    }
  },
  "required": [
    "id",
    "event"
  ]
}
```

## LocalBindings

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "bindings": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalBinding"
      }
    }
  },
  "required": [
    "version",
    "revision",
    "bindings"
  ]
}
```

## LocalBinding

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "name": {
      "type": "string"
    },
    "path": {
      "type": "string"
    },
    "generation": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "availability": {
      "$ref": "#/components/schemas/Availability"
    }
  },
  "required": [
    "repository",
    "name",
    "path",
    "generation",
    "availability"
  ]
}
```

## RegistrationOutcome

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "binding": {
          "$ref": "#/components/schemas/LocalBinding"
        },
        "revision": {
          "type": "integer",
          "format": "uint64",
          "minimum": 0
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "request_id",
        "binding",
        "revision"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "not_known"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## LocalFolder

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "path": {
      "type": "string"
    },
    "metadata": {
      "type": "string"
    },
    "repository": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RepositoryId"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "path",
    "metadata",
    "repository"
  ]
}
```

## NativeCreationOutcome

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "repository": {
          "$ref": "#/components/schemas/RepositoryId"
        },
        "name": {
          "type": "string"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "request_id",
        "repository",
        "name"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "not_known"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## LocalRepositorySnapshot

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "profile": {
      "type": "string"
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "generation": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "heads": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "metadata_commit": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        {
          "type": "null"
        }
      ]
    },
    "actor": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/PrincipalId"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "profile",
    "repository",
    "generation",
    "heads",
    "metadata_commit",
    "actor"
  ]
}
```

## LocalRepositoryCapabilities

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "profile": {
      "type": "string"
    },
    "code_browse": {
      "type": "boolean"
    },
    "index_tasks": {
      "type": "boolean"
    },
    "automatic_code_index": {
      "type": [
        "boolean",
        "null"
      ]
    },
    "collaboration": {
      "type": "boolean"
    },
    "agents": {
      "type": "boolean"
    },
    "reviews": {
      "type": "boolean"
    },
    "review_actions": {
      "type": "boolean"
    },
    "review_repository_anchors": {
      "type": "boolean"
    },
    "review_code_anchors": {
      "type": "boolean"
    },
    "review_agent_anchors": {
      "type": "boolean"
    },
    "search": {
      "type": "boolean"
    },
    "code_mutations": {
      "type": "boolean"
    },
    "working_copy_edit": {
      "type": "boolean"
    },
    "conversion": {
      "type": "boolean"
    }
  },
  "required": [
    "version",
    "profile",
    "code_browse",
    "index_tasks",
    "automatic_code_index",
    "collaboration",
    "agents",
    "reviews",
    "review_actions",
    "review_repository_anchors",
    "review_code_anchors",
    "review_agent_anchors",
    "search",
    "code_mutations",
    "working_copy_edit",
    "conversion"
  ]
}
```

## LocalCodeQueryResponse

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/LocalCodeSnapshot"
    },
    "data": {
      "$ref": "#/components/schemas/LocalCodeResponse"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    },
    "previous_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "data",
    "next_cursor",
    "previous_cursor"
  ]
}
```

## LocalCodeResponse

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "graph"
        },
        "data": {
          "$ref": "#/components/schemas/LocalGraphPage"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "stack_history"
        },
        "data": {
          "$ref": "#/components/schemas/LocalStackHistoryPage"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "query_info"
        },
        "data": {
          "$ref": "#/components/schemas/QueryInfo"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "commit"
        },
        "data": {
          "$ref": "#/components/schemas/LocalCommitDetail"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "commit_summary"
        },
        "data": {
          "$ref": "#/components/schemas/LocalCodeObject"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "code_summary"
        },
        "data": {
          "$ref": "#/components/schemas/LocalCodeSummary"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "file"
        },
        "data": {
          "$ref": "#/components/schemas/LocalFileContent"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "file_at"
        },
        "data": {
          "$ref": "#/components/schemas/LocalFileContent"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "paths"
        },
        "data": {
          "$ref": "#/components/schemas/LocalPathsPage"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "references"
        },
        "data": {
          "$ref": "#/components/schemas/LocalReferencesPage"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "operation"
        },
        "data": {
          "$ref": "#/components/schemas/LocalOperationDetail"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "operation_diff"
        },
        "data": {
          "$ref": "#/components/schemas/LocalOperationDiffResponse"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "operation_changes"
        },
        "data": {
          "$ref": "#/components/schemas/LocalOperationChangesPage"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "diff"
        },
        "data": {
          "$ref": "#/components/schemas/LocalFileDiffResponse"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "diff_paths"
        },
        "data": {
          "$ref": "#/components/schemas/LocalDiffSummaryPage"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "conflicts"
        },
        "data": {
          "$ref": "#/components/schemas/LocalConflictResponse"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    }
  ]
}
```

## LocalGraphPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "data": {
      "$ref": "#/components/schemas/LocalGraphNodes"
    },
    "next": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/LocalCodeObjectKey"
        },
        {
          "type": "null"
        }
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/LocalGraphStats"
    }
  },
  "required": [
    "version",
    "operation",
    "data",
    "next",
    "stats"
  ]
}
```

## LocalGraphNodes

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "graph": {
          "type": "string",
          "const": "log"
        },
        "nodes": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/LocalGraphCommitNode"
          }
        }
      },
      "required": [
        "graph",
        "nodes"
      ]
    },
    {
      "type": "object",
      "properties": {
        "graph": {
          "type": "string",
          "const": "operations"
        },
        "nodes": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/LocalGraphOperationNode"
          }
        }
      },
      "required": [
        "graph",
        "nodes"
      ]
    },
    {
      "type": "object",
      "properties": {
        "graph": {
          "type": "string",
          "const": "evolution"
        },
        "nodes": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/LocalGraphEvolutionNode"
          }
        }
      },
      "required": [
        "graph",
        "nodes"
      ]
    }
  ]
}
```

## LocalGraphCommitNode

```text
{
  "type": "object",
  "properties": {
    "commit": {
      "$ref": "#/components/schemas/LocalGraphCommit"
    },
    "edges": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalGraphEdge"
      }
    }
  },
  "required": [
    "commit",
    "edges"
  ]
}
```

## LocalGraphCommit

```text
{
  "type": "object",
  "properties": {
    "id": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "change_id": {
      "$ref": "#/components/schemas/ChangeId"
    },
    "parents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "description": {
      "type": "string"
    },
    "author": {
      "$ref": "#/components/schemas/LocalGraphSignature"
    },
    "committer": {
      "$ref": "#/components/schemas/LocalGraphSignature"
    },
    "tree_terms": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "conflicted": {
      "type": "boolean"
    },
    "bookmarks": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalGraphBookmark"
      }
    },
    "workspaces": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "visible": {
      "type": "boolean"
    },
    "divergent": {
      "type": "boolean"
    }
  },
  "required": [
    "id",
    "change_id",
    "parents",
    "description",
    "author",
    "committer",
    "tree_terms",
    "conflicted",
    "bookmarks",
    "workspaces",
    "visible",
    "divergent"
  ]
}
```

## LocalGraphSignature

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "email": {
      "type": "string"
    },
    "timestamp": {
      "$ref": "#/components/schemas/LocalGraphTimestamp"
    }
  },
  "required": [
    "name",
    "email",
    "timestamp"
  ]
}
```

## LocalGraphTimestamp

```text
{
  "type": "object",
  "properties": {
    "millis": {
      "type": "integer",
      "format": "int64"
    },
    "offset_minutes": {
      "type": "integer",
      "format": "int32"
    }
  },
  "required": [
    "millis",
    "offset_minutes"
  ]
}
```

## LocalGraphBookmark

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "remote": {
      "type": [
        "string",
        "null"
      ]
    },
    "added": {
      "type": "boolean"
    },
    "removed": {
      "type": "boolean"
    },
    "conflicted": {
      "type": "boolean"
    }
  },
  "required": [
    "name",
    "remote",
    "added",
    "removed",
    "conflicted"
  ]
}
```

## LocalGraphEdge

```text
{
  "type": "object",
  "properties": {
    "target": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "kind": {
      "$ref": "#/components/schemas/LocalGraphEdgeKind"
    }
  },
  "required": [
    "target",
    "kind"
  ]
}
```

## LocalGraphEdgeKind

```text
{
  "type": "string",
  "enum": [
    "direct",
    "indirect",
    "missing"
  ]
}
```

## LocalGraphOperationNode

```text
{
  "type": "object",
  "properties": {
    "id": {
      "$ref": "#/components/schemas/OperationId"
    },
    "parents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "view": {
      "$ref": "#/components/schemas/ViewId"
    },
    "description": {
      "type": "string"
    },
    "user": {
      "type": "string"
    },
    "host": {
      "type": "string"
    },
    "start": {
      "$ref": "#/components/schemas/LocalGraphTimestamp"
    },
    "end": {
      "$ref": "#/components/schemas/LocalGraphTimestamp"
    },
    "is_snapshot": {
      "type": "boolean"
    },
    "workspace": {
      "type": [
        "string",
        "null"
      ]
    },
    "attributes": {
      "type": "object",
      "additionalProperties": {
        "type": "string"
      }
    },
    "predecessor_data_present": {
      "type": "boolean"
    }
  },
  "required": [
    "id",
    "parents",
    "view",
    "description",
    "user",
    "host",
    "start",
    "end",
    "is_snapshot",
    "workspace",
    "attributes",
    "predecessor_data_present"
  ]
}
```

## LocalGraphEvolutionNode

```text
{
  "type": "object",
  "properties": {
    "commit": {
      "$ref": "#/components/schemas/LocalGraphCommit"
    },
    "operation": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/OperationId"
        },
        {
          "type": "null"
        }
      ]
    },
    "predecessors": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "predecessors_known": {
      "type": "boolean"
    },
    "legacy_predecessors": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    }
  },
  "required": [
    "commit",
    "operation",
    "predecessors",
    "predecessors_known",
    "legacy_predecessors"
  ]
}
```

## LocalCodeObjectKey

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "commit"
        },
        "id": {
          "$ref": "#/components/schemas/GitSha1Id"
        }
      },
      "required": [
        "kind",
        "id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "tree"
        },
        "id": {
          "$ref": "#/components/schemas/GitSha1Id"
        }
      },
      "required": [
        "kind",
        "id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "file"
        },
        "id": {
          "$ref": "#/components/schemas/GitSha1Id"
        }
      },
      "required": [
        "kind",
        "id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "symlink"
        },
        "id": {
          "$ref": "#/components/schemas/GitSha1Id"
        }
      },
      "required": [
        "kind",
        "id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "operation"
        },
        "id": {
          "$ref": "#/components/schemas/OperationId"
        }
      },
      "required": [
        "kind",
        "id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "view"
        },
        "id": {
          "$ref": "#/components/schemas/ViewId"
        }
      },
      "required": [
        "kind",
        "id"
      ]
    }
  ]
}
```

## LocalGraphStats

```text
{
  "type": "object",
  "properties": {
    "metadata_nodes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "predicate_file_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "predicate_lines": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "object_reads": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "object_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "work": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "metadata_nodes",
    "predicate_file_bytes",
    "predicate_lines",
    "object_reads",
    "object_bytes",
    "work"
  ]
}
```

## LocalStackHistoryPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "head": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "base": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        {
          "type": "null"
        }
      ]
    },
    "commits": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalGraphCommitNode"
      }
    },
    "next": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        {
          "type": "null"
        }
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/LocalGraphStats"
    }
  },
  "required": [
    "version",
    "operation",
    "head",
    "base",
    "commits",
    "next",
    "stats"
  ]
}
```

## LocalCommitDetail

```text
{
  "type": "object",
  "properties": {
    "object": {
      "$ref": "#/components/schemas/LocalCodeObject"
    },
    "commit": {
      "$ref": "#/components/schemas/LocalGraphCommit"
    }
  },
  "required": [
    "object",
    "commit"
  ]
}
```

## LocalCodeObject

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "profile": {
      "type": "string"
    },
    "key": {
      "$ref": "#/components/schemas/LocalCodeObjectKey"
    },
    "virtual_object": {
      "type": "boolean"
    },
    "data": {
      "$ref": "#/components/schemas/LocalCodeObjectData"
    },
    "stats": {
      "$ref": "#/components/schemas/LocalGraphStats"
    }
  },
  "required": [
    "version",
    "profile",
    "key",
    "virtual_object",
    "data",
    "stats"
  ]
}
```

## LocalCodeObjectData

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "commit"
        },
        "data": {
          "$ref": "#/components/schemas/GitSourceCommitV1"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "operation"
        },
        "data": {
          "$ref": "#/components/schemas/GitSourceOperationV1"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "view"
        },
        "data": {
          "$ref": "#/components/schemas/GitSourceViewV1"
        }
      },
      "required": [
        "kind",
        "data"
      ]
    }
  ]
}
```

## GitSourceCommitV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "parents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "predecessors": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "root_tree": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "conflict_labels": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "change_id": {
      "$ref": "#/components/schemas/ChangeId"
    },
    "description": {
      "type": "string"
    },
    "author": {
      "$ref": "#/components/schemas/NativeSignature"
    },
    "committer": {
      "$ref": "#/components/schemas/NativeSignature"
    },
    "secure_sig": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/NativeSecureSig"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "parents",
    "predecessors",
    "root_tree",
    "conflict_labels",
    "change_id",
    "description",
    "author",
    "committer",
    "secure_sig"
  ]
}
```

## GitSourceOperationV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "view_id": {
      "$ref": "#/components/schemas/ViewId"
    },
    "parents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "metadata": {
      "$ref": "#/components/schemas/NativeMetadata"
    },
    "commit_predecessors": {
      "type": [
        "object",
        "null"
      ],
      "additionalProperties": false,
      "patternProperties": {
        "^[0-9a-f]{40}$": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/GitSha1Id"
          }
        }
      }
    }
  },
  "required": [
    "view_id",
    "parents",
    "metadata",
    "commit_predecessors"
  ]
}
```

## GitSourceViewV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "head_ids": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "local_bookmarks": {
      "type": "object",
      "additionalProperties": {
        "type": "array",
        "items": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/GitSha1Id"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "local_tags": {
      "type": "object",
      "additionalProperties": {
        "type": "array",
        "items": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/GitSha1Id"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "remote_views": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/GitSourceRemoteViewV1"
      }
    },
    "git_refs": {
      "type": "object",
      "additionalProperties": {
        "type": "array",
        "items": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/GitSha1Id"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "git_heads": {
      "type": "object",
      "additionalProperties": {
        "type": "array",
        "items": {
          "anyOf": [
            {
              "$ref": "#/components/schemas/GitSha1Id"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "wc_commit_ids": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    }
  },
  "required": [
    "head_ids",
    "local_bookmarks",
    "local_tags",
    "remote_views",
    "git_refs",
    "git_heads",
    "wc_commit_ids"
  ]
}
```

## GitSourceRemoteViewV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "bookmarks": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/GitSourceRemoteRefV1"
      }
    },
    "tags": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/GitSourceRemoteRefV1"
      }
    }
  },
  "required": [
    "bookmarks",
    "tags"
  ]
}
```

## GitSourceRemoteRefV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "target": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/GitSha1Id"
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "state": {
      "$ref": "#/components/schemas/NativeRemoteState"
    }
  },
  "required": [
    "target",
    "state"
  ]
}
```

## LocalCodeSummary

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "commits": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "commits_partial": {
      "type": "boolean"
    },
    "latest": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/LocalCodeSummaryLatest"
        },
        {
          "type": "null"
        }
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/LocalGraphStats"
    }
  },
  "required": [
    "version",
    "commits",
    "commits_partial",
    "latest",
    "stats"
  ]
}
```

## LocalCodeSummaryLatest

```text
{
  "type": "object",
  "properties": {
    "id": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "description": {
      "type": "string"
    },
    "author": {
      "$ref": "#/components/schemas/NativeSignature"
    },
    "committer": {
      "$ref": "#/components/schemas/NativeSignature"
    }
  },
  "required": [
    "id",
    "description",
    "author",
    "committer"
  ]
}
```

## LocalFileContent

```text
{
  "type": "object",
  "properties": {
    "bytes": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "text": {
      "type": [
        "string",
        "null"
      ]
    },
    "binary": {
      "type": "boolean"
    },
    "too_large": {
      "type": "boolean"
    }
  },
  "required": [
    "bytes",
    "text",
    "binary",
    "too_large"
  ]
}
```

## LocalPathsPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "commit": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "path": {
      "type": "string"
    },
    "recursive": {
      "type": "boolean"
    },
    "entries": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalPathsEntry"
      }
    },
    "previous": {
      "type": [
        "string",
        "null"
      ]
    },
    "next": {
      "type": [
        "string",
        "null"
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/LocalGraphStats"
    }
  },
  "required": [
    "version",
    "commit",
    "path",
    "recursive",
    "entries",
    "previous",
    "next",
    "stats"
  ]
}
```

## LocalPathsEntry

```text
{
  "type": "object",
  "properties": {
    "path": {
      "type": "string"
    },
    "terms": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/LocalTreeValue"
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "conflicted": {
      "type": "boolean"
    },
    "last_change": {
      "$ref": "#/components/schemas/LocalPathsLastChange"
    }
  },
  "required": [
    "path",
    "terms",
    "conflicted",
    "last_change"
  ]
}
```

## LocalTreeValue

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "id": {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        "executable": {
          "type": "boolean"
        },
        "copy": {
          "$ref": "#/components/schemas/LocalCopyReference"
        },
        "type": {
          "type": "string",
          "const": "file"
        }
      },
      "required": [
        "type",
        "id",
        "executable",
        "copy"
      ]
    },
    {
      "type": "object",
      "properties": {
        "id": {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        "type": {
          "type": "string",
          "const": "symlink"
        }
      },
      "required": [
        "type",
        "id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "id": {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        "type": {
          "type": "string",
          "const": "tree"
        }
      },
      "required": [
        "type",
        "id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "id": {
          "type": "array",
          "items": {
            "type": "integer",
            "format": "uint8",
            "minimum": 0,
            "maximum": 255
          }
        },
        "type": {
          "type": "string",
          "const": "submodule"
        }
      },
      "required": [
        "type",
        "id"
      ]
    }
  ]
}
```

## LocalCopyReference

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "placeholder"
        }
      },
      "required": [
        "kind"
      ]
    },
    {
      "type": "object",
      "properties": {
        "kind": {
          "type": "string",
          "const": "object"
        },
        "id": {
          "type": "string"
        }
      },
      "required": [
        "kind",
        "id"
      ]
    }
  ]
}
```

## LocalPathsLastChange

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "attribution": {
          "$ref": "#/components/schemas/LocalPathsAttribution"
        },
        "commit": {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        "change": {
          "$ref": "#/components/schemas/ChangeId"
        },
        "description": {
          "type": "string"
        },
        "author": {
          "$ref": "#/components/schemas/NativeSignature"
        },
        "committer": {
          "$ref": "#/components/schemas/NativeSignature"
        },
        "status": {
          "type": "string",
          "const": "available"
        }
      },
      "required": [
        "status",
        "attribution",
        "commit",
        "change",
        "description",
        "author",
        "committer"
      ]
    },
    {
      "type": "object",
      "properties": {
        "reason": {
          "$ref": "#/components/schemas/LocalPathsLastChangeUnavailable"
        },
        "status": {
          "type": "string",
          "const": "unavailable"
        }
      },
      "required": [
        "status",
        "reason"
      ]
    }
  ]
}
```

## LocalPathsAttribution

```text
{
  "type": "string",
  "enum": [
    "path_change",
    "snapshot"
  ],
  "description": "Provenance of the displayed commit, not a claim about all-branch history."
}
```

## LocalPathsLastChangeUnavailable

```text
{
  "type": "string",
  "enum": [
    "merge",
    "budget",
    "not_requested",
    "pending",
    "not_found"
  ]
}
```

## LocalReferencesPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "references": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalReferencesReference"
      }
    },
    "previous": {
      "type": [
        "string",
        "null"
      ]
    },
    "next": {
      "type": [
        "string",
        "null"
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/LocalGraphStats"
    }
  },
  "required": [
    "version",
    "operation",
    "references",
    "previous",
    "next",
    "stats"
  ]
}
```

## LocalReferencesReference

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "targets": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/LocalReferencesTarget"
          }
        },
        "kind": {
          "type": "string",
          "const": "workspace"
        }
      },
      "required": [
        "kind",
        "name",
        "targets"
      ]
    },
    {
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "terms": {
          "type": "array",
          "items": {
            "anyOf": [
              {
                "$ref": "#/components/schemas/GitSha1Id"
              },
              {
                "type": "null"
              }
            ]
          }
        },
        "conflicted": {
          "type": "boolean"
        },
        "workspaces": {
          "type": "array",
          "items": {
            "type": "string"
          }
        },
        "targets": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/LocalReferencesTarget"
          }
        },
        "kind": {
          "type": "string",
          "const": "bookmark"
        }
      },
      "required": [
        "kind",
        "name",
        "terms",
        "conflicted",
        "workspaces",
        "targets"
      ]
    },
    {
      "type": "object",
      "properties": {
        "change": {
          "$ref": "#/components/schemas/ChangeId"
        },
        "divergent": {
          "type": "boolean"
        },
        "targets": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/LocalReferencesTarget"
          }
        },
        "kind": {
          "type": "string",
          "const": "change"
        }
      },
      "required": [
        "kind",
        "change",
        "divergent",
        "targets"
      ]
    }
  ]
}
```

## LocalReferencesTarget

```text
{
  "type": "object",
  "properties": {
    "commit": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "change": {
      "$ref": "#/components/schemas/ChangeId"
    },
    "parents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "description": {
      "type": "string"
    },
    "author": {
      "$ref": "#/components/schemas/NativeSignature"
    },
    "committer": {
      "$ref": "#/components/schemas/NativeSignature"
    },
    "conflicted": {
      "type": "boolean"
    },
    "workspaces": {
      "type": "array",
      "items": {
        "type": "string"
      }
    }
  },
  "required": [
    "commit",
    "change",
    "parents",
    "description",
    "author",
    "committer",
    "conflicted",
    "workspaces"
  ]
}
```

## LocalOperationDetail

```text
{
  "type": "object",
  "properties": {
    "operation": {
      "$ref": "#/components/schemas/LocalCodeObject"
    },
    "view": {
      "$ref": "#/components/schemas/LocalCodeObject"
    }
  },
  "required": [
    "operation",
    "view"
  ]
}
```

## LocalOperationDiffResponse

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "data": {
      "$ref": "#/components/schemas/LocalOperationDiffData"
    },
    "stats": {
      "$ref": "#/components/schemas/LocalGraphStats"
    }
  },
  "required": [
    "version",
    "data",
    "stats"
  ]
}
```

## LocalOperationDiffData

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "operation": {
          "$ref": "#/components/schemas/OperationId"
        },
        "view": {
          "$ref": "#/components/schemas/ViewId"
        },
        "status": {
          "type": "string",
          "const": "root"
        }
      },
      "required": [
        "status",
        "operation",
        "view"
      ]
    },
    {
      "type": "object",
      "properties": {
        "operation": {
          "$ref": "#/components/schemas/OperationId"
        },
        "parents": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/OperationId"
          }
        },
        "status": {
          "type": "string",
          "const": "select_parent"
        }
      },
      "required": [
        "status",
        "operation",
        "parents"
      ]
    },
    {
      "required": [
        "status"
      ],
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "difference"
        }
      },
      "allOf": [
        {
          "$ref": "#/components/schemas/LocalOperationDiffDifference"
        }
      ]
    }
  ]
}
```

## LocalOperationDiffViewDelta

```text
{
  "type": "object",
  "properties": {
    "heads": {
      "$ref": "#/components/schemas/LocalSetDelta_GitSha1Id"
    },
    "local_bookmarks": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalNamedChange_Array_of_Nullable_GitSha1Id"
      }
    },
    "local_tags": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalNamedChange_Array_of_Nullable_GitSha1Id"
      }
    },
    "remote_views": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalNamedChange_GitSourceRemoteViewV1"
      }
    },
    "git_refs": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalNamedChange_Array_of_Nullable_GitSha1Id"
      }
    },
    "git_heads": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalNamedChange_Array_of_Nullable_GitSha1Id"
      }
    },
    "workspaces": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalNamedChange_GitSha1Id"
      }
    }
  },
  "required": [
    "heads",
    "local_bookmarks",
    "local_tags",
    "remote_views",
    "git_refs",
    "git_heads",
    "workspaces"
  ]
}
```

## LocalSetDelta_GitSha1Id

```text
{
  "type": "object",
  "properties": {
    "added": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "removed": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    }
  },
  "required": [
    "added",
    "removed"
  ]
}
```

## LocalNamedChange_Array_of_Nullable_GitSha1Id

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "before": {
      "type": [
        "array",
        "null"
      ],
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/GitSha1Id"
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "after": {
      "type": [
        "array",
        "null"
      ],
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/GitSha1Id"
          },
          {
            "type": "null"
          }
        ]
      }
    }
  },
  "required": [
    "name",
    "before",
    "after"
  ]
}
```

## LocalNamedChange_GitSourceRemoteViewV1

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "before": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GitSourceRemoteViewV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "after": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GitSourceRemoteViewV1"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "name",
    "before",
    "after"
  ]
}
```

## LocalNamedChange_GitSha1Id

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "before": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        {
          "type": "null"
        }
      ]
    },
    "after": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "name",
    "before",
    "after"
  ]
}
```

## LocalOperationDiffDifference

```text
{
  "type": "object",
  "properties": {
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "parent": {
      "$ref": "#/components/schemas/OperationId"
    },
    "before_view": {
      "$ref": "#/components/schemas/ViewId"
    },
    "after_view": {
      "$ref": "#/components/schemas/ViewId"
    },
    "view": {
      "$ref": "#/components/schemas/LocalOperationDiffViewDelta"
    },
    "visible_commits": {
      "$ref": "#/components/schemas/LocalSetDelta_GitSha1Id"
    },
    "commit_predecessors": {
      "type": [
        "object",
        "null"
      ],
      "additionalProperties": false,
      "patternProperties": {
        "^[0-9a-f]{40}$": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/GitSha1Id"
          }
        }
      }
    }
  },
  "required": [
    "operation",
    "parent",
    "before_view",
    "after_view",
    "view",
    "visible_commits",
    "commit_predecessors"
  ]
}
```

## LocalOperationChangesPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "parent": {
      "$ref": "#/components/schemas/OperationId"
    },
    "groups": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalOperationChangesGroup"
      }
    },
    "previous": {
      "type": [
        "string",
        "null"
      ]
    },
    "next": {
      "type": [
        "string",
        "null"
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/LocalGraphStats"
    }
  },
  "required": [
    "version",
    "operation",
    "parent",
    "groups",
    "previous",
    "next",
    "stats"
  ]
}
```

## LocalOperationChangesGroup

```text
{
  "type": "object",
  "properties": {
    "change": {
      "$ref": "#/components/schemas/ChangeId"
    },
    "before": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "after": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "comparison": {
      "$ref": "#/components/schemas/LocalOperationChangesComparison"
    }
  },
  "required": [
    "change",
    "before",
    "after",
    "comparison"
  ]
}
```

## LocalOperationChangesComparison

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "from": {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        "to": {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        "status": {
          "type": "string",
          "const": "exact"
        }
      },
      "required": [
        "status",
        "from",
        "to"
      ]
    },
    {
      "type": "object",
      "properties": {
        "before": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/GitSha1Id"
          }
        },
        "after": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/GitSha1Id"
          }
        },
        "status": {
          "type": "string",
          "const": "select"
        }
      },
      "required": [
        "status",
        "before",
        "after"
      ]
    },
    {
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "unavailable"
        }
      },
      "required": [
        "status"
      ]
    }
  ]
}
```

## LocalFileDiffResponse

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "from": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "to": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "path": {
      "type": "string"
    },
    "changes": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalFileDiffChange"
      }
    },
    "stats": {
      "$ref": "#/components/schemas/LocalGraphStats"
    }
  },
  "required": [
    "version",
    "from",
    "to",
    "path",
    "changes",
    "stats"
  ]
}
```

## LocalFileDiffChange

```text
{
  "type": "object",
  "properties": {
    "path": {
      "type": "string"
    },
    "before": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/LocalTreeValue"
          },
          {
            "type": "null"
          }
        ]
      },
      "description": "Local values at the target; source annotations never replace these."
    },
    "after": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/LocalTreeValue"
          },
          {
            "type": "null"
          }
        ]
      }
    },
    "sources": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalFileDiffSource"
      }
    },
    "content": {
      "$ref": "#/components/schemas/LocalFileDiffContent"
    }
  },
  "required": [
    "path",
    "before",
    "after",
    "sources",
    "content"
  ]
}
```

## LocalFileDiffSource

```text
{
  "type": "object",
  "properties": {
    "kind": {
      "$ref": "#/components/schemas/LocalFileDiffSourceKind"
    },
    "path": {
      "type": "string"
    },
    "terms": {
      "type": "array",
      "items": {
        "anyOf": [
          {
            "$ref": "#/components/schemas/LocalTreeValue"
          },
          {
            "type": "null"
          }
        ]
      }
    }
  },
  "required": [
    "kind",
    "path",
    "terms"
  ]
}
```

## LocalFileDiffSourceKind

```text
{
  "type": "string",
  "enum": [
    "normal",
    "copy",
    "rename"
  ]
}
```

## LocalFileDiffContent

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "hunks": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/LocalFileDiffHunk"
          }
        },
        "status": {
          "type": "string",
          "const": "text"
        }
      },
      "required": [
        "status",
        "hunks"
      ]
    },
    {
      "type": "object",
      "properties": {
        "reason": {
          "$ref": "#/components/schemas/LocalFileDiffUnavailable"
        },
        "status": {
          "type": "string",
          "const": "unavailable"
        }
      },
      "required": [
        "status",
        "reason"
      ]
    }
  ]
}
```

## LocalFileDiffHunk

```text
{
  "type": "object",
  "properties": {
    "kind": {
      "$ref": "#/components/schemas/LocalFileDiffHunkKind"
    },
    "before_start": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "after_start": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "before": {
      "type": "string"
    },
    "after": {
      "type": "string"
    }
  },
  "required": [
    "kind",
    "before_start",
    "after_start",
    "before",
    "after"
  ]
}
```

## LocalFileDiffHunkKind

```text
{
  "type": "string",
  "enum": [
    "matching",
    "different"
  ]
}
```

## LocalFileDiffUnavailable

```text
{
  "type": "string",
  "enum": [
    "not_requested",
    "conflict",
    "structural",
    "multiple_sources",
    "binary",
    "too_large",
    "complexity"
  ]
}
```

## LocalDiffSummaryPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "from": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "to": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "path": {
      "type": "string"
    },
    "changes": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalFileDiffChange"
      }
    },
    "previous": {
      "type": [
        "string",
        "null"
      ]
    },
    "next": {
      "type": [
        "string",
        "null"
      ]
    },
    "stats": {
      "$ref": "#/components/schemas/LocalGraphStats"
    }
  },
  "required": [
    "version",
    "from",
    "to",
    "path",
    "changes",
    "previous",
    "next",
    "stats"
  ]
}
```

## LocalConflictResponse

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "commit": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "change": {
      "$ref": "#/components/schemas/ChangeId"
    },
    "conflicts": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalConflictDescriptor"
      }
    },
    "stats": {
      "$ref": "#/components/schemas/LocalGraphStats"
    }
  },
  "required": [
    "version",
    "commit",
    "change",
    "conflicts",
    "stats"
  ]
}
```

## LocalConflictDescriptor

```text
{
  "type": "object",
  "properties": {
    "path": {
      "type": "string"
    },
    "fingerprint": {
      "$ref": "#/components/schemas/ConflictFingerprint"
    },
    "terms": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalConflictTerm"
      }
    },
    "editor": {
      "$ref": "#/components/schemas/LocalConflictEditor"
    }
  },
  "required": [
    "path",
    "fingerprint",
    "terms",
    "editor"
  ]
}
```

## LocalConflictTerm

```text
{
  "type": "object",
  "properties": {
    "id": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "role": {
      "$ref": "#/components/schemas/LocalConflictRole"
    },
    "label": {
      "type": [
        "string",
        "null"
      ]
    },
    "value": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/LocalTreeValue"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "id",
    "role",
    "label",
    "value"
  ]
}
```

## LocalConflictRole

```text
{
  "type": "string",
  "enum": [
    "add",
    "remove"
  ]
}
```

## LocalConflictEditor

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "contents": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "minItems": 3,
          "maxItems": 3
        },
        "executable": {
          "type": "boolean"
        },
        "status": {
          "type": "string",
          "const": "text"
        }
      },
      "required": [
        "status",
        "contents",
        "executable"
      ]
    },
    {
      "type": "object",
      "properties": {
        "reason": {
          "$ref": "#/components/schemas/LocalConflictUnsupported"
        },
        "status": {
          "type": "string",
          "const": "unsupported"
        }
      },
      "required": [
        "status",
        "reason"
      ]
    }
  ]
}
```

## LocalConflictUnsupported

```text
{
  "type": "string",
  "enum": [
    "multiple_sides",
    "delete_modify",
    "structural",
    "executable_mode",
    "copy_history",
    "binary",
    "too_large",
    "invalid_path"
  ]
}
```

## LocalIndexTask

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "id": {
      "type": "string"
    },
    "snapshot": {
      "$ref": "#/components/schemas/LocalCodeSnapshot"
    }
  },
  "required": [
    "version",
    "id",
    "snapshot"
  ],
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "running"
        }
      },
      "required": [
        "status"
      ]
    },
    {
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "canceling"
        }
      },
      "required": [
        "status"
      ]
    },
    {
      "type": "object",
      "properties": {
        "stats": {
          "$ref": "#/components/schemas/LocalGraphStats"
        },
        "status": {
          "type": "string",
          "const": "ready"
        }
      },
      "required": [
        "status",
        "stats"
      ]
    },
    {
      "type": "object",
      "properties": {
        "error": {
          "$ref": "#/components/schemas/GraphError"
        },
        "status": {
          "type": "string",
          "const": "failed"
        }
      },
      "required": [
        "status",
        "error"
      ]
    },
    {
      "type": "object",
      "properties": {
        "status": {
          "type": "string",
          "const": "canceled"
        }
      },
      "required": [
        "status"
      ]
    }
  ]
}
```

## LocalWorkPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/LocalWorkSnapshot"
    },
    "projects": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ProjectV1"
      }
    },
    "tasks": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/TaskV1"
      }
    },
    "comments": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/WorkCommentV1"
      }
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "projects",
    "tasks",
    "comments",
    "next_cursor"
  ]
}
```

## LocalWorkSnapshot

```text
{
  "type": "object",
  "properties": {
    "profile": {
      "$ref": "#/components/schemas/LocalProfile"
    },
    "repo_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RepositoryId"
        },
        {
          "type": "null"
        }
      ]
    },
    "metadata_commit": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        {
          "type": "null"
        }
      ]
    },
    "work_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "profile",
    "repo_id",
    "metadata_commit",
    "work_state_version"
  ]
}
```

## LocalProfile

```text
{
  "type": "string",
  "enum": [
    "local_jj_git"
  ]
}
```

## LocalWorkBrowsePage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/LocalWorkSnapshot"
    },
    "projects": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/WorkProjectSummary"
      }
    },
    "tasks": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/WorkTaskSummary"
      }
    },
    "total_matches": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "projects",
    "tasks",
    "total_matches",
    "next_cursor"
  ]
}
```

## LocalWorkActivityPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/LocalWorkSnapshot"
    },
    "groups": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/WorkActivityGroup"
      },
      "maxItems": 256
    },
    "total_groups": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "groups",
    "total_groups",
    "next_cursor"
  ]
}
```

## LocalWorkOutcome

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "receipt": {
          "$ref": "#/components/schemas/LocalWorkReceipt"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "receipt"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "not_known"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## LocalWorkReceipt

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "profile": {
      "$ref": "#/components/schemas/LocalProfile"
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "request_digest": {
      "$ref": "#/components/schemas/IntentDigest"
    },
    "metadata_sequence": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "work_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "work_object": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "result": {
      "$ref": "#/components/schemas/WorkCommandResultV1"
    }
  },
  "required": [
    "version",
    "profile",
    "request_id",
    "repository",
    "actor",
    "request_digest",
    "metadata_sequence",
    "work_state_version",
    "work_object",
    "result"
  ]
}
```

## Page5

```text
{
  "type": "object",
  "properties": {
    "data": {
      "$ref": "#/components/schemas/LocalBrowserAgentPage"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "data",
    "next_cursor"
  ]
}
```

## LocalBrowserAgentPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/LocalAgentSnapshot"
    },
    "sessions": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentSessionSummary"
      }
    },
    "next_after": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentCatalogKey"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "sessions",
    "next_after"
  ]
}
```

## LocalAgentSnapshot

```text
{
  "type": "object",
  "properties": {
    "profile": {
      "$ref": "#/components/schemas/LocalProfile"
    },
    "repo_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RepositoryId"
        },
        {
          "type": "null"
        }
      ]
    },
    "metadata_commit": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        {
          "type": "null"
        }
      ]
    },
    "agent_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "agent_root": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RecordDigest"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "profile",
    "repo_id",
    "metadata_commit",
    "agent_state_version",
    "agent_root"
  ]
}
```

## AgentCatalogKey

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{64}$",
  "minLength": 64,
  "maxLength": 64
}
```

## LocalBrowserAgentDetail

```text
{
  "type": "object",
  "properties": {
    "conversation_feedback": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentExecutionFeedback"
        },
        {
          "type": "null"
        }
      ]
    },
    "conversation_unavailable": {
      "type": [
        "string",
        "null"
      ]
    },
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/LocalAgentSnapshot"
    },
    "session": {
      "$ref": "#/components/schemas/AgentSessionSummary"
    },
    "evidence": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentObjectId"
      }
    },
    "event_batches": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentObjectId"
      }
    },
    "transcripts": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentTranscriptDescriptor"
      }
    },
    "tools": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentToolLink"
      }
    },
    "activity": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentActivity"
      }
    },
    "activity_truncated": {
      "type": "boolean"
    },
    "code": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalBrowserCodeAssociation"
      }
    },
    "code_truncated": {
      "type": "boolean"
    }
  },
  "required": [
    "version",
    "snapshot",
    "session",
    "evidence",
    "event_batches",
    "transcripts",
    "tools",
    "activity",
    "activity_truncated",
    "code",
    "code_truncated"
  ]
}
```

## LocalBrowserCodeAssociation

```text
{
  "type": "object",
  "properties": {
    "profile": {
      "$ref": "#/components/schemas/CodeProfileV1"
    },
    "source_repositories": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    "evidence": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CodeEvidenceRefV1"
      }
    },
    "association_id": {
      "$ref": "#/components/schemas/AssociationId"
    },
    "object_id": {
      "$ref": "#/components/schemas/AgentObjectId"
    },
    "session_id": {
      "$ref": "#/components/schemas/AgentSessionId"
    },
    "tool_call_ids": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ToolCallId"
      }
    },
    "relation": {
      "$ref": "#/components/schemas/AssociationRelation"
    },
    "before_operations": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "after_operations": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    "before_commits": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "after_commits": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "changes": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ChangeId"
      }
    },
    "paths": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "created_at_millis": {
      "type": "integer",
      "format": "int64"
    }
  },
  "required": [
    "profile",
    "source_repositories",
    "evidence",
    "association_id",
    "object_id",
    "session_id",
    "tool_call_ids",
    "relation",
    "before_operations",
    "after_operations",
    "before_commits",
    "after_commits",
    "changes",
    "paths",
    "created_at_millis"
  ]
}
```

## Page6

```text
{
  "type": "object",
  "properties": {
    "data": {
      "$ref": "#/components/schemas/LocalBrowserTranscriptPage"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "data",
    "next_cursor"
  ]
}
```

## LocalBrowserTranscriptPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/LocalAgentSnapshot"
    },
    "session_id": {
      "$ref": "#/components/schemas/AgentSessionId"
    },
    "transcript_id": {
      "$ref": "#/components/schemas/AgentObjectId"
    },
    "source_instance_id": {
      "$ref": "#/components/schemas/SourceInstanceId"
    },
    "source_generation": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "chunks": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentTranscriptChunk"
      }
    },
    "next_after_sequence": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "version",
    "snapshot",
    "session_id",
    "transcript_id",
    "source_instance_id",
    "source_generation",
    "chunks",
    "next_after_sequence"
  ]
}
```

## Page7

```text
{
  "type": "object",
  "properties": {
    "data": {
      "$ref": "#/components/schemas/LocalBrowserAgentRelations"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "data",
    "next_cursor"
  ]
}
```

## LocalBrowserAgentRelations

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/LocalAgentSnapshot"
    },
    "subject": {
      "$ref": "#/components/schemas/LocalAgentSubject"
    },
    "associations": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalBrowserCodeAssociation"
      }
    },
    "next_after": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentCatalogKey"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "subject",
    "associations",
    "next_after"
  ]
}
```

## LocalAgentSubject

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "session_id": {
          "$ref": "#/components/schemas/AgentSessionId"
        },
        "subject": {
          "type": "string",
          "const": "session"
        }
      },
      "required": [
        "subject",
        "session_id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "repository": {
          "$ref": "#/components/schemas/RepositoryId"
        },
        "operation_id": {
          "$ref": "#/components/schemas/OperationId"
        },
        "subject": {
          "type": "string",
          "const": "operation"
        }
      },
      "required": [
        "subject",
        "repository",
        "operation_id"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "repository": {
          "$ref": "#/components/schemas/RepositoryId"
        },
        "commit_id": {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        "subject": {
          "type": "string",
          "const": "revision"
        }
      },
      "required": [
        "subject",
        "repository",
        "commit_id"
      ]
    }
  ]
}
```

## LocalReviewPageResponse

```text
{
  "type": "object",
  "properties": {
    "data": {
      "$ref": "#/components/schemas/LocalReviewThreadPage"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "data",
    "next_cursor"
  ]
}
```

## LocalReviewThreadPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/LocalReviewSnapshot"
    },
    "threads": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewThreadSummary"
      }
    },
    "next_after": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReviewCatalogKey"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "snapshot",
    "threads",
    "next_after"
  ]
}
```

## LocalReviewSnapshot

```text
{
  "type": "object",
  "properties": {
    "profile": {
      "$ref": "#/components/schemas/LocalProfile"
    },
    "repo_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RepositoryId"
        },
        {
          "type": "null"
        }
      ]
    },
    "metadata_commit": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        {
          "type": "null"
        }
      ]
    },
    "review_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "review_root": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RecordDigest"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "profile",
    "repo_id",
    "metadata_commit",
    "review_state_version",
    "review_root"
  ]
}
```

## ReviewCatalogKey

```text
{
  "type": "string",
  "pattern": "^[0-9a-f]{64}$",
  "minLength": 64,
  "maxLength": 64
}
```

## LocalReviewActivitySummary

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/LocalReviewSnapshot"
    },
    "activity": {
      "$ref": "#/components/schemas/ReviewActivity"
    }
  },
  "required": [
    "version",
    "snapshot",
    "activity"
  ]
}
```

## LocalReviewThreadDetail

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "snapshot": {
      "$ref": "#/components/schemas/LocalReviewSnapshot"
    },
    "thread": {
      "$ref": "#/components/schemas/ReviewThreadSummary"
    },
    "anchor": {
      "$ref": "#/components/schemas/ReviewAnchor"
    },
    "comments": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewComment"
      }
    },
    "comment_conflicts": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewCommentConflict"
      }
    },
    "events": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ReviewEvent"
      }
    }
  },
  "required": [
    "version",
    "snapshot",
    "thread",
    "anchor",
    "comments",
    "comment_conflicts",
    "events"
  ]
}
```

## LocalReviewOutcome

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "receipt": {
          "$ref": "#/components/schemas/LocalReviewReceipt"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "receipt"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "not_known"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## LocalReviewReceipt

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "profile": {
      "$ref": "#/components/schemas/LocalProfile"
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "intent": {
      "$ref": "#/components/schemas/IntentDigest"
    },
    "metadata_sequence": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "review_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "review_root": {
      "$ref": "#/components/schemas/RecordDigest"
    },
    "accepted_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "command": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReviewCommandResultV1"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "profile",
    "repository",
    "actor",
    "request_id",
    "intent",
    "metadata_sequence",
    "review_state_version",
    "review_root",
    "accepted_at_millis",
    "command"
  ]
}
```

## LocalInboxSelection

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/LocalCollabSnapshot"
    },
    "actor": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/PrincipalId"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "snapshot",
    "actor"
  ]
}
```

## LocalCollabSnapshot

```text
{
  "type": "object",
  "properties": {
    "profile": {
      "$ref": "#/components/schemas/LocalProfile"
    },
    "repo_id": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RepositoryId"
        },
        {
          "type": "null"
        }
      ]
    },
    "metadata_commit": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GitSha1Id"
        },
        {
          "type": "null"
        }
      ]
    },
    "state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "profile",
    "repo_id",
    "metadata_commit",
    "state_version"
  ]
}
```

## LocalInboxPageResponse

```text
{
  "type": "object",
  "properties": {
    "data": {
      "$ref": "#/components/schemas/LocalInboxCollabPage"
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "data",
    "next_cursor"
  ]
}
```

## LocalInboxCollabPage

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/LocalCollabSnapshot"
    },
    "items": {
      "$ref": "#/components/schemas/CollabPageItems"
    },
    "next_after": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "snapshot",
    "items",
    "next_after"
  ]
}
```

## LocalInboxThreadDetail

```text
{
  "type": "object",
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/LocalCollabSnapshot"
    },
    "thread": {
      "$ref": "#/components/schemas/ThreadSummaryV1"
    }
  },
  "required": [
    "snapshot",
    "thread"
  ]
}
```

## LocalCollabOutcome

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "receipt": {
          "$ref": "#/components/schemas/LocalCollabReceipt"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "receipt"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "not_known"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## LocalCollabReceipt

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "profile": {
      "$ref": "#/components/schemas/LocalProfile"
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "actor": {
      "$ref": "#/components/schemas/PrincipalId"
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "intent": {
      "$ref": "#/components/schemas/IntentDigest"
    },
    "metadata_sequence": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "domain_state_version": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "domain_object": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "accepted_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "result": {
      "$ref": "#/components/schemas/CollabCommandResultV1"
    }
  },
  "required": [
    "version",
    "profile",
    "repository",
    "actor",
    "request_id",
    "intent",
    "metadata_sequence",
    "domain_state_version",
    "domain_object",
    "accepted_at_millis",
    "result"
  ]
}
```

## ImageAssetV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "schema_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "media_type": {
      "$ref": "#/components/schemas/ImageMediaTypeV1"
    },
    "width": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "height": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "frames": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "bytes": {
      "type": "array",
      "items": {
        "type": "integer",
        "format": "uint8",
        "minimum": 0,
        "maximum": 255
      }
    }
  },
  "required": [
    "schema_version",
    "media_type",
    "width",
    "height",
    "frames",
    "bytes"
  ]
}
```

## ImageMediaTypeV1

```text
{
  "type": "string",
  "enum": [
    "png",
    "jpeg",
    "gif",
    "webp"
  ]
}
```

## WorkingFile

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "selection": {
      "$ref": "#/components/schemas/WorkingFileSelection"
    },
    "contents": {
      "type": "string"
    }
  },
  "required": [
    "selection",
    "contents"
  ]
}
```

## WorkingFileSelection

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "generation": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "workspace": {
      "type": "string"
    },
    "path": {
      "type": "string"
    },
    "fingerprint": {
      "type": "string"
    }
  },
  "required": [
    "version",
    "repository",
    "generation",
    "workspace",
    "path",
    "fingerprint"
  ]
}
```

## ObservedWorkingTree

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "snapshot": {
      "$ref": "#/components/schemas/WorkingTreeSnapshot"
    },
    "changes": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/WorkingTreeChange"
      },
      "maxItems": 4096
    },
    "obstacle_count": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    }
  },
  "required": [
    "snapshot",
    "changes",
    "obstacle_count"
  ]
}
```

## WorkingTreeSnapshot

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "binding_generation": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "view": {
      "$ref": "#/components/schemas/ViewId"
    },
    "workspace": {
      "type": "string"
    },
    "working_copy": {
      "$ref": "#/components/schemas/GitSha1Id"
    },
    "trees": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/GitSha1Id"
      }
    },
    "tree_labels": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "physical_workspace": {
      "$ref": "#/components/schemas/RecordDigest"
    },
    "metadata_fingerprint": {
      "$ref": "#/components/schemas/RecordDigest"
    },
    "physical_fingerprint": {
      "$ref": "#/components/schemas/RecordDigest"
    }
  },
  "required": [
    "version",
    "repository",
    "binding_generation",
    "operation",
    "view",
    "workspace",
    "working_copy",
    "trees",
    "tree_labels",
    "physical_workspace",
    "metadata_fingerprint",
    "physical_fingerprint"
  ]
}
```

## WorkingTreeChange

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "path": {
      "type": "string"
    },
    "status": {
      "$ref": "#/components/schemas/WorkingTreeChangeStatus"
    },
    "before": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/WorkingTreeFileState"
        },
        {
          "type": "null"
        }
      ]
    },
    "after": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/WorkingTreeFileState"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "path",
    "status",
    "before",
    "after"
  ]
}
```

## WorkingTreeChangeStatus

```text
{
  "type": "string",
  "enum": [
    "added",
    "modified",
    "deleted"
  ]
}
```

## WorkingTreeFileState

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "kind": {
      "$ref": "#/components/schemas/WorkingTreeFileKind"
    },
    "bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "content_digest": {
      "$ref": "#/components/schemas/RecordDigest"
    }
  },
  "required": [
    "kind",
    "bytes",
    "content_digest"
  ]
}
```

## WorkingTreeFileKind

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "executable": {
          "type": "boolean"
        },
        "type": {
          "type": "string",
          "const": "file"
        }
      },
      "required": [
        "type",
        "executable"
      ]
    },
    {
      "type": "object",
      "properties": {
        "type": {
          "type": "string",
          "const": "symlink"
        }
      },
      "required": [
        "type"
      ],
      "additionalProperties": false
    }
  ]
}
```

## LocalStacksTopologyResponse

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "target": {
      "type": "string"
    },
    "target_commit": {
      "type": "string"
    },
    "topology": {
      "$ref": "#/components/schemas/LocalStacksTopology"
    }
  },
  "required": [
    "version",
    "operation",
    "target",
    "target_commit",
    "topology"
  ]
}
```

## LocalStacksTopology

```text
{
  "type": "object",
  "properties": {
    "lanes": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalStacksTopologyLane"
      }
    },
    "integrated": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "complete": {
      "type": "boolean"
    },
    "inventory": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalStacksTopologyLane"
      },
      "default": []
    }
  },
  "required": [
    "lanes",
    "integrated",
    "complete",
    "inventory"
  ]
}
```

## LocalStacksTopologyLane

```text
{
  "type": "object",
  "properties": {
    "heads": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalStacksTopologyHead"
      },
      "description": "Newest first. Names on one tip are aliases, not additional segments."
    },
    "applied": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "unapply": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "description": "Explicit members removable without changing another active lane's membership.",
      "default": []
    }
  },
  "required": [
    "heads",
    "applied",
    "unapply"
  ]
}
```

## LocalStacksTopologyHead

```text
{
  "type": "object",
  "properties": {
    "names": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "commit": {
      "type": "string"
    },
    "base_commit": {
      "type": [
        "string",
        "null"
      ],
      "description": "Exact lower bookmark or first excluded target ancestor; None at ambiguous merges.",
      "default": null
    },
    "shared": {
      "type": "boolean",
      "default": false
    }
  },
  "required": [
    "names",
    "commit",
    "base_commit",
    "shared"
  ]
}
```

## LocalStacksConflict

```text
{
  "type": "object",
  "properties": {
    "source": {
      "type": "string"
    },
    "path": {
      "type": "string"
    },
    "terms": {
      "type": "array",
      "items": {
        "type": [
          "string",
          "null"
        ]
      }
    },
    "reason": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "source",
    "path",
    "terms",
    "reason"
  ]
}
```

## LocalStacksHunkResponse

```text
{
  "type": "object",
  "properties": {
    "source": {
      "type": "string"
    },
    "path": {
      "type": "string"
    },
    "hunks": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalStacksHunk"
      }
    },
    "reason": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "source",
    "path",
    "hunks",
    "reason"
  ]
}
```

## LocalStacksHunk

```text
{
  "type": "object",
  "properties": {
    "id": {
      "type": "string"
    },
    "label": {
      "type": "string"
    },
    "patch": {
      "type": "string"
    }
  },
  "required": [
    "id",
    "label",
    "patch"
  ]
}
```

## LocalStacksInspection

```text
{
  "type": "object",
  "properties": {
    "commits": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalStacksCommitEligibility"
      }
    }
  },
  "required": [
    "commits"
  ]
}
```

## LocalStacksCommitEligibility

```text
{
  "type": "object",
  "properties": {
    "commit": {
      "type": "string"
    },
    "mutable": {
      "type": "boolean"
    },
    "reason": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "commit",
    "mutable",
    "reason"
  ]
}
```

## LocalStacksState

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "active": {
      "type": [
        "string",
        "null"
      ]
    },
    "workspaces": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/LocalStacksWorkspace"
      }
    },
    "warning": {
      "type": [
        "string",
        "null"
      ],
      "description": "Publication succeeded but settings/checkout need attention. Never retry the command."
    },
    "can_undo": {
      "type": "boolean",
      "default": false
    },
    "can_redo": {
      "type": "boolean",
      "default": false
    },
    "conflicts": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "default": []
    },
    "topology": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/LocalStacksTopology"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "operation",
    "revision",
    "active",
    "workspaces",
    "warning",
    "can_undo",
    "can_redo",
    "conflicts"
  ]
}
```

## LocalStacksWorkspace

```text
{
  "type": "object",
  "properties": {
    "id": {
      "type": "string"
    },
    "name": {
      "type": "string"
    },
    "target": {
      "type": "string"
    },
    "target_commit": {
      "type": "string"
    },
    "applied": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "merge_commit": {
      "type": "string"
    },
    "wip_commit": {
      "type": "string"
    },
    "conflicted": {
      "type": "boolean"
    },
    "unavailable_reason": {
      "type": [
        "string",
        "null"
      ]
    },
    "membership_changed": {
      "type": "boolean",
      "default": false
    }
  },
  "required": [
    "id",
    "name",
    "target",
    "target_commit",
    "applied",
    "merge_commit",
    "wip_commit",
    "conflicted",
    "membership_changed"
  ]
}
```

## LocalStacksPreview

```text
{
  "type": "object",
  "properties": {
    "title": {
      "type": "string"
    },
    "summary": {
      "type": "string"
    },
    "paths": {
      "type": "array",
      "items": {
        "type": "string"
      }
    }
  },
  "required": [
    "title",
    "summary",
    "paths"
  ]
}
```

## ConversionPreview

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "prepared_source": {
      "type": [
        "string",
        "null"
      ]
    },
    "source_bytes": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "history_selections": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "blocker": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "prepared_source",
    "source_bytes",
    "history_selections",
    "blocker"
  ]
}
```

## ConversionOutcome

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "report": {
          "$ref": "#/components/schemas/ConversionReport"
        },
        "status": {
          "type": "string",
          "const": "committed"
        }
      },
      "required": [
        "status",
        "report"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "outcome_unknown"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    },
    {
      "type": "object",
      "properties": {
        "request_id": {
          "$ref": "#/components/schemas/RequestId"
        },
        "status": {
          "type": "string",
          "const": "not_known"
        }
      },
      "required": [
        "status",
        "request_id"
      ]
    }
  ]
}
```

## ConversionReport

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "source": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "destination": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "destination_profile": {
      "$ref": "#/components/schemas/ConversionProfile"
    },
    "name": {
      "type": "string"
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "archive_digest": {
      "type": "string"
    },
    "code_mappings": {
      "type": "object",
      "additionalProperties": {
        "type": "object",
        "additionalProperties": {
          "type": "string"
        }
      }
    },
    "collaboration": {
      "$ref": "#/components/schemas/CollaborationMappingsV1"
    }
  },
  "required": [
    "version",
    "request_id",
    "source",
    "destination",
    "destination_profile",
    "name",
    "operation",
    "archive_digest",
    "code_mappings",
    "collaboration"
  ]
}
```

## ConversionProfile

```text
{
  "type": "string",
  "enum": [
    "native",
    "local_jj_git"
  ]
}
```

## CollaborationMappingsV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "selections": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/SelectionMappingV1"
      }
    },
    "objects": {
      "type": "object",
      "additionalProperties": {
        "type": "string"
      }
    }
  },
  "required": [
    "selections",
    "objects"
  ]
}
```

## SelectionMappingV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "source": {
      "type": "string"
    },
    "destination": {
      "type": "string"
    },
    "versions": {
      "type": "object",
      "additionalProperties": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0
      }
    }
  },
  "required": [
    "source",
    "destination",
    "versions"
  ]
}
```

## LocalFreshnessEvent

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "snapshot": {
          "$ref": "#/components/schemas/LocalRepositorySnapshot"
        },
        "type": {
          "type": "string",
          "const": "ready"
        }
      },
      "required": [
        "type",
        "snapshot"
      ]
    },
    {
      "type": "object",
      "properties": {
        "snapshot": {
          "$ref": "#/components/schemas/LocalRepositorySnapshot"
        },
        "type": {
          "type": "string",
          "const": "changed"
        }
      },
      "required": [
        "type",
        "snapshot"
      ]
    }
  ]
}
```

## LocalCaptureSettings

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "revision": {
      "type": "string",
      "minLength": 64,
      "maxLength": 64,
      "pattern": "^[0-9a-f]{64}$"
    },
    "desired": {
      "$ref": "#/components/schemas/CaptureConfiguration"
    },
    "effective": {
      "$ref": "#/components/schemas/CaptureConfiguration"
    },
    "restart_required": {
      "type": "boolean"
    },
    "defaults": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CaptureHarnessSetting"
      },
      "description": "Resolved process defaults, also available before capture is enabled.",
      "maxItems": 5
    }
  },
  "required": [
    "version",
    "revision",
    "desired",
    "effective",
    "restart_required",
    "defaults"
  ]
}
```

## CaptureConfiguration

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "enabled": {
      "type": "boolean"
    },
    "max_readers": {
      "type": "integer",
      "format": "uint",
      "minimum": 1,
      "maximum": 4
    },
    "rescan_seconds": {
      "type": "integer",
      "format": "uint64",
      "minimum": 1,
      "maximum": 30
    },
    "max_sources": {
      "type": "integer",
      "format": "uint",
      "minimum": 1,
      "maximum": 4096
    },
    "max_pending_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 1,
      "maximum": 67108864
    },
    "harnesses": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CaptureHarnessSetting"
      },
      "maxItems": 5
    }
  },
  "required": [
    "enabled",
    "max_readers",
    "rescan_seconds",
    "max_sources",
    "max_pending_bytes",
    "harnesses"
  ]
}
```

## CaptureHarnessSetting

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "profile": {
      "$ref": "#/components/schemas/CaptureHarnessProfile"
    },
    "product_version": {
      "type": "string",
      "minLength": 1,
      "maxLength": 32
    },
    "roots": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CaptureRootPath"
      },
      "maxItems": 16
    },
    "endpoint": {
      "type": [
        "string",
        "null"
      ],
      "maxLength": 4096
    }
  },
  "required": [
    "profile",
    "product_version",
    "roots"
  ]
}
```

## CaptureHarnessProfile

```text
{
  "type": "string",
  "enum": [
    "codex-cli-jsonl-v1",
    "codex-desktop-jsonl-v1",
    "cursor-desktop-transcript-v1",
    "cursor-cli-transcript-v1",
    "opencode-read-only-api-v1",
    "opencode-export-json-v1"
  ]
}
```

## CaptureRootPath

```text
{
  "type": "string",
  "minLength": 1,
  "maxLength": 4096,
  "description": "Startup-selected directory metadata, not a discovered transcript file."
}
```

## CaptureStatus

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "enabled": {
      "type": "boolean"
    },
    "policy": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CapturePolicy"
        },
        {
          "type": "null"
        }
      ]
    },
    "runtime": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CaptureRuntimeStatus"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "enabled",
    "policy",
    "runtime"
  ]
}
```

## CapturePolicy

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "generation": {
      "type": "string",
      "minLength": 32,
      "maxLength": 32,
      "pattern": "^[0-9a-f]{32}$"
    },
    "revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "paused": {
      "type": "boolean"
    }
  },
  "required": [
    "generation",
    "revision",
    "paused"
  ]
}
```

## CaptureRuntimeStatus

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "phase": {
      "$ref": "#/components/schemas/CaptureRuntimePhase"
    },
    "discovery": {
      "$ref": "#/components/schemas/CaptureDiscoveryStatus"
    },
    "degraded_sources": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "harnesses": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CaptureHarnessCoverage"
      },
      "maxItems": 5
    },
    "pending_capacity_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "ingress_capacity_refusals": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "ingress_required_capacity_bytes": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    },
    "delivery_required_capacity_bytes": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    },
    "tracked": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "observed_sources": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "awaiting_activity": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "caught_up": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "running_sources": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "resident_source_frames": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "resident_delivery_frames": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "pending_batches": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "unavailable": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "source_limit_reached": {
      "type": "boolean"
    }
  },
  "required": [
    "phase",
    "discovery",
    "degraded_sources",
    "harnesses",
    "pending_capacity_bytes",
    "ingress_capacity_refusals",
    "ingress_required_capacity_bytes",
    "delivery_required_capacity_bytes",
    "tracked",
    "observed_sources",
    "awaiting_activity",
    "caught_up",
    "running_sources",
    "resident_source_frames",
    "resident_delivery_frames",
    "pending_batches",
    "unavailable",
    "source_limit_reached"
  ]
}
```

## CaptureRuntimePhase

```text
{
  "type": "string",
  "enum": [
    "ready",
    "running",
    "overdue",
    "draining",
    "paused",
    "stopped"
  ]
}
```

## CaptureDiscoveryStatus

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "phase": {
      "$ref": "#/components/schemas/CaptureRuntimePhase"
    },
    "roots": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "unchecked_roots": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "available_roots": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "unavailable_roots": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "rescan_pending": {
      "type": "boolean"
    }
  },
  "required": [
    "phase",
    "roots",
    "unchecked_roots",
    "available_roots",
    "unavailable_roots",
    "rescan_pending"
  ]
}
```

## CaptureHarnessCoverage

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "configured_roots": {
      "type": [
        "array",
        "null"
      ],
      "items": {
        "$ref": "#/components/schemas/CaptureRootPath"
      },
      "maxItems": 16
    },
    "profile": {
      "$ref": "#/components/schemas/CaptureHarnessProfile"
    },
    "product_version": {
      "type": "string",
      "minLength": 1,
      "maxLength": 32
    },
    "roots": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "unchecked_roots": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "available_roots": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "unavailable_roots": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "tracked": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "observed_sources": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "awaiting_activity": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "caught_up": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "running_sources": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "degraded_sources": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    }
  },
  "required": [
    "profile",
    "product_version",
    "roots",
    "unchecked_roots",
    "available_roots",
    "unavailable_roots",
    "tracked",
    "observed_sources",
    "awaiting_activity",
    "caught_up",
    "running_sources",
    "degraded_sources"
  ]
}
```

## CaptureSourcePage

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "enabled": {
      "type": "boolean"
    },
    "runtime_generation": {
      "type": [
        "string",
        "null"
      ],
      "minLength": 32,
      "maxLength": 32,
      "pattern": "^[0-9a-f]{32}$"
    },
    "phase": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CaptureRuntimePhase"
        },
        {
          "type": "null"
        }
      ]
    },
    "total": {
      "type": "integer",
      "format": "uint",
      "minimum": 0,
      "maximum": 4096
    },
    "sources": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/CaptureSourceCoverage"
      },
      "maxItems": 100
    },
    "next_cursor": {
      "type": [
        "string",
        "null"
      ],
      "minLength": 49,
      "maxLength": 49,
      "pattern": "^[0-9a-f]{32}:[0-9a-f]{16}$"
    }
  },
  "required": [
    "version",
    "enabled",
    "runtime_generation",
    "phase",
    "total",
    "sources",
    "next_cursor"
  ]
}
```

## CaptureSourceCoverage

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "selector": {
      "type": "string",
      "minLength": 49,
      "maxLength": 49,
      "pattern": "^[0-9a-f]{32}:[0-9a-f]{16}$"
    },
    "profile": {
      "$ref": "#/components/schemas/CaptureHarnessProfile"
    },
    "product_version": {
      "type": "string",
      "minLength": 1,
      "maxLength": 32
    },
    "phase": {
      "$ref": "#/components/schemas/CaptureRuntimePhase"
    },
    "observed": {
      "type": "boolean"
    },
    "awaiting_activity": {
      "type": "boolean"
    },
    "caught_up": {
      "type": "boolean"
    },
    "running": {
      "type": "boolean"
    },
    "degraded": {
      "type": "boolean"
    },
    "last_bound_repository": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RepositoryId"
        },
        {
          "type": "null"
        }
      ]
    },
    "ingress_required_capacity_bytes": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    },
    "snapshot_publication": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CaptureSnapshotPublication"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "selector",
    "profile",
    "product_version",
    "phase",
    "observed",
    "awaiting_activity",
    "caught_up",
    "running",
    "degraded",
    "last_bound_repository",
    "ingress_required_capacity_bytes"
  ]
}
```

## CaptureSnapshotPublication

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "history_available": {
      "type": "boolean"
    },
    "captured_sequence": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "captured_logical_records": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "committed_sequence": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "committed_logical_records": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    },
    "outstanding_ranges": {
      "type": "integer",
      "format": "uint",
      "minimum": 0,
      "maximum": 4096
    },
    "receipts": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "history_available",
    "captured_sequence",
    "captured_logical_records",
    "committed_sequence",
    "committed_logical_records",
    "outstanding_ranges",
    "receipts"
  ],
  "description": "Source coverage only; no import completion, execution identity or current eligibility."
}
```

## CaptureControlOutcome

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "receipt": {
      "$ref": "#/components/schemas/CapturePolicyReceipt"
    },
    "replayed": {
      "type": "boolean"
    },
    "runtime_requested": {
      "type": "boolean"
    },
    "status": {
      "$ref": "#/components/schemas/CaptureStatus"
    }
  },
  "required": [
    "version",
    "receipt",
    "replayed",
    "runtime_requested",
    "status"
  ]
}
```

## CapturePolicyReceipt

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "generation": {
      "type": "string"
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "expected_revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "paused": {
      "type": "boolean"
    }
  },
  "required": [
    "generation",
    "request_id",
    "expected_revision",
    "revision",
    "paused"
  ]
}
```

## CaptureRescanOutcome

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "accepted": {
      "type": "boolean"
    },
    "status": {
      "$ref": "#/components/schemas/CaptureStatus"
    }
  },
  "required": [
    "version",
    "accepted",
    "status"
  ]
}
```

## CaptureImportOutcome

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "replayed": {
      "type": "boolean"
    },
    "receipt": {
      "$ref": "#/components/schemas/CaptureImportAcceptance"
    },
    "progress": {
      "$ref": "#/components/schemas/CaptureImportProgress"
    }
  },
  "required": [
    "version",
    "replayed",
    "receipt",
    "progress"
  ]
}
```

## CaptureImportAcceptance

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "command": {
      "$ref": "#/components/schemas/CaptureImportCommand"
    },
    "ordinal": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "profile": {
      "$ref": "#/components/schemas/CaptureHarnessProfile"
    },
    "product_version": {
      "type": "string",
      "minLength": 1,
      "maxLength": 32
    },
    "complete_byte_offset": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "complete_records": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "command",
    "ordinal",
    "profile",
    "product_version",
    "complete_byte_offset",
    "complete_records"
  ]
}
```

## CaptureImportCommand

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "generation": {
      "type": "string",
      "minLength": 32,
      "maxLength": 32,
      "pattern": "^[0-9a-f]{32}$"
    },
    "expected_revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "selector": {
      "type": "string",
      "minLength": 49,
      "maxLength": 49,
      "pattern": "^[0-9a-f]{32}:[0-9a-f]{16}$"
    }
  },
  "required": [
    "version",
    "request_id",
    "generation",
    "expected_revision",
    "selector"
  ]
}
```

## CaptureImportProgress

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "phase": {
      "$ref": "#/components/schemas/CaptureImportPhase"
    },
    "boundary_available": {
      "type": "boolean"
    },
    "captured_byte_offset": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "committed_byte_offset": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "tail_phase": {
      "$ref": "#/components/schemas/CaptureImportPhase"
    },
    "tail": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CaptureImportTail"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "phase",
    "boundary_available",
    "captured_byte_offset",
    "committed_byte_offset",
    "tail_phase",
    "tail"
  ]
}
```

## CaptureImportPhase

```text
{
  "type": "string",
  "enum": [
    "selected",
    "pending",
    "running",
    "withheld",
    "completed",
    "unavailable"
  ]
}
```

## CaptureImportTail

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "history_available": {
      "type": "boolean"
    },
    "captured_byte_offset": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "committed_byte_offset": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "outstanding_ranges": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "receipts": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "history_available",
    "captured_byte_offset",
    "committed_byte_offset",
    "outstanding_ranges",
    "receipts"
  ]
}
```

## CaptureImportStatus

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "receipt": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CaptureImportAcceptance"
        },
        {
          "type": "null"
        }
      ]
    },
    "progress": {
      "$ref": "#/components/schemas/CaptureImportProgress"
    }
  },
  "required": [
    "version",
    "request_id",
    "receipt",
    "progress"
  ]
}
```

## ThreadWorkspaceOutcome

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "home": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "thread": {
      "$ref": "#/components/schemas/ThreadId"
    },
    "revision": {
      "$ref": "#/components/schemas/CollabObjectRef"
    }
  },
  "required": [
    "request_id",
    "home",
    "thread",
    "revision"
  ]
}
```

## AgentWorkspaceInspection

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "session": {
      "type": "string"
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "workspace": {
      "type": "string"
    },
    "location": {
      "type": "string"
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "commit": {
      "type": "string"
    },
    "selection": {
      "type": "string"
    },
    "references": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/WorkspaceReference"
      }
    },
    "comparison": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentWorkspaceComparison"
        },
        {
          "type": "null"
        }
      ]
    },
    "incomplete": {
      "type": [
        "string",
        "null"
      ]
    },
    "files": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentWorkspaceFileEntry"
      }
    },
    "next": {
      "type": [
        "string",
        "null"
      ]
    },
    "file": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentWorkspaceFileContent"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "session",
    "repository",
    "workspace",
    "location",
    "operation",
    "commit",
    "selection",
    "references",
    "comparison",
    "incomplete",
    "files",
    "next",
    "file"
  ]
}
```

## WorkspaceReference

```text
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "commit": {
      "type": "string"
    }
  },
  "required": [
    "name",
    "commit"
  ]
}
```

## AgentWorkspaceComparison

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "range": {
      "$ref": "#/components/schemas/ComparisonRange"
    },
    "before_operation": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/OperationId"
        },
        {
          "type": "null"
        }
      ]
    },
    "before_commit": {
      "type": [
        "string",
        "null"
      ]
    },
    "after_operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "after_commit": {
      "type": "string"
    },
    "current_files": {
      "type": "boolean"
    },
    "request_id": {
      "type": [
        "string",
        "null"
      ]
    },
    "unavailable": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "range",
    "before_operation",
    "before_commit",
    "after_operation",
    "after_commit",
    "current_files",
    "request_id",
    "unavailable"
  ]
}
```

## ComparisonRange

```text
{
  "type": "string",
  "enum": [
    "since_last_commit",
    "last_turn",
    "branch"
  ]
}
```

## AgentWorkspaceFileEntry

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "path": {
      "type": "string"
    },
    "kind": {
      "type": "string"
    },
    "bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "fingerprint": {
      "type": "string"
    },
    "status": {
      "type": [
        "string",
        "null"
      ]
    },
    "additions": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    },
    "deletions": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "path",
    "kind",
    "bytes",
    "fingerprint",
    "status"
  ]
}
```

## AgentWorkspaceFileContent

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "path": {
      "type": "string"
    },
    "fingerprint": {
      "type": "string"
    },
    "contents": {
      "type": [
        "string",
        "null"
      ]
    },
    "before": {
      "type": [
        "string",
        "null"
      ]
    },
    "unavailable": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "path",
    "fingerprint",
    "contents",
    "before",
    "unavailable"
  ]
}
```

## ComputeStatus

```text
{
  "type": "object",
  "properties": {
    "hardware": true,
    "supported": {
      "type": "boolean"
    },
    "model_file_picker": {
      "type": "boolean",
      "default": false
    },
    "state": {
      "type": "string"
    },
    "settings": {
      "$ref": "#/components/schemas/ComputeSettings"
    },
    "models": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ComputeModel"
      }
    },
    "runtime_prepared": {
      "type": "boolean"
    },
    "invitation": true,
    "identity": {
      "type": [
        "string",
        "null"
      ]
    },
    "progress": true,
    "error": {
      "type": [
        "string",
        "null"
      ]
    },
    "operation": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentExecutionOutcome"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "hardware",
    "supported",
    "model_file_picker",
    "state",
    "settings",
    "models",
    "runtime_prepared",
    "invitation",
    "identity",
    "progress",
    "error",
    "operation"
  ]
}
```

## ComputeSettings

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0,
      "default": 0
    },
    "model": {
      "type": [
        "string",
        "null"
      ],
      "default": null
    },
    "memory_gb": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0,
      "default": null
    },
    "sharing": {
      "type": "boolean",
      "default": false
    },
    "peers": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/ComputePeer"
      },
      "default": {}
    },
    "relay_urls": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "default": []
    },
    "target_peer": {
      "type": [
        "string",
        "null"
      ],
      "default": null
    }
  },
  "required": [
    "revision",
    "model",
    "memory_gb",
    "sharing",
    "peers",
    "relay_urls",
    "target_peer"
  ]
}
```

## ComputePeer

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "name": {
      "type": "string"
    },
    "invitation": true
  },
  "required": [
    "name",
    "invitation"
  ]
}
```

## ComputeModel

```text
{
  "type": "object",
  "properties": {
    "id": {
      "type": "string"
    },
    "name": {
      "type": "string"
    },
    "installed": {
      "type": "boolean"
    }
  },
  "required": [
    "id",
    "name",
    "installed"
  ]
}
```

## AgentExecutionOutcome

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "comment_publication": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentCommentPublication"
        },
        {
          "type": "null"
        }
      ]
    },
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "session_id": {
      "type": [
        "string",
        "null"
      ]
    },
    "state": {
      "type": "string"
    },
    "error": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "request_id",
    "session_id",
    "state",
    "error"
  ]
}
```

## AgentCommentPublication

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "state": {
      "type": "string"
    },
    "batch": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "request_id",
    "state",
    "batch"
  ]
}
```

## ModelFileSelection

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "path": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "path"
  ]
}
```

## AgentChatGptStatus

```text
{
  "type": "object",
  "properties": {
    "supported": {
      "type": "boolean"
    },
    "accounts": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentChatGptAccount"
      }
    },
    "attempts": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentChatGptAttempt"
      }
    }
  },
  "required": [
    "supported",
    "accounts",
    "attempts"
  ]
}
```

## AgentChatGptAccount

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "id": {
      "type": "string"
    },
    "label": {
      "type": "string"
    },
    "email": {
      "type": [
        "string",
        "null"
      ]
    },
    "connected": {
      "type": "boolean"
    },
    "plan_enabled": {
      "type": "boolean"
    }
  },
  "required": [
    "id",
    "label",
    "email",
    "connected",
    "plan_enabled"
  ]
}
```

## AgentChatGptAttempt

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "id": {
      "type": "string"
    },
    "account": {
      "type": "string"
    },
    "state": {
      "type": "string"
    },
    "error": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "id",
    "account",
    "state",
    "error"
  ]
}
```

## AgentSettings

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0,
      "default": 0
    },
    "runner": {
      "type": "string",
      "default": "codex"
    },
    "preferences": {
      "default": {
        "model": null,
        "effort": null,
        "mode": null
      },
      "allOf": [
        {
          "$ref": "#/components/schemas/AgentExecutionPreferences"
        }
      ]
    },
    "options": {
      "default": {
        "inherit_defaults": false,
        "provider": null,
        "endpoint": null,
        "environment": {}
      },
      "allOf": [
        {
          "$ref": "#/components/schemas/AgentRuntimeOptions"
        }
      ]
    },
    "automatically_mention": {
      "type": "boolean",
      "default": false
    },
    "keep_awake": {
      "type": "boolean",
      "default": false
    },
    "keychain_supported": {
      "type": "boolean",
      "default": false
    },
    "metadata": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ConversationMetadataConfig"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "revision",
    "runner",
    "preferences",
    "options",
    "automatically_mention",
    "keep_awake",
    "keychain_supported"
  ]
}
```

## AgentExecutionPreferences

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "model": {
      "type": [
        "string",
        "null"
      ]
    },
    "effort": {
      "type": [
        "string",
        "null"
      ]
    },
    "mode": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "model",
    "effort",
    "mode"
  ]
}
```

## AgentRuntimeOptions

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "inherit_defaults": {
      "type": "boolean",
      "default": false
    },
    "provider": {
      "type": [
        "string",
        "null"
      ],
      "default": null
    },
    "endpoint": {
      "type": [
        "string",
        "null"
      ],
      "default": null
    },
    "account": {
      "type": [
        "string",
        "null"
      ]
    },
    "environment": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/AgentEnvironmentValue"
      },
      "default": {}
    }
  },
  "required": [
    "inherit_defaults",
    "provider",
    "endpoint",
    "environment"
  ]
}
```

## AgentEnvironmentValue

```text
{
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "source": {
          "type": "string",
          "const": "literal"
        },
        "value": {
          "type": "string"
        }
      },
      "required": [
        "source",
        "value"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "source": {
          "type": "string",
          "const": "environment"
        },
        "value": {
          "type": "string"
        }
      },
      "required": [
        "source",
        "value"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "source": {
          "type": "string",
          "const": "keychain"
        },
        "value": {
          "type": "string"
        }
      },
      "required": [
        "source",
        "value"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "source": {
          "type": "string",
          "const": "unset"
        }
      },
      "required": [
        "source"
      ],
      "additionalProperties": false
    }
  ]
}
```

## ConversationMetadataConfig

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "model": {
      "type": "string"
    },
    "options": {
      "$ref": "#/components/schemas/AgentRuntimeOptions"
    }
  },
  "required": [
    "model",
    "options"
  ]
}
```

## AgentSecretStatus

```text
{
  "type": "object",
  "properties": {
    "keychain_supported": {
      "type": "boolean"
    }
  },
  "required": [
    "keychain_supported"
  ]
}
```

## AgentExecutionCatalog

```text
{
  "type": "object",
  "properties": {
    "status": {
      "type": [
        "string",
        "null"
      ],
      "default": null
    },
    "runner": {
      "type": "string"
    },
    "facts": true
  },
  "required": [
    "status",
    "runner",
    "facts"
  ]
}
```

## AgentExecutionStatus

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "runtime": {
      "type": "string"
    },
    "revision": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "setups": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentExecutionSetup"
      }
    },
    "sessions": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentExecutionSession"
      }
    },
    "runners": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentExecutionRunner"
      }
    },
    "live_agents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentFleetStatus"
      },
      "default": []
    },
    "conversations": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentConversation"
      },
      "default": []
    }
  },
  "required": [
    "version",
    "runtime",
    "revision",
    "setups",
    "sessions",
    "runners",
    "live_agents",
    "conversations"
  ]
}
```

## AgentExecutionSetup

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "id": {
      "type": "string"
    },
    "name": {
      "type": "string"
    },
    "avatar": {
      "type": "string"
    },
    "description": {
      "type": "string",
      "default": ""
    },
    "instructions": {
      "type": "string"
    },
    "runner": {
      "type": "string"
    },
    "preferences": {
      "$ref": "#/components/schemas/AgentExecutionPreferences"
    },
    "options": {
      "default": {
        "inherit_defaults": false,
        "provider": null,
        "endpoint": null,
        "environment": {}
      },
      "allOf": [
        {
          "$ref": "#/components/schemas/AgentRuntimeOptions"
        }
      ]
    }
  },
  "required": [
    "id",
    "name",
    "avatar",
    "description",
    "instructions",
    "runner",
    "preferences",
    "options"
  ]
}
```

## AgentExecutionSession

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "id": {
      "type": "string"
    },
    "conversation_id": {
      "type": [
        "string",
        "null"
      ],
      "default": null
    },
    "repository_bindings": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentConversationWorkspace"
      },
      "default": []
    },
    "setup_id": {
      "type": [
        "string",
        "null"
      ],
      "default": null
    },
    "recorded_session_ids": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentSessionId"
      },
      "description": "Read-only capture-normalizer aliases for this installation-owned session.",
      "default": []
    },
    "workspace_name": {
      "type": [
        "string",
        "null"
      ],
      "description": "Read-only name from the retained ready workspace proof, absent after cleanup.",
      "default": null
    },
    "workspace_operation": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/OperationId"
        },
        {
          "type": "null"
        }
      ],
      "description": "Operation from that ready workspace proof, including creation before the first turn.",
      "default": null
    },
    "connected": {
      "type": "boolean",
      "description": "Read-only live supervisor projection, never journal authority.",
      "default": false
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "target_id": {
      "type": "string"
    },
    "runner": {
      "type": "string"
    },
    "title": {
      "type": "string"
    },
    "conversation_title": {
      "type": [
        "string",
        "null"
      ]
    },
    "discovery_text": {
      "type": [
        "string",
        "null"
      ]
    },
    "avatar": {
      "type": "string"
    },
    "workspace": {
      "type": "string"
    },
    "base_commit": {
      "type": "string"
    },
    "review_commit": {
      "type": "string"
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "state": {
      "type": "string"
    },
    "adapter_session_id": {
      "type": [
        "string",
        "null"
      ]
    },
    "capabilities": true,
    "preferences": {
      "$ref": "#/components/schemas/AgentExecutionPreferences"
    },
    "last_sequence": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "last_activity_at_millis": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0,
      "description": "Device observation time of the latest retained execution event, if known.",
      "default": null
    },
    "published_sequence": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "last_error": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "id",
    "conversation_id",
    "repository_bindings",
    "setup_id",
    "recorded_session_ids",
    "workspace_name",
    "workspace_operation",
    "connected",
    "repository",
    "target_id",
    "runner",
    "title",
    "avatar",
    "workspace",
    "base_commit",
    "review_commit",
    "operation",
    "state",
    "adapter_session_id",
    "capabilities",
    "preferences",
    "last_sequence",
    "last_activity_at_millis",
    "published_sequence",
    "last_error"
  ]
}
```

## AgentConversationWorkspace

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "target_id": {
      "type": "string"
    },
    "profile": {
      "type": "string"
    },
    "workspace": {
      "type": "string"
    },
    "workspace_name": {
      "type": [
        "string",
        "null"
      ],
      "description": "Derived from the retained owned workspace, never from a path basename."
    },
    "selected_operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "base_commit": {
      "type": "string"
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "commit": {
      "type": "string"
    }
  },
  "required": [
    "repository",
    "target_id",
    "profile",
    "workspace",
    "selected_operation",
    "base_commit",
    "operation",
    "commit"
  ]
}
```

## AgentExecutionRunner

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "providers": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentProvider"
      },
      "default": []
    },
    "id": {
      "type": "string"
    },
    "label": {
      "type": "string"
    },
    "available": {
      "type": "boolean"
    },
    "qualified": {
      "type": "boolean"
    },
    "install_hint": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "providers",
    "id",
    "label",
    "available",
    "qualified",
    "install_hint"
  ]
}
```

## AgentProvider

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "id": {
      "type": "string"
    },
    "label": {
      "type": "string"
    },
    "runtime_provider": {
      "type": "string"
    },
    "endpoint_env": {
      "type": "string"
    },
    "default_endpoint": {
      "type": [
        "string",
        "null"
      ]
    },
    "endpoint_required": {
      "type": "boolean"
    },
    "credential_env": {
      "type": [
        "string",
        "null"
      ]
    },
    "model_required": {
      "type": "boolean"
    },
    "managed": {
      "type": "boolean"
    },
    "guidance": {
      "type": "string"
    },
    "authentication": {
      "type": [
        "string",
        "null"
      ]
    },
    "available": {
      "type": [
        "boolean",
        "null"
      ]
    }
  },
  "required": [
    "id",
    "label",
    "runtime_provider",
    "endpoint_env",
    "default_endpoint",
    "endpoint_required",
    "credential_env",
    "model_required",
    "managed",
    "guidance"
  ]
}
```

## AgentFleetStatus

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "agent_id": {
      "type": "string"
    },
    "state": {
      "type": "string"
    },
    "enabled": {
      "type": "boolean"
    },
    "generation": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "error": {
      "type": [
        "string",
        "null"
      ]
    },
    "restart_required": {
      "type": "boolean",
      "default": false
    },
    "inference_host": {
      "type": [
        "string",
        "null"
      ],
      "default": null
    }
  },
  "required": [
    "agent_id",
    "state",
    "enabled",
    "generation",
    "error",
    "restart_required",
    "inference_host"
  ]
}
```

## AgentConversation

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "id": {
      "type": "string"
    },
    "origin": {
      "$ref": "#/components/schemas/AgentConversationOrigin"
    },
    "home": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "discussion_thread": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ThreadId"
        },
        {
          "type": "null"
        }
      ],
      "default": null
    },
    "participants": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "metadata_auto_eligible": {
      "type": "boolean"
    }
  },
  "required": [
    "id",
    "origin",
    "home",
    "discussion_thread",
    "participants"
  ]
}
```

## AgentConversationOrigin

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "scope": {
      "type": "string"
    },
    "home": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RepositoryId"
        },
        {
          "type": "null"
        }
      ]
    },
    "thread": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ThreadId"
        },
        {
          "type": "null"
        }
      ]
    },
    "agent_target": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ReactionTargetV1"
        },
        {
          "type": "null"
        }
      ],
      "default": null
    },
    "context": {
      "type": "string"
    }
  },
  "required": [
    "scope",
    "home",
    "thread",
    "agent_target",
    "context"
  ]
}
```

## AgentRuntimeStatus

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "runtimes": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentRuntime"
      }
    },
    "node_version": {
      "type": [
        "string",
        "null"
      ]
    },
    "npm_available": {
      "type": "boolean"
    },
    "install_supported": {
      "type": "boolean"
    },
    "install_unavailable": {
      "type": [
        "string",
        "null"
      ]
    },
    "pending": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentRuntimeOutcome"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "runtimes",
    "node_version",
    "npm_available",
    "install_supported",
    "install_unavailable",
    "pending"
  ]
}
```

## AgentRuntime

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "id": {
      "type": "string"
    },
    "label": {
      "type": "string"
    },
    "state": {
      "type": "string"
    },
    "adapter_version": {
      "type": [
        "string",
        "null"
      ]
    },
    "cli_version": {
      "type": [
        "string",
        "null"
      ]
    },
    "managed": {
      "type": "boolean"
    },
    "action": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentRuntimeAction"
        },
        {
          "type": "null"
        }
      ]
    },
    "detail": {
      "type": "string"
    },
    "guide_url": {
      "type": "string"
    },
    "target_version": {
      "type": [
        "string",
        "null"
      ]
    },
    "target_cli_version": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "id",
    "label",
    "state",
    "adapter_version",
    "cli_version",
    "managed",
    "action",
    "detail",
    "guide_url",
    "target_version",
    "target_cli_version"
  ]
}
```

## AgentRuntimeAction

```text
{
  "type": "string",
  "enum": [
    "install",
    "update"
  ]
}
```

## AgentRuntimeOutcome

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "runtime_id": {
      "type": "string"
    },
    "state": {
      "type": "string"
    },
    "error": {
      "type": [
        "string",
        "null"
      ]
    }
  },
  "required": [
    "version",
    "request_id",
    "runtime_id",
    "state",
    "error"
  ]
}
```

## AgentExecutionTarget

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "id": {
      "type": "string"
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "name": {
      "type": "string"
    },
    "folder": {
      "type": "string"
    },
    "profile": {
      "type": "string"
    },
    "operation": {
      "$ref": "#/components/schemas/OperationId"
    },
    "commit": {
      "type": "string"
    }
  },
  "required": [
    "id",
    "repository",
    "name",
    "folder",
    "profile",
    "operation",
    "commit"
  ]
}
```

## ConversationMetadataView

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "target": {
      "$ref": "#/components/schemas/ConversationMetadataTargetV1"
    },
    "thread": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ThreadId"
        },
        {
          "type": "null"
        }
      ]
    },
    "revision": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ]
    },
    "value": {
      "$ref": "#/components/schemas/ConversationMetadataValueV1"
    },
    "sources": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ConversationMetadataSourceV1"
      }
    },
    "title": {
      "type": [
        "string",
        "null"
      ]
    },
    "fresh": {
      "type": "boolean"
    },
    "writable": {
      "type": "boolean"
    },
    "unavailable": {
      "type": [
        "string",
        "null"
      ]
    },
    "details": true,
    "job": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ConversationMetadataJob"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "version",
    "repository",
    "target",
    "thread",
    "revision",
    "value",
    "sources",
    "title",
    "fresh",
    "writable",
    "unavailable",
    "details",
    "job"
  ]
}
```

## ConversationMetadataValueV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "custom_title": {
      "type": [
        "string",
        "null"
      ],
      "default": null
    },
    "generated": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GeneratedConversationMetadataV1"
        },
        {
          "type": "null"
        }
      ],
      "default": null
    },
    "agent_titles": {
      "type": "object",
      "additionalProperties": {
        "type": "string"
      },
      "default": {}
    },
    "agent_title_sources": {
      "type": "object",
      "additionalProperties": {
        "$ref": "#/components/schemas/ConversationMetadataSourceV1"
      }
    }
  },
  "required": [
    "custom_title",
    "generated",
    "agent_titles"
  ]
}
```

## GeneratedConversationMetadataV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "title": {
      "type": "string"
    },
    "summary": {
      "type": "string"
    },
    "keywords": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "goals": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "sources": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ConversationMetadataSourceV1"
      }
    },
    "provider": {
      "type": "string"
    },
    "model": {
      "type": "string"
    },
    "prompt_version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "generated_at_millis": {
      "type": "integer",
      "format": "int64"
    },
    "truncated": {
      "type": "boolean"
    }
  },
  "required": [
    "title",
    "summary",
    "keywords",
    "goals",
    "sources",
    "provider",
    "model",
    "prompt_version",
    "generated_at_millis",
    "truncated"
  ]
}
```

## ConversationMetadataSourceV1

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "session": {
      "$ref": "#/components/schemas/AgentSessionId"
    },
    "snapshot": {
      "$ref": "#/components/schemas/AgentObjectRef"
    }
  },
  "required": [
    "session",
    "snapshot"
  ]
}
```

## ConversationMetadataJob

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "state": {
      "type": "string"
    },
    "error": {
      "type": [
        "string",
        "null"
      ]
    },
    "result": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/GeneratedConversationMetadataV1"
        },
        {
          "type": "null"
        }
      ]
    },
    "original_command": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/ConversationMetadataCommand"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "request_id",
    "state",
    "error",
    "result"
  ]
}
```

## ConversationMetadataCommand

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "query": {
      "$ref": "#/components/schemas/ConversationMetadataQuery"
    },
    "expected": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/CollabObjectRef"
        },
        {
          "type": "null"
        }
      ]
    },
    "action": {
      "$ref": "#/components/schemas/Action"
    }
  },
  "required": [
    "version",
    "request_id",
    "query",
    "expected",
    "action"
  ]
}
```

## ConversationMetadataQuery

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "session": {
      "type": "string"
    },
    "repository": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/RepositoryId"
        },
        {
          "type": "null"
        }
      ]
    }
  },
  "required": [
    "session",
    "repository"
  ]
}
```

## Action

```text
{
  "oneOf": [
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "title": {
          "type": "string"
        },
        "action": {
          "type": "string",
          "const": "rename"
        }
      },
      "required": [
        "action",
        "title"
      ]
    },
    {
      "type": "object",
      "properties": {
        "action": {
          "type": "string",
          "const": "reset_title"
        }
      },
      "required": [
        "action"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "action": {
          "type": "string",
          "const": "capture_titles"
        }
      },
      "required": [
        "action"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "action": {
          "type": "string",
          "const": "refresh"
        }
      },
      "required": [
        "action"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "action": {
          "type": "string",
          "const": "regenerate_title"
        }
      },
      "required": [
        "action"
      ],
      "additionalProperties": false
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "job": {
          "$ref": "#/components/schemas/RequestId"
        },
        "action": {
          "type": "string",
          "const": "cancel"
        }
      },
      "required": [
        "action",
        "job"
      ]
    },
    {
      "type": "object",
      "additionalProperties": false,
      "properties": {
        "job": {
          "$ref": "#/components/schemas/RequestId"
        },
        "action": {
          "type": "string",
          "const": "publish"
        }
      },
      "required": [
        "action",
        "job"
      ]
    }
  ]
}
```

## AgentConversationOutcome

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "request_id": {
      "$ref": "#/components/schemas/RequestId"
    },
    "conversation_id": {
      "type": "string"
    },
    "agents": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentExecutionOutcome"
      }
    },
    "comment_publication": {
      "anyOf": [
        {
          "$ref": "#/components/schemas/AgentCommentPublication"
        },
        {
          "type": "null"
        }
      ],
      "default": null
    }
  },
  "required": [
    "request_id",
    "conversation_id",
    "agents",
    "comment_publication"
  ]
}
```

## AgentExecutionEventPage

```text
{
  "type": "object",
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "events": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/AgentExecutionEvent"
      }
    },
    "next": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "version",
    "events",
    "next"
  ]
}
```

## AgentExecutionEvent

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "sequence": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "observed_at_millis": {
      "type": [
        "integer",
        "null"
      ],
      "format": "uint64",
      "minimum": 0,
      "description": "Device observation time captured when this event was retained, if known.",
      "default": null
    },
    "payload": true,
    "committed": {
      "type": "boolean"
    }
  },
  "required": [
    "sequence",
    "observed_at_millis",
    "payload",
    "committed"
  ]
}
```

## LocalServerSettings

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "version": {
      "type": "integer",
      "format": "uint32",
      "minimum": 0
    },
    "revision": {
      "type": "string"
    },
    "desired": {
      "$ref": "#/components/schemas/ServerConfiguration"
    },
    "effective": {
      "$ref": "#/components/schemas/ServerConfiguration"
    },
    "restart_required": {
      "type": "boolean"
    },
    "sync_defaults": {
      "$ref": "#/components/schemas/SyncLimits"
    },
    "inventory": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/ConfiguredValue"
      }
    }
  },
  "required": [
    "version",
    "revision",
    "desired",
    "effective",
    "restart_required",
    "sync_defaults",
    "inventory"
  ]
}
```

## ServerConfiguration

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "service": {
      "$ref": "#/components/schemas/ServiceConfiguration"
    },
    "protection": {
      "$ref": "#/components/schemas/ProtectionConfiguration"
    },
    "sync": {
      "type": "array",
      "items": {
        "$ref": "#/components/schemas/SyncConfiguration"
      }
    }
  },
  "required": [
    "service",
    "protection",
    "sync"
  ]
}
```

## ServiceConfiguration

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "concurrent_requests": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "request_seconds": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "max_blob_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "inventory_nodes": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    }
  },
  "required": [
    "concurrent_requests",
    "request_seconds",
    "max_blob_bytes",
    "inventory_nodes"
  ]
}
```

## ProtectionConfiguration

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "bookmarks": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "type": "string"
      }
    },
    "commits": {
      "type": "array",
      "uniqueItems": true,
      "items": {
        "$ref": "#/components/schemas/CommitId"
      }
    }
  },
  "required": [
    "bookmarks",
    "commits"
  ]
}
```

## SyncConfiguration

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "repository": {
      "$ref": "#/components/schemas/RepositoryId"
    },
    "upstream": {
      "type": "string"
    },
    "insecure_loopback": {
      "type": "boolean"
    },
    "token_environment": {
      "type": "string"
    },
    "ca_certificate": {
      "type": [
        "string",
        "null"
      ]
    },
    "limits": {
      "$ref": "#/components/schemas/SyncLimits"
    }
  },
  "required": [
    "repository",
    "upstream",
    "insecure_loopback",
    "token_environment",
    "ca_certificate",
    "limits"
  ]
}
```

## SyncLimits

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "max_batch_origins": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "max_closure_entries": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "max_objects_per_step": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "max_bytes_per_step": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "max_native_object_bytes": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "poll_millis": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "retry_initial_millis": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "retry_max_seconds": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "circuit_failures": {
      "type": "integer",
      "format": "uint",
      "minimum": 0
    },
    "circuit_cooldown_seconds": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    },
    "timeout_seconds": {
      "type": "integer",
      "format": "uint64",
      "minimum": 0
    }
  },
  "required": [
    "max_batch_origins",
    "max_closure_entries",
    "max_objects_per_step",
    "max_bytes_per_step",
    "max_native_object_bytes",
    "poll_millis",
    "retry_initial_millis",
    "retry_max_seconds",
    "circuit_failures",
    "circuit_cooldown_seconds",
    "timeout_seconds"
  ]
}
```

## ConfiguredValue

```text
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "section": {
      "type": "string"
    },
    "name": {
      "type": "string"
    },
    "saved": {
      "type": "string"
    },
    "effective": {
      "type": "string"
    },
    "reason": {
      "type": "string"
    }
  },
  "required": [
    "section",
    "name",
    "saved",
    "effective",
    "reason"
  ]
}
```

