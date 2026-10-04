# Repository administration and replication

Generated from OpenAPI. See [transport semantics](../transport.md), [capabilities](../capabilities.md) and [data types](types.md).

## GET /v1/repos/{repo}/query-info

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "query_info",
  "tags": [
    "reads"
  ],
  "description": "Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "at_op",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "style": "form",
      "explode": true
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/Envelope_QueryInfo"
          }
        }
      }
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "401": {
      "description": "Missing repository Bearer credential.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "405": {
      "description": "Unsupported method.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "408": {
      "description": "Native query canceled.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "409": {
      "description": "Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "410": {
      "description": "Index task expired or was lost on server restart.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "413": {
      "description": "Request or response resource limit exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "415": {
      "description": "Unsupported request media type or content encoding.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "504": {
      "description": "Request or response deadline exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "at_op": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      }
    }
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## HEAD /v1/repos/{repo}/query-info

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "query_info_head",
  "tags": [
    "reads"
  ],
  "description": "Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "at_op",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "style": "form",
      "explode": true
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot."
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor."
    },
    "401": {
      "description": "Missing repository Bearer credential."
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin."
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route."
    },
    "405": {
      "description": "Unsupported method."
    },
    "408": {
      "description": "Native query canceled."
    },
    "409": {
      "description": "Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index."
    },
    "410": {
      "description": "Index task expired or was lost on server restart."
    },
    "413": {
      "description": "Request or response resource limit exceeded."
    },
    "415": {
      "description": "Unsupported request media type or content encoding."
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history."
    },
    "504": {
      "description": "Request or response deadline exceeded."
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "at_op": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      }
    }
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## GET /v1/repos/{repo}/capabilities

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "capabilities",
  "tags": [
    "reads"
  ],
  "description": "Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/Capabilities"
          }
        }
      }
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "401": {
      "description": "Missing repository Bearer credential.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "405": {
      "description": "Unsupported method.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "408": {
      "description": "Native query canceled.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "409": {
      "description": "Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "410": {
      "description": "Index task expired or was lost on server restart.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "413": {
      "description": "Request or response resource limit exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "415": {
      "description": "Unsupported request media type or content encoding.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "504": {
      "description": "Request or response deadline exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## HEAD /v1/repos/{repo}/capabilities

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "capabilities_head",
  "tags": [
    "reads"
  ],
  "description": "Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot."
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor."
    },
    "401": {
      "description": "Missing repository Bearer credential."
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin."
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route."
    },
    "405": {
      "description": "Unsupported method."
    },
    "408": {
      "description": "Native query canceled."
    },
    "409": {
      "description": "Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index."
    },
    "410": {
      "description": "Index task expired or was lost on server restart."
    },
    "413": {
      "description": "Request or response resource limit exceeded."
    },
    "415": {
      "description": "Unsupported request media type or content encoding."
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history."
    },
    "504": {
      "description": "Request or response deadline exceeded."
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## GET /v1/repos/{repo}/reaction-emojis

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_emojis",
  "tags": [
    "collaboration"
  ],
  "description": "Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "limit",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint32",
        "minimum": 0,
        "default": 50
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "cursor",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/CollabPage"
          }
        }
      }
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "401": {
      "description": "Missing repository Bearer credential.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "405": {
      "description": "Unsupported method.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "408": {
      "description": "Native query canceled.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "409": {
      "description": "cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "410": {
      "description": "Index task expired or was lost on server restart.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "413": {
      "description": "Request or response resource limit exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "415": {
      "description": "Unsupported request media type or content encoding.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "504": {
      "description": "Request or response deadline exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "limit": {
        "type": "integer",
        "format": "uint32",
        "minimum": 0,
        "default": 50
      },
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      }
    }
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## HEAD /v1/repos/{repo}/reaction-emojis

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "collab_emojis_head",
  "tags": [
    "collaboration"
  ],
  "description": "Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "limit",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint32",
        "minimum": 0,
        "default": 50
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "cursor",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot."
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor."
    },
    "401": {
      "description": "Missing repository Bearer credential."
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin."
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route."
    },
    "405": {
      "description": "Unsupported method."
    },
    "408": {
      "description": "Native query canceled."
    },
    "409": {
      "description": "cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry."
    },
    "410": {
      "description": "Index task expired or was lost on server restart."
    },
    "413": {
      "description": "Request or response resource limit exceeded."
    },
    "415": {
      "description": "Unsupported request media type or content encoding."
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history."
    },
    "504": {
      "description": "Request or response deadline exceeded."
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "limit": {
        "type": "integer",
        "format": "uint32",
        "minimum": 0,
        "default": 50
      },
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      }
    }
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## POST /v1/repos/{repo}/reaction-emojis/{emoji_id}

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_emoji_define",
  "tags": [
    "collaboration"
  ],
  "description": "Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "emoji_id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/CustomEmojiId"
      }
    },
    {
      "name": "version",
      "in": "query",
      "required": true,
      "schema": {
        "type": "integer",
        "format": "uint32",
        "minimum": 0
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "request_id",
      "in": "query",
      "required": true,
      "schema": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "expected",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "pattern": "^[0-9a-f]{64}$",
        "minLength": 64,
        "maxLength": 64
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "shortcode",
      "in": "query",
      "required": true,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "label",
      "in": "query",
      "required": true,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/CollabCommandOutcome"
          }
        }
      }
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "401": {
      "description": "Missing repository Bearer credential.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "405": {
      "description": "Unsupported method.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "408": {
      "description": "Native query canceled.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "409": {
      "description": "cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "410": {
      "description": "Index task expired or was lost on server restart.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "413": {
      "description": "Request or response resource limit exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "415": {
      "description": "Unsupported request media type or content encoding.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "504": {
      "description": "Request or response deadline exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "version": {
        "type": "integer",
        "format": "uint32",
        "minimum": 0
      },
      "request_id": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "expected": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{64}$",
        "minLength": 64,
        "maxLength": 64
      },
      "shortcode": {
        "type": "string"
      },
      "label": {
        "type": "string"
      }
    },
    "required": [
      "version",
      "request_id",
      "shortcode",
      "label"
    ]
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ],
  "requestBody": {
    "required": true,
    "description": "At most 2097152 encoded bytes; only bounded PNG/JPEG/GIF/WebP are admitted. Static preview is retained by the service.",
    "content": {
      "application/octet-stream": {
        "schema": {
          "type": "string",
          "format": "binary"
        }
      }
    }
  }
}
```

## POST /v1/repos/{repo}/reaction-emojis/commands

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_emoji_retire",
  "tags": [
    "collaboration"
  ],
  "description": "Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/CollabCommandOutcome"
          }
        }
      }
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "401": {
      "description": "Missing repository Bearer credential.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "405": {
      "description": "Unsupported method.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "408": {
      "description": "Native query canceled.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "409": {
      "description": "cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "410": {
      "description": "Index task expired or was lost on server restart.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "413": {
      "description": "Request or response resource limit exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "415": {
      "description": "Unsupported request media type or content encoding.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "504": {
      "description": "Request or response deadline exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "version": {
              "type": "integer",
              "format": "uint32",
              "minimum": 0
            },
            "request_id": {
              "type": "string",
              "pattern": "^[0-9a-f]{32}$",
              "minLength": 32,
              "maxLength": 32
            },
            "emoji_id": {
              "type": "string",
              "pattern": "^[0-9a-f]{32}$",
              "minLength": 32,
              "maxLength": 32
            },
            "expected": {
              "type": "string",
              "pattern": "^[0-9a-f]{64}$",
              "minLength": 64,
              "maxLength": 64
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "request_id",
            "emoji_id",
            "expected"
          ]
        }
      }
    }
  }
}
```

