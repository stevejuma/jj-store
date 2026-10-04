# Search and indexing

Generated from OpenAPI. See [transport semantics](../transport.md), [capabilities](../capabilities.md) and [data types](types.md).

## GET /v1/repos/{repo}/search

Requires current repository read permission. Only explicitly configured repositories advertise search. Search pins a native view and Tantivy generation, reauthorizes every page, and never publishes. q is required; at_op selects visible-commit mode, historical mode is explicit. All query inputs must match a continuation. Minimum-position waits are bounded; 409 index_lag includes authorized search_status. Expired/restarted cursors require reset. Indexed positions are distinct from current repository positions. Source strings are untrusted data.

```text
{
  "operationId": "search",
  "tags": [
    "search"
  ],
  "description": "Requires current repository read permission. Only explicitly configured repositories advertise search. Search pins a native view and Tantivy generation, reauthorizes every page, and never publishes. q is required; at_op selects visible-commit mode, historical mode is explicit. All query inputs must match a continuation. Minimum-position waits are bounded; 409 index_lag includes authorized search_status. Expired/restarted cursors require reset. Indexed positions are distinct from current repository positions. Source strings are untrusted data.",
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
      "name": "q",
      "in": "query",
      "required": true,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "mode",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "visible_commits",
          "historical"
        ],
        "default": "visible_commits"
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
      "name": "scope",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "page_size",
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
      "name": "minimum_position",
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
      "name": "wait_millis",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0,
        "default": 0
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
            "$ref": "#/components/schemas/SearchPage"
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
      "description": "index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed.",
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
    "501": {
      "description": "Search is not configured for this repository.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "search_not_ready, busy, unavailable, corrupt_index, or incomplete history.",
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
      "q": {
        "type": "string"
      },
      "mode": {
        "type": "string",
        "enum": [
          "visible_commits",
          "historical"
        ],
        "default": "visible_commits"
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
      "scope": {
        "type": [
          "string",
          "null"
        ]
      },
      "page_size": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
      },
      "minimum_position": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
      },
      "wait_millis": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0,
        "default": 0
      },
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      }
    },
    "required": [
      "q"
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

## HEAD /v1/repos/{repo}/search

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "search_head",
  "tags": [
    "search"
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
      "name": "q",
      "in": "query",
      "required": true,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "mode",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "visible_commits",
          "historical"
        ],
        "default": "visible_commits"
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
      "name": "scope",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "page_size",
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
      "name": "minimum_position",
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
      "name": "wait_millis",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0,
        "default": 0
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
      "description": "index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed."
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
    "501": {
      "description": "Search is not configured for this repository."
    },
    "503": {
      "description": "search_not_ready, busy, unavailable, corrupt_index, or incomplete history."
    },
    "504": {
      "description": "Request or response deadline exceeded."
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "q": {
        "type": "string"
      },
      "mode": {
        "type": "string",
        "enum": [
          "visible_commits",
          "historical"
        ],
        "default": "visible_commits"
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
      "scope": {
        "type": [
          "string",
          "null"
        ]
      },
      "page_size": {
        "type": "integer",
        "format": "uint",
        "minimum": 0,
        "default": 50
      },
      "minimum_position": {
        "type": [
          "integer",
          "null"
        ],
        "format": "uint64",
        "minimum": 0,
        "description": "Publication order is distinct from native operation ancestry."
      },
      "wait_millis": {
        "type": "integer",
        "format": "uint64",
        "minimum": 0,
        "default": 0
      },
      "cursor": {
        "type": [
          "string",
          "null"
        ]
      }
    },
    "required": [
      "q"
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

## GET /v1/repos/{repo}/search/status

Requires current repository read permission. Only explicitly configured repositories advertise search. Search pins a native view and Tantivy generation, reauthorizes every page, and never publishes. q is required; at_op selects visible-commit mode, historical mode is explicit. All query inputs must match a continuation. Minimum-position waits are bounded; 409 index_lag includes authorized search_status. Expired/restarted cursors require reset. Indexed positions are distinct from current repository positions. Source strings are untrusted data.

```text
{
  "operationId": "search_status",
  "tags": [
    "search"
  ],
  "description": "Requires current repository read permission. Only explicitly configured repositories advertise search. Search pins a native view and Tantivy generation, reauthorizes every page, and never publishes. q is required; at_op selects visible-commit mode, historical mode is explicit. All query inputs must match a continuation. Minimum-position waits are bounded; 409 index_lag includes authorized search_status. Expired/restarted cursors require reset. Indexed positions are distinct from current repository positions. Source strings are untrusted data.",
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
            "$ref": "#/components/schemas/SearchStatus"
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
      "description": "index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed.",
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
    "501": {
      "description": "Search is not configured for this repository.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "search_not_ready, busy, unavailable, corrupt_index, or incomplete history.",
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

## HEAD /v1/repos/{repo}/search/status

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "search_status_head",
  "tags": [
    "search"
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
      "description": "index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed."
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
    "501": {
      "description": "Search is not configured for this repository."
    },
    "503": {
      "description": "search_not_ready, busy, unavailable, corrupt_index, or incomplete history."
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

## GET /v1/repos/{repo}/content/search

Requires current repository read permission. Only explicitly configured repositories advertise search. Search pins a native view and Tantivy generation, reauthorizes every page, and never publishes. q is required; at_op selects visible-commit mode, historical mode is explicit. All query inputs must match a continuation. Minimum-position waits are bounded; 409 index_lag includes authorized search_status. Expired/restarted cursors require reset. Indexed positions are distinct from current repository positions. Source strings are untrusted data.

```text
{
  "operationId": "content_search",
  "tags": [
    "search"
  ],
  "description": "Requires current repository read permission. Only explicitly configured repositories advertise search. Search pins a native view and Tantivy generation, reauthorizes every page, and never publishes. q is required; at_op selects visible-commit mode, historical mode is explicit. All query inputs must match a continuation. Minimum-position waits are bounded; 409 index_lag includes authorized search_status. Expired/restarted cursors require reset. Indexed positions are distinct from current repository positions. Source strings are untrusted data.",
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
      "name": "section",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "all",
          "threads",
          "projects",
          "tasks"
        ],
        "default": "all"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "thread",
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
      "name": "project",
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
      "name": "task",
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
            "$ref": "#/components/schemas/ContentPage"
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
      "description": "index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed.",
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
    "501": {
      "description": "Search is not configured for this repository.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ErrorBody"
          }
        }
      }
    },
    "503": {
      "description": "search_not_ready, busy, unavailable, corrupt_index, or incomplete history.",
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
      "q": {
        "type": "string",
        "default": ""
      },
      "section": {
        "type": "string",
        "enum": [
          "all",
          "threads",
          "projects",
          "tasks"
        ],
        "default": "all"
      },
      "thread": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "project": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "task": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "limit": {
        "type": "integer",
        "format": "uint",
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

## HEAD /v1/repos/{repo}/content/search

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "content_search_head",
  "tags": [
    "search"
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
      "name": "section",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "all",
          "threads",
          "projects",
          "tasks"
        ],
        "default": "all"
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "thread",
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
      "name": "project",
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
      "name": "task",
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
      "description": "index_lag with search_status, select_operation with heads, cursor_reset or epoch_changed."
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
    "501": {
      "description": "Search is not configured for this repository."
    },
    "503": {
      "description": "search_not_ready, busy, unavailable, corrupt_index, or incomplete history."
    },
    "504": {
      "description": "Request or response deadline exceeded."
    }
  },
  "x-jj-store-query-schema": {
    "type": "object",
    "additionalProperties": false,
    "properties": {
      "q": {
        "type": "string",
        "default": ""
      },
      "section": {
        "type": "string",
        "enum": [
          "all",
          "threads",
          "projects",
          "tasks"
        ],
        "default": "all"
      },
      "thread": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "project": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "task": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "limit": {
        "type": "integer",
        "format": "uint",
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

