import 'dart:io';

void main() {
  final lines = File("input.txt")
      .readAsLinesSync()
      .map((l) => int.parse(l.split(":")[1].replaceAll(' ', '')));
  final td = (lines.first, lines.last);
  final wins = List.generate(td.$1, (t) => (td.$1 - t) * t)
      .where((d) => d > td.$2)
      .length;
  print(wins);
}
