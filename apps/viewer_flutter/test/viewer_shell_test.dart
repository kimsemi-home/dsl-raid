import 'package:dslraid_viewer/graph_view.dart';
import 'package:dslraid_viewer/main.dart';
import 'package:dslraid_viewer/shell.dart';
import 'package:dslraid_viewer/view_model_loader.dart';
import 'package:flutter/widgets.dart';
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
    expect(find.byType(GraphViewport), findsOneWidget);
    expect(find.byKey(const Key('graph-viewport')), findsOneWidget);
    expect(find.text('DSLRaid'), findsOneWidget);
    expect(find.text('Inspector'), findsOneWidget);
    expect(find.text('Diagnostics'), findsOneWidget);
    expect(find.text(viewModel.source.coreIr), findsOneWidget);
  });
}