## GET /v1/repos/{repo}/reaction-emojis/commands/{request_id}

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_status_reaction-emojis",
  "tags": [
    "collaboration"
  ],
  "description": "Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "request_id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RequestId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/CollabCommandStatus"
          }
        }
      }
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "401": {
      "description": "Missing repository Bearer credential.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "405": {
      "description": "Unsupported method.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "408": {
      "description": "Native query canceled.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "409": {
      "description": "cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "410": {
      "description": "Index task expired or was lost on server restart.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "413": {
      "description": "Request or response resource limit exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "415": {
      "description": "Unsupported request media type or content encoding.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "504": {
      "description": "Request or response deadline exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## HEAD /v1/repos/{repo}/reaction-emojis/commands/{request_id}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "collab_status_reaction-emojis_head",
  "tags": [
    "collaboration"
  ],
  "description": "Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "request_id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RequestId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot."
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor."
    },
    "401": {
      "description": "Missing repository Bearer credential."
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin."
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route."
    },
    "405": {
      "description": "Unsupported method."
    },
    "408": {
      "description": "Native query canceled."
    },
    "409": {
      "description": "cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry."
    },
    "410": {
      "description": "Index task expired or was lost on server restart."
    },
    "413": {
      "description": "Request or response resource limit exceeded."
    },
    "415": {
      "description": "Unsupported request media type or content encoding."
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history."
    },
    "504": {
      "description": "Request or response deadline exceeded."
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## GET /v1/repos/{repo}/reaction-assets/{asset}

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_image",
  "tags": [
    "collaboration"
  ],
  "description": "Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "asset",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/CollabObjectId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized retained raster. Source and static preview IDs are independent immutable assets. No byte ranges or external URLs; snapshot/content identity is in headers.",
      "headers": {
        "X-JJ-Store-Repository": {
          "schema": {
            "$ref": "#/components/schemas/RepositoryId"
          }
        },
        "X-JJ-Store-Epoch": {
          "schema": {
            "$ref": "#/components/schemas/EpochId"
          }
        },
        "X-JJ-Store-Position": {
          "schema": {
            "type": "integer",
            "minimum": 0
          }
        },
        "X-JJ-Store-Revision": {
          "schema": {
            "$ref": "#/components/schemas/RecordDigest"
          }
        },
        "X-JJ-Store-Reaction-Asset": {
          "schema": {
            "$ref": "#/components/schemas/CollabObjectId"
          }
        }
      },
      "content": {
        "image/png": {
          "schema": {
            "type": "string",
            "format": "binary"
          }
        },
        "image/jpeg": {
          "schema": {
            "type": "string",
            "format": "binary"
          }
        },
        "image/gif": {
          "schema": {
            "type": "string",
            "format": "binary"
          }
        },
        "image/webp": {
          "schema": {
            "type": "string",
            "format": "binary"
          }
        }
      }
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "401": {
      "description": "Missing repository Bearer credential.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "405": {
      "description": "Unsupported method.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "408": {
      "description": "Native query canceled.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "409": {
      "description": "cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "410": {
      "description": "Index task expired or was lost on server restart.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "413": {
      "description": "Request or response resource limit exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "415": {
      "description": "Unsupported request media type or content encoding.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "504": {
      "description": "Request or response deadline exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## HEAD /v1/repos/{repo}/reaction-assets/{asset}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "collab_image_head",
  "tags": [
    "collaboration"
  ],
  "description": "Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "asset",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/CollabObjectId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized retained raster. Source and static preview IDs are independent immutable assets. No byte ranges or external URLs; snapshot/content identity is in headers.",
      "headers": {
        "X-JJ-Store-Repository": {
          "schema": {
            "$ref": "#/components/schemas/RepositoryId"
          }
        },
        "X-JJ-Store-Epoch": {
          "schema": {
            "$ref": "#/components/schemas/EpochId"
          }
        },
        "X-JJ-Store-Position": {
          "schema": {
            "type": "integer",
            "minimum": 0
          }
        },
        "X-JJ-Store-Revision": {
          "schema": {
            "$ref": "#/components/schemas/RecordDigest"
          }
        },
        "X-JJ-Store-Reaction-Asset": {
          "schema": {
            "$ref": "#/components/schemas/CollabObjectId"
          }
        }
      }
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor."
    },
    "401": {
      "description": "Missing repository Bearer credential."
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin."
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route."
    },
    "405": {
      "description": "Unsupported method."
    },
    "408": {
      "description": "Native query canceled."
    },
    "409": {
      "description": "cursor_reset, stale_collaboration, intent_conflict or collaboration_replication_unsupported. No latest-state fallback or automatic retry."
    },
    "410": {
      "description": "Index task expired or was lost on server restart."
    },
    "413": {
      "description": "Request or response resource limit exceeded."
    },
    "415": {
      "description": "Unsupported request media type or content encoding."
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history."
    },
    "504": {
      "description": "Request or response deadline exceeded."
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## GET /v1/repos/{repo}/events

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "events",
  "tags": [
    "reads"
  ],
  "description": "Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "after",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "maxLength": 128,
        "pattern": "^v1:[0-9a-f]{32}:[0-9a-f]{32}:(0|[1-9][0-9]*)$",
        "description": "Committed replay cursor, not a credential. Position must fit u64."
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "Last-Event-ID",
      "in": "header",
      "required": false,
      "description": "Must agree with after when both are supplied.",
      "schema": {
        "type": "string",
        "maxLength": 128,
        "pattern": "^v1:[0-9a-f]{32}:[0-9a-f]{32}:(0|[1-9][0-9]*)$",
        "description": "Committed replay cursor, not a credential. Position must fit u64."
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Bounded SSE stream. JSON data appears inside ready, repository-changed and reset frames. Authorized heartbeat comments do not advance IDs. Reset closes the stream. Resume the last fully parsed ID after EOF; post-header failures terminate the stream.",
      "content": {
        "text/event-stream": {
          "schema": {
            "type": "string"
          },
          "x-sse-data-schema": {
            "$ref": "#/components/schemas/CommittedEvent"
          },
          "x-sse-events": {
            "ready": "ready",
            "repository-changed": "changed",
            "reset": "reset"
          },
          "x-sse-id-schema": {
            "type": "string",
            "maxLength": 128,
            "pattern": "^v1:[0-9a-f]{32}:[0-9a-f]{32}:(0|[1-9][0-9]*)$",
            "description": "Committed replay cursor, not a credential. Position must fit u64."
          }
        }
      }
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "401": {
      "description": "Missing repository Bearer credential.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "405": {
      "description": "Unsupported method.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "408": {
      "description": "Native query canceled.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "409": {
      "description": "Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "410": {
      "description": "Index task expired or was lost on server restart.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "413": {
      "description": "Request or response resource limit exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "415": {
      "description": "Unsupported request media type or content encoding.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "504": {
      "description": "Request or response deadline exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "after": {
        "type": [
          "string",
          "null"
        ]
      }
    }
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## HEAD /v1/repos/{repo}/events

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "events_head",
  "tags": [
    "reads"
  ],
  "description": "Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "after",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "maxLength": 128,
        "pattern": "^v1:[0-9a-f]{32}:[0-9a-f]{32}:(0|[1-9][0-9]*)$",
        "description": "Committed replay cursor, not a credential. Position must fit u64."
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "Last-Event-ID",
      "in": "header",
      "required": false,
      "description": "Must agree with after when both are supplied.",
      "schema": {
        "type": "string",
        "maxLength": 128,
        "pattern": "^v1:[0-9a-f]{32}:[0-9a-f]{32}:(0|[1-9][0-9]*)$",
        "description": "Committed replay cursor, not a credential. Position must fit u64."
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Bounded SSE stream. JSON data appears inside ready, repository-changed and reset frames. Authorized heartbeat comments do not advance IDs. Reset closes the stream. Resume the last fully parsed ID after EOF; post-header failures terminate the stream."
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor."
    },
    "401": {
      "description": "Missing repository Bearer credential."
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin."
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route."
    },
    "405": {
      "description": "Unsupported method."
    },
    "408": {
      "description": "Native query canceled."
    },
    "409": {
      "description": "Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index."
    },
    "410": {
      "description": "Index task expired or was lost on server restart."
    },
    "413": {
      "description": "Request or response resource limit exceeded."
    },
    "415": {
      "description": "Unsupported request media type or content encoding."
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history."
    },
    "504": {
      "description": "Request or response deadline exceeded."
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "after": {
        "type": [
          "string",
          "null"
        ]
      }
    }
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## GET /v1/repos/{repo}/objects/{kind}/{id}

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "object",
  "tags": [
    "reads"
  ],
  "description": "Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "kind",
      "in": "path",
      "required": true,
      "schema": {
        "type": "string",
        "enum": [
          "commit",
          "tree",
          "file",
          "symlink",
          "copy",
          "view",
          "operation"
        ]
      }
    },
    {
      "name": "id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    {
      "name": "at_op",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "style": "form",
      "explode": true
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/Envelope_ContentObject"
          }
        }
      }
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "401": {
      "description": "Missing repository Bearer credential.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "405": {
      "description": "Unsupported method.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "408": {
      "description": "Native query canceled.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "409": {
      "description": "Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "410": {
      "description": "Index task expired or was lost on server restart.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "413": {
      "description": "Request or response resource limit exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "415": {
      "description": "Unsupported request media type or content encoding.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "504": {
      "description": "Request or response deadline exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "at_op": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      }
    }
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## HEAD /v1/repos/{repo}/objects/{kind}/{id}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "object_head",
  "tags": [
    "reads"
  ],
  "description": "Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "kind",
      "in": "path",
      "required": true,
      "schema": {
        "type": "string",
        "enum": [
          "commit",
          "tree",
          "file",
          "symlink",
          "copy",
          "view",
          "operation"
        ]
      }
    },
    {
      "name": "id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    {
      "name": "at_op",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "style": "form",
      "explode": true
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot."
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor."
    },
    "401": {
      "description": "Missing repository Bearer credential."
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin."
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route."
    },
    "405": {
      "description": "Unsupported method."
    },
    "408": {
      "description": "Native query canceled."
    },
    "409": {
      "description": "Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index."
    },
    "410": {
      "description": "Index task expired or was lost on server restart."
    },
    "413": {
      "description": "Request or response resource limit exceeded."
    },
    "415": {
      "description": "Unsupported request media type or content encoding."
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history."
    },
    "504": {
      "description": "Request or response deadline exceeded."
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "at_op": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      }
    }
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## GET /v1/repos/{repo}/references

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "references",
  "tags": [
    "reads"
  ],
  "description": "Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "at_op",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "cursor",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "q",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "default": ""
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "kind",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "bookmarks",
          "changes",
          "all"
        ],
        "default": "all"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "limit",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "anchor",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "direction",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "forward",
          "backward"
        ],
        "default": "forward"
      },
      "style": "form",
      "explode": true
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/Window"
          }
        }
      }
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "401": {
      "description": "Missing repository Bearer credential.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "405": {
      "description": "Unsupported method.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "408": {
      "description": "Native query canceled.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "409": {
      "description": "Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "410": {
      "description": "Index task expired or was lost on server restart.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "413": {
      "description": "Request or response resource limit exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "415": {
      "description": "Unsupported request media type or content encoding.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "504": {
      "description": "Request or response deadline exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "at_op": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      },
      "q": {
        "type": "string",
        "default": ""
      },
      "kind": {
        "type": "string",
        "enum": [
          "bookmarks",
          "changes",
          "all"
        ],
        "default": "all"
      },
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
      },
      "anchor": {
        "type": [
          "string",
          "null"
        ]
      },
      "direction": {
        "type": "string",
        "enum": [
          "forward",
          "backward"
        ],
        "default": "forward"
      }
    }
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## HEAD /v1/repos/{repo}/references

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "references_head",
  "tags": [
    "reads"
  ],
  "description": "Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "at_op",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "cursor",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "q",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "default": ""
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "kind",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "bookmarks",
          "changes",
          "all"
        ],
        "default": "all"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "limit",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "anchor",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "direction",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "forward",
          "backward"
        ],
        "default": "forward"
      },
      "style": "form",
      "explode": true
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot."
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor."
    },
    "401": {
      "description": "Missing repository Bearer credential."
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin."
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route."
    },
    "405": {
      "description": "Unsupported method."
    },
    "408": {
      "description": "Native query canceled."
    },
    "409": {
      "description": "Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index."
    },
    "410": {
      "description": "Index task expired or was lost on server restart."
    },
    "413": {
      "description": "Request or response resource limit exceeded."
    },
    "415": {
      "description": "Unsupported request media type or content encoding."
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history."
    },
    "504": {
      "description": "Request or response deadline exceeded."
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "at_op": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      },
      "q": {
        "type": "string",
        "default": ""
      },
      "kind": {
        "type": "string",
        "enum": [
          "bookmarks",
          "changes",
          "all"
        ],
        "default": "all"
      },
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
      },
      "anchor": {
        "type": [
          "string",
          "null"
        ]
      },
      "direction": {
        "type": "string",
        "enum": [
          "forward",
          "backward"
        ],
        "default": "forward"
      }
    }
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## POST /v1/repos/{repo}/conflict-uploads

