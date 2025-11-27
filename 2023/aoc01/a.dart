import 'dart:io';

void main() {
  var lines = File("input.txt").readAsLinesSync();
  var sum = lines.map((line) {
    var matches = RegExp(r'(\d)').allMatches(line);
    return int.parse(matches.first.group(0)!) * 10 +
        int.parse(matches.last.group(0)!);
  }).fold(0, (sum, e) => e + sum);
  print(sum);
}
