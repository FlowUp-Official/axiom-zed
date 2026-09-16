; Highlights for the Axiom (`.axm`) language, driven by the dedicated
; `tree-sitter-axm` grammar (see extension.toml).

(line_comment) @comment

("import" @keyword)
("from" @keyword)
("as" @keyword)
("type" @keyword)
("model" @keyword)
("extends" @keyword)
("select" @keyword)
("query" @keyword)
("true" @keyword)
("false" @keyword)

(string) @string
(number) @number

(imported_name
  name: (identifier) @type)

(imported_name
  alias: (identifier) @type)

(type_decl
  name: (identifier) @type)

; Model/query field names.
(field_decl
  name: (identifier) @property)

; Query parameters are `$`-prefixed identifiers.
(param_decl
  name: (identifier) @variable.parameter)

(query_decl
  name: (identifier) @function)

; Rule/transform calls: `.trim()`, `.email()`, `.min(1)`, ...
(call
  method: (identifier) @function.call)

; Type references. The grammar lexes primitives and named (schema) types with
; the same `identifier` token, so canonical primitives are re-scoped to
; `type.builtin` here.
(type_ref
  (identifier) @type)

((type_ref
   (identifier) @type.builtin)
  (#any-of? @type.builtin
    "String" "Int" "BigInt" "Float" "Boolean"
    "UUID" "Date" "DateTime" "Json" "Bytes"))
