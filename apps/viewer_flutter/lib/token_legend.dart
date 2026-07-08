import 'package:flutter/material.dart';
import 'package:shadcn_ui/shadcn_ui.dart';

import 'graph_tokens.dart';
import 'view_model.dart';

class DslraidTokenLegend extends StatelessWidget {
  const DslraidTokenLegend({super.key});

  @override
  Widget build(BuildContext context) {
    final theme = ShadTheme.of(context);
    return DecoratedBox(
      key: const Key('token-legend'),
      decoration: BoxDecoration(
        color: theme.colorScheme.card,
        border: Border.all(color: theme.colorScheme.border),
      ),
      child: Padding(
        padding: const EdgeInsets.all(DslraidGraphTokens.compactPanelPadding),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                const Icon(LucideIcons.swatchBook, size: 18),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    'Token Legend',
                    overflow: TextOverflow.ellipsis,
                    style: theme.textTheme.h4,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 10),
            for (final entry in DslraidGraphTokens.toneLegend)
              _TokenLegendRow(entry: entry),
          ],
        ),
      ),
    );
  }
}

class _TokenLegendRow extends StatelessWidget {
  const _TokenLegendRow({required this.entry});

  final DslraidTokenLegendEntry entry;

  @override
  Widget build(BuildContext context) {
    final theme = ShadTheme.of(context);
    return Padding(
      padding: const EdgeInsets.only(bottom: 10),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _TokenSwatch(tone: entry.tone),
          const SizedBox(width: 10),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(entry.label, style: theme.textTheme.small),
                const SizedBox(height: 4),
                Wrap(
                  spacing: 6,
                  runSpacing: 4,
                  crossAxisAlignment: WrapCrossAlignment.center,
                  children: [
                    ShadBadge.outline(child: Text(entry.token)),
                    Text(entry.description, style: theme.textTheme.muted),
                  ],
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _TokenSwatch extends StatelessWidget {
  const _TokenSwatch({required this.tone});

  final DslraidTone tone;

  @override
  Widget build(BuildContext context) {
    final theme = ShadTheme.of(context);
    return DecoratedBox(
      decoration: BoxDecoration(
        color: DslraidGraphTokens.toneFill(tone, theme),
        border: Border.all(color: DslraidGraphTokens.toneStroke(tone, theme)),
        borderRadius: const BorderRadius.all(DslraidGraphTokens.badgeRadius),
      ),
      child: const SizedBox(width: 18, height: 18),
    );
  }
}
