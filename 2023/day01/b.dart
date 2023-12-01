import 'dart:io';

var english_digits = {
  "one": 1,
  "two": 2,
  "three": 3,
  "four": 4,
  "five": 5,
  "six": 6,
  "seven": 7,
  "eight": 8,
  "nine": 9,
};

int parseDigit(String group) {
  return english_digits[group] ?? int.parse(group);
}

void main() {
  var digits = english_digits.keys.join("|");
  var regex = RegExp('(?=(\\d|$digits))');
  var lines = File("input.txt").readAsLinesSync();
  var sum = lines.map((line) {
    var matches = regex.allMatches(line);
    return parseDigit(matches.first[1]!) * 10 + parseDigit(matches.last[1]!);
  }).fold(0, (sum, e) => e + sum);
  print(sum);
}
