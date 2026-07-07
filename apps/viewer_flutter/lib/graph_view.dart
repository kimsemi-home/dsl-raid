import 'package:flutter/material.dart';
import 'package:shadcn_ui/shadcn_ui.dart';

import 'graph_tokens.dart';
import 'view_model.dart';

class GraphViewport extends StatelessWidget {
  const GraphViewport({super.key, required this.viewModel});

  final DslraidViewModel viewModel;

  @override
  Widget build(BuildContext context) {
    final theme = ShadTheme.of(context);
    return DecoratedBox(
      decoration: BoxDecoration(
        color: theme.colorScheme.background,
        border: Border.all(color: theme.colorScheme.border),
        borderRadius: theme.radius,
      ),
      child: ClipRRect(
        borderRadius: theme.radius,
        child: CustomPaint(
          key: const Key('graph-viewport'),
          painter: GraphPainter(viewModel, theme),
          child: const SizedBox.expand(),
        ),
      ),
    );
  }
}

class GraphPainter extends CustomPainter {
  GraphPainter(this.viewModel, this.theme);

  final DslraidViewModel viewModel;
  final ShadThemeData theme;

  @override
  void paint(Canvas canvas, Size size) {
    final scale = _scaleFor(size);
    canvas.save();
    canvas.translate(28, 28);
    canvas.scale(scale);
    _drawGrid(canvas, size / scale);
    for (final edge in viewModel.edges) {
      _drawEdge(canvas, edge);
    }
    for (final node in viewModel.nodes) {
      _drawNode(canvas, node);
    }
    canvas.restore();
  }

  double _scaleFor(Size size) {
    final x = (size.width - 56) / 620;
    final y = (size.height - 56) / 340;
    return x < y ? x.clamp(.55, 1.5) : y.clamp(.55, 1.5);
  }

  void _drawGrid(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = theme.colorScheme.border.withValues(alpha: .45)
      ..strokeWidth = 1;
    for (double x = 0; x < size.width; x += DslraidGraphTokens.gridStep) {
      canvas.drawLine(Offset(x, 0), Offset(x, size.height), paint);
    }
    for (double y = 0; y < size.height; y += DslraidGraphTokens.gridStep) {
      canvas.drawLine(Offset(0, y), Offset(size.width, y), paint);
    }
  }

  void _drawEdge(Canvas canvas, SceneEdge edge) {
    final paint = Paint()
      ..color = DslraidGraphTokens.toneStroke(edge.tone, theme)
      ..strokeWidth = 2.4
      ..style = PaintingStyle.stroke;
    final path = Path()..moveTo(edge.route.first.dx, edge.route.first.dy);
    for (final point in edge.route.skip(1)) {
      path.lineTo(point.dx, point.dy);
    }
    canvas.drawPath(path, paint);
    if (edge.label != null) {
      final center = Offset(
        (edge.route.first.dx + edge.route.last.dx) / 2,
        (edge.route.first.dy + edge.route.last.dy) / 2,
      );
      _drawText(canvas, edge.label!, center, fontSize: 12);
    }
  }

  void _drawNode(Canvas canvas, SceneNode node) {
    final rect = RRect.fromRectAndRadius(
      node.bounds,
      DslraidGraphTokens.nodeRadius,
    );
    final fill = Paint()
      ..color = DslraidGraphTokens.toneFill(node.tone, theme)
      ..style = PaintingStyle.fill;
    final stroke = Paint()
      ..color = DslraidGraphTokens.toneStroke(node.tone, theme)
      ..strokeWidth = DslraidGraphTokens.toneStrokeWidth(node.tone)
      ..style = PaintingStyle.stroke;
    canvas.drawRRect(rect, fill);
    canvas.drawRRect(rect, stroke);
    _drawText(canvas, node.label, node.bounds.topLeft + const Offset(14, 18));
    for (var index = 0; index < node.badges.length; index += 1) {
      _drawBadge(
        canvas,
        node.badges[index],
        node.tone,
        node.bounds.topLeft + Offset(14, 42 + index * 18),
      );
    }
  }

  void _drawBadge(Canvas canvas, String text, DslraidTone tone, Offset offset) {
    final textPainter = _textPainter(text, fontSize: 10);
    final rect = RRect.fromRectAndRadius(
      Rect.fromLTWH(offset.dx, offset.dy, textPainter.width + 14, 16),
      DslraidGraphTokens.badgeRadius,
    );
    canvas.drawRRect(
      rect,
      Paint()
        ..color = DslraidGraphTokens.badgeFill(
          badge: text,
          tone: tone,
          theme: theme,
        ),
    );
    canvas.drawRRect(
      rect,
      Paint()
        ..color = theme.colorScheme.border
        ..style = PaintingStyle.stroke,
    );
    textPainter.paint(canvas, offset + const Offset(7, 2));
  }

  void _drawText(
    Canvas canvas,
    String text,
    Offset offset, {
    double fontSize = 13,
  }) {
    _textPainter(text, fontSize: fontSize).paint(canvas, offset);
  }

  TextPainter _textPainter(String text, {double fontSize = 13}) {
    return TextPainter(
      text: TextSpan(
        text: text,
        style: TextStyle(
          color: theme.colorScheme.foreground,
          fontSize: fontSize,
          fontWeight: FontWeight.w600,
        ),
      ),
      textDirection: TextDirection.ltr,
    )..layout(maxWidth: 180);
  }

  @override
  bool shouldRepaint(GraphPainter oldDelegate) {
    return oldDelegate.viewModel != viewModel || oldDelegate.theme != theme;
  }
}
