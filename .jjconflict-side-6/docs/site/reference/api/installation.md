# Installation and device

Generated from OpenAPI. See [transport semantics](../transport.md), [capabilities](../capabilities.md) and [data types](types.md).

## GET /v1/bootstrap

Transport-protected deployment bootstrap. Exact authority, protocol-version, Origin and Fetch Metadata checks apply before this response. LocalTrusted repository entries are installation-owned discovery metadata, never repository authority. Authenticated deployments expose no repository catalog.

```text
{
  "operationId": "bootstrap",
  "tags": [
    "reads"
  ],
  "description": "Transport-protected deployment bootstrap. Exact authority, protocol-version, Origin and Fetch Metadata checks apply before this response. LocalTrusted repository entries are installation-owned discovery metadata, never repository authority. Authenticated deployments expose no repository catalog.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/Bootstrap"
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
    {}
  ]
}
```

## HEAD /v1/bootstrap

Transport-protected deployment bootstrap. Exact authority, protocol-version, Origin and Fetch Metadata checks apply before this response. LocalTrusted repository entries are installation-owned discovery metadata, never repository authority. Authenticated deployments expose no repository catalog.

```text
{
  "operationId": "bootstrap_head",
  "tags": [
    "reads"
  ],
  "description": "Transport-protected deployment bootstrap. Exact authority, protocol-version, Origin and Fetch Metadata checks apply before this response. LocalTrusted repository entries are installation-owned discovery metadata, never repository authority. Authenticated deployments expose no repository catalog.",
  "parameters": [],
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
    {}
  ]
}
```

## GET /v1/repositories

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "repositories",
  "tags": [
    "reads"
  ],
  "description": "Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.",
  "parameters": [
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
            "$ref": "#/components/schemas/RepositoryPage"
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

## HEAD /v1/repositories

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "repositories_head",
  "tags": [
    "reads"
  ],
  "description": "Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.",
  "parameters": [
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

## GET /v1/repos/{repo}/local/search

Requires current repository read permission. Only explicitly configured repositories advertise search. Search pins a native view and Tantivy generation, reauthorizes every page, and never publishes. q is required; at_op selects visible-commit mode, historical mode is explicit. All query inputs must match a continuation. Minimum-position waits are bounded; 409 index_lag includes authorized search_status. Expired/restarted cursors require reset. Indexed positions are distinct from current repository positions. Source strings are untrusted data.

```text
{
  "operationId": "local_search_settings",
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
            "$ref": "#/components/schemas/LocalSearchSettings"
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

## HEAD /v1/repos/{repo}/local/search

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "local_search_settings_head",
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

## POST /v1/repos/{repo}/local/search/save

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "save_search_settings",
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
            "$ref": "#/components/schemas/LocalSearchSettings"
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
            "version": {
              "type": "integer",
              "format": "uint32",
              "minimum": 0
            },
            "expected_revision": {
              "type": "string"
            },
            "configuration": {
              "type": "object",
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
                    "oneOf": [
                      {
                        "type": "object",
                        "properties": {
                          "name": {
                            "type": "string"
                          },
                          "kind": {
                            "type": "string",
                            "const": "bookmark"
                          }
                        },
                        "additionalProperties": false,
                        "required": [
                          "kind",
                          "name"
                        ]
                      },
                      {
                        "type": "object",
                        "properties": {
                          "name": {
                            "type": "string"
                          },
                          "kind": {
                            "type": "string",
                            "const": "workspace"
                          }
                        },
                        "additionalProperties": false,
                        "required": [
                          "kind",
                          "name"
                        ]
                      }
                    ]
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
              "additionalProperties": false,
              "required": [
                "enabled",
                "index_source",
                "scopes",
                "include_prefixes",
                "exclude_prefixes",
                "index_bytes"
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "expected_revision",
            "configuration"
          ]
        }
      }
    }
  }
}
```

## POST /v1/repos/{repo}/local/search/control

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "control_search",
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
            "$ref": "#/components/schemas/LocalSearchSettings"
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
            "version": {
              "type": "integer",
              "format": "uint32",
              "minimum": 0
            },
            "expected_revision": {
              "type": "string"
            },
            "action": {
              "type": "string",
              "enum": [
                "apply",
                "retry",
                "rebuild"
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "expected_revision",
            "action"
          ]
        }
      }
    }
  }
}
```

## GET /v1/repos/{repo}/local/storage

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "local_storage_overview",
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
            "$ref": "#/components/schemas/StorageOverview"
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

## HEAD /v1/repos/{repo}/local/storage

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "local_storage_overview_head",
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

## POST /v1/repos/{repo}/local/storage/operations

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "local_storage_command",
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
            "$ref": "#/components/schemas/StorageOperation"
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
            "action": {
              "type": "string",
              "enum": [
                "check",
                "prepare_checkpoint",
                "publish_checkpoint",
                "cancel_checkpoint"
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "request_id",
            "action"
          ]
        }
      }
    }
  }
}
```

## GET /v1/repos/{repo}/local/storage/operations/{request_id}

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "local_storage_operation",
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
            "$ref": "#/components/schemas/StorageOperation"
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

