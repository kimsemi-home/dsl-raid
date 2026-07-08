import 'package:flutter/widgets.dart';

enum DslraidTone { normal, success, warning, danger, muted }

class DslraidViewModel {
  const DslraidViewModel({
    required this.viewVersion,
    required this.source,
    required this.layout,
    required this.nodes,
    required this.edges,
    required this.inspectorPanels,
    required this.diagnostics,
  });

  final String viewVersion;
  final ViewSource source;
  final ViewLayout layout;
  final List<SceneNode> nodes;
  final List<SceneEdge> edges;
  final List<InspectorPanel> inspectorPanels;
  final List<DiagnosticItem> diagnostics;

  List<ViewStatusSignal> get statusSignals {
    final badges = nodes.expand((node) => node.badges).toSet();
    return [
      ViewStatusSignal(
        label: 'Contract',
        value: 'v$viewVersion schema',
        tone: DslraidTone.success,
      ),
      ViewStatusSignal(
        label: 'Source',
        value: source.hash ?? source.projection,
        tone: source.hash == null ? DslraidTone.warning : DslraidTone.success,
      ),
      ViewStatusSignal(
        label: 'Layout',
        value: layout.version.isEmpty
            ? layout.engine
            : '${layout.engine} / ${layout.version}',
        tone: layout.engine == 'none'
            ? DslraidTone.warning
            : DslraidTone.normal,
      ),
      _reviewSignal(),
      _coverageSignal(badges),
      _codegenSignal(badges),
      _traceSignal(),
    ];
  }

  ViewStatusSignal _reviewSignal() {
    final dangerCount = diagnostics
        .where((item) => item.tone == DslraidTone.danger)
        .length;
    final warningCount = diagnostics
        .where((item) => item.tone == DslraidTone.warning)
        .length;
    if (dangerCount > 0) {
      return ViewStatusSignal(
        label: 'Review',
        value: '$dangerCount blocked',
        tone: DslraidTone.danger,
      );
    }
    if (warningCount > 0) {
      return ViewStatusSignal(
        label: 'Review',
        value: '$warningCount warning',
        tone: DslraidTone.warning,
      );
    }
    return const ViewStatusSignal(
      label: 'Review',
      value: 'clear',
      tone: DslraidTone.success,
    );
  }

  ViewStatusSignal _coverageSignal(Set<String> badges) {
    if (badges.any(_coverageGapBadges.contains)) {
      return const ViewStatusSignal(
        label: 'Coverage',
        value: 'gap tagged',
        tone: DslraidTone.warning,
      );
    }
    final coverageCount = badges.where(_coverageOkBadges.contains).length;
    if (coverageCount > 0) {
      return ViewStatusSignal(
        label: 'Coverage',
        value: '$coverageCount tag',
        tone: DslraidTone.success,
      );
    }
    return const ViewStatusSignal(
      label: 'Coverage',
      value: 'not tagged',
      tone: DslraidTone.muted,
    );
  }

  ViewStatusSignal _codegenSignal(Set<String> badges) {
    final staleBadges = badges.where((badge) => badge.contains('stale'));
    if (staleBadges.isNotEmpty) {
      return ViewStatusSignal(
        label: 'Codegen',
        value: staleBadges.first,
        tone: DslraidTone.warning,
      );
    }
    final generated =
        badges.contains('generated') ||
        nodes.any((node) => node.label.toLowerCase().contains('generated'));
    if (generated) {
      return const ViewStatusSignal(
        label: 'Codegen',
        value: 'fresh',
        tone: DslraidTone.success,
      );
    }
    return const ViewStatusSignal(
      label: 'Codegen',
      value: 'not linked',
      tone: DslraidTone.muted,
    );
  }

  ViewStatusSignal _traceSignal() {
    final traceEdges = edges.where((edge) {
      final label = edge.label?.toLowerCase() ?? '';
      final subject = edge.subject.toLowerCase();
      return label.contains('trace') || subject.contains('trace');
    }).toList();
    if (traceEdges.isEmpty) {
      return const ViewStatusSignal(
        label: 'Trace',
        value: 'not linked',
        tone: DslraidTone.muted,
      );
    }
    final hasRisk = traceEdges.any(
      (edge) =>
          edge.tone == DslraidTone.warning || edge.tone == DslraidTone.danger,
    );
    return ViewStatusSignal(
      label: 'Trace',
      value: '${traceEdges.length} linked',
      tone: hasRisk ? DslraidTone.warning : DslraidTone.success,
    );
  }

  factory DslraidViewModel.fromJson(Map<String, Object?> json) {
    final nodes = _list(json['nodes']).map(SceneNode.fromJson).toList();
    final edges = _list(json['edges']).map(SceneEdge.fromJson).toList();
    return DslraidViewModel(
      viewVersion: _string(json['view_version']),
      source: ViewSource.fromJson(_map(json['source'])),
      layout: ViewLayout.fromJson(_map(json['layout'])),
      nodes: nodes,
      edges: edges,
      inspectorPanels: _list(
        json['inspector_panels'],
      ).map(InspectorPanel.fromJson).toList(),
      diagnostics: _diagnosticsFrom(nodes, edges),
    );
  }
}

class ViewStatusSignal {
  const ViewStatusSignal({
    required this.label,
    required this.value,
    required this.tone,
  });

  final String label;
  final String value;
  final DslraidTone tone;
}

class ViewSource {
  const ViewSource({required this.coreIr, required this.projection, this.hash});

  final String coreIr;
  final String projection;
  final String? hash;

