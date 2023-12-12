import 'dart:io';

void main() {
  final locations = File('input.txt')
      .readAsLinesSync()
      .indexed
      .expand((row) => row.$2
          .split("")
          .indexed
          .where((col) => col.$2 == '#')
          .map((col) => (col.$1, row.$1)))
      .toSet();
  print(locations);
}
