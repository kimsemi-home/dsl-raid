import 'package:flutter/material.dart';
import 'package:shadcn_ui/shadcn_ui.dart';

import 'graph_view.dart';
import 'graph_tokens.dart';
import 'view_model.dart';

class DslraidShell extends StatelessWidget {
  const DslraidShell({super.key, required this.viewModel});

  final DslraidViewModel viewModel;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SafeArea(
        child: LayoutBuilder(
          builder: (context, constraints) {
            if (constraints.maxWidth < 840) {
              return _NarrowShell(viewModel: viewModel);
            }
            return _WideShell(viewModel: viewModel);
          },
        ),
      ),
    );
  }
}

class _WideShell extends StatelessWidget {
  const _WideShell({required this.viewModel});

  final DslraidViewModel viewModel;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        SizedBox(
          width: 280,
          child: SingleChildScrollView(
            child: _ProjectPanel(viewModel: viewModel),
          ),
        ),
        Expanded(child: _Workspace(viewModel: viewModel)),
        SizedBox(width: 340, child: _InspectorPanel(viewModel: viewModel)),
      ],
    );
  }
}

class _NarrowShell extends StatelessWidget {
  const _NarrowShell({required this.viewModel});

  final DslraidViewModel viewModel;

  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: const EdgeInsets.all(12),
      children: [
        _ProjectPanel(viewModel: viewModel),
        const SizedBox(height: 12),
        SizedBox(height: 420, child: _Workspace(viewModel: viewModel)),
        const SizedBox(height: 12),
        _InspectorPanel(viewModel: viewModel),
      ],
    );
  }
}

class _ProjectPanel extends StatelessWidget {
  const _ProjectPanel({required this.viewModel});

  final DslraidViewModel viewModel;

  @override
  Widget build(BuildContext context) {
    return _Panel(
      title: 'DSLRaid',
      icon: LucideIcons.network,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            viewModel.source.coreIr,
            style: ShadTheme.of(context).textTheme.small,
          ),
          const SizedBox(height: 12),
          _BadgeRow(
            values: [viewModel.source.projection, viewModel.layout.engine],
          ),
          const SizedBox(height: 14),
          _StatusSignalGrid(signals: viewModel.statusSignals),
          const SizedBox(height: 16),
          for (final node in viewModel.nodes)
            _SubjectTile(
              label: node.label,
              subject: node.subject,
              tone: node.tone,
            ),
        ],
      ),
    );
  }
}

class _Workspace extends StatelessWidget {
  const _Workspace({required this.viewModel});

  final DslraidViewModel viewModel;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        _Toolbar(viewModel: viewModel),
        Expanded(
          child: Padding(
            padding: const EdgeInsets.all(12),
            child: GraphViewport(viewModel: viewModel),
          ),
        ),
        _DiagnosticsPanel(viewModel: viewModel),
      ],
    );
  }
}

class _Toolbar extends StatelessWidget {
  const _Toolbar({required this.viewModel});

  final DslraidViewModel viewModel;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: BoxDecoration(
        color: ShadTheme.of(context).colorScheme.card,
        border: Border(
          bottom: BorderSide(color: ShadTheme.of(context).colorScheme.border),
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
        child: Row(
          children: [
            const Icon(LucideIcons.gitBranch, size: 18),
            const SizedBox(width: 8),
            Expanded(child: Text('Projection ${viewModel.source.projection}')),
            ShadButton.outline(
              onPressed: () {},
              leading: const Icon(LucideIcons.search, size: 16),
              child: const Text('Search'),
            ),
          ],
        ),
      ),
    );
  }
}

class _InspectorPanel extends StatelessWidget {
  const _InspectorPanel({required this.viewModel});

  final DslraidViewModel viewModel;

  @override
  Widget build(BuildContext context) {
    final panel = viewModel.inspectorPanels.first;
    return _Panel(
      title: 'Inspector',
      icon: LucideIcons.panelRight,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(panel.title, style: ShadTheme.of(context).textTheme.h4),
          const SizedBox(height: 10),
          for (final section in panel.sections) ...[
            Text(section.title, style: ShadTheme.of(context).textTheme.muted),
            const SizedBox(height: 6),
            for (final row in section.rows)
              _InfoRow(label: row.label, value: row.value),
            const SizedBox(height: 14),
          ],
        ],
      ),
    );
  }
}

class _DiagnosticsPanel extends StatelessWidget {
  const _DiagnosticsPanel({required this.viewModel});

  final DslraidViewModel viewModel;