## HEAD /v1/repos/{repo}/local/storage/operations/{request_id}

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "local_storage_operation_head",
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

## GET /v1/repos/{repo}/local/sync

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "local_sync_status",
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
            "$ref": "#/components/schemas/LocalSyncStatus"
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

## HEAD /v1/repos/{repo}/local/sync

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "local_sync_status_head",
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

## POST /v1/repos/{repo}/local/sync/barrier

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "local_sync_barrier",
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
            "$ref": "#/components/schemas/SyncBarrier"
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
          "additionalProperties": false
        }
      }
    }
  }
}
```

## POST /v1/repos/{repo}/local/sync/wait

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "local_sync_wait",
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
            "$ref": "#/components/schemas/LocalSyncWait"
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
            "version": {
              "type": "integer",
              "format": "uint32",
              "minimum": 0
            },
            "barrier": {
              "type": "object",
              "properties": {
                "version": {
                  "type": "integer",
                  "format": "uint32",
                  "minimum": 0
                },
                "repository": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{32}$",
                  "minLength": 32,
                  "maxLength": 32
                },
                "local_replica": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{32}$",
                  "minLength": 32,
                  "maxLength": 32
                },
                "peer_replica": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{32}$",
                  "minLength": 32,
                  "maxLength": 32
                },
                "push_epoch": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{32}$",
                  "minLength": 32,
                  "maxLength": 32
                },
                "push_through": {
                  "description": "Publication order is distinct from native operation ancestry.",
                  "type": "integer",
                  "format": "uint64",
                  "minimum": 0
                },
                "pull_epoch": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{32}$",
                  "minLength": 32,
                  "maxLength": 32
                },
                "pull_through": {
                  "description": "Publication order is distinct from native operation ancestry.",
                  "type": "integer",
                  "format": "uint64",
                  "minimum": 0
                }
              },
              "additionalProperties": false,
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
            },
            "timeout_millis": {
              "type": "integer",
              "format": "uint64",
              "minimum": 0
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "barrier",
            "timeout_millis"
          ]
        }
      }
    }
  }
}
```

## POST /v1/repos/{repo}/local/sync/control

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "local_sync_control",
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
            "$ref": "#/components/schemas/LocalSyncStatus"
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
            "version": {
              "type": "integer",
              "format": "uint32",
              "minimum": 0
            },
            "action": {
              "type": "string",
              "enum": [
                "pause",
                "resume",
                "retry"
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "action"
          ]
        }
      }
    }
  }
}
```

## GET /v1/repos/{repo}/local/workspaces

Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.

```text
{
  "operationId": "local_workspaces",
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
            "$ref": "#/components/schemas/WorkspaceLinks"
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

## HEAD /v1/repos/{repo}/local/workspaces

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "local_workspaces_head",
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

## POST /v1/repos/{repo}/local/workspaces/pick

Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.

```text
{
  "operationId": "pick_workspace_folder",
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
            "$ref": "#/components/schemas/FolderSelection"
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
          "additionalProperties": false
        }
      }
    }
  }
}
```

## POST /v1/repos/{repo}/local/workspaces/link

Home-authorized native project/task metadata. Reads retain an immutable WorkStore root across query-bound opaque continuations. Commands derive authenticated actors and return durable receipts; unknown outcomes retain request IDs. Opening linked code requires separate target repository access. WorkStore origin replication is unsupported.

```text
{
  "operationId": "link_workspace",
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
            "$ref": "#/components/schemas/WorkspaceLinks"
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
            "expected_revision": {
              "type": "integer",
              "format": "uint64",
              "minimum": 0
            },
            "path": {
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "expected_revision",
            "path"
          ]
        }
      }
    }
  }
}
```

## GET /v1/events

Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.

```text
{
  "operationId": "application_events",
  "tags": [
    "reads"
  ],
  "description": "Requires current repository read permission. Unknown query fields are rejected. Source strings are untrusted data. Reads never reconcile or publish native operations.",
  "parameters": [
    {
      "name": "after",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "maxLength": 128,
        "pattern": "^jse1:[0-9a-f]{32}:[0-9a-f]{32}:(0|[1-9][0-9]*)$"
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
        "pattern": "^jse1:[0-9a-f]{32}:[0-9a-f]{32}:(0|[1-9][0-9]*)$"
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
            "$ref": "#/components/schemas/ApplicationFreshnessEvent"
          },
          "x-sse-events": {
            "ready": "ready",
            "changed": "changed",
            "reset": "reset"
          },
          "x-sse-id-schema": {
            "type": "string",
            "maxLength": 128,
            "pattern": "^jse1:[0-9a-f]{32}:[0-9a-f]{32}:(0|[1-9][0-9]*)$"
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

## HEAD /v1/events

Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.

```text
{
  "operationId": "application_events_head",
  "tags": [
    "reads"
  ],
  "description": "Runs the same authorized GET validation and drops the body. No JSON, binary bytes or SSE frames are returned.",
  "parameters": [
    {
      "name": "after",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "maxLength": 128,
        "pattern": "^jse1:[0-9a-f]{32}:[0-9a-f]{32}:(0|[1-9][0-9]*)$"
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
        "pattern": "^jse1:[0-9a-f]{32}:[0-9a-f]{32}:(0|[1-9][0-9]*)$"
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

## GET /v1/local/repositories

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_list",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/LocalBindings"
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
    {}
  ]
}
```

## HEAD /v1/local/repositories

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_list_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
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
    {}
  ]
}
```

## POST /v1/local/repositories

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_register",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/RegistrationOutcome"
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
            "expected_revision": {
              "type": "integer",
              "format": "uint64",
              "minimum": 0
            },
            "path": {
              "type": "string"
            },
            "name": {
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "request_id",
            "expected_revision",
            "path",
            "name"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repositories/inspect

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_inspect",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/LocalFolder"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "path": {
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "path"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repositories/native

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_native_repository_create",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/NativeCreationOutcome"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "description": "Installation-owned creation, independent of physical workspace attachment.",
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
            "name": {
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "request_id",
            "name"
          ]
        }
      }
    }
  }
}
```

## GET /v1/local/repositories/native/requests/{request_id}

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_native_repository_status",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [
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
            "$ref": "#/components/schemas/NativeCreationOutcome"
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
    {}
  ]
}
```

