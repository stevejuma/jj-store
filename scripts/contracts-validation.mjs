import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020.js";

export function schemaValidator(document) {
  // OpenAPI annotations and Rust integer formats do not change JSON validation.
  const ajv = new Ajv2020({
    strict: false,
    validateFormats: false,
    allErrors: true,
  });
  const cache = new Map();
  return (schema, value) => {
    const key = JSON.stringify(schema);
    if (!cache.has(key))
      cache.set(
        key,
        ajv.compile({ ...schema, components: document.components }),
      );
    const validate = cache.get(key);
    assert.ok(validate(value), JSON.stringify(validate.errors));
  };
}

export function responseValidator(document) {
  const validate = schemaValidator(document);
  return ({ path, method, status, media, data }) => {
    const operation = document.paths[path]?.[method];
    assert.ok(operation, `Missing route contract: ${method} ${path}`);
    const response = operation.responses[status];
    assert.ok(
      response,
      `Missing response contract: ${status} ${method} ${path}`,
    );
    const content = response.content?.[media];
    assert.ok(content, `Missing media contract: ${media} ${method} ${path}`);
    const schema =
      media === "text/event-stream"
        ? content["x-sse-data-schema"]
        : content.schema;
    assert.ok(schema, `Missing body schema: ${method} ${path}`);
    validate(schema, data);
  };
}
