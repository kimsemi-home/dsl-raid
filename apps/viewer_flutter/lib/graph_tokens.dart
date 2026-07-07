import 'package:flutter/material.dart';
import 'package:shadcn_ui/shadcn_ui.dart';

import 'view_model.dart';

class DslraidGraphTokens {
  static const primary = Color(0xFF0F766E);
  static const foreground = Color(0xFF171717);
  static const background = Color(0xFFFAFAF6);
  static const panel = Color(0xFFFFFEFA);
  static const border = Color(0xFFDFD8C8);
  static const warning = Color(0xFFB45309);
  static const danger = Color(0xFFB42318);
  static const mutedForeground = Color(0xFF68635A);
  static const successSurface = Color(0xFFCCFBF1);
  static const generatedSurface = Color(0xFFDBEAFE);
  static const warningSurface = Color(0xFFFEF3C7);
  static const dangerSurface = Color(0xFFFEE2E2);
  static const mutedSurface = Color(0xFFE5E7EB);
  static const neutralSurface = Color(0xFFEBE6DA);

  static const panelPadding = 16.0;
  static const compactPanelPadding = 12.0;
  static const nodeRadius = Radius.circular(8);
  static const badgeRadius = Radius.circular(8);
  static const gridStep = 32.0;

  static Color toneStroke(DslraidTone tone, ShadThemeData theme) {
    return switch (tone) {
      DslraidTone.success => primary,
      DslraidTone.warning => warning,
      DslraidTone.danger => danger,
      DslraidTone.muted => theme.colorScheme.mutedForeground,
      DslraidTone.normal => theme.colorScheme.foreground,
    };
  }

  static Color toneFill(DslraidTone tone, ShadThemeData theme) {
    return toneStroke(tone, theme).withValues(alpha: .12);
  }

  static double toneStrokeWidth(DslraidTone tone) {
    return switch (tone) {
      DslraidTone.warning || DslraidTone.danger => 2.6,
      _ => 1.8,
    };
  }

  static String toneLabel(DslraidTone tone) {
    return switch (tone) {
      DslraidTone.success => 'ok',
      DslraidTone.warning => 'watch',
      DslraidTone.danger => 'risk',
      DslraidTone.muted => 'muted',
      DslraidTone.normal => 'view',
    };
  }

  static Color badgeFill({
    required String badge,
    required DslraidTone tone,
    required ShadThemeData theme,
  }) {
    if (badge == 'generated') {
      return generatedSurface;
    }
    if (badge == 'covered' ||
        badge == 'coverage' ||
        badge == 'deployed' ||
        badge == 'tested' ||
        tone == DslraidTone.success) {
      return successSurface;
    }
    if (badge == 'uncovered' || badge == 'not_deployed') {
      return mutedSurface;
    }
    if (badge == 'failed' || tone == DslraidTone.danger) {
      return dangerSurface;
    }
    if (badge == 'flaky' || tone == DslraidTone.warning) {
      return warningSurface;
    }
    return neutralSurface;
  }
}
