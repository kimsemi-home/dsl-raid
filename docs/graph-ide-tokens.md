# Graph IDE Tokens

These tokens keep the TypeScript Canvas viewer and Flutter `shadcn_ui` pilot
aligned around semantic graph states instead of renderer-specific color choices.

## Tone Tokens

| Semantic state | Schema tone | TypeScript token | Flutter token resolver | Meaning |
| --- | --- | --- | --- | --- |
| Normal view | `default` | `graphToneTokens.default` | `DslraidGraphTokens.toneStroke(normal)` | Ordinary projected graph element. |
| Verified | `success` | `graphToneTokens.success` | `DslraidGraphTokens.toneStroke(success)` | Covered, generated, deployed, passing, or otherwise healthy. |
| Needs review | `warning` | `graphToneTokens.warning` | `DslraidGraphTokens.toneStroke(warning)` | Stale, incomplete, flaky, uncovered, or review-needed. |
| Risk | `danger` | `graphToneTokens.danger` | `DslraidGraphTokens.toneStroke(danger)` | Failed, blocked, unsafe, or release-risk state. |
| Background | `muted` | `graphToneTokens.muted` | `DslraidGraphTokens.toneStroke(muted)` | Filtered, disabled, out-of-scope, or secondary context. |

## Badge Tokens

| Badge state | TypeScript token | Flutter token resolver | Meaning |
| --- | --- | --- | --- |
| `generated` | `graphBadgeTokens.generated` | `DslraidGraphTokens.badgeFill(generated, ...)` | Generated artifact or codegen output. |
| `covered`, `coverage`, `deployed`, `tested` | `graphToneTokens.success.badgeFill` | `DslraidGraphTokens.badgeFill(... success ...)` | Verified runtime/design coverage. |
| `uncovered`, `not_deployed` | `graphToneTokens.muted.badgeFill` | `DslraidGraphTokens.badgeFill(... muted ...)` | Known but not covered or deployed. |
| `failed` | `graphToneTokens.danger.badgeFill` | `DslraidGraphTokens.badgeFill(... danger ...)` | Failed or unsafe state. |
| `flaky` | `graphToneTokens.warning.badgeFill` | `DslraidGraphTokens.badgeFill(... warning ...)` | Unstable or needs review. |
| Other | `graphBadgeTokens.neutral` | `DslraidGraphTokens.badgeFill(... neutral ...)` | Informational metadata. |

## Density Tokens

| Token | Value | Use |
| --- | --- | --- |
| `panelPadding` | `16` | Full side panels and inspector body padding. |
| `compactPanelPadding` | `12` | Diagnostics and compact utility panels. |
| `nodeRadius` | `8px` | Graph node rectangle radius. |
| `badgeRadius` | `8px` | Graph badge radius. |
| `gridStep` | `32px` | Canvas and CustomPainter grid spacing. |

## Renderer Mapping

- TypeScript Canvas: `apps/viewer/src/canvas/style.ts` exposes
  `graphToneTokens` and `graphBadgeTokens` through `toneStroke` and
  `badgeFill`.
- Flutter: `apps/viewer_flutter/lib/graph_tokens.dart` exposes
  `DslraidGraphTokens` for `DslraidTheme`, `GraphPainter`, panels, badges, and
  diagnostic rows.

New renderer work should add semantic tokens first, then map those tokens to
colors, stroke widths, labels, or shadcn component variants.

## Guardrails

- Do not add one-off hex colors inside graph renderers when a semantic token
  already exists.
- Add schema tones only when the state is renderer-independent.
- Keep hover, selection, camera, focus, and search query as app state, not
  ViewModel schema state.
- Use `danger` only for blocked, failed, unsafe, or release-risk information.
