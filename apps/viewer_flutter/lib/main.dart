import 'package:flutter/material.dart';
import 'package:shadcn_ui/shadcn_ui.dart';

import 'graph_tokens.dart';
import 'shell.dart';
import 'view_model.dart';
import 'view_model_loader.dart';

void main() {
  runApp(const DslraidViewerApp());
}

class DslraidViewerApp extends StatelessWidget {
  const DslraidViewerApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ShadApp.custom(
      theme: DslraidTheme.light,
      appBuilder: (context) {
        return MaterialApp(
          debugShowCheckedModeBanner: false,
          title: 'DSLRaid Viewer',
          theme: DslraidTheme.material(context),
          home: const _ViewerBootstrap(),
          builder: (_, child) => ShadAppBuilder(child: child),
          scrollBehavior: const ShadScrollBehavior(),
        );
      },
    );
  }
}

class _ViewerBootstrap extends StatelessWidget {
  const _ViewerBootstrap();

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<DslraidViewModel>(
      future: loadSampleViewModel(),
      builder: (context, snapshot) {
        if (snapshot.hasData) {
          return DslraidShell(viewModel: snapshot.requireData);
        }
        if (snapshot.hasError) {
          return Scaffold(body: Center(child: Text('${snapshot.error}')));
        }
        return const Scaffold(body: Center(child: CircularProgressIndicator()));
      },
    );
  }
}

class DslraidTheme {
  static ShadThemeData get light {
    return ShadThemeData(
      colorScheme: const ShadNeutralColorScheme.light(
        background: DslraidGraphTokens.background,
        foreground: DslraidGraphTokens.foreground,
        card: DslraidGraphTokens.panel,
        cardForeground: DslraidGraphTokens.foreground,
        primary: DslraidGraphTokens.primary,
        primaryForeground: Color(0xFFFFFFFF),
        secondary: Color(0xFFE7F4F0),
        secondaryForeground: DslraidGraphTokens.foreground,
        muted: Color(0xFFF0EEE7),
        mutedForeground: DslraidGraphTokens.mutedForeground,
        accent: Color(0xFFDDF7F1),
        accentForeground: DslraidGraphTokens.foreground,
        destructive: DslraidGraphTokens.danger,
        border: DslraidGraphTokens.border,
        input: DslraidGraphTokens.border,
        ring: DslraidGraphTokens.primary,
        selection: Color(0xFFCFF7ED),
      ),
      radius: const BorderRadius.all(Radius.circular(8)),
    );
  }

  static ThemeData material(BuildContext context) {
    final shad = ShadTheme.of(context);
    return ThemeData(
      colorScheme: ColorScheme.fromSeed(seedColor: DslraidGraphTokens.primary),
      scaffoldBackgroundColor: shad.colorScheme.background,
      dividerColor: shad.colorScheme.border,
      useMaterial3: true,
    );
  }
}