## HEAD /v1/local/repositories/native/requests/{request_id}

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_native_repository_status_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [
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
    {}
  ]
}
```

## POST /v1/local/repositories/pick

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_pick",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/FolderSelection"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "additionalProperties": false
        }
      }
    }
  }
}
```

## GET /v1/local/repositories/requests/{request_id}

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_status",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [
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
            "$ref": "#/components/schemas/RegistrationOutcome"
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
    {}
  ]
}
```

## HEAD /v1/local/repositories/requests/{request_id}

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_status_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [
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
    {}
  ]
}
```

## POST /v1/local/repositories/{repo}/reconnect

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_reconnect",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalBindings"
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
            "expected_revision": {
              "type": "integer",
              "format": "uint64",
              "minimum": 0
            },
            "path": {
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "expected_revision",
            "path"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repositories/{repo}/remove

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_remove",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalBindings"
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
            "expected_revision": {
              "type": "integer",
              "format": "uint64",
              "minimum": 0
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "expected_revision"
          ]
        }
      }
    }
  }
}
```

## GET /v1/local/repos/{repo}/snapshot

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_code_snapshot",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalRepositorySnapshot"
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
    {}
  ]
}
```

## HEAD /v1/local/repos/{repo}/snapshot

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_code_snapshot_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
    {}
  ]
}
```

## GET /v1/local/repos/{repo}/capabilities

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_code_capabilities",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalRepositoryCapabilities"
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
    {}
  ]
}
```

