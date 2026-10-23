import ts from "typescript";
import { parse } from "svelte/compiler";
export function walk(node, visit) {
  if (!node || typeof node !== "object") return;
  if (Array.isArray(node)) {
    for (const value of node) walk(value, visit);
    return;
  }
  if (visit(node) === false) return;
  for (const [key, value] of Object.entries(node))
    if (!["loc", "name_loc", "parent"].includes(key)) walk(value, visit);
}
export function value(node, environment = {}) {
  if (!node) return undefined;
  if (node.type === "Literal") return node.value;
  if (node.type === "Identifier") return environment[node.name];
  if (
    node.type === "CallExpression" &&
    node.callee?.name === "untrack" &&
    node.arguments[0]?.type === "ArrowFunctionExpression"
  )
    return value(node.arguments[0].body, environment);
  if (node.type === "ConditionalExpression") {
    const test = value(node.test, environment);
    return test === undefined
      ? undefined
      : value(test ? node.consequent : node.alternate, environment);
  }
  if (node.type === "ObjectExpression")
    return Object.fromEntries(
      node.properties.map((p) => [
        p.key?.name ?? p.key?.value,
        value(p.value, environment),
      ]),
    );
  if (node.type === "ArrayExpression")
    return node.elements.map((n) => value(n, environment));
  if (node.type === "UnaryExpression" && node.operator === "!") {
    const v = value(node.argument, environment);
    return v === undefined ? undefined : !v;
  }
  if (node.type === "BinaryExpression") {
    const a = value(node.left, environment),
      b = value(node.right, environment);
    if (a === undefined || b === undefined) return undefined;
    if (["===", "=="].includes(node.operator)) return a === b;
    if (["!==", "!="].includes(node.operator)) return a !== b;
  }
  return undefined;
}
export function attr(node, name, environment) {
  const a = node.attributes?.find(
    (a) => a.type === "Attribute" && a.name === name,
  );
  if (!a) return undefined;
  if (a.value === true) return true;
  if (Array.isArray(a.value))
    return a.value.length === 1 && a.value[0].type === "Text"
      ? a.value[0].data
      : undefined;
  return value(a.value?.expression, environment);
}
export function storyModel(source, file) {
  const ast = parse(source, { modern: true });
  const imports = new Map();
  for (const script of [ast.module, ast.instance])
    for (const node of script?.content.body ?? [])
      if (node.type === "ImportDeclaration")
        for (const binding of node.specifiers)
          imports.set(binding.local.name, node.source.value);
  let meta;
  walk(ast.module, (node) => {
    if (node.type === "CallExpression" && node.callee.name === "defineMeta")
      meta = node.arguments[0];
  });
  const metadata = value(meta) ?? {};
  const component = meta?.properties?.find((p) => p.key.name === "component")
    ?.value?.name;
  const stories = [];
  walk(ast.fragment, (node) => {
    if (node.type === "Component" && node.name === "Story") {
      stories.push(node);
      return false;
    }
  });
  return { ast, imports, meta, metadata, component, stories };
}

export function resolveDefaultComponent(file, target, read, seen = new Set()) {
  if (!file || seen.has(file)) return null;
  if (file.endsWith(".svelte")) return file;
  seen.add(file);
  const tree = ts.createSourceFile(
    file,
    read(file),
    ts.ScriptTarget.Latest,
    true,
  );
  const exports = tree.statements.filter(
    (n) =>
      ts.isExportDeclaration(n) &&
      n.moduleSpecifier &&
      n.exportClause?.elements?.some((e) => e.name.text === "default"),
  );
  if (exports.length !== 1) return null;
  return resolveDefaultComponent(
    target(file, exports[0].moduleSpecifier.text),
    target,
    read,
    seen,
  );
}
