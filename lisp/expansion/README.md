# Expansion

Expansion turns authoring forms into canonical, deterministic IR-shaped data.

This is the Common Lisp layer's main advantage. Surface forms may be pleasant
and macro-friendly, but expansion must produce ordinary data that can be
emitted as Canonical IR.

Expansion may:

- derive stable semantic IDs
- expand shorthand syntax
- preserve deterministic authoring order
- preserve source information

Expansion must not:

- write files
- run external commands
- perform backend code generation
- hide conformance checks as macro side effects