## HEAD /v1/local/repos/{repo}/capabilities

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_code_capabilities_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
    {}
  ]
}
```

## POST /v1/local/repos/{repo}/query

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_code_query",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalCodeQueryResponse"
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
            "request": {
              "oneOf": [
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "type": "string",
                      "const": "graph"
                    },
                    "query": {
                      "type": "array",
                      "prefixItems": [
                        {
                          "oneOf": [
                            {
                              "type": "object",
                              "properties": {
                                "revset": {
                                  "type": "string"
                                },
                                "workspace": {
                                  "type": [
                                    "string",
                                    "null"
                                  ]
                                },
                                "user_email": {
                                  "type": "string"
                                },
                                "at_time_millis": {
                                  "type": "integer",
                                  "format": "int64"
                                },
                                "graph": {
                                  "type": "string",
                                  "const": "log"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "graph",
                                "revset",
                                "user_email",
                                "at_time_millis"
                              ]
                            },
                            {
                              "type": "object",
                              "properties": {
                                "graph": {
                                  "type": "string",
                                  "const": "operations"
                                }
                              },
                              "required": [
                                "graph"
                              ],
                              "additionalProperties": false
                            },
                            {
                              "type": "object",
                              "properties": {
                                "query": {
                                  "type": "string"
                                },
                                "graph": {
                                  "type": "string",
                                  "const": "operations_filtered"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "graph",
                                "query"
                              ]
                            },
                            {
                              "type": "object",
                              "properties": {
                                "commit": {
                                  "type": "string",
                                  "pattern": "^[0-9a-f]{40}$",
                                  "minLength": 40,
                                  "maxLength": 40
                                },
                                "graph": {
                                  "type": "string",
                                  "const": "evolution"
                                }
                              },
                              "additionalProperties": false,
                              "required": [
                                "graph",
                                "commit"
                              ]
                            }
                          ]
                        },
                        {
                          "type": "object",
                          "properties": {
                            "limit": {
                              "type": "integer",
                              "format": "uint",
                              "minimum": 0
                            },
                            "after": {
                              "anyOf": [
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
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{40}$",
                                          "minLength": 40,
                                          "maxLength": 40
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
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{40}$",
                                          "minLength": 40,
                                          "maxLength": 40
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
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{40}$",
                                          "minLength": 40,
                                          "maxLength": 40
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
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{40}$",
                                          "minLength": 40,
                                          "maxLength": 40
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
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{128}$",
                                          "minLength": 128,
                                          "maxLength": 128
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
                                          "type": "string",
                                          "pattern": "^[0-9a-f]{128}$",
                                          "minLength": 128,
                                          "maxLength": 128
                                        }
                                      },
                                      "required": [
                                        "kind",
                                        "id"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "type": "null"
                                }
                              ]
                            }
                          },
                          "additionalProperties": false,
                          "required": [
                            "limit"
                          ]
                        }
                      ],
                      "minItems": 2,
                      "maxItems": 2
                    }
                  },
                  "required": [
                    "kind",
                    "query"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "type": "string",
                      "const": "query_info"
                    }
                  },
                  "required": [
                    "kind"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "type": "string",
                      "const": "commit"
                    },
                    "query": {
                      "type": "object",
                      "properties": {
                        "commit": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{40}$",
                          "minLength": 40,
                          "maxLength": 40
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "commit"
                      ]
                    }
                  },
                  "required": [
                    "kind",
                    "query"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "type": "string",
                      "const": "commit_summary"
                    },
                    "query": {
                      "type": "object",
                      "properties": {
                        "commit": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{40}$",
                          "minLength": 40,
                          "maxLength": 40
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "commit"
                      ]
                    }
                  },
                  "required": [
                    "kind",
                    "query"
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
                    "query": {
                      "type": "object",
                      "properties": {
                        "file": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{40}$",
                          "minLength": 40,
                          "maxLength": 40
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "file"
                      ]
                    }
                  },
                  "required": [
                    "kind",
                    "query"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "type": "string",
                      "const": "file_at"
                    },
                    "query": {
                      "type": "object",
                      "properties": {
                        "commit": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{40}$",
                          "minLength": 40,
                          "maxLength": 40
                        },
                        "path": {
                          "type": "string"
                        },
                        "file": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{40}$",
                          "minLength": 40,
                          "maxLength": 40
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "commit",
                        "path",
                        "file"
                      ]
                    }
                  },
                  "required": [
                    "kind",
                    "query"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "type": "string",
                      "const": "paths"
                    },
                    "query": {
                      "type": "object",
                      "properties": {
                        "commit": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{40}$",
                          "minLength": 40,
                          "maxLength": 40
                        },
                        "path": {
                          "type": "string"
                        },
                        "recursive": {
                          "type": "boolean"
                        },
                        "include_last_change": {
                          "type": "boolean",
                          "default": true
                        },
                        "limit": {
                          "type": "integer",
                          "format": "uint",
                          "minimum": 0
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
                          ]
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "commit",
                        "path",
                        "recursive",
                        "limit",
                        "direction"
                      ]
                    }
                  },
                  "required": [
                    "kind",
                    "query"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "type": "string",
                      "const": "references"
                    },
                    "query": {
                      "type": "object",
                      "properties": {
                        "operation": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{128}$",
                          "minLength": 128,
                          "maxLength": 128
                        },
                        "query": {
                          "type": "string"
                        },
                        "exact": {
                          "type": [
                            "string",
                            "null"
                          ]
                        },
                        "kind": {
                          "type": "string",
                          "enum": [
                            "bookmarks",
                            "workspaces",
                            "changes",
                            "all"
                          ]
                        },
                        "limit": {
                          "type": "integer",
                          "format": "uint",
                          "minimum": 0
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
                          ]
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "operation",
                        "query",
                        "kind",
                        "limit",
                        "direction"
                      ]
                    }
                  },
                  "required": [
                    "kind",
                    "query"
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
                    "query": {
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
                  },
                  "required": [
                    "kind",
                    "query"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "type": "string",
                      "const": "operation_diff"
                    },
                    "query": {
                      "type": "object",
                      "properties": {
                        "operation": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{128}$",
                          "minLength": 128,
                          "maxLength": 128
                        },
                        "parent": {
                          "type": [
                            "string",
                            "null"
                          ],
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
                  },
                  "required": [
                    "kind",
                    "query"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "type": "string",
                      "const": "operation_changes"
                    },
                    "query": {
                      "type": "object",
                      "properties": {
                        "operation": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{128}$",
                          "minLength": 128,
                          "maxLength": 128
                        },
                        "parent": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{128}$",
                          "minLength": 128,
                          "maxLength": 128
                        },
                        "limit": {
                          "type": "integer",
                          "format": "uint",
                          "minimum": 0
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
                          ]
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "operation",
                        "parent",
                        "limit",
                        "direction"
                      ]
                    }
                  },
                  "required": [
                    "kind",
                    "query"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "type": "string",
                      "const": "diff"
                    },
                    "query": {
                      "type": "object",
                      "properties": {
                        "from": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{40}$",
                          "minLength": 40,
                          "maxLength": 40
                        },
                        "to": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{40}$",
                          "minLength": 40,
                          "maxLength": 40
                        },
                        "path": {
                          "type": "string"
                        },
                        "content": {
                          "type": "boolean"
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "from",
                        "to",
                        "path",
                        "content"
                      ]
                    }
                  },
                  "required": [
                    "kind",
                    "query"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "type": "string",
                      "const": "diff_paths"
                    },
                    "query": {
                      "type": "object",
                      "properties": {
                        "from": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{40}$",
                          "minLength": 40,
                          "maxLength": 40
                        },
                        "to": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{40}$",
                          "minLength": 40,
                          "maxLength": 40
                        },
                        "path": {
                          "type": "string"
                        },
                        "limit": {
                          "type": "integer",
                          "format": "uint",
                          "minimum": 0
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
                          ]
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "from",
                        "to",
                        "path",
                        "limit",
                        "direction"
                      ]
                    }
                  },
                  "required": [
                    "kind",
                    "query"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "kind": {
                      "type": "string",
                      "const": "conflicts"
                    },
                    "query": {
                      "type": "object",
                      "properties": {
                        "commit": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{40}$",
                          "minLength": 40,
                          "maxLength": 40
                        },
                        "path": {
                          "type": "string"
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "commit",
                        "path"
                      ]
                    }
                  },
                  "required": [
                    "kind",
                    "query"
                  ],
                  "additionalProperties": false
                }
              ]
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
            "operation",
            "request"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repos/{repo}/index/tasks

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_index_prepare",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalIndexTask"
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

## GET /v1/local/repos/{repo}/index/tasks/{task}

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_index_status",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalIndexTask"
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
    {}
  ]
}
```

