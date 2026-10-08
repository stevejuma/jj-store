# Agents, reviews and discussions

Generated from OpenAPI. See [transport semantics](../transport.md), [capabilities](../capabilities.md) and [data types](types.md).

## GET /v1/repos/{repo}/agent/sessions

Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.

```text
{
  "operationId": "agent_sessions",
  "tags": [
    "collaboration"
  ],
  "description": "Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.",
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
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/AgentSessionPage"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement.",
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
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      },
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
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

## HEAD /v1/repos/{repo}/agent/sessions

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "agent_sessions_head",
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement."
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
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      },
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
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

## GET /v1/repos/{repo}/agent/sessions/{session}

Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.

```text
{
  "operationId": "agent_session",
  "tags": [
    "collaboration"
  ],
  "description": "Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.",
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
      "name": "session",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/AgentSessionId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/AgentSessionDetail"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement.",
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

## HEAD /v1/repos/{repo}/agent/sessions/{session}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "agent_session_head",
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
      "name": "session",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/AgentSessionId"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement."
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

## GET /v1/repos/{repo}/agent/sessions/{session}/transcripts/{transcript}

Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.

```text
{
  "operationId": "agent_transcript",
  "tags": [
    "collaboration"
  ],
  "description": "Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.",
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
      "name": "session",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/AgentSessionId"
      }
    },
    {
      "name": "transcript",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/AgentObjectId"
      }
    },
    {
      "name": "source",
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
      "name": "generation",
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
      "name": "max_bytes",
      "in": "query",
      "required": true,
      "schema": {
        "type": "integer",
        "format": "uint",
        "minimum": 0
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
            "$ref": "#/components/schemas/AgentTranscriptPage"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement.",
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
      "source": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "generation": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0
      },
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      },
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
      },
      "max_bytes": {
        "type": "integer",
        "format": "uint",
        "minimum": 0
      }
    },
    "required": [
      "source",
      "generation",
      "max_bytes"
    ]
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## HEAD /v1/repos/{repo}/agent/sessions/{session}/transcripts/{transcript}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "agent_transcript_head",
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
      "name": "session",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/AgentSessionId"
      }
    },
    {
      "name": "transcript",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/AgentObjectId"
      }
    },
    {
      "name": "source",
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
      "name": "generation",
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
      "name": "max_bytes",
      "in": "query",
      "required": true,
      "schema": {
        "type": "integer",
        "format": "uint",
        "minimum": 0
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement."
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
      "source": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "generation": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0
      },
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      },
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
      },
      "max_bytes": {
        "type": "integer",
        "format": "uint",
        "minimum": 0
      }
    },
    "required": [
      "source",
      "generation",
      "max_bytes"
    ]
  },
  "security": [
    {
      "repositoryBearer": []
    },
    {}
  ]
}
```

## GET /v1/repos/{repo}/agent/associations

Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.

```text
{
  "operationId": "agent_associations",
  "tags": [
    "collaboration"
  ],
  "description": "Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.",
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
      "name": "session",
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
      "name": "tool_session",
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
      "name": "tool",
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
      "name": "operation",
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
      "name": "commit",
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
      "name": "checkpoint",
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
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/AgentRelationPage"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement.",
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
      "session": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "tool_session": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "tool": {
        "type": [
          "string",
          "null"
        ],
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
      "checkpoint": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      },
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
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

## HEAD /v1/repos/{repo}/agent/associations

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "agent_associations_head",
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
      "name": "session",
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
      "name": "tool_session",
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
      "name": "tool",
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
      "name": "operation",
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
      "name": "commit",
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
      "name": "checkpoint",
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement."
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
      "session": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "tool_session": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "tool": {
        "type": [
          "string",
          "null"
        ],
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
      "checkpoint": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      },
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
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

## GET /v1/repos/{repo}/agent/checkpoints

Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.

```text
{
  "operationId": "agent_checkpoints",
  "tags": [
    "collaboration"
  ],
  "description": "Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.",
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
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/AgentCheckpointPage"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement.",
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
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      },
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
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

## HEAD /v1/repos/{repo}/agent/checkpoints

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "agent_checkpoints_head",
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement."
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
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      },
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
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

## GET /v1/repos/{repo}/agent/checkpoints/{checkpoint}

Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.

