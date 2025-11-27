import 'dart:io';

void main() {
  final board =
      File("input.txt").readAsLinesSync().map((l) => l.split('')).toList();
  final gearSpaces = getGearSpaces(board);
  print(sumGearRatios(getGearNumbers(board, gearSpaces)));
}

Map<(int, int), Set<(int, int)>> getGearSpaces(List<List<String>> board) {
  final gearSpaces = <(int, int), Set<(int, int)>>{};
  for (final (y, row) in board.indexed) {
    for (final (x, ent) in row.indexed) {
      if (isGear(ent)) {
        for (var i = x - 1; i <= x + 1; i++) {
          for (var j = y - 1; j <= y + 1; j++) {
            gearSpaces.putIfAbsent((i, j), (() => <(int, int)>{}));
            gearSpaces[(i, j)]?.add((x, y));
          }
        }
      }
    }
  }
  return gearSpaces;
}

bool isGear(String ent) {
  return ent == '*';
}

Map<(int, int), List<int>> getGearNumbers(
    List<List<String>> board, Map<(int, int), Set<(int, int)>> gearSpaces) {
  // current number we're parsing
  var curNum = 0;
  // gear coordinates cur num corresponds to
  final gearNumSpaces = <(int, int)>{};
  // gear coordinate to the number
  final gearNums = <(int, int), List<int>>{};
  for (final (y, row) in board.indexed) {
    for (final (x, ent) in row.indexed) {
      final d = int.tryParse(ent);
      if (d == null) {
        // end of current number
        for (final (gearX, gearY) in gearNumSpaces) {
          gearNums
              .putIfAbsent((gearX, gearY), () => List.empty(growable: true));
          gearNums[(gearX, gearY)]?.add(curNum);
        }
        curNum = 0;
        gearNumSpaces.clear();
        continue;
      } else {
        // current number continues
        curNum *= 10;
        curNum += d;
        if (gearSpaces.containsKey((x, y))) {
          gearNumSpaces.addAll(gearSpaces[(x, y)]!);
        }
      }
    }
  }
  // end of current number (if any)
  for (final (gearX, gearY) in gearNumSpaces) {
    gearNums.putIfAbsent((gearX, gearY), List.empty);
    gearNums[(gearX, gearY)]?.add(curNum);
  }
  return gearNums;
}

int sumGearRatios(Map<(int, int), List<int>> gearNumbers) {
  return gearNumbers.values.fold(0, (sum, nums) {
    if (nums.length != 2) return sum;
    return sum + (nums[0] * nums[1]);
  });
}
