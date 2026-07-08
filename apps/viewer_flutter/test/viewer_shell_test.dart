import 'package:dslraid_viewer/astryx_tokens.dart';
import 'package:dslraid_viewer/graph_tokens.dart';
import 'package:dslraid_viewer/graph_view.dart';
import 'package:dslraid_viewer/main.dart';
import 'package:dslraid_viewer/shell.dart';
import 'package:dslraid_viewer/token_legend.dart';
import 'package:dslraid_viewer/view_model.dart';
import 'package:dslraid_viewer/view_model_loader.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shadcn_ui/shadcn_ui.dart';

void main() {
  testWidgets('mounts the shadcn Graph IDE shell', (tester) async {
    tester.view.physicalSize = const Size(1200, 820);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.reset);

    await tester.pumpWidget(const DslraidViewerApp());
    await tester.pumpAndSettle();
    final viewModel = await loadSampleViewModel();

    expect(find.byType(ShadApp), findsOneWidget);
    expect(find.byType(ShadAppBuilder), findsOneWidget);
    expect(find.byType(DslraidShell), findsOneWidget);
    expect(find.byType(DslraidTokenLegend), findsOneWidget);
    expect(find.byKey(const Key('token-legend')), findsOneWidget);
    expect(find.byType(GraphViewport), findsOneWidget);
    expect(find.byKey(const Key('graph-viewport')), findsOneWidget);
    expect(find.text('DSLRaid'), findsOneWidget);
    expect(find.text('Inspector'), findsOneWidget);
    expect(find.text('Diagnostics'), findsOneWidget);
    expect(find.text(viewModel.source.coreIr), findsOneWidget);
    expect(find.text('Contract'), findsAtLeastNWidgets(1));
    expect(find.text('Review'), findsAtLeastNWidgets(1));
    expect(find.text('1 warning'), findsAtLeastNWidgets(1));
    expect(find.text('Codegen'), findsAtLeastNWidgets(1));
    expect(find.text('stale-check'), findsAtLeastNWidgets(1));
    expect(find.text('Trace'), findsAtLeastNWidgets(1));
    expect(find.text('1 linked'), findsAtLeastNWidgets(1));
    expect(find.text('Token Legend'), findsOneWidget);
    expect(find.text('Verified'), findsOneWidget);
    expect(find.text('Needs review'), findsOneWidget);
    expect(find.text('Risk'), findsOneWidget);
    expect(find.text('Muted'), findsOneWidget);
    expect(find.text('Normal'), findsOneWidget);
    expect(find.text('graphTone.success'), findsOneWidget);
    expect(find.text('graphTone.warning'), findsOneWidget);
    expect(
      viewModel.statusSignals.map((signal) => signal.label),
      containsAll(['Contract', 'Review', 'Coverage', 'Codegen', 'Trace']),
    );
    expect(
      DslraidGraphTokens.toneLegend.map((entry) => entry.token),
      containsAll(['graphTone.success', 'graphTone.warning']),
    );
    expect(viewModel.diagnostics.single.tone, DslraidTone.warning);

    final context = tester.element(find.byType(DslraidShell));
    final theme = ShadTheme.of(context);
    expect(theme.colorScheme.background, AstryxNeutralTokens.backgroundBody);
    expect(theme.colorScheme.foreground, AstryxNeutralTokens.textPrimary);
  });

  testWidgets('keeps the token legend stable on narrow screens', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(390, 900);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.reset);

    await tester.pumpWidget(
      ShadTheme(
        data: DslraidTheme.light,
        child: const Directionality(
          textDirection: TextDirection.ltr,
          child: DefaultTextStyle(
            style: TextStyle(fontSize: 14),
            child: SizedBox(width: 280, child: DslraidTokenLegend()),
          ),
        ),
      ),
    );
    await tester.pump();

    expect(find.byType(DslraidTokenLegend), findsOneWidget);
    expect(find.byKey(const Key('token-legend')), findsOneWidget);
    expect(find.text('Token Legend'), findsOneWidget);
    expect(find.text('graphTone.danger'), findsOneWidget);
  });
}
