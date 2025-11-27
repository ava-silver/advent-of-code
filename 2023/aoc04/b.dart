import 'dart:io';

void main() {
  final cards = File("input.txt").readAsLinesSync().map((l) => l.split(":")[1]);
  final numCards = List.generate(cards.length, (_) => 1, growable: false);
  for (final (idx, card) in cards.indexed) {
    final parts = card.split("|");
    final winningNumbers =
        parts[0].split(' ').where((e) => e.isNotEmpty).toList();
    final givenNumbers = parts[1].split(' ').where((e) => e.isNotEmpty);
    final numMatches =
        givenNumbers.where((n) => winningNumbers.contains(n)).length;
    final curCards = numCards[idx];
    for (var i = idx + 1; i <= idx + numMatches; i++) {
      numCards[i] += curCards;
    }
  }
  print(numCards.fold<int>(0, (s, i) => s + i));
}