## HEAD /v1/local/repos/{repo}/index/tasks/{task}

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_index_status_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
    {}
  ]
}
```

## POST /v1/local/repos/{repo}/index/tasks/{task}/cancel

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_index_cancel",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalIndexTask"
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
    {}
  ]
}
```

## POST /v1/local/repos/{repo}/work/page

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_work_page",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalWorkPage"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "metadata_commit": {
              "type": "string",
              "pattern": "^[0-9a-f]{40}$",
              "minLength": 40,
              "maxLength": 40
            },
            "query": {
              "type": "object",
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
              },
              "additionalProperties": false
            }
          },
          "additionalProperties": false,
          "required": [
            "metadata_commit",
            "query"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repos/{repo}/work/browse

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_work_browse",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalWorkBrowsePage"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "metadata_commit": {
              "type": "string",
              "pattern": "^[0-9a-f]{40}$",
              "minLength": 40,
              "maxLength": 40
            },
            "query": {
              "type": "object",
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
              },
              "additionalProperties": false
            }
          },
          "additionalProperties": false,
          "required": [
            "metadata_commit",
            "query"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repos/{repo}/work/commands

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_work_apply",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalWorkOutcome"
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
                    },
                    "command": {
                      "type": "string",
                      "const": "create_project"
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
                    },
                    "command": {
                      "type": "string",
                      "const": "update_project"
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
                    },
                    "command": {
                      "type": "string",
                      "const": "set_repository"
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
                    },
                    "command": {
                      "type": "string",
                      "const": "create_task"
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
                    },
                    "command": {
                      "type": "string",
                      "const": "update_task"
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
                    },
                    "command": {
                      "type": "string",
                      "const": "add_comment"
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
                    },
                    "command": {
                      "type": "string",
                      "const": "edit_comment"
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
                    "command": {
                      "type": "string",
                      "const": "tombstone_comment"
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

## GET /v1/local/repos/{repo}/work/commands/{request_id}

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_work_status",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalWorkOutcome"
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
    {}
  ]
}
```

## HEAD /v1/local/repos/{repo}/work/commands/{request_id}

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_work_status_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
    {}
  ]
}
```

