# Graph IDE Tokens

These tokens keep the TypeScript Canvas viewer and Flutter `shadcn_ui` pilot
aligned around semantic graph states instead of renderer-specific color choices.

## Astryx Bridge

The Flutter pilot uses Meta Astryx as a design-system reference through
`apps/viewer_flutter/lib/astryx_tokens.dart`. Astryx itself is a React and
StyleX system, so the Flutter app does not depend on the npm packages at
runtime. Instead, the bridge maps the public Astryx neutral theme tokens into
Dart constants, then feeds them into `ShadThemeData` and DSLRaid graph semantic
tokens.

Source references:

- `https://github.com/facebook/astryx`
- `https://astryx.atmeta.com/`
- `@astryxdesign/theme-neutral`

| Astryx token family | Flutter bridge | DSLRaid use |
| --- | --- | --- |
| `--color-background-*` | `AstryxNeutralTokens.background*` | shadcn app, card, muted, and scaffold surfaces. |
| `--color-text-*` | `AstryxNeutralTokens.text*` | foreground and muted foreground text. |
| `--color-border*` | `AstryxNeutralTokens.border*` | shadcn borders and graph panel borders. |
| `--color-success/error/warning*` | `AstryxNeutralTokens.success/error/warning*` | graph health, risk, and review tones. |
| categorical blue/teal muted surfaces | `blueMuted`, `tealMuted` | generated badges, focus, accent, and selection. |

## Tone Tokens

| Semantic state | Schema tone | TypeScript token | Flutter token resolver | Meaning |
| --- | --- | --- | --- | --- |
| Normal view | `default` | `graphToneTokens.default` | `DslraidGraphTokens.toneStroke(normal)` | Ordinary projected graph element. |
| Verified | `success` | `graphToneTokens.success` | `DslraidGraphTokens.toneStroke(success)` | Covered, generated, deployed, passing, or otherwise healthy. |
| Needs review | `warning` | `graphToneTokens.warning` | `DslraidGraphTokens.toneStroke(warning)` | Stale, incomplete, flaky, uncovered, or review-needed. |
| Risk | `danger` | `graphToneTokens.danger` | `DslraidGraphTokens.toneStroke(danger)` | Failed, blocked, unsafe, or release-risk state. |
| Background | `muted` | `graphToneTokens.muted` | `DslraidGraphTokens.toneStroke(muted)` | Filtered, disabled, out-of-scope, or secondary context. |

The Flutter pilot also renders these tone names in a read-only Token Legend
panel. The legend is sourced from `DslraidGraphTokens.toneLegend`, so the UI,
tests, and docs share the same agent-readable token names instead of duplicating
one-off labels.

## Badge Tokens

| Badge state | TypeScript token | Flutter token resolver | Meaning |
| --- | --- | --- | --- |
| `generated` | `graphBadgeTokens.generated` | `DslraidGraphTokens.badgeFill(generated, ...)` | Generated artifact or codegen output. |
| `covered`, `coverage`, `deployed`, `tested` | `graphToneTokens.success.badgeFill` | `DslraidGraphTokens.badgeFill(... success ...)` | Verified runtime/design coverage. |
| `uncovered`, `not_deployed` | `graphToneTokens.muted.badgeFill` | `DslraidGraphTokens.badgeFill(... muted ...)` | Known but not covered or deployed. |
| `failed` | `graphToneTokens.danger.badgeFill` | `DslraidGraphTokens.badgeFill(... danger ...)` | Failed or unsafe state. |
| `flaky` | `graphToneTokens.warning.badgeFill` | `DslraidGraphTokens.badgeFill(... warning ...)` | Unstable or needs review. |
| Other | `graphBadgeTokens.neutral` | `DslraidGraphTokens.badgeFill(... neutral ...)` | Informational metadata. |

## Status Signal Tokens

The Flutter pilot renders a compact project-panel status grid from derived
`ViewStatusSignal` values. The grid reuses the same tone stroke/fill resolvers
as graph nodes so contract health, coverage, codegen freshness, trace links,
and review state do not introduce a second palette.

| Status signal | Token source | Reader-facing state |
| --- | --- | --- |
| `Contract`, `Source`, `Trace` | `DslraidGraphTokens.toneStroke(success)` and `toneFill(success)` when linked and current. | Fixture contract, projection provenance, and trace link health. |
| `Review`, `Codegen`, `Coverage` | `toneStroke(warning/danger/success/muted)` based on derived ViewModel state. | Agent-readable review load, stale codegen tags, and coverage tags or gaps. |
| `Layout` | `toneStroke(normal)` unless the layout engine is `none`. | Renderer metadata without implying Core IR health. |

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
  diagnostic rows. It reads Astryx neutral values from
  `apps/viewer_flutter/lib/astryx_tokens.dart`.

New renderer work should add semantic tokens first, then map those tokens to
colors, stroke widths, labels, or shadcn component variants.

## Guardrails

- Do not add one-off hex colors inside graph renderers when a semantic token
  already exists.
- Do not import Astryx React packages into Flutter; port token values or
  component behavior through the bridge and keep the runtime Flutter-native.
- Add schema tones only when the state is renderer-independent.
- Keep hover, selection, camera, focus, and search query as app state, not
  ViewModel schema state.
- Use `danger` only for blocked, failed, unsafe, or release-risk information.