```text
{
  "operationId": "agent_checkpoint",
  "tags": [
    "collaboration"
  ],
  "description": "Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.",
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
      "name": "checkpoint",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/AgentCheckpointId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/AgentCheckpointDetail"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement.",
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

## HEAD /v1/repos/{repo}/agent/checkpoints/{checkpoint}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "agent_checkpoint_head",
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
      "name": "checkpoint",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/AgentCheckpointId"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement."
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

## GET /v1/repos/{repo}/agent/checkpoints/{checkpoint}/versions/{version}

Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.

```text
{
  "operationId": "agent_checkpoint_version",
  "tags": [
    "collaboration"
  ],
  "description": "Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.",
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
      "name": "checkpoint",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/AgentCheckpointId"
      }
    },
    {
      "name": "version",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/AgentObjectId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/AgentCheckpointDetail"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement.",
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

## HEAD /v1/repos/{repo}/agent/checkpoints/{checkpoint}/versions/{version}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "agent_checkpoint_version_head",
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
      "name": "checkpoint",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/AgentCheckpointId"
      }
    },
    {
      "name": "version",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/AgentObjectId"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement."
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

## GET /v1/repos/{repo}/reviews/threads

Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.

```text
{
  "operationId": "review_threads",
  "tags": [
    "collaboration"
  ],
  "description": "Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.",
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
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ReviewThreadPage"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement.",
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
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      },
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
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

## HEAD /v1/repos/{repo}/reviews/threads

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "review_threads_head",
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement."
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
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      },
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
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

## GET /v1/repos/{repo}/reviews/activity

Requires current repository read permission. Returns count and maximum accepted event timestamp for the first bounded radix-ordered thread window of one admitted immutable ReviewStore root. threads_partial indicates an incomplete repository count and sampled timestamp. Contains no presentation or continuation, never dispatches or publishes.

```text
{
  "operationId": "review_activity",
  "tags": [
    "collaboration"
  ],
  "description": "Requires current repository read permission. Returns count and maximum accepted event timestamp for the first bounded radix-ordered thread window of one admitted immutable ReviewStore root. threads_partial indicates an incomplete repository count and sampled timestamp. Contains no presentation or continuation, never dispatches or publishes.",
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
        "format": "uint",
        "minimum": 0,
        "default": 50
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
            "$ref": "#/components/schemas/ReviewActivitySummary"
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
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
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

## HEAD /v1/repos/{repo}/reviews/activity

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "review_activity_head",
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
        "format": "uint",
        "minimum": 0,
        "default": 50
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
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
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

## GET /v1/repos/{repo}/reviews/threads/{thread}

Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.

```text
{
  "operationId": "review_thread",
  "tags": [
    "collaboration"
  ],
  "description": "Requires current repository read permission. Rows resolve from one immutable AgentStore or ReviewStore root. Signed process-local continuations retain that exact root and page size, reauthorize every page, and reset after expiry, restart or epoch replacement. Reads never dispatch an agent or publish repository state.",
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
      "name": "thread",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/ReviewThreadId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ReviewThreadDetail"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement.",
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

## HEAD /v1/repos/{repo}/reviews/threads/{thread}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "review_thread_head",
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
      "name": "thread",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/ReviewThreadId"
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
      "description": "cursor_reset after expiry, process restart, or repository epoch replacement."
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

## POST /v1/repos/{repo}/reviews/commands

Requires current repository write permission. The authenticated session supplies the actor and repository-bound publication identity. Apply returns a manifest-selected durable receipt or outcome_unknown; status reauthorizes and returns that receipt after restart. not_known never proves rejection. Stale review preconditions, changed retries and cross-principal identities fail without publication.

```text
{
  "operationId": "review_command_apply",
  "tags": [
    "collaboration"
  ],
  "description": "Requires current repository write permission. The authenticated session supplies the actor and repository-bound publication identity. Apply returns a manifest-selected durable receipt or outcome_unknown; status reauthorizes and returns that receipt after restart. not_known never proves rejection. Stale review preconditions, changed retries and cross-principal identities fail without publication.",
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
            "$ref": "#/components/schemas/ReviewCommandOutcome"
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
      "description": "stale_review, thread_resolved, thread_open, or intent_conflict. No publication by this rejected attempt.",
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
                    "anchor": {
                      "type": "object",
                      "properties": {
                        "schema_version": {
                          "type": "integer",
                          "format": "uint32",
                          "minimum": 0
                        },
                        "repository_id": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{32}$",
                          "minLength": 32,
                          "maxLength": 32
                        },
                        "target": {
                          "oneOf": [
                            {
                              "type": "object",
                              "properties": {
                                "target": {
                                  "type": "string",
                                  "const": "code_text"
                                },
                                "value": {
                                  "type": "object",
                                  "properties": {
                                    "operation_id": {
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{128}$",
                                      "minLength": 128,
                                      "maxLength": 128
                                    },
                                    "commit_id": {
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{128}$",
                                      "minLength": 128,
                                      "maxLength": 128
                                    },
                                    "change_id": {
                                      "type": [
                                        "string",
                                        "null"
                                      ],
                                      "pattern": "^[0-9a-f]{32}$",
                                      "minLength": 32,
                                      "maxLength": 32
                                    },
                                    "path": {
                                      "type": "string"
                                    },
                                    "file_id": {
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{128}$",
                                      "minLength": 128,
                                      "maxLength": 128
                                    },
                                    "value_kind": {
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
                                    },
                                    "text": {
                                      "type": "object",
                                      "properties": {
                                        "encoding": {
                                          "type": "string",
                                          "enum": [
                                            "utf8"
                                          ]
                                        },
                                        "start_byte": {
                                          "type": "string",
                                          "pattern": "^(0|[1-9][0-9]*)$"
                                        },
                                        "end_byte": {
                                          "type": "string",
                                          "pattern": "^(0|[1-9][0-9]*)$"
                                        },
                                        "selected_sha256": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{64}$",
                                          "minLength": 64,
                                          "maxLength": 64
                                        }
                                      },
                                      "additionalProperties": false,
                                      "required": [
                                        "encoding",
                                        "start_byte",
                                        "end_byte",
                                        "selected_sha256"
                                      ]
                                    },
                                    "diff_context": {
                                      "type": [
                                        "object",
                                        "null"
                                      ],
                                      "properties": {
                                        "before_operation_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{128}$",
                                          "minLength": 128,
                                          "maxLength": 128
                                        },
                                        "before_commit_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{128}$",
                                          "minLength": 128,
                                          "maxLength": 128
                                        },
                                        "before_path": {
                                          "type": [
                                            "string",
                                            "null"
                                          ]
                                        },
                                        "after_operation_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{128}$",
                                          "minLength": 128,
                                          "maxLength": 128
                                        },
                                        "after_commit_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{128}$",
                                          "minLength": 128,
                                          "maxLength": 128
                                        },
                                        "after_path": {
                                          "type": [
                                            "string",
                                            "null"
                                          ]
                                        },
                                        "side": {
                                          "type": "string",
                                          "enum": [
                                            "before",
                                            "after"
                                          ]
                                        },
                                        "comparison": {
                                          "type": "string",
                                          "enum": [
                                            "explicit_pair"
                                          ]
                                        }
                                      },
                                      "additionalProperties": false,
                                      "required": [
                                        "before_operation_id",
                                        "before_commit_id",
                                        "after_operation_id",
                                        "after_commit_id",
                                        "side",
                                        "comparison"
                                      ]
                                    }
                                  },
                                  "additionalProperties": false,
                                  "required": [
                                    "operation_id",
                                    "commit_id",
                                    "path",
                                    "file_id",
                                    "value_kind",
                                    "text"
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
                                  "const": "code_artifact"
                                },
                                "value": {
                                  "type": "object",
                                  "properties": {
                                    "operation_id": {
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{128}$",
                                      "minLength": 128,
                                      "maxLength": 128
                                    },
                                    "commit_id": {
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{128}$",
                                      "minLength": 128,
                                      "maxLength": 128
                                    },
                                    "change_id": {
                                      "type": [
                                        "string",
                                        "null"
                                      ],
                                      "pattern": "^[0-9a-f]{32}$",
                                      "minLength": 32,
                                      "maxLength": 32
                                    },
                                    "path": {
                                      "type": "string"
                                    },
                                    "object": {
                                      "oneOf": [
                                        {
                                          "type": "object",
                                          "properties": {
                                            "kind": {
                                              "type": "string",
                                              "const": "commit"
                                            },
                                            "id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{128}$",
                                              "minLength": 128,
                                              "maxLength": 128
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
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{128}$",
                                              "minLength": 128,
                                              "maxLength": 128
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
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{128}$",
                                              "minLength": 128,
                                              "maxLength": 128
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
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{128}$",
                                              "minLength": 128,
                                              "maxLength": 128
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
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{128}$",
                                              "minLength": 128,
                                              "maxLength": 128
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
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{128}$",
                                              "minLength": 128,
                                              "maxLength": 128
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
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{128}$",
                                              "minLength": 128,
                                              "maxLength": 128
                                            }
                                          },
                                          "required": [
                                            "kind",
                                            "id"
                                          ],
                                          "additionalProperties": false
                                        }
                                      ]
                                    },
                                    "value_kind": {
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
                                  },
                                  "additionalProperties": false,
                                  "required": [
                                    "operation_id",
                                    "commit_id",
                                    "path",
                                    "object",
                                    "value_kind"
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
                                  "const": "agent_text"
                                },
                                "value": {
                                  "type": "object",
                                  "properties": {
                                    "session_id": {
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{32}$",
                                      "minLength": 32,
                                      "maxLength": 32
                                    },
                                    "session_snapshot": {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
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
                                    "owner": {
                                      "oneOf": [
                                        {
                                          "type": "object",
                                          "properties": {
                                            "tool_call_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "snapshot": {
                                              "type": "object",
                                              "properties": {
                                                "kind": {
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
                                            "attempt_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "kind": {
                                              "type": "string",
                                              "const": "tool"
                                            }
                                          },
                                          "additionalProperties": false,
                                          "required": [
                                            "kind",
                                            "tool_call_id",
                                            "snapshot",
                                            "attempt_id"
                                          ]
                                        },
                                        {
                                          "type": "object",
                                          "properties": {
                                            "message_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "snapshot": {
                                              "type": "object",
                                              "properties": {
                                                "kind": {
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
                                            "kind": {
                                              "type": "string",
                                              "const": "message"
                                            }
                                          },
                                          "additionalProperties": false,
                                          "required": [
                                            "kind",
                                            "message_id",
                                            "snapshot"
                                          ]
                                        },
                                        {
                                          "type": "object",
                                          "properties": {
                                            "block_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "snapshot": {
                                              "type": "object",
                                              "properties": {
                                                "kind": {
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
                                            "kind": {
                                              "type": "string",
                                              "const": "plan"
                                            }
                                          },
                                          "additionalProperties": false,
                                          "required": [
                                            "kind",
                                            "block_id",
                                            "snapshot"
                                          ]
                                        }
                                      ]
                                    },
                                    "block_id": {
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{32}$",
                                      "minLength": 32,
                                      "maxLength": 32
                                    },
                                    "block_version": {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
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
                                    "part": {
                                      "type": "string",
                                      "enum": [
                                        "input",
                                        "output",
                                        "message",
                                        "plan",
                                        "exposed_summary"
                                      ]
                                    },
                                    "content": {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
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
                                      "type": "string",
                                      "enum": [
                                        "complete",
                                        "partial",
                                        "redacted",
                                        "truncated",
                                        "unknown"
                                      ]
                                    },
                                    "text": {
                                      "type": "object",
                                      "properties": {
                                        "encoding": {
                                          "type": "string",
                                          "enum": [
                                            "utf8"
                                          ]
                                        },
                                        "start_byte": {
                                          "type": "string",
                                          "pattern": "^(0|[1-9][0-9]*)$"
                                        },
                                        "end_byte": {
                                          "type": "string",
                                          "pattern": "^(0|[1-9][0-9]*)$"
                                        },
                                        "selected_sha256": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{64}$",
                                          "minLength": 64,
                                          "maxLength": 64
                                        }
                                      },
                                      "additionalProperties": false,
                                      "required": [
                                        "encoding",
                                        "start_byte",
                                        "end_byte",
                                        "selected_sha256"
                                      ]
                                    }
                                  },
                                  "additionalProperties": false,
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
                                  "type": "object",
                                  "properties": {
                                    "session_id": {
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{32}$",
                                      "minLength": 32,
                                      "maxLength": 32
                                    },
                                    "session_snapshot": {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
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
                                    "owner": {
                                      "oneOf": [
                                        {
                                          "type": "object",
                                          "properties": {
                                            "tool_call_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "snapshot": {
                                              "type": "object",
                                              "properties": {
                                                "kind": {
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
                                            "attempt_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "kind": {
                                              "type": "string",
                                              "const": "tool"
                                            }
                                          },
                                          "additionalProperties": false,
                                          "required": [
                                            "kind",
                                            "tool_call_id",
                                            "snapshot",
                                            "attempt_id"
                                          ]
                                        },
                                        {
                                          "type": "object",
                                          "properties": {
                                            "message_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "snapshot": {
                                              "type": "object",
                                              "properties": {
                                                "kind": {
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
                                            "kind": {
                                              "type": "string",
                                              "const": "message"
                                            }
                                          },
                                          "additionalProperties": false,
                                          "required": [
                                            "kind",
                                            "message_id",
                                            "snapshot"
                                          ]
                                        },
                                        {
                                          "type": "object",
                                          "properties": {
                                            "block_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "snapshot": {
                                              "type": "object",
                                              "properties": {
                                                "kind": {
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
                                            "kind": {
                                              "type": "string",
                                              "const": "plan"
                                            }
                                          },
                                          "additionalProperties": false,
                                          "required": [
                                            "kind",
                                            "block_id",
                                            "snapshot"
                                          ]
                                        }
                                      ]
                                    }
                                  },
                                  "additionalProperties": false,
                                  "required": [
                                    "session_id",
                                    "session_snapshot",
                                    "owner"
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
                                  "const": "agent_session"
                                },
                                "value": {
                                  "type": "object",
                                  "properties": {
                                    "session_id": {
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{32}$",
                                      "minLength": 32,
                                      "maxLength": 32
                                    },
                                    "session_snapshot": {
                                      "type": "object",
                                      "properties": {
                                        "kind": {
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
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{128}$",
                                      "minLength": 128,
                                      "maxLength": 128
                                    },
                                    "commit_id": {
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{128}$",
                                      "minLength": 128,
                                      "maxLength": 128
                                    },
                                    "change_id": {
                                      "type": [
                                        "string",
                                        "null"
                                      ],
                                      "pattern": "^[0-9a-f]{32}$",
                                      "minLength": 32,
                                      "maxLength": 32
                                    }
                                  },
                                  "required": [
                                    "operation_id",
                                    "commit_id"
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
                                      "type": "string",
                                      "pattern": "^[0-9a-f]{128}$",
                                      "minLength": 128,
                                      "maxLength": 128
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
                        },
                        "created_context": {
                          "type": "object",
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
                              "type": [
                                "string",
                                "null"
                              ],
                              "pattern": "^[0-9a-f]{64}$",
                              "minLength": 64,
                              "maxLength": 64
                            }
                          },
                          "additionalProperties": false
                        },
                        "sensitivity": {
                          "type": "string",
                          "enum": [
                            "public",
                            "repository",
                            "restricted"
                          ]
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "schema_version",
                        "repository_id",
                        "target",
                        "created_context",
                        "sensitivity"
                      ]
                    },
                    "body": {
                      "type": "object",
                      "properties": {
                        "schema_version": {
                          "type": "integer",
                          "format": "uint32",
                          "minimum": 0
                        },
                        "format": {
                          "type": "string",
                          "enum": [
                            "plain_text",
                            "safe_markdown"
                          ]
                        },
                        "text": {
                          "type": "string"
                        },
                        "sanitizer_version": {
                          "type": "string"
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "schema_version",
                        "format",
                        "text",
                        "sanitizer_version"
                      ]
                    },
                    "command": {
                      "type": "string",
                      "const": "create_thread"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "anchor",
                    "body"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_workflow_version": {
                      "type": "integer",
                      "format": "uint64",
                      "minimum": 0
                    },
                    "parent_comment_id": {
                      "type": [
                        "string",
                        "null"
                      ],
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "body": {
                      "type": "object",
                      "properties": {
                        "schema_version": {
                          "type": "integer",
                          "format": "uint32",
                          "minimum": 0
                        },
                        "format": {
                          "type": "string",
                          "enum": [
                            "plain_text",
                            "safe_markdown"
                          ]
                        },
                        "text": {
                          "type": "string"
                        },
                        "sanitizer_version": {
                          "type": "string"
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "schema_version",
                        "format",
                        "text",
                        "sanitizer_version"
                      ]
                    },
                    "command": {
                      "type": "string",
                      "const": "append_reply"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "thread_id",
                    "expected_workflow_version",
                    "body"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "comment_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_revision_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{64}$",
                      "minLength": 64,
                      "maxLength": 64
                    },
                    "replacement_body": {
                      "type": "object",
                      "properties": {
                        "schema_version": {
                          "type": "integer",
                          "format": "uint32",
                          "minimum": 0
                        },
                        "format": {
                          "type": "string",
                          "enum": [
                            "plain_text",
                            "safe_markdown"
                          ]
                        },
                        "text": {
                          "type": "string"
                        },
                        "sanitizer_version": {
                          "type": "string"
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "schema_version",
                        "format",
                        "text",
                        "sanitizer_version"
                      ]
                    },
                    "command": {
                      "type": "string",
                      "const": "edit_comment"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "comment_id",
                    "expected_revision_id",
                    "replacement_body"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "comment_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_revision_ids": {
                      "type": "array",
                      "items": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{64}$",
                        "minLength": 64,
                        "maxLength": 64
                      }
                    },
                    "replacement_body": {
                      "type": "object",
                      "properties": {
                        "schema_version": {
                          "type": "integer",
                          "format": "uint32",
                          "minimum": 0
                        },
                        "format": {
                          "type": "string",
                          "enum": [
                            "plain_text",
                            "safe_markdown"
                          ]
                        },
                        "text": {
                          "type": "string"
                        },
                        "sanitizer_version": {
                          "type": "string"
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "schema_version",
                        "format",
                        "text",
                        "sanitizer_version"
                      ]
                    },
                    "command": {
                      "type": "string",
                      "const": "resolve_comment_conflict"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "comment_id",
                    "expected_revision_ids",
                    "replacement_body"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "comment_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_revision_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{64}$",
                      "minLength": 64,
                      "maxLength": 64
                    },
                    "reason": {
                      "type": [
                        "string",
                        "null"
                      ]
                    },
                    "command": {
                      "type": "string",
                      "const": "tombstone_comment"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "comment_id",
                    "expected_revision_id"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "comment_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_revision_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{64}$",
                      "minLength": 64,
                      "maxLength": 64
                    },
                    "moderation_reason": {
                      "type": "string"
                    },
                    "command": {
                      "type": "string",
                      "const": "hide_comment"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "comment_id",
                    "expected_revision_id",
                    "moderation_reason"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_thread_version": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{64}$",
                      "minLength": 64,
                      "maxLength": 64
                    },
                    "reason": {
                      "type": [
                        "string",
                        "null"
                      ]
                    },
                    "command": {
                      "type": "string",
                      "const": "resolve_thread"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "thread_id",
                    "expected_thread_version"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_thread_version": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{64}$",
                      "minLength": 64,
                      "maxLength": 64
                    },
                    "reason": {
                      "type": [
                        "string",
                        "null"
                      ]
                    },
                    "command": {
                      "type": "string",
                      "const": "reopen_thread"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "thread_id",
                    "expected_thread_version"
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

## GET /v1/repos/{repo}/reviews/commands/{request_id}

Requires current repository write permission. The authenticated session supplies the actor and repository-bound publication identity. Apply returns a manifest-selected durable receipt or outcome_unknown; status reauthorizes and returns that receipt after restart. not_known never proves rejection. Stale review preconditions, changed retries and cross-principal identities fail without publication.

```text
{
  "operationId": "review_command_status",
  "tags": [
    "collaboration"
  ],
  "description": "Requires current repository write permission. The authenticated session supplies the actor and repository-bound publication identity. Apply returns a manifest-selected durable receipt or outcome_unknown; status reauthorizes and returns that receipt after restart. not_known never proves rejection. Stale review preconditions, changed retries and cross-principal identities fail without publication.",
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
            "$ref": "#/components/schemas/ReviewCommandStatus"
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
      "description": "stale_review, thread_resolved, thread_open, or intent_conflict. No publication by this rejected attempt.",
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

## HEAD /v1/repos/{repo}/reviews/commands/{request_id}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "review_command_status_head",
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
      "description": "stale_review, thread_resolved, thread_open, or intent_conflict. No publication by this rejected attempt."
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

## GET /v1/repos/{repo}/threads

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_threads",
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

## HEAD /v1/repos/{repo}/threads

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "collab_threads_head",
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

## GET /v1/repos/{repo}/threads/{thread_id}

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_thread",
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
      "name": "thread_id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/ThreadId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ThreadDetailV1"
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

## HEAD /v1/repos/{repo}/threads/{thread_id}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "collab_thread_head",
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
      "name": "thread_id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/ThreadId"
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

## GET /v1/repos/{repo}/threads/{thread_id}/timeline

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_events",
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
      "name": "thread_id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/ThreadId"
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

## HEAD /v1/repos/{repo}/threads/{thread_id}/timeline

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "collab_events_head",
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
      "name": "thread_id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/ThreadId"
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

## GET /v1/repos/{repo}/threads/{thread_id}/messages/{message_id}/revisions/{message_revision}

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_message",
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
      "name": "thread_id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/ThreadId"
      }
    },
    {
      "name": "message_id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/HumanMessageId"
      }
    },
    {
      "name": "message_revision",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/CollabObjectId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/HumanMessageDetailV1"
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

## HEAD /v1/repos/{repo}/threads/{thread_id}/messages/{message_id}/revisions/{message_revision}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "collab_message_head",
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
      "name": "thread_id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/ThreadId"
      }
    },
    {
      "name": "message_id",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/HumanMessageId"
      }
    },
    {
      "name": "message_revision",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/CollabObjectId"
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

## POST /v1/repos/{repo}/threads/commands

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_thread_apply",
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
            "command": {
              "oneOf": [
                {
                  "type": "object",
                  "properties": {
                    "submission": {
                      "type": "object",
                      "properties": {
                        "thread_id": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{32}$",
                          "minLength": 32,
                          "maxLength": 32
                        },
                        "expected_thread": {
                          "type": "object",
                          "properties": {
                            "kind": {
                              "type": "string",
                              "enum": [
                                "index_page",
                                "thread_version",
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
                        "message_id": {
                          "description": "Ordinary thread messages only; agent prompt text remains in AgentStore.",
                          "type": [
                            "string",
                            "null"
                          ],
                          "pattern": "^[0-9a-f]{32}$",
                          "minLength": 32,
                          "maxLength": 32
                        },
                        "text": {
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "reply_to": {
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
                        "comments": {
                          "type": "array",
                          "items": {
                            "type": "object",
                            "properties": {
                              "thread_id": {
                                "type": "string",
                                "pattern": "^[0-9a-f]{32}$",
                                "minLength": 32,
                                "maxLength": 32
                              },
                              "expected_thread": {
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
                              "source": {
                                "type": "object",
                                "properties": {
                                  "target": {
                                    "description": "Identity, immutable content and validation witnesses are explicit, never a UI position.",
                                    "oneOf": [
                                      {
                                        "type": "object",
                                        "properties": {
                                          "thread_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "type": {
                                            "type": "string",
                                            "const": "thread"
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "thread_id"
                                        ]
                                      },
                                      {
                                        "type": "object",
                                        "properties": {
                                          "thread_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "message_id": {
                                            "type": "string",
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
                                          "type": {
                                            "type": "string",
                                            "const": "human_message"
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "thread_id",
                                          "message_id",
                                          "revision"
                                        ]
                                      },
                                      {
                                        "type": "object",
                                        "properties": {
                                          "session_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "message_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "event_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "batch": {
                                            "type": "object",
                                            "properties": {
                                              "kind": {
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
                                          "content": {
                                            "type": "object",
                                            "properties": {
                                              "kind": {
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
                                          "type": {
                                            "type": "string",
                                            "const": "agent_message"
                                          }
                                        },
                                        "additionalProperties": false,
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
                                        "properties": {
                                          "session_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "tool_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "snapshot": {
                                            "type": "object",
                                            "properties": {
                                              "kind": {
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
                                          "type": {
                                            "type": "string",
                                            "const": "tool_invocation"
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "session_id",
                                          "tool_id",
                                          "snapshot"
                                        ]
                                      },
                                      {
                                        "type": "object",
                                        "properties": {
                                          "session_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "tool_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "snapshot": {
                                            "type": "object",
                                            "properties": {
                                              "kind": {
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
                                          "content": {
                                            "type": "object",
                                            "properties": {
                                              "kind": {
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
                                          "type": {
                                            "type": "string",
                                            "const": "tool_output"
                                          }
                                        },
                                        "additionalProperties": false,
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
                                        "properties": {
                                          "thread_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "type": {
                                            "type": "string",
                                            "const": "review_thread"
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "thread_id"
                                        ]
                                      },
                                      {
                                        "type": "object",
                                        "properties": {
                                          "thread_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "comment_id": {
                                            "type": "string",
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
                                                  "anchor",
                                                  "comment_body",
                                                  "comment_revision",
                                                  "thread_event",
                                                  "event_page",
                                                  "thread_snapshot",
                                                  "relation_page",
                                                  "status_page"
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
                                          "type": {
                                            "type": "string",
                                            "const": "review_comment"
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "thread_id",
                                          "comment_id",
                                          "revision"
                                        ]
                                      },
                                      {
                                        "type": "object",
                                        "properties": {
                                          "task_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "comment_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
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
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "task_id",
                                          "comment_id",
                                          "version"
                                        ]
                                      },
                                      {
                                        "type": "object",
                                        "properties": {
                                          "session_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "checkpoint_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "version": {
                                            "type": "object",
                                            "properties": {
                                              "kind": {
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
                                          "type": {
                                            "type": "string",
                                            "const": "agent_checkpoint"
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "session_id",
                                          "checkpoint_id",
                                          "version"
                                        ]
                                      }
                                    ]
                                  },
                                  "part": {
                                    "description": "JSON pointer to an exact string in captured JSON; None selects plain text.",
                                    "type": [
                                      "string",
                                      "null"
                                    ]
                                  },
                                  "selection": {
                                    "type": [
                                      "object",
                                      "null"
                                    ],
                                    "properties": {
                                      "encoding": {
                                        "type": "string",
                                        "enum": [
                                          "utf8"
                                        ]
                                      },
                                      "start_byte": {
                                        "type": "string",
                                        "pattern": "^(0|[1-9][0-9]*)$"
                                      },
                                      "end_byte": {
                                        "type": "string",
                                        "pattern": "^(0|[1-9][0-9]*)$"
                                      },
                                      "selected_sha256": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{64}$",
                                        "minLength": 64,
                                        "maxLength": 64
                                      }
                                    },
                                    "additionalProperties": false,
                                    "required": [
                                      "encoding",
                                      "start_byte",
                                      "end_byte",
                                      "selected_sha256"
                                    ]
                                  },
                                  "quote": {
                                    "description": "Retained display evidence, validated against the selected bytes.",
                                    "type": "string"
                                  }
                                },
                                "additionalProperties": false,
                                "required": [
                                  "target",
                                  "quote"
                                ]
                              },
                              "message_id": {
                                "type": "string",
                                "pattern": "^[0-9a-f]{32}$",
                                "minLength": 32,
                                "maxLength": 32
                              },
                              "expected_revision": {
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
                              "reply_to": {
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
                              "body": {
                                "type": "string"
                              }
                            },
                            "additionalProperties": false,
                            "required": [
                              "thread_id",
                              "source",
                              "message_id",
                              "body"
                            ]
                          }
                        },
                        "references": {
                          "type": "array",
                          "items": {
                            "description": "Identity, immutable content and validation witnesses are explicit, never a UI position.",
                            "oneOf": [
                              {
                                "type": "object",
                                "properties": {
                                  "thread_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "type": {
                                    "type": "string",
                                    "const": "thread"
                                  }
                                },
                                "additionalProperties": false,
                                "required": [
                                  "type",
                                  "thread_id"
                                ]
                              },
                              {
                                "type": "object",
                                "properties": {
                                  "thread_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "message_id": {
                                    "type": "string",
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
                                  "type": {
                                    "type": "string",
                                    "const": "human_message"
                                  }
                                },
                                "additionalProperties": false,
                                "required": [
                                  "type",
                                  "thread_id",
                                  "message_id",
                                  "revision"
                                ]
                              },
                              {
                                "type": "object",
                                "properties": {
                                  "session_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "message_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "event_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "batch": {
                                    "type": "object",
                                    "properties": {
                                      "kind": {
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
                                  "content": {
                                    "type": "object",
                                    "properties": {
                                      "kind": {
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
                                  "type": {
                                    "type": "string",
                                    "const": "agent_message"
                                  }
                                },
                                "additionalProperties": false,
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
                                "properties": {
                                  "session_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "tool_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "snapshot": {
                                    "type": "object",
                                    "properties": {
                                      "kind": {
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
                                  "type": {
                                    "type": "string",
                                    "const": "tool_invocation"
                                  }
                                },
                                "additionalProperties": false,
                                "required": [
                                  "type",
                                  "session_id",
                                  "tool_id",
                                  "snapshot"
                                ]
                              },
                              {
                                "type": "object",
                                "properties": {
                                  "session_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "tool_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "snapshot": {
                                    "type": "object",
                                    "properties": {
                                      "kind": {
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
                                  "content": {
                                    "type": "object",
                                    "properties": {
                                      "kind": {
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
                                  "type": {
                                    "type": "string",
                                    "const": "tool_output"
                                  }
                                },
                                "additionalProperties": false,
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
                                "properties": {
                                  "thread_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "type": {
                                    "type": "string",
                                    "const": "review_thread"
                                  }
                                },
                                "additionalProperties": false,
                                "required": [
                                  "type",
                                  "thread_id"
                                ]
                              },
                              {
                                "type": "object",
                                "properties": {
                                  "thread_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "comment_id": {
                                    "type": "string",
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
                                          "anchor",
                                          "comment_body",
                                          "comment_revision",
                                          "thread_event",
                                          "event_page",
                                          "thread_snapshot",
                                          "relation_page",
                                          "status_page"
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
                                  "type": {
                                    "type": "string",
                                    "const": "review_comment"
                                  }
                                },
                                "additionalProperties": false,
                                "required": [
                                  "type",
                                  "thread_id",
                                  "comment_id",
                                  "revision"
                                ]
                              },
                              {
                                "type": "object",
                                "properties": {
                                  "task_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "comment_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
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
                                "additionalProperties": false,
                                "required": [
                                  "type",
                                  "task_id",
                                  "comment_id",
                                  "version"
                                ]
                              },
                              {
                                "type": "object",
                                "properties": {
                                  "session_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "checkpoint_id": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{32}$",
                                    "minLength": 32,
                                    "maxLength": 32
                                  },
                                  "version": {
                                    "type": "object",
                                    "properties": {
                                      "kind": {
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
                                  "type": {
                                    "type": "string",
                                    "const": "agent_checkpoint"
                                  }
                                },
                                "additionalProperties": false,
                                "required": [
                                  "type",
                                  "session_id",
                                  "checkpoint_id",
                                  "version"
                                ]
                              }
                            ]
                          }
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "thread_id",
                        "expected_thread",
                        "comments",
                        "references"
                      ]
                    },
                    "type": {
                      "type": "string",
                      "const": "submit"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "submission"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "title": {
                      "type": "string"
                    },
                    "source": {
                      "type": "object",
                      "properties": {
                        "target": {
                          "description": "Identity, immutable content and validation witnesses are explicit, never a UI position.",
                          "oneOf": [
                            {
                              "type": "object",
                              "properties": {
                                "thread_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "type": {
                                  "type": "string",
                                  "const": "thread"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "type",
                                "thread_id"
                              ]
                            },
                            {
                              "type": "object",
                              "properties": {
                                "thread_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "message_id": {
                                  "type": "string",
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
                                "type": {
                                  "type": "string",
                                  "const": "human_message"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "type",
                                "thread_id",
                                "message_id",
                                "revision"
                              ]
                            },
                            {
                              "type": "object",
                              "properties": {
                                "session_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "message_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "event_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "batch": {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
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
                                "content": {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
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
                                "type": {
                                  "type": "string",
                                  "const": "agent_message"
                                }
                              },
                              "additionalProperties": false,
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
                              "properties": {
                                "session_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "tool_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "snapshot": {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
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
                                "type": {
                                  "type": "string",
                                  "const": "tool_invocation"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "type",
                                "session_id",
                                "tool_id",
                                "snapshot"
                              ]
                            },
                            {
                              "type": "object",
                              "properties": {
                                "session_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "tool_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "snapshot": {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
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
                                "content": {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
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
                                "type": {
                                  "type": "string",
                                  "const": "tool_output"
                                }
                              },
                              "additionalProperties": false,
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
                              "properties": {
                                "thread_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "type": {
                                  "type": "string",
                                  "const": "review_thread"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "type",
                                "thread_id"
                              ]
                            },
                            {
                              "type": "object",
                              "properties": {
                                "thread_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "comment_id": {
                                  "type": "string",
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
                                        "anchor",
                                        "comment_body",
                                        "comment_revision",
                                        "thread_event",
                                        "event_page",
                                        "thread_snapshot",
                                        "relation_page",
                                        "status_page"
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
                                "type": {
                                  "type": "string",
                                  "const": "review_comment"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "type",
                                "thread_id",
                                "comment_id",
                                "revision"
                              ]
                            },
                            {
                              "type": "object",
                              "properties": {
                                "task_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "comment_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
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
                              "additionalProperties": false,
                              "required": [
                                "type",
                                "task_id",
                                "comment_id",
                                "version"
                              ]
                            },
                            {
                              "type": "object",
                              "properties": {
                                "session_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "checkpoint_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "version": {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
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
                                "type": {
                                  "type": "string",
                                  "const": "agent_checkpoint"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "type",
                                "session_id",
                                "checkpoint_id",
                                "version"
                              ]
                            }
                          ]
                        },
                        "part": {
                          "description": "JSON pointer to an exact string in captured JSON; None selects plain text.",
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "selection": {
                          "type": [
                            "object",
                            "null"
                          ],
                          "properties": {
                            "encoding": {
                              "type": "string",
                              "enum": [
                                "utf8"
                              ]
                            },
                            "start_byte": {
                              "type": "string",
                              "pattern": "^(0|[1-9][0-9]*)$"
                            },
                            "end_byte": {
                              "type": "string",
                              "pattern": "^(0|[1-9][0-9]*)$"
                            },
                            "selected_sha256": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{64}$",
                              "minLength": 64,
                              "maxLength": 64
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "encoding",
                            "start_byte",
                            "end_byte",
                            "selected_sha256"
                          ]
                        },
                        "quote": {
                          "description": "Retained display evidence, validated against the selected bytes.",
                          "type": "string"
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "target",
                        "quote"
                      ]
                    },
                    "type": {
                      "type": "string",
                      "const": "create_comment_discussion"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "thread_id",
                    "title",
                    "source"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "title": {
                      "type": "string"
                    },
                    "target": {
                      "description": "Identity, immutable content and validation witnesses are explicit, never a UI position.",
                      "oneOf": [
                        {
                          "type": "object",
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "type": {
                              "type": "string",
                              "const": "thread"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "message_id": {
                              "type": "string",
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
                            "type": {
                              "type": "string",
                              "const": "human_message"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id",
                            "message_id",
                            "revision"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "message_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "event_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "batch": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "content": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "agent_message"
                            }
                          },
                          "additionalProperties": false,
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
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "tool_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "snapshot": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "tool_invocation"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "session_id",
                            "tool_id",
                            "snapshot"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "tool_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "snapshot": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "content": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "tool_output"
                            }
                          },
                          "additionalProperties": false,
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
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "type": {
                              "type": "string",
                              "const": "review_thread"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "comment_id": {
                              "type": "string",
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
                                    "anchor",
                                    "comment_body",
                                    "comment_revision",
                                    "thread_event",
                                    "event_page",
                                    "thread_snapshot",
                                    "relation_page",
                                    "status_page"
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
                            "type": {
                              "type": "string",
                              "const": "review_comment"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id",
                            "comment_id",
                            "revision"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "comment_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
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
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "task_id",
                            "comment_id",
                            "version"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "checkpoint_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "version": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "agent_checkpoint"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "session_id",
                            "checkpoint_id",
                            "version"
                          ]
                        }
                      ]
                    },
                    "type": {
                      "type": "string",
                      "const": "create_agent_discussion"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "thread_id",
                    "title",
                    "target"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "title": {
                      "type": "string"
                    },
                    "baseline": {
                      "anyOf": [
                        {
                          "description": "Native IDs and portable evidence remain distinct; neither is a device path.",
                          "oneOf": [
                            {
                              "type": "object",
                              "properties": {
                                "profile": {
                                  "type": "string",
                                  "const": "native"
                                },
                                "anchor": {
                                  "type": "object",
                                  "properties": {
                                    "operations": {
                                      "type": "array",
                                      "uniqueItems": true,
                                      "items": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{128}$",
                                        "minLength": 128,
                                        "maxLength": 128
                                      }
                                    },
                                    "commits": {
                                      "type": "array",
                                      "uniqueItems": true,
                                      "items": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{128}$",
                                        "minLength": 128,
                                        "maxLength": 128
                                      }
                                    }
                                  },
                                  "additionalProperties": false,
                                  "required": [
                                    "operations",
                                    "commits"
                                  ]
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
                                  "type": "object",
                                  "properties": {
                                    "operations": {
                                      "type": "array",
                                      "uniqueItems": true,
                                      "items": {
                                        "type": "object",
                                        "properties": {
                                          "repository": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "profile": {
                                            "type": "string",
                                            "enum": [
                                              "native_v1",
                                              "git_sha1_jj045"
                                            ]
                                          },
                                          "operation": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{128}$",
                                            "minLength": 128,
                                            "maxLength": 128
                                          },
                                          "capsule": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{64}$",
                                            "minLength": 64,
                                            "maxLength": 64
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "repository",
                                          "profile",
                                          "operation",
                                          "capsule"
                                        ]
                                      }
                                    },
                                    "commits": {
                                      "type": "array",
                                      "uniqueItems": true,
                                      "items": {
                                        "type": "object",
                                        "properties": {
                                          "evidence": {
                                            "type": "object",
                                            "properties": {
                                              "repository": {
                                                "type": "string",
                                                "pattern": "^[0-9a-f]{32}$",
                                                "minLength": 32,
                                                "maxLength": 32
                                              },
                                              "profile": {
                                                "type": "string",
                                                "enum": [
                                                  "native_v1",
                                                  "git_sha1_jj045"
                                                ]
                                              },
                                              "operation": {
                                                "type": "string",
                                                "pattern": "^[0-9a-f]{128}$",
                                                "minLength": 128,
                                                "maxLength": 128
                                              },
                                              "capsule": {
                                                "type": "string",
                                                "pattern": "^[0-9a-f]{64}$",
                                                "minLength": 64,
                                                "maxLength": 64
                                              }
                                            },
                                            "additionalProperties": false,
                                            "required": [
                                              "repository",
                                              "profile",
                                              "operation",
                                              "capsule"
                                            ]
                                          },
                                          "commit": {
                                            "oneOf": [
                                              {
                                                "type": "object",
                                                "properties": {
                                                  "profile": {
                                                    "type": "string",
                                                    "const": "native_v1"
                                                  },
                                                  "id": {
                                                    "type": "string",
                                                    "pattern": "^[0-9a-f]{128}$",
                                                    "minLength": 128,
                                                    "maxLength": 128
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
                                                    "type": "string",
                                                    "pattern": "^[0-9a-f]{40}$",
                                                    "minLength": 40,
                                                    "maxLength": 40
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
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "evidence",
                                          "commit"
                                        ]
                                      }
                                    }
                                  },
                                  "additionalProperties": false,
                                  "required": [
                                    "operations",
                                    "commits"
                                  ]
                                }
                              },
                              "required": [
                                "profile",
                                "anchor"
                              ],
                              "additionalProperties": false
                            }
                          ]
                        },
                        {
                          "type": "null"
                        }
                      ]
                    },
                    "type": {
                      "type": "string",
                      "const": "create"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "thread_id",
                    "title"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected": {
                      "type": "object",
                      "properties": {
                        "kind": {
                          "type": "string",
                          "enum": [
                            "index_page",
                            "thread_version",
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
                    "title": {
                      "type": "string"
                    },
                    "type": {
                      "type": "string",
                      "const": "rename"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "thread_id",
                    "expected",
                    "title"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected": {
                      "type": "object",
                      "properties": {
                        "kind": {
                          "type": "string",
                          "enum": [
                            "index_page",
                            "thread_version",
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
                    "action": {
                      "type": "string",
                      "enum": [
                        "activate",
                        "idle",
                        "archive",
                        "tombstone",
                        "restore"
                      ]
                    },
                    "type": {
                      "type": "string",
                      "const": "lifecycle"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "thread_id",
                    "expected",
                    "action"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_thread": {
                      "type": "object",
                      "properties": {
                        "kind": {
                          "type": "string",
                          "enum": [
                            "index_page",
                            "thread_version",
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
                    "message_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_revision": {
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
                    "reply_to": {
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
                    "body": {
                      "type": [
                        "string",
                        "null"
                      ]
                    },
                    "type": {
                      "type": "string",
                      "const": "message"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "thread_id",
                    "expected_thread",
                    "message_id"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_thread": {
                      "type": "object",
                      "properties": {
                        "kind": {
                          "type": "string",
                          "enum": [
                            "index_page",
                            "thread_version",
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
                    "link_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "target": {
                      "oneOf": [
                        {
                          "type": "object",
                          "properties": {
                            "reply": {
                              "type": "object",
                              "properties": {
                                "prompt": {
                                  "description": "Exact user message capture containing a successful context delivery.",
                                  "oneOf": [
                                    {
                                      "type": "object",
                                      "properties": {
                                        "thread_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "type": {
                                          "type": "string",
                                          "const": "thread"
                                        }
                                      },
                                      "additionalProperties": false,
                                      "required": [
                                        "type",
                                        "thread_id"
                                      ]
                                    },
                                    {
                                      "type": "object",
                                      "properties": {
                                        "thread_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "message_id": {
                                          "type": "string",
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
                                        "type": {
                                          "type": "string",
                                          "const": "human_message"
                                        }
                                      },
                                      "additionalProperties": false,
                                      "required": [
                                        "type",
                                        "thread_id",
                                        "message_id",
                                        "revision"
                                      ]
                                    },
                                    {
                                      "type": "object",
                                      "properties": {
                                        "session_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "message_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "event_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "batch": {
                                          "type": "object",
                                          "properties": {
                                            "kind": {
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
                                        "content": {
                                          "type": "object",
                                          "properties": {
                                            "kind": {
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
                                        "type": {
                                          "type": "string",
                                          "const": "agent_message"
                                        }
                                      },
                                      "additionalProperties": false,
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
                                      "properties": {
                                        "session_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "tool_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "snapshot": {
                                          "type": "object",
                                          "properties": {
                                            "kind": {
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
                                        "type": {
                                          "type": "string",
                                          "const": "tool_invocation"
                                        }
                                      },
                                      "additionalProperties": false,
                                      "required": [
                                        "type",
                                        "session_id",
                                        "tool_id",
                                        "snapshot"
                                      ]
                                    },
                                    {
                                      "type": "object",
                                      "properties": {
                                        "session_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "tool_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "snapshot": {
                                          "type": "object",
                                          "properties": {
                                            "kind": {
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
                                        "content": {
                                          "type": "object",
                                          "properties": {
                                            "kind": {
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
                                        "type": {
                                          "type": "string",
                                          "const": "tool_output"
                                        }
                                      },
                                      "additionalProperties": false,
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
                                      "properties": {
                                        "thread_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "type": {
                                          "type": "string",
                                          "const": "review_thread"
                                        }
                                      },
                                      "additionalProperties": false,
                                      "required": [
                                        "type",
                                        "thread_id"
                                      ]
                                    },
                                    {
                                      "type": "object",
                                      "properties": {
                                        "thread_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "comment_id": {
                                          "type": "string",
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
                                                "anchor",
                                                "comment_body",
                                                "comment_revision",
                                                "thread_event",
                                                "event_page",
                                                "thread_snapshot",
                                                "relation_page",
                                                "status_page"
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
                                        "type": {
                                          "type": "string",
                                          "const": "review_comment"
                                        }
                                      },
                                      "additionalProperties": false,
                                      "required": [
                                        "type",
                                        "thread_id",
                                        "comment_id",
                                        "revision"
                                      ]
                                    },
                                    {
                                      "type": "object",
                                      "properties": {
                                        "task_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "comment_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
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
                                      "additionalProperties": false,
                                      "required": [
                                        "type",
                                        "task_id",
                                        "comment_id",
                                        "version"
                                      ]
                                    },
                                    {
                                      "type": "object",
                                      "properties": {
                                        "session_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "checkpoint_id": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{32}$",
                                          "minLength": 32,
                                          "maxLength": 32
                                        },
                                        "version": {
                                          "type": "object",
                                          "properties": {
                                            "kind": {
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
                                        "type": {
                                          "type": "string",
                                          "const": "agent_checkpoint"
                                        }
                                      },
                                      "additionalProperties": false,
                                      "required": [
                                        "type",
                                        "session_id",
                                        "checkpoint_id",
                                        "version"
                                      ]
                                    }
                                  ]
                                },
                                "answer": {
                                  "type": "object",
                                  "properties": {
                                    "target": {
                                      "description": "Identity, immutable content and validation witnesses are explicit, never a UI position.",
                                      "oneOf": [
                                        {
                                          "type": "object",
                                          "properties": {
                                            "thread_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "type": {
                                              "type": "string",
                                              "const": "thread"
                                            }
                                          },
                                          "additionalProperties": false,
                                          "required": [
                                            "type",
                                            "thread_id"
                                          ]
                                        },
                                        {
                                          "type": "object",
                                          "properties": {
                                            "thread_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "message_id": {
                                              "type": "string",
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
                                            "type": {
                                              "type": "string",
                                              "const": "human_message"
                                            }
                                          },
                                          "additionalProperties": false,
                                          "required": [
                                            "type",
                                            "thread_id",
                                            "message_id",
                                            "revision"
                                          ]
                                        },
                                        {
                                          "type": "object",
                                          "properties": {
                                            "session_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "message_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "event_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "batch": {
                                              "type": "object",
                                              "properties": {
                                                "kind": {
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
                                            "content": {
                                              "type": "object",
                                              "properties": {
                                                "kind": {
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
                                            "type": {
                                              "type": "string",
                                              "const": "agent_message"
                                            }
                                          },
                                          "additionalProperties": false,
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
                                          "properties": {
                                            "session_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "tool_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "snapshot": {
                                              "type": "object",
                                              "properties": {
                                                "kind": {
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
                                            "type": {
                                              "type": "string",
                                              "const": "tool_invocation"
                                            }
                                          },
                                          "additionalProperties": false,
                                          "required": [
                                            "type",
                                            "session_id",
                                            "tool_id",
                                            "snapshot"
                                          ]
                                        },
                                        {
                                          "type": "object",
                                          "properties": {
                                            "session_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "tool_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "snapshot": {
                                              "type": "object",
                                              "properties": {
                                                "kind": {
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
                                            "content": {
                                              "type": "object",
                                              "properties": {
                                                "kind": {
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
                                            "type": {
                                              "type": "string",
                                              "const": "tool_output"
                                            }
                                          },
                                          "additionalProperties": false,
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
                                          "properties": {
                                            "thread_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "type": {
                                              "type": "string",
                                              "const": "review_thread"
                                            }
                                          },
                                          "additionalProperties": false,
                                          "required": [
                                            "type",
                                            "thread_id"
                                          ]
                                        },
                                        {
                                          "type": "object",
                                          "properties": {
                                            "thread_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "comment_id": {
                                              "type": "string",
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
                                                    "anchor",
                                                    "comment_body",
                                                    "comment_revision",
                                                    "thread_event",
                                                    "event_page",
                                                    "thread_snapshot",
                                                    "relation_page",
                                                    "status_page"
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
                                            "type": {
                                              "type": "string",
                                              "const": "review_comment"
                                            }
                                          },
                                          "additionalProperties": false,
                                          "required": [
                                            "type",
                                            "thread_id",
                                            "comment_id",
                                            "revision"
                                          ]
                                        },
                                        {
                                          "type": "object",
                                          "properties": {
                                            "task_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "comment_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
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
                                          "additionalProperties": false,
                                          "required": [
                                            "type",
                                            "task_id",
                                            "comment_id",
                                            "version"
                                          ]
                                        },
                                        {
                                          "type": "object",
                                          "properties": {
                                            "session_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "checkpoint_id": {
                                              "type": "string",
                                              "pattern": "^[0-9a-f]{32}$",
                                              "minLength": 32,
                                              "maxLength": 32
                                            },
                                            "version": {
                                              "type": "object",
                                              "properties": {
                                                "kind": {
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
                                            "type": {
                                              "type": "string",
                                              "const": "agent_checkpoint"
                                            }
                                          },
                                          "additionalProperties": false,
                                          "required": [
                                            "type",
                                            "session_id",
                                            "checkpoint_id",
                                            "version"
                                          ]
                                        }
                                      ]
                                    },
                                    "part": {
                                      "description": "JSON pointer to an exact string in captured JSON; None selects plain text.",
                                      "type": [
                                        "string",
                                        "null"
                                      ]
                                    },
                                    "selection": {
                                      "type": [
                                        "object",
                                        "null"
                                      ],
                                      "properties": {
                                        "encoding": {
                                          "type": "string",
                                          "enum": [
                                            "utf8"
                                          ]
                                        },
                                        "start_byte": {
                                          "type": "string",
                                          "pattern": "^(0|[1-9][0-9]*)$"
                                        },
                                        "end_byte": {
                                          "type": "string",
                                          "pattern": "^(0|[1-9][0-9]*)$"
                                        },
                                        "selected_sha256": {
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{64}$",
                                          "minLength": 64,
                                          "maxLength": 64
                                        }
                                      },
                                      "additionalProperties": false,
                                      "required": [
                                        "encoding",
                                        "start_byte",
                                        "end_byte",
                                        "selected_sha256"
                                      ]
                                    },
                                    "quote": {
                                      "description": "Retained display evidence, validated against the selected bytes.",
                                      "type": "string"
                                    }
                                  },
                                  "additionalProperties": false,
                                  "required": [
                                    "target",
                                    "quote"
                                  ]
                                },
                                "comments": {
                                  "type": "array",
                                  "items": {
                                    "description": "Identity, immutable content and validation witnesses are explicit, never a UI position.",
                                    "oneOf": [
                                      {
                                        "type": "object",
                                        "properties": {
                                          "thread_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "type": {
                                            "type": "string",
                                            "const": "thread"
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "thread_id"
                                        ]
                                      },
                                      {
                                        "type": "object",
                                        "properties": {
                                          "thread_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "message_id": {
                                            "type": "string",
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
                                          "type": {
                                            "type": "string",
                                            "const": "human_message"
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "thread_id",
                                          "message_id",
                                          "revision"
                                        ]
                                      },
                                      {
                                        "type": "object",
                                        "properties": {
                                          "session_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "message_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "event_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "batch": {
                                            "type": "object",
                                            "properties": {
                                              "kind": {
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
                                          "content": {
                                            "type": "object",
                                            "properties": {
                                              "kind": {
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
                                          "type": {
                                            "type": "string",
                                            "const": "agent_message"
                                          }
                                        },
                                        "additionalProperties": false,
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
                                        "properties": {
                                          "session_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "tool_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "snapshot": {
                                            "type": "object",
                                            "properties": {
                                              "kind": {
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
                                          "type": {
                                            "type": "string",
                                            "const": "tool_invocation"
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "session_id",
                                          "tool_id",
                                          "snapshot"
                                        ]
                                      },
                                      {
                                        "type": "object",
                                        "properties": {
                                          "session_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "tool_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "snapshot": {
                                            "type": "object",
                                            "properties": {
                                              "kind": {
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
                                          "content": {
                                            "type": "object",
                                            "properties": {
                                              "kind": {
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
                                          "type": {
                                            "type": "string",
                                            "const": "tool_output"
                                          }
                                        },
                                        "additionalProperties": false,
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
                                        "properties": {
                                          "thread_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "type": {
                                            "type": "string",
                                            "const": "review_thread"
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "thread_id"
                                        ]
                                      },
                                      {
                                        "type": "object",
                                        "properties": {
                                          "thread_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "comment_id": {
                                            "type": "string",
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
                                                  "anchor",
                                                  "comment_body",
                                                  "comment_revision",
                                                  "thread_event",
                                                  "event_page",
                                                  "thread_snapshot",
                                                  "relation_page",
                                                  "status_page"
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
                                          "type": {
                                            "type": "string",
                                            "const": "review_comment"
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "thread_id",
                                          "comment_id",
                                          "revision"
                                        ]
                                      },
                                      {
                                        "type": "object",
                                        "properties": {
                                          "task_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "comment_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
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
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "task_id",
                                          "comment_id",
                                          "version"
                                        ]
                                      },
                                      {
                                        "type": "object",
                                        "properties": {
                                          "session_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "checkpoint_id": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{32}$",
                                            "minLength": 32,
                                            "maxLength": 32
                                          },
                                          "version": {
                                            "type": "object",
                                            "properties": {
                                              "kind": {
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
                                          "type": {
                                            "type": "string",
                                            "const": "agent_checkpoint"
                                          }
                                        },
                                        "additionalProperties": false,
                                        "required": [
                                          "type",
                                          "session_id",
                                          "checkpoint_id",
                                          "version"
                                        ]
                                      }
                                    ]
                                  }
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "prompt",
                                "answer",
                                "comments"
                              ]
                            },
                            "type": {
                              "type": "string",
                              "const": "comment_reply"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "reply"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "request_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "comments": {
                              "type": "array",
                              "items": {
                                "description": "Identity, immutable content and validation witnesses are explicit, never a UI position.",
                                "oneOf": [
                                  {
                                    "type": "object",
                                    "properties": {
                                      "thread_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "type": {
                                        "type": "string",
                                        "const": "thread"
                                      }
                                    },
                                    "additionalProperties": false,
                                    "required": [
                                      "type",
                                      "thread_id"
                                    ]
                                  },
                                  {
                                    "type": "object",
                                    "properties": {
                                      "thread_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "message_id": {
                                        "type": "string",
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
                                      "type": {
                                        "type": "string",
                                        "const": "human_message"
                                      }
                                    },
                                    "additionalProperties": false,
                                    "required": [
                                      "type",
                                      "thread_id",
                                      "message_id",
                                      "revision"
                                    ]
                                  },
                                  {
                                    "type": "object",
                                    "properties": {
                                      "session_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "message_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "event_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "batch": {
                                        "type": "object",
                                        "properties": {
                                          "kind": {
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
                                      "content": {
                                        "type": "object",
                                        "properties": {
                                          "kind": {
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
                                      "type": {
                                        "type": "string",
                                        "const": "agent_message"
                                      }
                                    },
                                    "additionalProperties": false,
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
                                    "properties": {
                                      "session_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "tool_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "snapshot": {
                                        "type": "object",
                                        "properties": {
                                          "kind": {
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
                                      "type": {
                                        "type": "string",
                                        "const": "tool_invocation"
                                      }
                                    },
                                    "additionalProperties": false,
                                    "required": [
                                      "type",
                                      "session_id",
                                      "tool_id",
                                      "snapshot"
                                    ]
                                  },
                                  {
                                    "type": "object",
                                    "properties": {
                                      "session_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "tool_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "snapshot": {
                                        "type": "object",
                                        "properties": {
                                          "kind": {
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
                                      "content": {
                                        "type": "object",
                                        "properties": {
                                          "kind": {
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
                                      "type": {
                                        "type": "string",
                                        "const": "tool_output"
                                      }
                                    },
                                    "additionalProperties": false,
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
                                    "properties": {
                                      "thread_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "type": {
                                        "type": "string",
                                        "const": "review_thread"
                                      }
                                    },
                                    "additionalProperties": false,
                                    "required": [
                                      "type",
                                      "thread_id"
                                    ]
                                  },
                                  {
                                    "type": "object",
                                    "properties": {
                                      "thread_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "comment_id": {
                                        "type": "string",
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
                                              "anchor",
                                              "comment_body",
                                              "comment_revision",
                                              "thread_event",
                                              "event_page",
                                              "thread_snapshot",
                                              "relation_page",
                                              "status_page"
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
                                      "type": {
                                        "type": "string",
                                        "const": "review_comment"
                                      }
                                    },
                                    "additionalProperties": false,
                                    "required": [
                                      "type",
                                      "thread_id",
                                      "comment_id",
                                      "revision"
                                    ]
                                  },
                                  {
                                    "type": "object",
                                    "properties": {
                                      "task_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "comment_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
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
                                    "additionalProperties": false,
                                    "required": [
                                      "type",
                                      "task_id",
                                      "comment_id",
                                      "version"
                                    ]
                                  },
                                  {
                                    "type": "object",
                                    "properties": {
                                      "session_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "checkpoint_id": {
                                        "type": "string",
                                        "pattern": "^[0-9a-f]{32}$",
                                        "minLength": 32,
                                        "maxLength": 32
                                      },
                                      "version": {
                                        "type": "object",
                                        "properties": {
                                          "kind": {
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
                                      "type": {
                                        "type": "string",
                                        "const": "agent_checkpoint"
                                      }
                                    },
                                    "additionalProperties": false,
                                    "required": [
                                      "type",
                                      "session_id",
                                      "checkpoint_id",
                                      "version"
                                    ]
                                  }
                                ]
                              }
                            },
                            "message": {
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
                            "reply_to": {
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
                            "type": {
                              "type": "string",
                              "const": "comment_batch"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "request_id",
                            "comments"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "source_state": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "event_batch": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "binding": {
                              "description": "Capture adapters may bind explicitly. Absence is the supported unlinked state.",
                              "type": "object",
                              "properties": {
                                "thread_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "observed_thread": {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
                                      "type": "string",
                                      "enum": [
                                        "index_page",
                                        "thread_version",
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
                                "observed_assignment": {
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
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "thread_id",
                                "observed_thread"
                              ]
                            },
                            "type": {
                              "type": "string",
                              "const": "capture_gap"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "source_state",
                            "event_batch",
                            "binding"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "snapshot": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "primary": {
                              "type": "boolean"
                            },
                            "type": {
                              "type": "string",
                              "const": "session"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "session_id",
                            "snapshot",
                            "primary"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "anchor": {
                              "description": "Native IDs and portable evidence remain distinct; neither is a device path.",
                              "oneOf": [
                                {
                                  "type": "object",
                                  "properties": {
                                    "profile": {
                                      "type": "string",
                                      "const": "native"
                                    },
                                    "anchor": {
                                      "type": "object",
                                      "properties": {
                                        "operations": {
                                          "type": "array",
                                          "uniqueItems": true,
                                          "items": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{128}$",
                                            "minLength": 128,
                                            "maxLength": 128
                                          }
                                        },
                                        "commits": {
                                          "type": "array",
                                          "uniqueItems": true,
                                          "items": {
                                            "type": "string",
                                            "pattern": "^[0-9a-f]{128}$",
                                            "minLength": 128,
                                            "maxLength": 128
                                          }
                                        }
                                      },
                                      "additionalProperties": false,
                                      "required": [
                                        "operations",
                                        "commits"
                                      ]
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
                                      "type": "object",
                                      "properties": {
                                        "operations": {
                                          "type": "array",
                                          "uniqueItems": true,
                                          "items": {
                                            "type": "object",
                                            "properties": {
                                              "repository": {
                                                "type": "string",
                                                "pattern": "^[0-9a-f]{32}$",
                                                "minLength": 32,
                                                "maxLength": 32
                                              },
                                              "profile": {
                                                "type": "string",
                                                "enum": [
                                                  "native_v1",
                                                  "git_sha1_jj045"
                                                ]
                                              },
                                              "operation": {
                                                "type": "string",
                                                "pattern": "^[0-9a-f]{128}$",
                                                "minLength": 128,
                                                "maxLength": 128
                                              },
                                              "capsule": {
                                                "type": "string",
                                                "pattern": "^[0-9a-f]{64}$",
                                                "minLength": 64,
                                                "maxLength": 64
                                              }
                                            },
                                            "additionalProperties": false,
                                            "required": [
                                              "repository",
                                              "profile",
                                              "operation",
                                              "capsule"
                                            ]
                                          }
                                        },
                                        "commits": {
                                          "type": "array",
                                          "uniqueItems": true,
                                          "items": {
                                            "type": "object",
                                            "properties": {
                                              "evidence": {
                                                "type": "object",
                                                "properties": {
                                                  "repository": {
                                                    "type": "string",
                                                    "pattern": "^[0-9a-f]{32}$",
                                                    "minLength": 32,
                                                    "maxLength": 32
                                                  },
                                                  "profile": {
                                                    "type": "string",
                                                    "enum": [
                                                      "native_v1",
                                                      "git_sha1_jj045"
                                                    ]
                                                  },
                                                  "operation": {
                                                    "type": "string",
                                                    "pattern": "^[0-9a-f]{128}$",
                                                    "minLength": 128,
                                                    "maxLength": 128
                                                  },
                                                  "capsule": {
                                                    "type": "string",
                                                    "pattern": "^[0-9a-f]{64}$",
                                                    "minLength": 64,
                                                    "maxLength": 64
                                                  }
                                                },
                                                "additionalProperties": false,
                                                "required": [
                                                  "repository",
                                                  "profile",
                                                  "operation",
                                                  "capsule"
                                                ]
                                              },
                                              "commit": {
                                                "oneOf": [
                                                  {
                                                    "type": "object",
                                                    "properties": {
                                                      "profile": {
                                                        "type": "string",
                                                        "const": "native_v1"
                                                      },
                                                      "id": {
                                                        "type": "string",
                                                        "pattern": "^[0-9a-f]{128}$",
                                                        "minLength": 128,
                                                        "maxLength": 128
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
                                                        "type": "string",
                                                        "pattern": "^[0-9a-f]{40}$",
                                                        "minLength": 40,
                                                        "maxLength": 40
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
                                            },
                                            "additionalProperties": false,
                                            "required": [
                                              "evidence",
                                              "commit"
                                            ]
                                          }
                                        }
                                      },
                                      "additionalProperties": false,
                                      "required": [
                                        "operations",
                                        "commits"
                                      ]
                                    }
                                  },
                                  "required": [
                                    "profile",
                                    "anchor"
                                  ],
                                  "additionalProperties": false
                                }
                              ]
                            },
                            "evidence_method": {
                              "type": "string",
                              "enum": [
                                "controlled_scope",
                                "hook_boundary",
                                "transcript_recovery",
                                "historical_import",
                                "manual_assertion"
                              ]
                            },
                            "evidence": {
                              "type": [
                                "object",
                                "null"
                              ],
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "code"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "anchor",
                            "evidence_method"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "review_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "snapshot": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "review"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "review_id",
                            "snapshot"
                          ]
                        }
                      ]
                    },
                    "observed_primary": {
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
                    "type": {
                      "type": "string",
                      "const": "link"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "thread_id",
                    "expected_thread",
                    "link_id",
                    "target"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_thread": {
                      "type": "object",
                      "properties": {
                        "kind": {
                          "type": "string",
                          "enum": [
                            "index_page",
                            "thread_version",
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
                    "expected_link": {
                      "type": "object",
                      "properties": {
                        "kind": {
                          "type": "string",
                          "enum": [
                            "index_page",
                            "thread_version",
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
                    "type": {
                      "type": "string",
                      "const": "detach"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "thread_id",
                    "expected_thread",
                    "expected_link"
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

## GET /v1/repos/{repo}/threads/commands/{request_id}

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_status_threads",
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

## HEAD /v1/repos/{repo}/threads/commands/{request_id}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "collab_status_threads_head",
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

## POST /v1/repos/{repo}/reactions/query

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_reactions",
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
            "target": {
              "description": "Identity, immutable content and validation witnesses are explicit, never a UI position.",
              "oneOf": [
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "type": {
                      "type": "string",
                      "const": "thread"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "thread_id"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "message_id": {
                      "type": "string",
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
                    "type": {
                      "type": "string",
                      "const": "human_message"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "thread_id",
                    "message_id",
                    "revision"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "session_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "message_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "event_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "batch": {
                      "type": "object",
                      "properties": {
                        "kind": {
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
                    "content": {
                      "type": "object",
                      "properties": {
                        "kind": {
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
                    "type": {
                      "type": "string",
                      "const": "agent_message"
                    }
                  },
                  "additionalProperties": false,
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
                  "properties": {
                    "session_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "tool_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "snapshot": {
                      "type": "object",
                      "properties": {
                        "kind": {
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
                    "type": {
                      "type": "string",
                      "const": "tool_invocation"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "session_id",
                    "tool_id",
                    "snapshot"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "session_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "tool_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "snapshot": {
                      "type": "object",
                      "properties": {
                        "kind": {
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
                    "content": {
                      "type": "object",
                      "properties": {
                        "kind": {
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
                    "type": {
                      "type": "string",
                      "const": "tool_output"
                    }
                  },
                  "additionalProperties": false,
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
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "type": {
                      "type": "string",
                      "const": "review_thread"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "thread_id"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "thread_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "comment_id": {
                      "type": "string",
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
                            "anchor",
                            "comment_body",
                            "comment_revision",
                            "thread_event",
                            "event_page",
                            "thread_snapshot",
                            "relation_page",
                            "status_page"
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
                    "type": {
                      "type": "string",
                      "const": "review_comment"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "thread_id",
                    "comment_id",
                    "revision"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "task_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "comment_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
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
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "task_id",
                    "comment_id",
                    "version"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "session_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "checkpoint_id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "version": {
                      "type": "object",
                      "properties": {
                        "kind": {
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
                    "type": {
                      "type": "string",
                      "const": "agent_checkpoint"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "session_id",
                    "checkpoint_id",
                    "version"
                  ]
                }
              ]
            },
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
          },
          "additionalProperties": false,
          "required": [
            "target"
          ]
        }
      }
    }
  }
}
```

## POST /v1/repos/{repo}/reactions/commands

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_reaction_apply",
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
            "command": {
              "description": "A typed reaction command never accepts an actor from an API caller.",
              "oneOf": [
                {
                  "type": "object",
                  "properties": {
                    "target": {
                      "description": "Identity, immutable content and validation witnesses are explicit, never a UI position.",
                      "oneOf": [
                        {
                          "type": "object",
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "type": {
                              "type": "string",
                              "const": "thread"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "message_id": {
                              "type": "string",
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
                            "type": {
                              "type": "string",
                              "const": "human_message"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id",
                            "message_id",
                            "revision"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "message_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "event_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "batch": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "content": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "agent_message"
                            }
                          },
                          "additionalProperties": false,
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
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "tool_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "snapshot": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "tool_invocation"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "session_id",
                            "tool_id",
                            "snapshot"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "tool_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "snapshot": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "content": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "tool_output"
                            }
                          },
                          "additionalProperties": false,
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
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "type": {
                              "type": "string",
                              "const": "review_thread"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "comment_id": {
                              "type": "string",
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
                                    "anchor",
                                    "comment_body",
                                    "comment_revision",
                                    "thread_event",
                                    "event_page",
                                    "thread_snapshot",
                                    "relation_page",
                                    "status_page"
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
                            "type": {
                              "type": "string",
                              "const": "review_comment"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id",
                            "comment_id",
                            "revision"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "comment_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
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
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "task_id",
                            "comment_id",
                            "version"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "checkpoint_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "version": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "agent_checkpoint"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "session_id",
                            "checkpoint_id",
                            "version"
                          ]
                        }
                      ]
                    },
                    "value": {
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
                          "properties": {
                            "sequence": {
                              "type": "string"
                            },
                            "type": {
                              "type": "string",
                              "const": "emoji"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "sequence"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "emoji_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "version": {
                              "type": "object",
                              "properties": {
                                "kind": {
                                  "type": "string",
                                  "enum": [
                                    "index_page",
                                    "thread_version",
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
                            "type": {
                              "type": "string",
                              "const": "custom"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "emoji_id",
                            "version"
                          ]
                        }
                      ]
                    },
                    "observed": {
                      "type": "array",
                      "uniqueItems": true,
                      "items": {
                        "type": "object",
                        "properties": {
                          "kind": {
                            "type": "string",
                            "enum": [
                              "index_page",
                              "thread_version",
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
                      }
                    },
                    "type": {
                      "type": "string",
                      "const": "set"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "target",
                    "value",
                    "observed"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "target": {
                      "description": "Identity, immutable content and validation witnesses are explicit, never a UI position.",
                      "oneOf": [
                        {
                          "type": "object",
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "type": {
                              "type": "string",
                              "const": "thread"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "message_id": {
                              "type": "string",
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
                            "type": {
                              "type": "string",
                              "const": "human_message"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id",
                            "message_id",
                            "revision"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "message_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "event_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "batch": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "content": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "agent_message"
                            }
                          },
                          "additionalProperties": false,
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
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "tool_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "snapshot": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "tool_invocation"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "session_id",
                            "tool_id",
                            "snapshot"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "tool_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "snapshot": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "content": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "tool_output"
                            }
                          },
                          "additionalProperties": false,
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
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "type": {
                              "type": "string",
                              "const": "review_thread"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "comment_id": {
                              "type": "string",
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
                                    "anchor",
                                    "comment_body",
                                    "comment_revision",
                                    "thread_event",
                                    "event_page",
                                    "thread_snapshot",
                                    "relation_page",
                                    "status_page"
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
                            "type": {
                              "type": "string",
                              "const": "review_comment"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id",
                            "comment_id",
                            "revision"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "comment_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
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
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "task_id",
                            "comment_id",
                            "version"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "checkpoint_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "version": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "agent_checkpoint"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "session_id",
                            "checkpoint_id",
                            "version"
                          ]
                        }
                      ]
                    },
                    "observed": {
                      "type": "array",
                      "uniqueItems": true,
                      "items": {
                        "type": "object",
                        "properties": {
                          "kind": {
                            "type": "string",
                            "enum": [
                              "index_page",
                              "thread_version",
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
                      }
                    },
                    "type": {
                      "type": "string",
                      "const": "clear"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "target",
                    "observed"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "target": {
                      "description": "Identity, immutable content and validation witnesses are explicit, never a UI position.",
                      "oneOf": [
                        {
                          "type": "object",
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "type": {
                              "type": "string",
                              "const": "thread"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "message_id": {
                              "type": "string",
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
                            "type": {
                              "type": "string",
                              "const": "human_message"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id",
                            "message_id",
                            "revision"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "message_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "event_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "batch": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "content": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "agent_message"
                            }
                          },
                          "additionalProperties": false,
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
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "tool_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "snapshot": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "tool_invocation"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "session_id",
                            "tool_id",
                            "snapshot"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "tool_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "snapshot": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "content": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "tool_output"
                            }
                          },
                          "additionalProperties": false,
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
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "type": {
                              "type": "string",
                              "const": "review_thread"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "thread_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "comment_id": {
                              "type": "string",
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
                                    "anchor",
                                    "comment_body",
                                    "comment_revision",
                                    "thread_event",
                                    "event_page",
                                    "thread_snapshot",
                                    "relation_page",
                                    "status_page"
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
                            "type": {
                              "type": "string",
                              "const": "review_comment"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "thread_id",
                            "comment_id",
                            "revision"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "task_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "comment_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
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
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "task_id",
                            "comment_id",
                            "version"
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "session_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "checkpoint_id": {
                              "type": "string",
                              "pattern": "^[0-9a-f]{32}$",
                              "minLength": 32,
                              "maxLength": 32
                            },
                            "version": {
                              "type": "object",
                              "properties": {
                                "kind": {
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
                            "type": {
                              "type": "string",
                              "const": "agent_checkpoint"
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "type",
                            "session_id",
                            "checkpoint_id",
                            "version"
                          ]
                        }
                      ]
                    },
                    "value": {
                      "anyOf": [
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
                              "properties": {
                                "sequence": {
                                  "type": "string"
                                },
                                "type": {
                                  "type": "string",
                                  "const": "emoji"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "type",
                                "sequence"
                              ]
                            },
                            {
                              "type": "object",
                              "properties": {
                                "emoji_id": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{32}$",
                                  "minLength": 32,
                                  "maxLength": 32
                                },
                                "version": {
                                  "type": "object",
                                  "properties": {
                                    "kind": {
                                      "type": "string",
                                      "enum": [
                                        "index_page",
                                        "thread_version",
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
                                "type": {
                                  "type": "string",
                                  "const": "custom"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "type",
                                "emoji_id",
                                "version"
                              ]
                            }
                          ]
                        },
                        {
                          "type": "null"
                        }
                      ]
                    },
                    "observed": {
                      "type": "array",
                      "uniqueItems": true,
                      "items": {
                        "type": "object",
                        "properties": {
                          "kind": {
                            "type": "string",
                            "enum": [
                              "index_page",
                              "thread_version",
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
                      }
                    },
                    "type": {
                      "type": "string",
                      "const": "resolve_conflict"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "target",
                    "observed"
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

## GET /v1/repos/{repo}/reactions/commands/{request_id}

Current repository permission is checked for every request. Commands and status require Write; all other routes require Read. Authenticated actors and publication identities are service-derived. Signed pages bind complete captured roots, viewer, exact query and page size; expiry/restart returns cursor_reset. Retain original request IDs after outcome_unknown; not_known never proves rejection. Native code writes require reducer 3; WorkStore homes retain their frozen origin ledger. Reads and imports never execute agents.

```text
{
  "operationId": "collab_status_reactions",
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

## HEAD /v1/repos/{repo}/reactions/commands/{request_id}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "collab_status_reactions_head",
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