## POST /v1/local/repos/{repo}/agents/sessions

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_agent_sessions",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/Page5"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "metadata_commit": {
              "type": "string",
              "pattern": "^[0-9a-f]{40}$",
              "minLength": 40,
              "maxLength": 40
            },
            "query": {
              "type": "object",
              "properties": {
                "limit": {
                  "type": "integer",
                  "format": "uint",
                  "minimum": 0
                }
              },
              "additionalProperties": false,
              "required": [
                "limit"
              ]
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
            "metadata_commit",
            "query"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repos/{repo}/agents/session

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_agent_session",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalBrowserAgentDetail"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "metadata_commit": {
              "type": "string",
              "pattern": "^[0-9a-f]{40}$",
              "minLength": 40,
              "maxLength": 40
            },
            "query": {
              "type": "object",
              "properties": {
                "session_id": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{32}$",
                  "minLength": 32,
                  "maxLength": 32
                }
              },
              "additionalProperties": false,
              "required": [
                "session_id"
              ]
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
            "metadata_commit",
            "query"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repos/{repo}/agents/transcript

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_agent_transcript",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/Page6"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "metadata_commit": {
              "type": "string",
              "pattern": "^[0-9a-f]{40}$",
              "minLength": 40,
              "maxLength": 40
            },
            "query": {
              "type": "object",
              "properties": {
                "session_id": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{32}$",
                  "minLength": 32,
                  "maxLength": 32
                },
                "transcript_id": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{64}$",
                  "minLength": 64,
                  "maxLength": 64
                },
                "source_instance_id": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{32}$",
                  "minLength": 32,
                  "maxLength": 32
                },
                "source_generation": {
                  "type": "integer",
                  "format": "uint64",
                  "minimum": 0
                },
                "limit": {
                  "type": "integer",
                  "format": "uint",
                  "minimum": 0
                },
                "max_bytes": {
                  "type": "integer",
                  "format": "uint",
                  "minimum": 0
                }
              },
              "additionalProperties": false,
              "required": [
                "session_id",
                "transcript_id",
                "source_instance_id",
                "source_generation",
                "limit",
                "max_bytes"
              ]
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
            "metadata_commit",
            "query"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repos/{repo}/agents/relations

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_agent_relations",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/Page7"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "metadata_commit": {
              "type": "string",
              "pattern": "^[0-9a-f]{40}$",
              "minLength": 40,
              "maxLength": 40
            },
            "query": {
              "type": "object",
              "properties": {
                "subject": {
                  "oneOf": [
                    {
                      "type": "object",
                      "properties": {
                        "session_id": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{32}$",
                          "minLength": 32,
                          "maxLength": 32
                        },
                        "subject": {
                          "type": "string",
                          "const": "session"
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "subject",
                        "session_id"
                      ]
                    },
                    {
                      "type": "object",
                      "properties": {
                        "repository": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{32}$",
                          "minLength": 32,
                          "maxLength": 32
                        },
                        "operation_id": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{128}$",
                          "minLength": 128,
                          "maxLength": 128
                        },
                        "subject": {
                          "type": "string",
                          "const": "operation"
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "subject",
                        "repository",
                        "operation_id"
                      ]
                    },
                    {
                      "type": "object",
                      "properties": {
                        "repository": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{32}$",
                          "minLength": 32,
                          "maxLength": 32
                        },
                        "commit_id": {
                          "type": "string",
                          "pattern": "^[0-9a-f]{40}$",
                          "minLength": 40,
                          "maxLength": 40
                        },
                        "subject": {
                          "type": "string",
                          "const": "revision"
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "subject",
                        "repository",
                        "commit_id"
                      ]
                    }
                  ]
                },
                "limit": {
                  "type": "integer",
                  "format": "uint",
                  "minimum": 0
                }
              },
              "additionalProperties": false,
              "required": [
                "subject",
                "limit"
              ]
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
            "metadata_commit",
            "query"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repos/{repo}/reviews/page

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_review_page",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalReviewPageResponse"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "metadata_commit": {
              "type": "string",
              "pattern": "^[0-9a-f]{40}$",
              "minLength": 40,
              "maxLength": 40
            },
            "limit": {
              "type": "integer",
              "format": "uint",
              "minimum": 0
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
            "metadata_commit",
            "limit"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repos/{repo}/reviews/detail

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_review_detail",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalReviewThreadDetail"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "metadata_commit": {
              "type": "string",
              "pattern": "^[0-9a-f]{40}$",
              "minLength": 40,
              "maxLength": 40
            },
            "thread_id": {
              "type": "string",
              "pattern": "^[0-9a-f]{32}$",
              "minLength": 32,
              "maxLength": 32
            }
          },
          "additionalProperties": false,
          "required": [
            "metadata_commit",
            "thread_id"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repos/{repo}/reviews/commands

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_review_apply",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalReviewOutcome"
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

## GET /v1/local/repos/{repo}/reviews/commands/{request_id}

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_review_status",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/LocalReviewOutcome"
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
    {}
  ]
}
```

## HEAD /v1/local/repos/{repo}/reviews/commands/{request_id}

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_review_status_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
    {}
  ]
}
```

## POST /v1/local/repos/{repo}/working-copy/read

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_working_file_read",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/WorkingFile"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "path": {
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "path"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repos/{repo}/working-copy/save

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_working_file_save",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
            "$ref": "#/components/schemas/WorkingFile"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "selection": {
              "type": "object",
              "properties": {
                "version": {
                  "type": "integer",
                  "format": "uint32",
                  "minimum": 0
                },
                "repository": {
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
              "additionalProperties": false,
              "required": [
                "version",
                "repository",
                "generation",
                "workspace",
                "path",
                "fingerprint"
              ]
            },
            "contents": {
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "selection",
            "contents"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repositories/conversion/preview

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_conversion_preview",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ConversionPreview"
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
            "source": {
              "type": "string",
              "pattern": "^[0-9a-f]{32}$",
              "minLength": 32,
              "maxLength": 32
            },
            "source_profile": {
              "type": "string",
              "enum": [
                "native",
                "local_jj_git"
              ]
            },
            "operation": {
              "type": "string",
              "pattern": "^[0-9a-f]{128}$",
              "minLength": 128,
              "maxLength": 128
            },
            "destination_profile": {
              "type": "string",
              "enum": [
                "native",
                "local_jj_git"
              ]
            },
            "name": {
              "type": "string"
            },
            "path": {
              "type": [
                "string",
                "null"
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "source",
            "source_profile",
            "operation",
            "destination_profile",
            "name"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/repositories/conversion

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_conversion_start",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/ConversionOutcome"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "type": "object",
          "properties": {
            "request_id": {
              "type": "string",
              "pattern": "^[0-9a-f]{32}$",
              "minLength": 32,
              "maxLength": 32
            },
            "intent": {
              "type": "object",
              "properties": {
                "version": {
                  "type": "integer",
                  "format": "uint32",
                  "minimum": 0
                },
                "source": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{32}$",
                  "minLength": 32,
                  "maxLength": 32
                },
                "source_profile": {
                  "type": "string",
                  "enum": [
                    "native",
                    "local_jj_git"
                  ]
                },
                "operation": {
                  "type": "string",
                  "pattern": "^[0-9a-f]{128}$",
                  "minLength": 128,
                  "maxLength": 128
                },
                "destination_profile": {
                  "type": "string",
                  "enum": [
                    "native",
                    "local_jj_git"
                  ]
                },
                "name": {
                  "type": "string"
                },
                "path": {
                  "type": [
                    "string",
                    "null"
                  ]
                }
              },
              "additionalProperties": false,
              "required": [
                "version",
                "source",
                "source_profile",
                "operation",
                "destination_profile",
                "name"
              ]
            },
            "prepared_source": {
              "type": "string"
            }
          },
          "additionalProperties": false,
          "required": [
            "request_id",
            "intent",
            "prepared_source"
          ]
        }
      }
    }
  }
}
```

## GET /v1/local/repositories/conversion/requests/{request_id}

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_conversion_status",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [
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
            "$ref": "#/components/schemas/ConversionOutcome"
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
    {}
  ]
}
```

