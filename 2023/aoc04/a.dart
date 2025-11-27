import 'dart:io';
import 'dart:math';

void main() {
  final cards = File("input.txt").readAsLinesSync().map((l) => l.split(":")[1]);
  final points = cards.fold(0, (sum, card) {
    final parts = card.split("|");
    final winningNumbers =
        parts[0].split(' ').where((e) => e.isNotEmpty).toList();
    final givenNumbers = parts[1].split(' ').where((e) => e.isNotEmpty);
    final numMatches =
        givenNumbers.where((n) => winningNumbers.contains(n)).length;
    if (numMatches == 0) {
      return sum;
    } else {
      return sum + pow(2, numMatches - 1).toInt();
    }
  });
  print(points);
}
