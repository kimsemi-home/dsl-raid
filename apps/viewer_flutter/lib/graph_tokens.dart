import 'package:flutter/material.dart';
import 'package:shadcn_ui/shadcn_ui.dart';

import 'astryx_tokens.dart';
import 'view_model.dart';

class DslraidGraphTokens {
  static const primary = AstryxNeutralTokens.accent;
  static const foreground = AstryxNeutralTokens.textPrimary;
  static const background = AstryxNeutralTokens.backgroundBody;
  static const panel = AstryxNeutralTokens.backgroundCard;
  static const border = AstryxNeutralTokens.border;
  static const success = AstryxNeutralTokens.success;
  static const warning = AstryxNeutralTokens.warning;
  static const danger = AstryxNeutralTokens.error;
  static const mutedForeground = AstryxNeutralTokens.textSecondary;
  static const successSurface = AstryxNeutralTokens.successMuted;
  static const generatedSurface = AstryxNeutralTokens.blueMuted;
  static const warningSurface = AstryxNeutralTokens.warningMuted;
  static const dangerSurface = AstryxNeutralTokens.errorMuted;
  static const mutedSurface = AstryxNeutralTokens.backgroundMuted;
  static const neutralSurface = AstryxNeutralTokens.neutralOverlay;

  static const panelPadding = 16.0;
  static const compactPanelPadding = 12.0;
  static const nodeRadius = Radius.circular(8);
  static const badgeRadius = Radius.circular(8);
  static const gridStep = 32.0;

  static Color toneStroke(DslraidTone tone, ShadThemeData theme) {
    return switch (tone) {
      DslraidTone.success => success,
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
