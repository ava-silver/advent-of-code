import 'dart:io';

void main() {
  final board =
      File("input.txt").readAsLinesSync().map((l) => l.split('')).toList();
  final partSpaces = getPartSpaces(board);
  print(sumPartNumbers(board, partSpaces));
}

Set<(int, int)> getPartSpaces(List<List<String>> board) {
  final partSpaces = <(int, int)>{};
  for (final (y, row) in board.indexed) {
    for (final (x, ent) in row.indexed) {
      if (isSymbol(ent)) {
        for (var i = x - 1; i <= x + 1; i++) {
          for (var j = y - 1; j <= y + 1; j++) {
            partSpaces.add((i, j));
          }
        }
      }
    }
  }
  return partSpaces;
}

bool isSymbol(String ent) {
  return ent != '.' && int.tryParse(ent) == null;
}

int sumPartNumbers(List<List<String>> board, Set<(int, int)> partSpaces) {
  var sum = 0;
  var curNum = 0;
  var isPartNum = false;
  for (final (y, row) in board.indexed) {
    for (final (x, ent) in row.indexed) {
      final d = int.tryParse(ent);
      if (d == null) {
        if (isPartNum) {
          sum += curNum;
        }
        curNum = 0;
        isPartNum = false;
        continue;
      }
      curNum *= 10;
      curNum += d;
      if (partSpaces.contains((x, y))) {
        isPartNum = true;
      }
    }
  }
  if (isPartNum) {
    sum += curNum;
  }
  return sum;
}