  @override
  Widget build(BuildContext context) {
    return _Panel(
      title: 'Diagnostics',
      icon: LucideIcons.triangleAlert,
      compact: true,
      child: Column(
        children: [
          for (final item in viewModel.diagnostics)
            _InfoRow(label: item.code, value: item.message, tone: item.tone),
        ],
      ),
    );
  }
}

class _Panel extends StatelessWidget {
  const _Panel({
    required this.title,
    required this.icon,
    required this.child,
    this.compact = false,
  });

  final String title;
  final IconData icon;
  final Widget child;
  final bool compact;

  @override
  Widget build(BuildContext context) {
    final theme = ShadTheme.of(context);
    return DecoratedBox(
      decoration: BoxDecoration(
        color: theme.colorScheme.card,
        border: Border.all(color: theme.colorScheme.border),
      ),
      child: Padding(
        padding: EdgeInsets.all(
          compact
              ? DslraidGraphTokens.compactPanelPadding
              : DslraidGraphTokens.panelPadding,
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(icon, size: 18),
                const SizedBox(width: 8),
                Text(title, style: theme.textTheme.h4),
              ],
            ),
            const SizedBox(height: 12),
            child,
          ],
        ),
      ),
    );
  }
}

class _SubjectTile extends StatelessWidget {
  const _SubjectTile({
    required this.label,
    required this.subject,
    required this.tone,
  });

  final String label;
  final String subject;
  final DslraidTone tone;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 8),
      child: ShadCard(
        padding: const EdgeInsets.all(10),
        title: Text(label),
        description: Text(subject),
        trailing: _ToneBadge(tone: tone),
      ),
    );
  }
}

class _BadgeRow extends StatelessWidget {
  const _BadgeRow({required this.values});

  final List<String> values;

  @override
  Widget build(BuildContext context) {
    return Wrap(
      spacing: 6,
      runSpacing: 6,
      children: [
        for (final value in values) ShadBadge.outline(child: Text(value)),
      ],
    );
  }
}

class _StatusSignalGrid extends StatelessWidget {
  const _StatusSignalGrid({required this.signals});

  final List<ViewStatusSignal> signals;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        final isTwoColumn = constraints.maxWidth >= 232;
        final itemWidth = isTwoColumn
            ? (constraints.maxWidth - 8) / 2
            : constraints.maxWidth;
        return Wrap(
          spacing: 8,
          runSpacing: 8,
          children: [
            for (final signal in signals)
              SizedBox(
                width: itemWidth,
                child: _StatusSignalTile(signal: signal),
              ),
          ],
        );
      },
    );
  }
}

class _StatusSignalTile extends StatelessWidget {
  const _StatusSignalTile({required this.signal});

  final ViewStatusSignal signal;

  @override
  Widget build(BuildContext context) {
    final theme = ShadTheme.of(context);
    return DecoratedBox(
      decoration: BoxDecoration(
        color: DslraidGraphTokens.toneFill(signal.tone, theme),
        border: Border.all(
          color: DslraidGraphTokens.toneStroke(signal.tone, theme),
        ),
        borderRadius: const BorderRadius.all(DslraidGraphTokens.badgeRadius),
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 9, vertical: 8),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(signal.label, style: theme.textTheme.small),
            const SizedBox(height: 4),
            Tooltip(
              message: signal.value,
              child: Text(
                signal.value,
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
                style: theme.textTheme.p,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _InfoRow extends StatelessWidget {
  const _InfoRow({
    required this.label,
    required this.value,
    this.tone = DslraidTone.normal,
  });

  final String label;
  final String value;
  final DslraidTone tone;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 5),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: 118,
            child: Text(label, style: ShadTheme.of(context).textTheme.small),
          ),
          Expanded(child: Text(value)),
          const SizedBox(width: 8),
          _ToneBadge(tone: tone),
        ],
      ),
    );
  }
}

class _ToneBadge extends StatelessWidget {
  const _ToneBadge({required this.tone});

  final DslraidTone tone;

  @override
  Widget build(BuildContext context) {
    final label = DslraidGraphTokens.toneLabel(tone);
    return switch (tone) {
      DslraidTone.success => ShadBadge(child: Text(label)),
      DslraidTone.warning => ShadBadge.outline(child: Text(label)),
      DslraidTone.danger => ShadBadge.destructive(child: Text(label)),
      DslraidTone.muted => ShadBadge.secondary(child: Text(label)),
      DslraidTone.normal => ShadBadge.secondary(child: Text(label)),
    };
  }
}
