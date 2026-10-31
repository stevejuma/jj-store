# Workflows and execution history

Generated from OpenAPI. See [transport semantics](../transport.md), [capabilities](../capabilities.md) and [data types](types.md).

## GET /v1/repos/{repo}/workflows

Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.

```text
{
  "operationId": "workflow_page",
  "tags": [
    "work"
  ],
  "description": "Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.",
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
      "name": "workflow",
      "in": "query",
      "required": false,
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
      "name": "run",
      "in": "query",
      "required": false,
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
      "name": "at",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "after",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
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
        "minimum": 0
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "evidence_after",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "attempt",
      "in": "query",
      "required": false,
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
      "name": "pending_only",
      "in": "query",
      "required": false,
      "schema": {
        "type": "boolean"
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
            "$ref": "#/components/schemas/WorkflowPage"
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
      "description": "stale_work, cursor_reset, intent_conflict or work_replication_unsupported.",
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
      "workflow": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "run": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "at": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
      },
      "after": {
        "type": [
          "string",
          "null"
        ]
      },
      "limit": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint",
        "minimum": 0
      },
      "evidence_after": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint64",
        "minimum": 0
      },
      "attempt": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "pending_only": {
        "type": [
          "boolean",
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

## HEAD /v1/repos/{repo}/workflows

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "workflow_page_head",
  "tags": [
    "work"
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
      "name": "workflow",
      "in": "query",
      "required": false,
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
      "name": "run",
      "in": "query",
      "required": false,
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
      "name": "at",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "after",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
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
        "minimum": 0
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "evidence_after",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "attempt",
      "in": "query",
      "required": false,
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
      "name": "pending_only",
      "in": "query",
      "required": false,
      "schema": {
        "type": "boolean"
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
      "description": "stale_work, cursor_reset, intent_conflict or work_replication_unsupported."
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
      "workflow": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "run": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "at": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
      },
      "after": {
        "type": [
          "string",
          "null"
        ]
      },
      "limit": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint",
        "minimum": 0
      },
      "evidence_after": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint64",
        "minimum": 0
      },
      "attempt": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "pending_only": {
        "type": [
          "boolean",
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

## GET /v1/repos/{repo}/workflow-events

Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.

```text
{
  "operationId": "workflow_events",
  "tags": [
    "work"
  ],
  "description": "Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.",
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
      "required": false,
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
      "name": "after",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
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
        "minimum": 0
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "until",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "filters",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "description": "JSON array of bounded WorkflowConditions (portable through URL query transports)."
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
            "$ref": "#/components/schemas/WorkflowEventPage"
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
      "description": "stale_work, cursor_reset, intent_conflict or work_replication_unsupported.",
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
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "after": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
      },
      "limit": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint",
        "minimum": 0
      },
      "until": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
      },
      "filters": {
        "type": [
          "string",
          "null"
        ],
        "description": "JSON array of bounded WorkflowConditions (portable through URL query transports)."
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

## HEAD /v1/repos/{repo}/workflow-events

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "workflow_events_head",
  "tags": [
    "work"
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
      "name": "epoch",
      "in": "query",
      "required": false,
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
      "name": "after",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
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
        "minimum": 0
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "until",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "filters",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "description": "JSON array of bounded WorkflowConditions (portable through URL query transports)."
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
      "description": "stale_work, cursor_reset, intent_conflict or work_replication_unsupported."
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
      "epoch": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "after": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
      },
      "limit": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint",
        "minimum": 0
      },
      "until": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
      },
      "filters": {
        "type": [
          "string",
          "null"
        ],
        "description": "JSON array of bounded WorkflowConditions (portable through URL query transports)."
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

## POST /v1/repos/{repo}/workflows/commands

Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.

```text
{
  "operationId": "workflow_apply",
  "tags": [
    "work"
  ],
  "description": "Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.",
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
            "$ref": "#/components/schemas/WorkflowCommandOutcome"
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
      "description": "stale_work, cursor_reset, intent_conflict or work_replication_unsupported.",
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
            "command": {
              "oneOf": [
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "put"
                    },
                    "workflow": {
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
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "definition": {
                      "type": "object",
                      "properties": {
                        "name": {
                          "type": "string"
                        },
                        "description": {
                          "type": "string",
                          "default": ""
                        },
                        "enabled": {
                          "type": "boolean",
                          "default": false
                        },
                        "repository": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{32}$",
                          "minLength": 32,
                          "maxLength": 32
                        },
                        "trigger": {
                          "oneOf": [
                            {
                              "type": "object",
                              "properties": {
                                "on": {
                                  "type": "string",
                                  "const": "manual"
                                }
                              },
                              "required": [
                                "on"
                              ],
                              "additionalProperties": false
                            },
                            {
                              "type": "object",
                              "properties": {
                                "on": {
                                  "type": "string",
                                  "const": "code_published"
                                }
                              },
                              "required": [
                                "on"
                              ],
                              "additionalProperties": false
                            },
                            {
                              "type": "object",
                              "properties": {
                                "on": {
                                  "type": "string",
                                  "const": "message_posted"
                                }
                              },
                              "required": [
                                "on"
                              ],
                              "additionalProperties": false
                            },
                            {
                              "type": "object",
                              "properties": {
                                "on": {
                                  "type": "string",
                                  "const": "reaction_added"
                                }
                              },
                              "required": [
                                "on"
                              ],
                              "additionalProperties": false
                            },
                            {
                              "type": "object",
                              "properties": {
                                "on": {
                                  "type": "string",
                                  "const": "bookmark_changed"
                                },
                                "bookmark": {
                                  "type": "string"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "on",
                                "bookmark"
                              ]
                            }
                          ]
                        },
                        "conditions": {
                          "type": "object",
                          "properties": {
                            "revset": {
                              "type": [
                                "string",
                                "null"
                              ]
                            },
                            "authors": {
                              "type": "array",
                              "uniqueItems": true,
                              "items": {
                                "type": "string"
                              }
                            },
                            "excluded_authors": {
                              "type": "array",
                              "uniqueItems": true,
                              "items": {
                                "type": "string"
                              }
                            },
                            "text": {
                              "type": [
                                "object",
                                "null"
                              ],
                              "properties": {
                                "operator": {
                                  "type": "string",
                                  "enum": [
                                    "contains",
                                    "not_contains",
                                    "starts_with",
                                    "ends_with",
                                    "equals",
                                    "not_equals",
                                    "is_empty",
                                    "is_not_empty"
                                  ]
                                },
                                "value": {
                                  "type": "string"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "operator",
                                "value"
                              ]
                            },
                            "thread": {
                              "type": [
                                "string",
                                "null"
                              ]
                            },
                            "message": {
                              "type": [
                                "string",
                                "null"
                              ]
                            },
                            "emoji": {
                              "type": [
                                "string",
                                "null"
                              ]
                            },
                            "excluded_emoji": {
                              "type": [
                                "string",
                                "null"
                              ]
                            }
                          },
                          "additionalProperties": false
                        },
                        "labels": {
                          "type": "array",
                          "uniqueItems": true,
                          "items": {
                            "type": "string"
                          },
                          "default": []
                        },
                        "actions": {
                          "type": "array",
                          "items": {
                            "type": "object",
                            "properties": {
                              "id": {
                                "type": "string"
                              },
                              "executor": {
                                "oneOf": [
                                  {
                                    "type": "object",
                                    "properties": {
                                      "type": {
                                        "type": "string",
                                        "const": "command"
                                      },
                                      "name": {
                                        "type": "string"
                                      }
                                    },
                                    "additionalProperties": false,
                                    "required": [
                                      "type",
                                      "name"
                                    ]
                                  },
                                  {
                                    "type": "object",
                                    "properties": {
                                      "type": {
                                        "type": "string",
                                        "const": "github_workflow"
                                      },
                                      "path": {
                                        "type": "string"
                                      }
                                    },
                                    "additionalProperties": false,
                                    "required": [
                                      "type",
                                      "path"
                                    ]
                                  },
                                  {
                                    "type": "object",
                                    "properties": {
                                      "type": {
                                        "type": "string",
                                        "const": "agent"
                                      },
                                      "agent": {
                                        "type": "string"
                                      },
                                      "prompt": {
                                        "type": "string"
                                      }
                                    },
                                    "additionalProperties": false,
                                    "required": [
                                      "type",
                                      "agent",
                                      "prompt"
                                    ]
                                  }
                                ]
                              },
                              "inputs": {
                                "type": "object",
                                "additionalProperties": {
                                  "type": "string"
                                },
                                "default": {}
                              },
                              "timeout_seconds": {
                                "type": "integer",
                                "format": "uint64",
                                "minimum": 0,
                                "default": 300
                              }
                            },
                            "additionalProperties": false,
                            "required": [
                              "id",
                              "executor"
                            ]
                          }
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "name",
                        "repository",
                        "trigger",
                        "actions"
                      ]
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "workflow",
                    "definition"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "remove"
                    },
                    "workflow": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "workflow",
                    "expected"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "evaluate"
                    },
                    "event": {
                      "type": "object",
                      "properties": {
                        "id": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{64}$",
                          "minLength": 64,
                          "maxLength": 64
                        },
                        "publication": {
                          "description": "Canonical origin key/digest, or issuing repository/epoch/selected receipt.",
                          "type": "string"
                        },
                        "kind": {
                          "type": "string",
                          "enum": [
                            "message_posted",
                            "reaction_added",
                            "code_published",
                            "bookmark_changed"
                          ]
                        },
                        "source": {
                          "description": "Source anchors stay distinct from workflow and source-event identities.",
                          "type": "object",
                          "properties": {
                            "repository": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "operation": {
                              "type": [
                                "string",
                                "null"
                              ],
                              "pattern": "^[0-9a-f]{128}$",
                              "minLength": 128,
                              "maxLength": 128
                            },
                            "commit": {
                              "type": [
                                "string",
                                "null"
                              ],
                              "pattern": "^[0-9a-f]{128}$",
                              "minLength": 128,
                              "maxLength": 128
                            },
                            "bookmark": {
                              "type": [
                                "string",
                                "null"
                              ]
                            },
                            "conversation": {
                              "description": "Exact conversation anchors, never a message body copied into the workflow home.",
                              "type": [
                                "object",
                                "null"
                              ],
                              "properties": {
                                "thread": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "message": {
                                  "type": [
                                    "string",
                                    "null"
                                  ],
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "revision": {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
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
                                    },
                                    "id": {
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{64}$",
                                      "minLength": 64,
                                      "maxLength": 64
                                    }
                                  },
                                  "additionalProperties": false,
                                  "required": [
                                    "kind",
                                    "id"
                                  ]
                                },
                                "reaction": {
                                  "type": [
                                    "object",
                                    "null"
                                  ],
                                  "properties": {
                                    "kind": {
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
                                    },
                                    "id": {
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{64}$",
                                      "minLength": 64,
                                      "maxLength": 64
                                    }
                                  },
                                  "additionalProperties": false,
                                  "required": [
                                    "kind",
                                    "id"
                                  ]
                                },
                                "position": {
                                  "description": "Publication order is distinct from native operation ancestry.",
                                  "type": "integer",
                                  "format": "uint64",
                                  "minimum": 0
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "thread",
                                "revision",
                                "position"
                              ]
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "repository"
                          ]
                        },
                        "filter_results": {
                          "type": "object",
                          "additionalProperties": false,
                          "patternProperties": {
                            "^[0-9a-f]{64}$": {
                              "type": "boolean"
                            }
                          }
                        },
                        "code_context": {
                          "type": [
                            "object",
                            "null"
                          ],
                          "properties": {
                            "before": {
                              "type": [
                                "string",
                                "null"
                              ],
                              "pattern": "^[0-9a-f]{128}$",
                              "minLength": 128,
                              "maxLength": 128
                            },
                            "comparable": {
                              "type": "boolean"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "comparable"
                          ]
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "id",
                        "publication",
                        "kind",
                        "source"
                      ]
                    },
                    "definitions": {
                      "type": [
                        "object",
                        "null"
                      ],
                      "additionalProperties": false,
                      "patternProperties": {
                        "^[0-9a-f]{32}$": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{32}$",
                          "minLength": 32,
                          "maxLength": 32
                        }
                      }
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "event"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "run"
                    },
                    "workflow": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "source": {
                      "description": "Source anchors stay distinct from workflow and source-event identities.",
                      "type": "object",
                      "properties": {
                        "repository": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{32}$",
                          "minLength": 32,
                          "maxLength": 32
                        },
                        "operation": {
                          "type": [
                            "string",
                            "null"
                          ],
                          "pattern": "^[0-9a-f]{128}$",
                          "minLength": 128,
                          "maxLength": 128
                        },
                        "commit": {
                          "type": [
                            "string",
                            "null"
                          ],
                          "pattern": "^[0-9a-f]{128}$",
                          "minLength": 128,
                          "maxLength": 128
                        },
                        "bookmark": {
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "conversation": {
                          "description": "Exact conversation anchors, never a message body copied into the workflow home.",
                          "type": [
                            "object",
                            "null"
                          ],
                          "properties": {
                            "thread": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "message": {
                              "type": [
                                "string",
                                "null"
                              ],
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "revision": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                                },
                                "id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{64}$",
                                  "minLength": 64,
                                  "maxLength": 64
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "kind",
                                "id"
                              ]
                            },
                            "reaction": {
                              "type": [
                                "object",
                                "null"
                              ],
                              "properties": {
                                "kind": {
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
                                },
                                "id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{64}$",
                                  "minLength": 64,
                                  "maxLength": 64
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "kind",
                                "id"
                              ]
                            },
                            "position": {
                              "description": "Publication order is distinct from native operation ancestry.",
                              "type": "integer",
                              "format": "uint64",
                              "minimum": 0
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "thread",
                            "revision",
                            "position"
                          ]
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "repository"
                      ]
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "workflow",
                    "expected",
                    "source"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "claim"
                    },
                    "run": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "attempt": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "host": {
                      "type": "string"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "run",
                    "attempt",
                    "host"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "report"
                    },
                    "run": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "attempt": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "sequence": {
                      "type": "integer",
                      "format": "uint64",
                      "minimum": 0
                    },
                    "data": {
                      "oneOf": [
                        {
                          "type": "object",
                          "properties": {
                            "type": {
                              "type": "string",
                              "const": "log"
                            },
                            "step": {
                              "type": [
                                "string",
                                "null"
                              ]
                            },
                            "stream": {
                              "type": "string"
                            },
                            "text": {
                              "type": "string"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "stream",
                            "text"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "type": {
                              "type": "string",
                              "const": "step"
                            },
                            "step": {
                              "type": "string"
                            },
                            "status": {
                              "type": "string",
                              "enum": [
                                "pending",
                                "running",
                                "cancel_requested",
                                "completed",
                                "failed",
                                "cancelled",
                                "unknown",
                                "skipped"
                              ]
                            },
                            "output": {
                              "type": "object",
                              "additionalProperties": {
                                "type": "string"
                              }
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "step",
                            "status",
                            "output"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "type": {
                              "type": "string",
                              "const": "artifact"
                            },
                            "name": {
                              "type": "string"
                            },
                            "media_type": {
                              "type": "string"
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
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "name",
                            "media_type",
                            "bytes"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "type": {
                              "type": "string",
                              "const": "agent"
                            },
                            "session": {
                              "type": "string"
                            },
                            "request_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "session",
                            "request_id"
                          ]
                        }
                      ]
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "run",
                    "attempt",
                    "sequence",
                    "data"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "finish"
                    },
                    "run": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "attempt": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "status": {
                      "type": "string",
                      "enum": [
                        "pending",
                        "running",
                        "cancel_requested",
                        "completed",
                        "failed",
                        "cancelled",
                        "unknown",
                        "skipped"
                      ]
                    },
                    "result": {
                      "type": "string"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "run",
                    "attempt",
                    "status",
                    "result"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "heartbeat"
                    },
                    "run": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "attempt": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "run",
                    "attempt"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "interrupt"
                    },
                    "run": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "attempt": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "run",
                    "attempt"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "cancel"
                    },
                    "run": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "run"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "retry"
                    },
                    "run": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "run",
                    "expected"
                  ]
                }
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "request_id",
            "command"
          ]
        }
      }
    }
  }
}
```

## GET /v1/repos/{repo}/workflows/commands/{request_id}

Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.

```text
{
  "operationId": "workflow_status",
  "tags": [
    "work"
  ],
  "description": "Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.",
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
            "$ref": "#/components/schemas/WorkflowCommandStatus"
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
      "description": "stale_work, cursor_reset, intent_conflict or work_replication_unsupported.",
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

## HEAD /v1/repos/{repo}/workflows/commands/{request_id}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "workflow_status_head",
  "tags": [
    "work"
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
      "description": "stale_work, cursor_reset, intent_conflict or work_replication_unsupported."
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

