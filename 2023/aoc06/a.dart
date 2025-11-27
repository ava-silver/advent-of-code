import 'dart:io';
import 'package:quiver/iterables.dart';

void main() {
  final lines = File("input.txt").readAsLinesSync().map((l) => l
      .split(":")[1]
      .split(' ')
      .where((s) => s.isNotEmpty)
      .map(int.parse)
      .toList());
  final timeDistances = zip(lines).map((e) => (e[0], e[1])).toList();
  final wins = timeDistances
      .map((td) => List.generate(td.$1, (t) => (td.$1 - t) * t)
          .where((d) => d > td.$2)
          .length)
      .fold(1, (a, b) => a * b);
  print(wins);
}
