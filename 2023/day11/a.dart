import 'dart:io';
import 'dart:math';
import 'package:trotter/trotter.dart';

List<int> emptyKeys(Map<int, Iterable> x) {
  final largest = x.keys.reduce(max);
  return List.generate(largest, (i) => i).where((i) => x[i] == null).toList();
}

(List<int>, List<int>) emptyColsAndRows(Set<(int, int)> locations) {
  final byCol = <int, Set<int>>{};
  final byRow = <int, Set<int>>{};
  for (final (col, row) in locations) {
    (byCol[col] ??= {}).add(row);
    (byRow[row] ??= {}).add(col);
  }
  return (emptyKeys(byCol), emptyKeys(byRow));
}

void main() {
  var locations = File('input.txt')
      .readAsLinesSync()
      .indexed
      .expand((row) => row.$2
          .split("")
          .indexed
          .where((col) => col.$2 == '#')
          .map((col) => (col.$1, row.$1)))
      .toSet();
  final (cols, rows) = emptyColsAndRows(locations);
  locations = expand(locations, cols, rows);

  final distances = Combinations(2, locations.toList())().map((points) {
    final (x1, y1) = points.first;
    final (x2, y2) = points.last;
    return (x2 - x1).abs() + (y2 - y1).abs();
  });
  print(distances.reduce((a, b) => a + b));
}

Set<(int, int)> expand(
    Set<(int, int)> locations, List<int> cols, List<int> rows) {
  return locations
      .map((e) =>
          (e.$1 + countLessThan(e.$1, cols), e.$2 + countLessThan(e.$2, rows)))
      .toSet();
}

int countLessThan(int n, List<int> l) {
  return l.where((e) => e < n).length;
}
