import 'dart:io';

void main() {
  final lines = File('input.txt').readAsLinesSync();
  final instructions = lines[0].split('');
  final network = {
    for (final line in lines.sublist(2))
      line.substring(0, 3): (line.substring(7, 10), line.substring(12, 15))
  };
  print(run(instructions, network));
}

int run(List<String> instructions, Map<String, (String, String)> network) {
  var steps = 0;
  var node = "AAA";
  while (true) {
    for (final i in instructions) {
      final (l, r) = network[node]!;
      node = (i == "L") ? l : r;
      steps++;
      if (node == "ZZZ") {
        return steps;
      }
    }
  }
}
