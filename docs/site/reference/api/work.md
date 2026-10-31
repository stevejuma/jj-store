# Projects and tasks

Generated from OpenAPI. See [transport semantics](../transport.md), [capabilities](../capabilities.md) and [data types](types.md).

## GET /v1/repos/{repo}/work/browse

Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.

```text
{
  "operationId": "work_browse",
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
      "name": "kind",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "projects",
          "tasks"
        ],
        "default": "projects"
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
      "name": "repository",
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
      "name": "parent",
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
      "name": "state",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "active",
          "open",
          "in_progress",
          "blocked",
          "done",
          null
        ]
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
            "$ref": "#/components/schemas/WorkBrowsePage"
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
      "kind": {
        "type": "string",
        "enum": [
          "projects",
          "tasks"
        ],
        "default": "projects"
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
      "repository": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "parent": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "state": {
        "type": [
          "string",
          "null"
        ],
        "enum": [
          "active",
          "open",
          "in_progress",
          "blocked",
          "done",
          null
        ]
      },
      "q": {
        "type": "string",
        "default": ""
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

## HEAD /v1/repos/{repo}/work/browse

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "work_browse_head",
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
      "name": "kind",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "projects",
          "tasks"
        ],
        "default": "projects"
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
      "name": "repository",
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
      "name": "parent",
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
      "name": "state",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "enum": [
          "active",
          "open",
          "in_progress",
          "blocked",
          "done",
          null
        ]
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
      "kind": {
        "type": "string",
        "enum": [
          "projects",
          "tasks"
        ],
        "default": "projects"
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
      "repository": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "parent": {
        "type": [
          "string",
          "null"
        ],
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      },
      "state": {
        "type": [
          "string",
          "null"
        ],
        "enum": [
          "active",
          "open",
          "in_progress",
          "blocked",
          "done",
          null
        ]
      },
      "q": {
        "type": "string",
        "default": ""
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

## GET /v1/repos/{repo}/work/activity

Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.

```text
{
  "operationId": "work_activity",
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
      "name": "limit",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint",
        "minimum": 1,
        "maximum": 256,
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
            "$ref": "#/components/schemas/WorkActivityPage"
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
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 1,
        "maximum": 256,
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

## HEAD /v1/repos/{repo}/work/activity

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "work_activity_head",
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
      "name": "limit",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint",
        "minimum": 1,
        "maximum": 256,
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
      "limit": {
        "type": "integer",
        "format": "uint",
        "minimum": 1,
        "maximum": 256,
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

## GET /v1/repos/{repo}/work

Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.

```text
{
  "operationId": "work_page",
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
            "$ref": "#/components/schemas/WorkPage"
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

## HEAD /v1/repos/{repo}/work

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "work_page_head",
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

## POST /v1/repos/{repo}/work/commands

Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.

```text
{
  "operationId": "work_apply",
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
            "$ref": "#/components/schemas/WorkCommandOutcome"
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
                    "command": {
                      "type": "string",
                      "const": "create_project"
                    },
                    "id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "title": {
                      "type": "string"
                    },
                    "body": {
                      "type": "string"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "id",
                    "title",
                    "body"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "command": {
                      "type": "string",
                      "const": "update_project"
                    },
                    "id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_version": {
                      "type": "integer",
                      "format": "uint64",
                      "minimum": 0
                    },
                    "title": {
                      "type": "string"
                    },
                    "body": {
                      "type": "string"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "id",
                    "expected_version",
                    "title",
                    "body"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "command": {
                      "type": "string",
                      "const": "set_repository"
                    },
                    "project": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_version": {
                      "type": "integer",
                      "format": "uint64",
                      "minimum": 0
                    },
                    "repository": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "active": {
                      "type": "boolean"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "project",
                    "expected_version",
                    "repository",
                    "active"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "command": {
                      "type": "string",
                      "const": "create_task"
                    },
                    "id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "project": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_project_version": {
                      "type": "integer",
                      "format": "uint64",
                      "minimum": 0
                    },
                    "title": {
                      "type": "string"
                    },
                    "body": {
                      "type": "string"
                    },
                    "repository": {
                      "type": [
                        "string",
                        "null"
                      ],
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "parent": {
                      "type": [
                        "string",
                        "null"
                      ],
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "id",
                    "project",
                    "expected_project_version",
                    "title",
                    "body"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "command": {
                      "type": "string",
                      "const": "update_task"
                    },
                    "id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_version": {
                      "type": "integer",
                      "format": "uint64",
                      "minimum": 0
                    },
                    "expected_project_version": {
                      "type": "integer",
                      "format": "uint64",
                      "minimum": 0
                    },
                    "title": {
                      "type": "string"
                    },
                    "body": {
                      "type": "string"
                    },
                    "state": {
                      "type": "string",
                      "enum": [
                        "open",
                        "in_progress",
                        "blocked",
                        "done"
                      ]
                    },
                    "repository": {
                      "type": [
                        "string",
                        "null"
                      ],
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "parent": {
                      "type": [
                        "string",
                        "null"
                      ],
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "id",
                    "expected_version",
                    "expected_project_version",
                    "title",
                    "body",
                    "state"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "command": {
                      "type": "string",
                      "const": "add_comment"
                    },
                    "id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "task": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_task_version": {
                      "type": "integer",
                      "format": "uint64",
                      "minimum": 0
                    },
                    "reply_to": {
                      "type": [
                        "string",
                        "null"
                      ],
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "body": {
                      "type": "string"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "id",
                    "task",
                    "expected_task_version",
                    "body"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "command": {
                      "type": "string",
                      "const": "edit_comment"
                    },
                    "id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_version": {
                      "type": "integer",
                      "format": "uint64",
                      "minimum": 0
                    },
                    "expected_task_version": {
                      "type": "integer",
                      "format": "uint64",
                      "minimum": 0
                    },
                    "body": {
                      "type": "string"
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "id",
                    "expected_version",
                    "expected_task_version",
                    "body"
                  ]
                },
                {
                  "type": "object",
                  "properties": {
                    "command": {
                      "type": "string",
                      "const": "tombstone_comment"
                    },
                    "id": {
                      "type": "string",
                      "pattern": "^[0-9a-f]{32}$",
                      "minLength": 32,
                      "maxLength": 32
                    },
                    "expected_version": {
                      "type": "integer",
                      "format": "uint64",
                      "minimum": 0
                    },
                    "expected_task_version": {
                      "type": "integer",
                      "format": "uint64",
                      "minimum": 0
                    }
                  },
                  "additionalProperties": false,
                  "required": [
                    "command",
                    "id",
                    "expected_version",
                    "expected_task_version"
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

## GET /v1/repos/{repo}/work/commands/{request_id}

Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.

```text
{
  "operationId": "work_status",
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
            "$ref": "#/components/schemas/WorkCommandStatus"
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

## HEAD /v1/repos/{repo}/work/commands/{request_id}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "work_status_head",
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

## POST /v1/repos/{repo}/index/kinds/tasks

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "prepare_kind",
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
    "202": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/KindPreparationTask"
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
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "operation": {
              "type": "string",
              "pattern": "^[0-9a-f]{128}$",
              "minLength": 128,
              "maxLength": 128
            },
            "kind": {
              "type": "string",
              "enum": [
                "operations",
                "copies"
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "operation",
            "kind"
          ]
        }
      }
    }
  }
}
```

## GET /v1/repos/{repo}/index/kinds/tasks/{task}

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "kind_task",
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
      "name": "task",
      "in": "path",
      "required": true,
      "schema": {
        "type": "string",
        "description": "Opaque process-local task ID; grants no access."
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/KindPreparationTask"
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

## HEAD /v1/repos/{repo}/index/kinds/tasks/{task}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "kind_task_head",
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
      "name": "task",
      "in": "path",
      "required": true,
      "schema": {
        "type": "string",
        "description": "Opaque process-local task ID; grants no access."
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

## POST /v1/repos/{repo}/index/kinds/tasks/{task}/cancel

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "cancel_kind",
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
      "name": "task",
      "in": "path",
      "required": true,
      "schema": {
        "type": "string",
        "description": "Opaque process-local task ID; grants no access."
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/KindPreparationTask"
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

## POST /v1/repos/{repo}/code-index/tasks

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "prepare_code",
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
    "202": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/IndexTask"
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
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "operation": {
              "type": "string",
              "pattern": "^[0-9a-f]{128}$",
              "minLength": 128,
              "maxLength": 128
            }
          },
          "additionalProperties": false,
          "required": [
            "operation"
          ]
        }
      }
    }
  }
}
```

## GET /v1/repos/{repo}/code-index/tasks/{task}

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "code_task",
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
      "name": "task",
      "in": "path",
      "required": true,
      "schema": {
        "type": "string",
        "description": "Opaque process-local task ID; grants no access."
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/IndexTask"
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

## HEAD /v1/repos/{repo}/code-index/tasks/{task}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "code_task_head",
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
      "name": "task",
      "in": "path",
      "required": true,
      "schema": {
        "type": "string",
        "description": "Opaque process-local task ID; grants no access."
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

## POST /v1/repos/{repo}/code-index/tasks/{task}/cancel

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "cancel_code",
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
      "name": "task",
      "in": "path",
      "required": true,
      "schema": {
        "type": "string",
        "description": "Opaque process-local task ID; grants no access."
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/IndexTask"
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

## POST /v1/repos/{repo}/index/tasks

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "prepare",
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
    "202": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/IndexTask"
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
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "operation": {
              "type": "string",
              "pattern": "^[0-9a-f]{128}$",
              "minLength": 128,
              "maxLength": 128
            }
          },
          "additionalProperties": false,
          "required": [
            "operation"
          ]
        }
      }
    }
  }
}
```

## GET /v1/repos/{repo}/index/tasks/{task}

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "task",
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
      "name": "task",
      "in": "path",
      "required": true,
      "schema": {
        "type": "string",
        "description": "Opaque process-local task ID; grants no access."
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/IndexTask"
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

## HEAD /v1/repos/{repo}/index/tasks/{task}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "task_head",
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
      "name": "task",
      "in": "path",
      "required": true,
      "schema": {
        "type": "string",
        "description": "Opaque process-local task ID; grants no access."
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

## POST /v1/repos/{repo}/index/tasks/{task}/cancel

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "cancel",
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
      "name": "task",
      "in": "path",
      "required": true,
      "schema": {
        "type": "string",
        "description": "Opaque process-local task ID; grants no access."
      }
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/IndexTask"
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

