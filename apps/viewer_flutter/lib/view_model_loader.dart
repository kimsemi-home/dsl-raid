import 'dart:convert';

import 'package:flutter/services.dart';

import 'view_model.dart';

const sampleViewModelAsset = 'assets/view_model_sample.json';

Future<DslraidViewModel> loadSampleViewModel() async {
  final raw = await rootBundle.loadString(sampleViewModelAsset);
  final decoded = jsonDecode(raw);
  if (decoded is! Map<String, Object?>) {
    throw const FormatException('ViewModel asset must be a JSON object');
  }
  return DslraidViewModel.fromJson(decoded);
}
