import 'dart:io';

final cards = {
  "A": 13,
  "K": 12,
  "Q": 11,
  "T": 9,
  "9": 8,
  "8": 7,
  "7": 6,
  "6": 5,
  "5": 4,
  "4": 3,
  "3": 2,
  "2": 1,
  "J": 0,
};

int handType(String hand) {
  final count = <String, int>{};
  for (final c in hand.split("")) {
    count.putIfAbsent(c, () => 0);
    count[c] = count[c]! + 1;
  }
  final jacks = count.remove('J') ?? 0;
  final mostCommon = count.entries.fold<MapEntry<String, int>?>(null, (e1, e2) {
    if (e1 == null || e2.value > e1.value) {
      return e2;
    }
    return e1;
  })?.key;
  if (mostCommon != null) {
    count[mostCommon] = count[mostCommon]! + jacks;
  } else {
    count['J'] = jacks;
  }
  switch (count.length) {
    case 5:
      return 1;
    case 4:
      return 2; // 1pair
    case 3: // 3kind or 2pair
      if (count.values.contains(3)) {
        return 4; // 3kind
      }
      return 3; // 2pair
    case 2: // full hourse or 4kind
      if (count.values.contains(4)) {
        return 6; // 4kind
      }
      return 5; // full house
    case 1:
      return 7; // 5kind
    default:
      throw Error();
  }
}

int compareHand(String hand1, String hand2) {
  for (var i = 0; i < 5; i++) {
    final comp = cards[hand1[i]]!.compareTo(cards[hand2[i]]!);
    if (comp != 0) {
      return comp;
    }
  }
  return 0;
}

void main() {
  final handsBids = File('input.txt').readAsLinesSync().map((l) {
    final p = l.split(' ');
    return (p[0], int.parse(p[1]));
  }).toList();

  handsBids.sort((a, b) {
    final comp = handType(a.$1).compareTo(handType(b.$1));
    if (comp == 0) {
      return compareHand(a.$1, b.$1);
    }
    return comp;
  });
  final result = handsBids.indexed
      .map((e) => (e.$1 + 1) * (e.$2.$2))
      .reduce((a, b) => a + b);
  print(result);
}