## HEAD /v1/local/repositories/conversion/requests/{request_id}

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_repository_conversion_status_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [
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
    {}
  ]
}
```

## GET /v1/local/repos/{repo}/events

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_freshness",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
      "description": "Bounded LocalTrusted freshness stream; no durable replay cursor. Compare ready heads and companion ref against the retained display selection after reconnect. Only explicit refresh changes that selection.",
      "content": {
        "text/event-stream": {
          "schema": {
            "type": "string"
          },
          "x-sse-data-schema": {
            "$ref": "#/components/schemas/LocalFreshnessEvent"
          },
          "x-sse-events": {
            "ready": "ready",
            "repository-changed": "changed"
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
    {}
  ]
}
```

## HEAD /v1/local/repos/{repo}/events

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_freshness_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
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
    {}
  ]
}
```

## GET /v1/local/capture/settings

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_capture_settings",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/LocalCaptureSettings"
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
    {}
  ]
}
```

## HEAD /v1/local/capture/settings

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_capture_settings_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
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
    {}
  ]
}
```

## POST /v1/local/capture/settings

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_capture_save_settings",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/LocalCaptureSettings"
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
            "expected_revision": {
              "type": "string",
              "minLength": 64,
              "maxLength": 64,
              "pattern": "^[0-9a-f]{64}$"
            },
            "configuration": {
              "type": "object",
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
                    "type": "object",
                    "properties": {
                      "profile": {
                        "type": "string",
                        "enum": [
                          "codex-cli-jsonl-v1",
                          "codex-desktop-jsonl-v1",
                          "cursor-desktop-transcript-v1",
                          "cursor-cli-transcript-v1",
                          "opencode-read-only-api-v1",
                          "opencode-export-json-v1"
                        ]
                      },
                      "product_version": {
                        "type": "string",
                        "minLength": 1,
                        "maxLength": 32
                      },
                      "roots": {
                        "type": "array",
                        "items": {
                          "description": "Startup-selected directory metadata, not a discovered transcript file.",
                          "type": "string",
                          "minLength": 1,
                          "maxLength": 4096
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
                    "additionalProperties": false,
                    "required": [
                      "profile",
                      "product_version",
                      "roots"
                    ]
                  },
                  "maxItems": 5
                }
              },
              "additionalProperties": false,
              "required": [
                "enabled",
                "max_readers",
                "rescan_seconds",
                "max_sources",
                "max_pending_bytes",
                "harnesses"
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "expected_revision",
            "configuration"
          ]
        }
      }
    }
  }
}
```

## GET /v1/local/capture

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_capture_status",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/CaptureStatus"
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
    {}
  ]
}
```

## HEAD /v1/local/capture

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_capture_status_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
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
    {}
  ]
}
```

## GET /v1/local/capture/sources

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_capture_sources",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [
    {
      "name": "limit",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint",
        "minimum": 1,
        "maximum": 100
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "cursor",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "minLength": 49,
        "maxLength": 49,
        "pattern": "^[0-9a-f]{32}:[0-9a-f]{16}$"
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
            "$ref": "#/components/schemas/CaptureSourcePage"
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
        "type": [
          "integer",
          "null"
        ],
        "format": "uint",
        "minimum": 1,
        "maximum": 100
      },
      "cursor": {
        "type": [
          "string",
          "null"
        ],
        "minLength": 49,
        "maxLength": 49,
        "pattern": "^[0-9a-f]{32}:[0-9a-f]{16}$"
      }
    }
  },
  "security": [
    {}
  ]
}
```

## HEAD /v1/local/capture/sources

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_capture_sources_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [
    {
      "name": "limit",
      "in": "query",
      "required": false,
      "schema": {
        "type": "integer",
        "format": "uint",
        "minimum": 1,
        "maximum": 100
      },
      "style": "form",
      "explode": true
    },
    {
      "name": "cursor",
      "in": "query",
      "required": false,
      "schema": {
        "type": "string",
        "minLength": 49,
        "maxLength": 49,
        "pattern": "^[0-9a-f]{32}:[0-9a-f]{16}$"
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
        "type": [
          "integer",
          "null"
        ],
        "format": "uint",
        "minimum": 1,
        "maximum": 100
      },
      "cursor": {
        "type": [
          "string",
          "null"
        ],
        "minLength": 49,
        "maxLength": 49,
        "pattern": "^[0-9a-f]{32}:[0-9a-f]{16}$"
      }
    }
  },
  "security": [
    {}
  ]
}
```

