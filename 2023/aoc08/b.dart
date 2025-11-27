import 'dart:io';

void main() {
  final lines = File('input.txt').readAsLinesSync();
  final instructions = lines[0].split('');
  final network = {
    for (final line in lines.sublist(2))
      line.substring(0, 3): (line.substring(7, 10), line.substring(12, 15))
  };
  final starts = network.keys.where((node) => node[2] == "A");
  final distances = starts.map((n) => run(n, instructions, network));
  final steps = distances.reduce(lcm);
  print(steps);
}

int run(String start, List<String> instructions,
    Map<String, (String, String)> network) {
  var steps = 0;
  var node = start;
  while (true) {
    for (final i in instructions) {
      final (l, r) = network[node]!;
      node = (i == "L") ? l : r;
      steps++;
      if (node[2] == "Z") {
        return steps;
      }
    }
  }
}

// shamelessly stolen from the internet
int lcm(int a, int b) {
  return ((a ~/ (a.gcd(b))) * b).abs();
}
