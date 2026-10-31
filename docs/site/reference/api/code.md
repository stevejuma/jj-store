# Code browsing and mutations

Generated from OpenAPI. See [transport semantics](../transport.md), [capabilities](../capabilities.md) and [data types](types.md).

## GET /v1/repos/{repo}/snapshot

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "snapshot",
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
            "$ref": "#/components/schemas/HeadSnapshot"
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

## HEAD /v1/repos/{repo}/snapshot

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "snapshot_head",
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

## GET /v1/repos/{repo}/log

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "log",
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
      "name": "revset",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "default": "all()"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "workspace",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "user_email",
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
      "name": "at_time_millis",
      "in": "query",
      "required": true,
      "schema": {
        "type": "integer",
        "format": "int64"
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
            "$ref": "#/components/schemas/Envelope_GraphPage"
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
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
      },
      "revset": {
        "type": "string",
        "default": "all()"
      },
      "workspace": {
        "type": [
          "string",
          "null"
        ]
      },
      "user_email": {
        "type": "string",
        "default": ""
      },
      "at_time_millis": {
        "type": "integer",
        "format": "int64"
      }
    },
    "required": [
      "at_time_millis"
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

## HEAD /v1/repos/{repo}/log

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "log_head",
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
      "name": "revset",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "default": "all()"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "workspace",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "user_email",
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
      "name": "at_time_millis",
      "in": "query",
      "required": true,
      "schema": {
        "type": "integer",
        "format": "int64"
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
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
      },
      "revset": {
        "type": "string",
        "default": "all()"
      },
      "workspace": {
        "type": [
          "string",
          "null"
        ]
      },
      "user_email": {
        "type": "string",
        "default": ""
      },
      "at_time_millis": {
        "type": "integer",
        "format": "int64"
      }
    },
    "required": [
      "at_time_millis"
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

## GET /v1/repos/{repo}/operations

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "operations",
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
      "name": "query",
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
      "name": "heads",
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
            "$ref": "#/components/schemas/Envelope_GraphPage"
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
      "query": {
        "type": "string",
        "default": ""
      },
      "at_op": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "heads": {
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

## HEAD /v1/repos/{repo}/operations

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "operations_head",
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
      "name": "query",
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
      "name": "heads",
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
      "query": {
        "type": "string",
        "default": ""
      },
      "at_op": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "heads": {
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

## GET /v1/repos/{repo}/operations/{operation}

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "operation",
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
      "name": "operation",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/OperationId"
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/Envelope_OperationDetail"
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

## HEAD /v1/repos/{repo}/operations/{operation}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "operation_head",
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
      "name": "operation",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/OperationId"
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

## GET /v1/repos/{repo}/operations/{operation}/diff

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "operation_diff",
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
      "name": "operation",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    {
      "name": "parent",
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
            "$ref": "#/components/schemas/Envelope_OperationDiffResponse"
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
      "parent": {
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

## HEAD /v1/repos/{repo}/operations/{operation}/diff

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "operation_diff_head",
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
      "name": "operation",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    {
      "name": "parent",
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
      "parent": {
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

## GET /v1/repos/{repo}/operations/{operation}/changes

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "operation_changes",
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
      "name": "operation",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    {
      "name": "parent",
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
            "$ref": "#/components/schemas/Window2"
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
      "parent": {
        "type": "string",
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
    },
    "required": [
      "parent"
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

## HEAD /v1/repos/{repo}/operations/{operation}/changes

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "operation_changes_head",
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
      "name": "operation",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/OperationId"
      }
    },
    {
      "name": "parent",
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
      "parent": {
        "type": "string",
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
    },
    "required": [
      "parent"
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

## GET /v1/repos/{repo}/evolution

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "evolution",
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
            "$ref": "#/components/schemas/Envelope_GraphPage"
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
      "commit": {
        "type": "string",
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
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
      }
    },
    "required": [
      "commit"
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

## HEAD /v1/repos/{repo}/evolution

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "evolution_head",
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
      "commit": {
        "type": "string",
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
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
      }
    },
    "required": [
      "commit"
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

## GET /v1/repos/{repo}/commits/{commit}

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "commit",
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
      "name": "commit",
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
            "$ref": "#/components/schemas/Envelope_CommitDetail"
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

## HEAD /v1/repos/{repo}/commits/{commit}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "commit_head",
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
      "name": "commit",
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

## GET /v1/repos/{repo}/commits/{commit}/tree

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "tree",
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
      "name": "commit",
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
      "name": "path",
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
            "$ref": "#/components/schemas/Envelope_TreePage"
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
      "path": {
        "type": "string",
        "default": ""
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

## HEAD /v1/repos/{repo}/commits/{commit}/tree

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "tree_head",
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
      "name": "commit",
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
      "name": "path",
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
      "path": {
        "type": "string",
        "default": ""
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

## GET /v1/repos/{repo}/commits/{commit}/blame

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "blame",
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
      "name": "commit",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    {
      "name": "at_op",
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
      "name": "continuation",
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
            "$ref": "#/components/schemas/BlamePage"
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
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "path": {
        "type": "string"
      },
      "continuation": {
        "type": [
          "string",
          "null"
        ]
      }
    },
    "required": [
      "at_op",
      "path"
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

## HEAD /v1/repos/{repo}/commits/{commit}/blame

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "blame_head",
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
      "name": "commit",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/CommitId"
      }
    },
    {
      "name": "at_op",
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
      "name": "continuation",
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
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "path": {
        "type": "string"
      },
      "continuation": {
        "type": [
          "string",
          "null"
        ]
      }
    },
    "required": [
      "at_op",
      "path"
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

## GET /v1/repos/{repo}/commits/{commit}/paths

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "paths",
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
      "name": "commit",
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
      "name": "path",
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
      "name": "recursive",
      "in": "query",
      "required": false,
      "schema": {
        "type": "boolean",
        "default": true
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "include_last_change",
      "in": "query",
      "required": false,
      "schema": {
        "type": "boolean",
        "default": true
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "last_change_mode",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "quick",
          "history"
        ],
        "default": "quick"
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
            "$ref": "#/components/schemas/Window3"
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
      "path": {
        "type": "string",
        "default": ""
      },
      "recursive": {
        "type": "boolean",
        "default": true
      },
      "include_last_change": {
        "type": "boolean",
        "default": true
      },
      "last_change_mode": {
        "type": "string",
        "enum": [
          "quick",
          "history"
        ],
        "default": "quick"
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

## HEAD /v1/repos/{repo}/commits/{commit}/paths

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "paths_head",
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
      "name": "commit",
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
      "name": "path",
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
      "name": "recursive",
      "in": "query",
      "required": false,
      "schema": {
        "type": "boolean",
        "default": true
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "include_last_change",
      "in": "query",
      "required": false,
      "schema": {
        "type": "boolean",
        "default": true
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "last_change_mode",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "quick",
          "history"
        ],
        "default": "quick"
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
      "path": {
        "type": "string",
        "default": ""
      },
      "recursive": {
        "type": "boolean",
        "default": true
      },
      "include_last_change": {
        "type": "boolean",
        "default": true
      },
      "last_change_mode": {
        "type": "string",
        "enum": [
          "quick",
          "history"
        ],
        "default": "quick"
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

## GET /v1/repos/{repo}/commits/{commit}/conflicts

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "conflicts",
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
      "name": "commit",
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
    },
    {
      "name": "path",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "default": ""
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
            "$ref": "#/components/schemas/Envelope_ConflictResponse"
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
      "path": {
        "type": "string",
        "default": ""
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

## HEAD /v1/repos/{repo}/commits/{commit}/conflicts

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "conflicts_head",
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
      "name": "commit",
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
    },
    {
      "name": "path",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "default": ""
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
      "path": {
        "type": "string",
        "default": ""
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

## GET /v1/repos/{repo}/files/{file}

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "file",
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
      "name": "file",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/FileId"
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
      "name": "Range",
      "in": "header",
      "required": false,
      "schema": {
        "type": "string"
      },
      "description": "One bytes=start-end, bytes=start- or bytes=-count range. Bounds are clamped to EOF after verifying the complete native file."
    }
  ],
  "responses": {
    "200": {
      "description": "Verified binary attachment. Snapshot identity is in response headers. Chunks reauthorize access; errors after headers terminate delivery.",
      "content": {
        "application/octet-stream": {
          "schema": {
            "type": "string",
            "format": "binary"
          }
        }
      },
      "headers": {
        "Content-Disposition": {
          "schema": {
            "type": "string"
          },
          "description": "attachment; filename=\"<file ID>.bin\""
        },
        "Accept-Ranges": {
          "schema": {
            "const": "bytes"
          }
        },
        "Content-Length": {
          "schema": {
            "type": "string",
            "pattern": "^[0-9]+$"
          }
        },
        "x-jj-store-repository": {
          "schema": {
            "$ref": "#/components/schemas/RepositoryId"
          }
        },
        "x-jj-store-epoch": {
          "schema": {
            "$ref": "#/components/schemas/EpochId"
          }
        },
        "x-jj-store-operation": {
          "schema": {
            "$ref": "#/components/schemas/OperationId"
          }
        },
        "x-jj-store-position": {
          "schema": {
            "type": "string",
            "pattern": "^[0-9]+$"
          }
        },
        "x-jj-store-revision": {
          "schema": {
            "type": "string",
            "pattern": "^[0-9]+$"
          }
        }
      }
    },
    "206": {
      "description": "Verified binary attachment. Snapshot identity is in response headers. Chunks reauthorize access; errors after headers terminate delivery.",
      "content": {
        "application/octet-stream": {
          "schema": {
            "type": "string",
            "format": "binary"
          }
        }
      },
      "headers": {
        "Content-Disposition": {
          "schema": {
            "type": "string"
          },
          "description": "attachment; filename=\"<file ID>.bin\""
        },
        "Accept-Ranges": {
          "schema": {
            "const": "bytes"
          }
        },
        "Content-Length": {
          "schema": {
            "type": "string",
            "pattern": "^[0-9]+$"
          }
        },
        "x-jj-store-repository": {
          "schema": {
            "$ref": "#/components/schemas/RepositoryId"
          }
        },
        "x-jj-store-epoch": {
          "schema": {
            "$ref": "#/components/schemas/EpochId"
          }
        },
        "x-jj-store-operation": {
          "schema": {
            "$ref": "#/components/schemas/OperationId"
          }
        },
        "x-jj-store-position": {
          "schema": {
            "type": "string",
            "pattern": "^[0-9]+$"
          }
        },
        "x-jj-store-revision": {
          "schema": {
            "type": "string",
            "pattern": "^[0-9]+$"
          }
        },
        "Content-Range": {
          "schema": {
            "type": "string"
          },
          "description": "bytes <start>-<inclusive end>/<full length>"
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
    "416": {
      "description": "Invalid or unsatisfiable byte range.",
      "headers": {
        "Content-Range": {
          "schema": {
            "type": "string"
          },
          "description": "bytes */<full length>"
        }
      },
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

## HEAD /v1/repos/{repo}/files/{file}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "file_head",
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
      "name": "file",
      "in": "path",
      "required": true,
      "schema": {
        "$ref": "#/components/schemas/FileId"
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
      "name": "Range",
      "in": "header",
      "required": false,
      "schema": {
        "type": "string"
      },
      "description": "One bytes=start-end, bytes=start- or bytes=-count range. Bounds are clamped to EOF after verifying the complete native file."
    }
  ],
  "responses": {
    "200": {
      "description": "Verified binary attachment. Snapshot identity is in response headers. Chunks reauthorize access; errors after headers terminate delivery.",
      "headers": {
        "Content-Disposition": {
          "schema": {
            "type": "string"
          },
          "description": "attachment; filename=\"<file ID>.bin\""
        },
        "Accept-Ranges": {
          "schema": {
            "const": "bytes"
          }
        },
        "Content-Length": {
          "schema": {
            "type": "string",
            "pattern": "^[0-9]+$"
          }
        },
        "x-jj-store-repository": {
          "schema": {
            "$ref": "#/components/schemas/RepositoryId"
          }
        },
        "x-jj-store-epoch": {
          "schema": {
            "$ref": "#/components/schemas/EpochId"
          }
        },
        "x-jj-store-operation": {
          "schema": {
            "$ref": "#/components/schemas/OperationId"
          }
        },
        "x-jj-store-position": {
          "schema": {
            "type": "string",
            "pattern": "^[0-9]+$"
          }
        },
        "x-jj-store-revision": {
          "schema": {
            "type": "string",
            "pattern": "^[0-9]+$"
          }
        }
      }
    },
    "206": {
      "description": "Verified binary attachment. Snapshot identity is in response headers. Chunks reauthorize access; errors after headers terminate delivery.",
      "headers": {
        "Content-Disposition": {
          "schema": {
            "type": "string"
          },
          "description": "attachment; filename=\"<file ID>.bin\""
        },
        "Accept-Ranges": {
          "schema": {
            "const": "bytes"
          }
        },
        "Content-Length": {
          "schema": {
            "type": "string",
            "pattern": "^[0-9]+$"
          }
        },
        "x-jj-store-repository": {
          "schema": {
            "$ref": "#/components/schemas/RepositoryId"
          }
        },
        "x-jj-store-epoch": {
          "schema": {
            "$ref": "#/components/schemas/EpochId"
          }
        },
        "x-jj-store-operation": {
          "schema": {
            "$ref": "#/components/schemas/OperationId"
          }
        },
        "x-jj-store-position": {
          "schema": {
            "type": "string",
            "pattern": "^[0-9]+$"
          }
        },
        "x-jj-store-revision": {
          "schema": {
            "type": "string",
            "pattern": "^[0-9]+$"
          }
        },
        "Content-Range": {
          "schema": {
            "type": "string"
          },
          "description": "bytes <start>-<inclusive end>/<full length>"
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
    "416": {
      "description": "Invalid or unsatisfiable byte range.",
      "headers": {
        "Content-Range": {
          "schema": {
            "type": "string"
          },
          "description": "bytes */<full length>"
        }
      }
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

## GET /v1/repos/{repo}/diff

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "diff",
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
      "name": "from",
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
      "name": "to",
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
      "required": false,
      "schema": {
        "type": "string",
        "default": ""
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "content",
      "in": "query",
      "required": false,
      "schema": {
        "type": "boolean",
        "default": false
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
            "$ref": "#/components/schemas/Envelope_FileDiffResponse"
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
      "from": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "to": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "path": {
        "type": "string",
        "default": ""
      },
      "content": {
        "type": "boolean",
        "default": false
      }
    },
    "required": [
      "from",
      "to"
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

## HEAD /v1/repos/{repo}/diff

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "diff_head",
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
      "name": "from",
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
      "name": "to",
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
      "required": false,
      "schema": {
        "type": "string",
        "default": ""
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "content",
      "in": "query",
      "required": false,
      "schema": {
        "type": "boolean",
        "default": false
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
      "from": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "to": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "path": {
        "type": "string",
        "default": ""
      },
      "content": {
        "type": "boolean",
        "default": false
      }
    },
    "required": [
      "from",
      "to"
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

## GET /v1/repos/{repo}/diff/paths

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "diff_paths",
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
      "name": "from",
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
      "name": "to",
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
      "required": false,
      "schema": {
        "type": "string",
        "default": ""
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
            "$ref": "#/components/schemas/Window4"
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
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "from": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "to": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "path": {
        "type": "string",
        "default": ""
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
    },
    "required": [
      "at_op",
      "from",
      "to"
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

## HEAD /v1/repos/{repo}/diff/paths

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "diff_paths_head",
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
      "name": "from",
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
      "name": "to",
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
      "required": false,
      "schema": {
        "type": "string",
        "default": ""
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
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "from": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "to": {
        "type": "string",
        "pattern": "^[0-9a-f]{128}$",
        "minLength": 128,
        "maxLength": 128
      },
      "path": {
        "type": "string",
        "default": ""
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
    },
    "required": [
      "at_op",
      "from",
      "to"
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

## POST /v1/repos/{repo}/mutations/preview

Requires current repository write permission, including status. Strict typed commands only. Preview does not publish. Apply may return outcome_unknown; status not_known never proves rejection. Retain the original request ID across every retry or transport failure. Request bytes are capped at 524288. Stale snapshots, bookmark expectations, multiple heads and changed intent return distinct 409 codes; protected transitions return 403. Browser mutation work remains deferred.

```text
{
  "operationId": "mutation_preview",
  "tags": [
    "mutations"
  ],
  "description": "Requires current repository write permission, including status. Strict typed commands only. Preview does not publish. Apply may return outcome_unknown; status not_known never proves rejection. Retain the original request ID across every retry or transport failure. Request bytes are capped at 524288. Stale snapshots, bookmark expectations, multiple heads and changed intent return distinct 409 codes; protected transitions return 403. Browser mutation work remains deferred.",
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
            "$ref": "#/components/schemas/Preview"
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
      "description": "stale_snapshot, stale_bookmark, stale_conflict, protected_transition, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt.",
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
            "expected": {
              "type": "object",
              "properties": {
                "epoch": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{32}$",
                  "minLength": 32,
                  "maxLength": 32
                },
                "position": {
                  "description": "Publication order is distinct from native operation ancestry.",
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
                    "type": "string",
                    "pattern": "^[0-9a-f]{128}$",
                    "minLength": 128,
                    "maxLength": 128
                  }
                }
              },
              "additionalProperties": false,
              "required": [
                "epoch",
                "position",
                "heads"
              ]
            },
            "command": {
              "oneOf": [
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "edit_file"
                    },
                    "bookmark": {
                      "type": "string"
                    },
                    "expected": {
                      "type": "array",
                      "items": {
                        "type": [
                          "string",
                          "null"
                        ],
                        "pattern": "^[0-9a-f]{128}$",
                        "minLength": 128,
                        "maxLength": 128
                      }
                    },
                    "source": {
                      "type": "string"
                    },
                    "destination": {
                      "type": "string"
                    },
                    "contents": {
                      "type": "string"
                    },
                    "description": {
                      "type": "string"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "bookmark",
                    "expected",
                    "source",
                    "destination",
                    "contents",
                    "description"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "resolve_conflicts"
                    },
                    "commit": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{128}$",
                      "minLength": 128,
                      "maxLength": 128
                    },
                    "paths": {
                      "type": "array",
                      "items": {
                        "description": "One path in an atomic resolution of the original commit tree.",
                        "type": "object",
                        "properties": {
                          "path": {
                            "type": "string"
                          },
                          "fingerprint": {
                            "type": "string",
                            "pattern": "^[0-9a-f]{128}$",
                            "minLength": 128,
                            "maxLength": 128
                          },
                          "resolution": {
                            "oneOf": [
                              {
                                "type": "object",
                                "properties": {
                                  "type": {
                                    "type": "string",
                                    "const": "uploaded_file"
                                  },
                                  "upload": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{64}$",
                                    "minLength": 64,
                                    "maxLength": 64
                                  },
                                  "executable": {
                                    "type": "boolean"
                                  },
                                  "copy_from_term": {
                                    "type": [
                                      "integer",
                                      "null"
                                    ],
                                    "format": "uint32",
                                    "minimum": 0
                                  }
                                },
                                "additionalProperties": false,
                                "required": [
                                  "type",
                                  "upload",
                                  "executable"
                                ]
                              },
                              {
                                "type": "object",
                                "properties": {
                                  "type": {
                                    "type": "string",
                                    "const": "select_term"
                                  },
                                  "term": {
                                    "type": "integer",
                                    "format": "uint32",
                                    "minimum": 0
                                  }
                                },
                                "additionalProperties": false,
                                "required": [
                                  "type",
                                  "term"
                                ]
                              },
                              {
                                "type": "object",
                                "properties": {
                                  "type": {
                                    "type": "string",
                                    "const": "delete"
                                  }
                                },
                                "required": [
                                  "type"
                                ],
                                "additionalProperties": false
                              }
                            ]
                          }
                        },
                        "additionalProperties": false,
                        "required": [
                          "path",
                          "fingerprint",
                          "resolution"
                        ]
                      }
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "commit",
                    "paths"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "new"
                    },
                    "parents": {
                      "type": "array",
                      "items": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{128}$",
                        "minLength": 128,
                        "maxLength": 128
                      }
                    },
                    "description": {
                      "type": "string"
                    },
                    "workspace": {
                      "type": [
                        "string",
                        "null"
                      ]
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "parents",
                    "description"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "describe"
                    },
                    "commit": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{128}$",
                      "minLength": 128,
                      "maxLength": 128
                    },
                    "description": {
                      "type": "string"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "commit",
                    "description"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "bookmark_create"
                    },
                    "name": {
                      "type": "string"
                    },
                    "target": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{128}$",
                      "minLength": 128,
                      "maxLength": 128
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "name",
                    "target"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "bookmark_move"
                    },
                    "name": {
                      "type": "string"
                    },
                    "expected": {
                      "type": "array",
                      "items": {
                        "type": [
                          "string",
                          "null"
                        ],
                        "pattern": "^[0-9a-f]{128}$",
                        "minLength": 128,
                        "maxLength": 128
                      }
                    },
                    "target": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{128}$",
                      "minLength": 128,
                      "maxLength": 128
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "name",
                    "expected",
                    "target"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "bookmark_delete"
                    },
                    "name": {
                      "type": "string"
                    },
                    "expected": {
                      "type": "array",
                      "items": {
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
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "name",
                    "expected"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "reconcile_operations"
                    },
                    "base": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{128}$",
                      "minLength": 128,
                      "maxLength": 128
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "base"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "abandon"
                    },
                    "commits": {
                      "type": "array",
                      "items": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{128}$",
                        "minLength": 128,
                        "maxLength": 128
                      }
                    },
                    "retain_bookmarks": {
                      "type": "boolean"
                    },
                    "restore_descendants": {
                      "type": "boolean"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "commits",
                    "retain_bookmarks",
                    "restore_descendants"
                  ]
                }
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "request_id",
            "expected",
            "command"
          ]
        }
      }
    }
  }
}
```

## POST /v1/repos/{repo}/mutations

Requires current repository write permission, including status. Strict typed commands only. Preview does not publish. Apply may return outcome_unknown; status not_known never proves rejection. Retain the original request ID across every retry or transport failure. Request bytes are capped at 524288. Stale snapshots, bookmark expectations, multiple heads and changed intent return distinct 409 codes; protected transitions return 403. Browser mutation work remains deferred.

```text
{
  "operationId": "mutation_apply",
  "tags": [
    "mutations"
  ],
  "description": "Requires current repository write permission, including status. Strict typed commands only. Preview does not publish. Apply may return outcome_unknown; status not_known never proves rejection. Retain the original request ID across every retry or transport failure. Request bytes are capped at 524288. Stale snapshots, bookmark expectations, multiple heads and changed intent return distinct 409 codes; protected transitions return 403. Browser mutation work remains deferred.",
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
            "$ref": "#/components/schemas/Outcome"
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
      "description": "stale_snapshot, stale_bookmark, stale_conflict, protected_transition, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt.",
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
            "expected": {
              "type": "object",
              "properties": {
                "epoch": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{32}$",
                  "minLength": 32,
                  "maxLength": 32
                },
                "position": {
                  "description": "Publication order is distinct from native operation ancestry.",
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
                    "type": "string",
                    "pattern": "^[0-9a-f]{128}$",
                    "minLength": 128,
                    "maxLength": 128
                  }
                }
              },
              "additionalProperties": false,
              "required": [
                "epoch",
                "position",
                "heads"
              ]
            },
            "command": {
              "oneOf": [
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "edit_file"
                    },
                    "bookmark": {
                      "type": "string"
                    },
                    "expected": {
                      "type": "array",
                      "items": {
                        "type": [
                          "string",
                          "null"
                        ],
                        "pattern": "^[0-9a-f]{128}$",
                        "minLength": 128,
                        "maxLength": 128
                      }
                    },
                    "source": {
                      "type": "string"
                    },
                    "destination": {
                      "type": "string"
                    },
                    "contents": {
                      "type": "string"
                    },
                    "description": {
                      "type": "string"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "bookmark",
                    "expected",
                    "source",
                    "destination",
                    "contents",
                    "description"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "resolve_conflicts"
                    },
                    "commit": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{128}$",
                      "minLength": 128,
                      "maxLength": 128
                    },
                    "paths": {
                      "type": "array",
                      "items": {
                        "description": "One path in an atomic resolution of the original commit tree.",
                        "type": "object",
                        "properties": {
                          "path": {
                            "type": "string"
                          },
                          "fingerprint": {
                            "type": "string",
                            "pattern": "^[0-9a-f]{128}$",
                            "minLength": 128,
                            "maxLength": 128
                          },
                          "resolution": {
                            "oneOf": [
                              {
                                "type": "object",
                                "properties": {
                                  "type": {
                                    "type": "string",
                                    "const": "uploaded_file"
                                  },
                                  "upload": {
                                    "type": "string",
                                    "pattern": "^[0-9a-f]{64}$",
                                    "minLength": 64,
                                    "maxLength": 64
                                  },
                                  "executable": {
                                    "type": "boolean"
                                  },
                                  "copy_from_term": {
                                    "type": [
                                      "integer",
                                      "null"
                                    ],
                                    "format": "uint32",
                                    "minimum": 0
                                  }
                                },
                                "additionalProperties": false,
                                "required": [
                                  "type",
                                  "upload",
                                  "executable"
                                ]
                              },
                              {
                                "type": "object",
                                "properties": {
                                  "type": {
                                    "type": "string",
                                    "const": "select_term"
                                  },
                                  "term": {
                                    "type": "integer",
                                    "format": "uint32",
                                    "minimum": 0
                                  }
                                },
                                "additionalProperties": false,
                                "required": [
                                  "type",
                                  "term"
                                ]
                              },
                              {
                                "type": "object",
                                "properties": {
                                  "type": {
                                    "type": "string",
                                    "const": "delete"
                                  }
                                },
                                "required": [
                                  "type"
                                ],
                                "additionalProperties": false
                              }
                            ]
                          }
                        },
                        "additionalProperties": false,
                        "required": [
                          "path",
                          "fingerprint",
                          "resolution"
                        ]
                      }
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "commit",
                    "paths"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "new"
                    },
                    "parents": {
                      "type": "array",
                      "items": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{128}$",
                        "minLength": 128,
                        "maxLength": 128
                      }
                    },
                    "description": {
                      "type": "string"
                    },
                    "workspace": {
                      "type": [
                        "string",
                        "null"
                      ]
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "parents",
                    "description"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "describe"
                    },
                    "commit": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{128}$",
                      "minLength": 128,
                      "maxLength": 128
                    },
                    "description": {
                      "type": "string"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "commit",
                    "description"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "bookmark_create"
                    },
                    "name": {
                      "type": "string"
                    },
                    "target": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{128}$",
                      "minLength": 128,
                      "maxLength": 128
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "name",
                    "target"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "bookmark_move"
                    },
                    "name": {
                      "type": "string"
                    },
                    "expected": {
                      "type": "array",
                      "items": {
                        "type": [
                          "string",
                          "null"
                        ],
                        "pattern": "^[0-9a-f]{128}$",
                        "minLength": 128,
                        "maxLength": 128
                      }
                    },
                    "target": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{128}$",
                      "minLength": 128,
                      "maxLength": 128
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "name",
                    "expected",
                    "target"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "bookmark_delete"
                    },
                    "name": {
                      "type": "string"
                    },
                    "expected": {
                      "type": "array",
                      "items": {
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
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "name",
                    "expected"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "reconcile_operations"
                    },
                    "base": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{128}$",
                      "minLength": 128,
                      "maxLength": 128
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "base"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "type": {
                      "type": "string",
                      "const": "abandon"
                    },
                    "commits": {
                      "type": "array",
                      "items": {
                        "type": "string",
                        "pattern": "^[0-9a-f]{128}$",
                        "minLength": 128,
                        "maxLength": 128
                      }
                    },
                    "retain_bookmarks": {
                      "type": "boolean"
                    },
                    "restore_descendants": {
                      "type": "boolean"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "type",
                    "commits",
                    "retain_bookmarks",
                    "restore_descendants"
                  ]
                }
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "request_id",
            "expected",
            "command"
          ]
        }
      }
    }
  }
}
```

## GET /v1/repos/{repo}/mutations/{request_id}

Requires current repository write permission, including status. Strict typed commands only. Preview does not publish. Apply may return outcome_unknown; status not_known never proves rejection. Retain the original request ID across every retry or transport failure. Request bytes are capped at 524288. Stale snapshots, bookmark expectations, multiple heads and changed intent return distinct 409 codes; protected transitions return 403. Browser mutation work remains deferred.

```text
{
  "operationId": "mutation_status",
  "tags": [
    "mutations"
  ],
  "description": "Requires current repository write permission, including status. Strict typed commands only. Preview does not publish. Apply may return outcome_unknown; status not_known never proves rejection. Retain the original request ID across every retry or transport failure. Request bytes are capped at 524288. Stale snapshots, bookmark expectations, multiple heads and changed intent return distinct 409 codes; protected transitions return 403. Browser mutation work remains deferred.",
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
            "$ref": "#/components/schemas/Status"
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
      "description": "stale_snapshot, stale_bookmark, stale_conflict, protected_transition, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt.",
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

## HEAD /v1/repos/{repo}/mutations/{request_id}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "mutation_status_head",
  "tags": [
    "mutations"
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
      "description": "stale_snapshot, stale_bookmark, stale_conflict, protected_transition, reconciliation_required, no_changes, epoch_changed or intent_conflict. No publication by this rejected attempt."
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

