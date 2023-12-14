import 'dart:collection';
import 'dart:io';

void main() {
  final rows = File('input.txt').readAsLinesSync().map((l) {
    final halves = l.split(' ');
    return (halves[0], halves[1].split(',').map(int.parse).toList());
  });
  final total =
      rows.fold(0, (total, row) => total + arrangments(row.$1, row.$2));
  print(total);
}

int arrangments(String row, List<int> placements) {
  final placementRegex =
      RegExp(r'^\.*' + placements.map((p) => "#{$p}").join(r'\.+') + r'\.*$');
  final queue = Queue<String>();
  queue.add(row);
  var total = 0;
  while (queue.isNotEmpty) {
    var r = queue.removeFirst();
    if (r.contains('?')) {
      queue.add(r.replaceFirst('?', '#'));
      queue.add(r.replaceFirst('?', '.'));
      continue;
    }
    if (placementRegex.hasMatch(r)) {
      total++;
    }
  }
  return total;
}
