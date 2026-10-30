import fs from "node:fs";
import path from "node:path";
import ts from "typescript";

const root = path.resolve(import.meta.dirname, "..");
const i18nDir = path.join(root, "src", "i18n");
const indexPath = path.join(i18nDir, "index.tsx");

function parse(file) {
  return ts.createSourceFile(
    file,
    fs.readFileSync(file, "utf8"),
    ts.ScriptTarget.Latest,
    true,
    file.endsWith(".tsx") ? ts.ScriptKind.TSX : ts.ScriptKind.TS,
  );
}

function propertyName(node) {
  if (ts.isIdentifier(node) || ts.isStringLiteral(node) || ts.isNumericLiteral(node)) return node.text;
  return null;
}

function variable(source, name) {
  for (const statement of source.statements) {
    if (!ts.isVariableStatement(statement)) continue;
    for (const decl of statement.declarationList.declarations) {
      if (ts.isIdentifier(decl.name) && decl.name.text === name) return decl;
    }
  }
  return null;
}

const index = parse(indexPath);
let languages = [];
for (const statement of index.statements) {
  if (!ts.isTypeAliasDeclaration(statement) || statement.name.text !== "LanguageCode") continue;
  if (!ts.isUnionTypeNode(statement.type)) throw new Error("LanguageCode must remain a literal union");
  languages = statement.type.types.map((t) => {
    if (!ts.isLiteralTypeNode(t) || !ts.isStringLiteral(t.literal)) {
      throw new Error("LanguageCode must contain string literals only");
    }
    return t.literal.text;
  });
}
if (languages.length === 0) throw new Error("Could not read LanguageCode");
const expected = [...languages].sort();

function assertExact(label, keys) {
  const actual = [...new Set(keys)].sort();
  const missing = expected.filter((x) => !actual.includes(x));
  const extra = actual.filter((x) => !expected.includes(x));
  if (missing.length || extra.length || actual.length !== expected.length) {
    throw new Error(`${label}: expected [${expected.join(", ")}], got [${actual.join(", ")}]; missing=[${missing}], extra=[${extra}]`);
  }
}

const languagesDecl = variable(index, "LANGUAGES");
if (!languagesDecl || !languagesDecl.initializer || !ts.isArrayLiteralExpression(languagesDecl.initializer)) {
  throw new Error("LANGUAGES must be an array literal");
}
assertExact(
  "LANGUAGES",
  languagesDecl.initializer.elements.flatMap((element) => {
    if (!ts.isObjectLiteralExpression(element)) return [];
    const code = element.properties.find(
      (p) => ts.isPropertyAssignment(p) && propertyName(p.name) === "code",
    );
    return code && ts.isPropertyAssignment(code) && ts.isStringLiteral(code.initializer)
      ? [code.initializer.text]
      : [];
  }),
);

const packsDecl = variable(index, "PACKS");
if (!packsDecl || !packsDecl.initializer || !ts.isObjectLiteralExpression(packsDecl.initializer)) {
  throw new Error("PACKS must be an object literal");
}
assertExact("PACKS", packsDecl.initializer.properties.map((p) => propertyName(p.name)).filter(Boolean));

const languagePackNames = new Set([...languages, "index"]);
const files = fs.readdirSync(i18nDir).filter((name) => /\.(ts|tsx)$/.test(name));
let checkedMaps = 0;
for (const name of files) {
  const stem = name.replace(/\.(ts|tsx)$/, "");
  if (languagePackNames.has(stem)) continue;
  const source = parse(path.join(i18nDir, name));
  for (const statement of source.statements) {
    if (!ts.isVariableStatement(statement)) continue;
    for (const decl of statement.declarationList.declarations) {
      if (!decl.type || !decl.initializer || !ts.isObjectLiteralExpression(decl.initializer)) continue;
      if (!ts.isTypeReferenceNode(decl.type) || decl.type.typeName.getText(source) !== "Record") continue;
      const first = decl.type.typeArguments?.[0];
      if (!first || first.getText(source) !== "LanguageCode") continue;
      const label = ts.isIdentifier(decl.name) ? `${name}:${decl.name.text}` : name;
      assertExact(label, decl.initializer.properties.map((p) => propertyName(p.name)).filter(Boolean));
      checkedMaps += 1;
    }
  }
}
if (checkedMaps === 0) throw new Error("No Record<LanguageCode, ...> maps were checked");
console.log(`i18n parity OK: ${expected.join(", ")}; checked ${checkedMaps} specialized maps.`);
