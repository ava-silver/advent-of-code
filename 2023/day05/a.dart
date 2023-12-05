import 'dart:io';
import 'dart:math';

class RangeDict {
  List<(int, int, int)> ranges;

  RangeDict(this.ranges);

  int get(int x) {
    for (final (outputStart, inputStart, len) in this.ranges) {
      if (inputStart <= x && x < inputStart + len) {
        return outputStart + (x - inputStart);
      }
    }
    return x;
  }
}

void main() {
  final data = File("input.txt").readAsStringSync().split("\n\n");
  final seeds = data
      .removeAt(0)
      .split(":")[1]
      .split(" ")
      .where((s) => s.isNotEmpty)
      .map(int.parse)
      .toList();
  final maps = data.map((m) {
    final lines = m.split('\n');
    lines.removeAt(0);
    return RangeDict(lines.map((l) {
      final ints = l.split(' ').map(int.parse).toList();
      return (ints[0], ints[1], ints[2]);
    }).toList());
  });
  final minThing = maps
      .fold<Iterable<int>>(seeds, (values, map) => values.map(map.get))
      .fold(0x7FFFFFFFFFFFFFFF, min);
  print(minThing);
}