Requires current repository write permission before body consumption. Binary staging only; never publishes history. Binding must name the exact current single-head snapshot and a committed target. Repeating identical bytes/binding returns the same scoped handle. Deadline or unavailable responses permit safe re-upload; no mutation receipt is created. Upload limits are advertised by capabilities.

```text
{
  "operationId": "conflict_upload",
  "tags": [
    "mutations"
  ],
  "description": "Requires current repository write permission before body consumption. Binary staging only; never publishes history. Binding must name the exact current single-head snapshot and a committed target. Repeating identical bytes/binding returns the same scoped handle. Deadline or unavailable responses permit safe re-upload; no mutation receipt is created. Upload limits are advertised by capabilities.",
  "parameters": [
    {
      "name": "repo",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/RepositoryId"
      }
    },
    {
      "name": "epoch",
      "in": "query",
      "required": true,
      "schema": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "position",
      "in": "query",
      "required": true,
      "schema": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "code_state_version",
      "in": "query",
      "required": true,
      "schema": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "operation",
      "in": "query",
      "required": true,
      "schema": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "commit",
      "in": "query",
      "required": true,
      "schema": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "path",
      "in": "query",
      "required": true,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "fingerprint",
      "in": "query",
      "required": true,
      "schema": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "style": "form",
      "explode": true
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ConflictUpload"
          }
        }
      }
    },
    "400": {
      "description": "Invalid request, unknown query field or invalid cursor.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "401": {
      "description": "Missing repository Bearer credential.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "403": {
      "description": "Access denied, including revoked credentials or disallowed Origin.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "404": {
      "description": "Object not committed in the selected snapshot or unknown route.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "405": {
      "description": "Unsupported method.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "408": {
      "description": "Native query canceled.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "409": {
      "description": "Select an operation, reselect after epoch/cursor reset, or prepare the reported snapshot index.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "410": {
      "description": "Index task expired or was lost on server restart.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "413": {
      "description": "Request or response resource limit exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "415": {
      "description": "Unsupported request media type or content encoding.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "Busy, unavailable, corrupt or incomplete history.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "504": {
      "description": "Request or response deadline exceeded.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "epoch": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "position": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0
      },
      "code_state_version": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0
      },
      "operation": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "commit": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "path": {
        "type": "string"
      },
      "fingerprint": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      }
    },
    "required": [
      "epoch",
      "position",
      "code_state_version",
      "operation",
      "commit",
      "path",
      "fingerprint"
    ]
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/octet-stream": {
        "schema": {
          "type": "string",
          "format": "binary"
        }
      }
    }
  }
}
```