  factory ViewSource.fromJson(Map<String, Object?> json) {
    return ViewSource(
      coreIr: _string(json['core_ir']),
      projection: _string(json['projection']),
      hash: json['hash'] as String?,
    );
  }
}

class ViewLayout {
  const ViewLayout({required this.engine, required this.version});

  final String engine;
  final String version;

  factory ViewLayout.fromJson(Map<String, Object?> json) {
    return ViewLayout(
      engine: _string(json['engine']),
      version: _string(json['version']),
    );
  }
}

class SceneNode {
  const SceneNode({
    required this.id,
    required this.subject,
    required this.bounds,
    required this.label,
    this.badges = const [],
    this.tone = DslraidTone.normal,
  });

  final String id;
  final String subject;
  final Rect bounds;
  final String label;
  final List<String> badges;
  final DslraidTone tone;

  factory SceneNode.fromJson(Map<String, Object?> json) {
    return SceneNode(
      id: _string(json['id']),
      subject: _string(json['subject']),
      bounds: Rect.fromLTWH(
        _number(json['x']),
        _number(json['y']),
        _number(json['width']),
        _number(json['height']),
      ),
      label: _string(json['label']),
      badges: _strings(json['badges']),
      tone: _tone(_mapOrEmpty(json['style'])['tone']),
    );
  }
}

class SceneEdge {
  const SceneEdge({
    required this.id,
    required this.subject,
    required this.from,
    required this.to,
    required this.route,
    this.label,
    this.tone = DslraidTone.normal,
  });

  final String id;
  final String subject;
  final String from;
  final String to;
  final List<Offset> route;
  final String? label;
  final DslraidTone tone;

  factory SceneEdge.fromJson(Map<String, Object?> json) {
    return SceneEdge(
      id: _string(json['id']),
      subject: _string(json['subject']),
      from: _string(json['from']),
      to: _string(json['to']),
      label: json['label'] as String?,
      route: _list(json['route']).map(_offset).toList(),
      tone: _tone(_mapOrEmpty(json['style'])['tone']),
    );
  }
}

class InspectorPanel {
  const InspectorPanel({
    required this.subject,
    required this.title,
    required this.sections,
  });

  final String subject;
  final String title;
  final List<InspectorSection> sections;

  factory InspectorPanel.fromJson(Map<String, Object?> json) {
    return InspectorPanel(
      subject: _string(json['subject']),
      title: _string(json['title']),
      sections: _list(json['sections']).map(InspectorSection.fromJson).toList(),
    );
  }
}

class InspectorSection {
  const InspectorSection({required this.title, required this.rows});

  final String title;
  final List<InspectorRow> rows;

  factory InspectorSection.fromJson(Map<String, Object?> json) {
    return InspectorSection(
      title: _string(json['title']),
      rows: _list(json['rows']).map(InspectorRow.fromJson).toList(),
    );
  }
}

class InspectorRow {
  const InspectorRow({required this.label, required this.value});

  final String label;
  final String value;

  factory InspectorRow.fromJson(Map<String, Object?> json) {
    return InspectorRow(
      label: _string(json['label']),
      value: _string(json['value']),
    );
  }
}

class DiagnosticItem {
  const DiagnosticItem({
    required this.code,
    required this.message,
    required this.subject,
    this.tone = DslraidTone.warning,
  });

  final String code;
  final String message;
  final String subject;
  final DslraidTone tone;
}

List<DiagnosticItem> _diagnosticsFrom(
  List<SceneNode> nodes,
  List<SceneEdge> edges,
) {
  return [
    for (final node in nodes)
      if (node.tone == DslraidTone.warning || node.tone == DslraidTone.danger)
        DiagnosticItem(
          code: 'DSLR-VIEW-NODE',
          subject: node.subject,
          message: '${node.label} needs review in the projected ViewModel.',
          tone: node.tone,
        ),
    for (final edge in edges)
      if (edge.tone == DslraidTone.warning || edge.tone == DslraidTone.danger)
        DiagnosticItem(
          code: 'DSLR-VIEW-EDGE',
          subject: edge.subject,
          message:
              '${edge.label ?? edge.id} needs review in the projected ViewModel.',
          tone: edge.tone,
        ),
  ];
}

const Set<String> _coverageOkBadges = {'coverage', 'covered', 'tested'};
const Set<String> _coverageGapBadges = {'uncovered', 'not_deployed'};

Map<String, Object?> _map(Object? value) {
  if (value is Map<String, Object?>) {
    return value;
  }
  throw FormatException('expected object, got $value');
}

Map<String, Object?> _mapOrEmpty(Object? value) {
  return value is Map<String, Object?> ? value : const {};
}

List<Map<String, Object?>> _list(Object? value) {
  if (value is List<Object?>) {
    return value.whereType<Map<String, Object?>>().toList();
  }
  return const [];
}

String _string(Object? value) {
  if (value is String && value.isNotEmpty) {
    return value;
  }
  throw FormatException('expected non-empty string, got $value');
}

double _number(Object? value) {
  if (value is num) {
    return value.toDouble();
  }
  throw FormatException('expected number, got $value');
}

List<String> _strings(Object? value) {
  return value is List<Object?> ? value.whereType<String>().toList() : const [];
}

Offset _offset(Map<String, Object?> value) {
  return Offset(_number(value['x']), _number(value['y']));
}

DslraidTone _tone(Object? value) {
  return switch (value) {
    'success' => DslraidTone.success,
    'warning' => DslraidTone.warning,
    'danger' => DslraidTone.danger,
    'muted' => DslraidTone.muted,
    _ => DslraidTone.normal,
  };
}