## POST /v1/local/capture/control

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_capture_control",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/CaptureControlOutcome"
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
            "action": {
              "type": "string",
              "enum": [
                "pause",
                "resume"
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "request_id",
            "generation",
            "expected_revision",
            "action"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/capture/rescan

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_capture_rescan",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/CaptureRescanOutcome"
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
    {}
  ],
  "requestBody": {
    "required": true,
    "content": {
      "application/json": {
        "schema": {
          "description": "Transient conditional discovery hint; no durable request identity or actor.",
          "type": "object",
          "properties": {
            "version": {
              "type": "integer",
              "format": "uint32",
              "minimum": 0
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
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "generation",
            "expected_revision"
          ]
        }
      }
    }
  }
}
```

## POST /v1/local/capture/import

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_capture_import",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/CaptureImportOutcome"
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
          "additionalProperties": false,
          "required": [
            "version",
            "request_id",
            "generation",
            "expected_revision",
            "selector"
          ]
        }
      }
    }
  }
}
```

## GET /v1/local/capture/imports

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_capture_import_status",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [
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
    }
  ],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/CaptureImportStatus"
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
      "request_id": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      }
    },
    "required": [
      "request_id"
    ]
  },
  "security": [
    {}
  ]
}
```

## HEAD /v1/local/capture/imports

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_capture_import_status_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [
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
      "request_id": {
        "type": "string",
        "pattern": "^[0-9a-f]{32}$",
        "minLength": 32,
        "maxLength": 32
      }
    },
    "required": [
      "request_id"
    ]
  },
  "security": [
    {}
  ]
}
```

## GET /v1/local/settings

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_server_settings",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/LocalServerSettings"
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
    {}
  ]
}
```

## HEAD /v1/local/settings

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_server_settings_head",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
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
    {}
  ]
}
```

## POST /v1/local/settings

LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.

```text
{
  "operationId": "local_server_save_settings",
  "tags": [
    "local"
  ],
  "description": "LocalTrusted loopback administration only. Exact protocol, authority, Origin and Fetch Metadata admission applies before reading or changing device state. Filesystem code queries pin an explicit operation and never publish heads, refs or checkout files. Registration and collaboration writes retain original request identities for unknown outcomes and replay.",
  "parameters": [],
  "responses": {
    "200": {
      "description": "Authorized result at the reported snapshot.",
      "content": {
        "application/json": {
          "schema": {
            "$ref": "#/components/schemas/LocalServerSettings"
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
            "expected_revision": {
              "type": "string",
              "pattern": "^[0-9a-f]{64}$"
            },
            "update": {
              "oneOf": [
                {
                  "type": "object",
                  "properties": {
                    "section": {
                      "type": "string",
                      "const": "service"
                    },
                    "configuration": {
                      "type": "object",
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
                      "additionalProperties": false,
                      "required": [
                        "concurrent_requests",
                        "request_seconds",
                        "max_blob_bytes",
                        "inventory_nodes"
                      ]
                    }
                  },
                  "required": [
                    "section",
                    "configuration"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "section": {
                      "type": "string",
                      "const": "protection"
                    },
                    "configuration": {
                      "type": "object",
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
                            "type": "string",
                            "pattern": "^[0-9a-f]{128}$",
                            "minLength": 128,
                            "maxLength": 128
                          }
                        }
                      },
                      "additionalProperties": false,
                      "required": [
                        "bookmarks",
                        "commits"
                      ]
                    }
                  },
                  "required": [
                    "section",
                    "configuration"
                  ],
                  "additionalProperties": false
                },
                {
                  "type": "object",
                  "properties": {
                    "section": {
                      "type": "string",
                      "const": "sync"
                    },
                    "configuration": {
                      "type": "array",
                      "items": {
                        "type": "object",
                        "properties": {
                          "repository": {
                            "type": "string",
                            "pattern": "^[0-9a-f]{32}$",
                            "minLength": 32,
                            "maxLength": 32
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
                            "type": "object",
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
                            "additionalProperties": false,
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
                        },
                        "additionalProperties": false,
                        "required": [
                          "repository",
                          "upstream",
                          "insecure_loopback",
                          "token_environment",
                          "limits"
                        ]
                      }
                    }
                  },
                  "required": [
                    "section",
                    "configuration"
                  ],
                  "additionalProperties": false
                }
              ]
            }
          },
          "additionalProperties": false,
          "required": [
            "version",
            "expected_revision",
            "update"
          ]
        }
      }
    }
  }
}
```

