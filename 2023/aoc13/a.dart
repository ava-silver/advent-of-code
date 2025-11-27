import 'dart:io';
import 'dart:math';

void main() {
  final boards = File('input.txt')
      .readAsStringSync()
      .split('\n\n')
      .map((b) => b.split('\n'));
  final value = boards.fold(0, (total, board) => total + boardValue(board));
  print(value);
}

int boardValue(List<String> board) {
  colLoop:
  for (var x = 1; x < board[0].length; x++) {
    for (var row in board) {
      if (row.substring(max(0, (2 * x) - row.length), x) !=
          row.substring(x, min(row.length, 2 * x)).split('').reversed.join()) {
        continue colLoop;
      }
    }
    return x;
  }
  for (var y = 1; y < board.length; y++) {
    final s1 = board.sublist(max(0, (2 * y) - board.length), y);
    final s2 = board.sublist(y, min(board.length, 2 * y)).reversed.toList();
    if (listEqual(s1, s2)) {
      return 100 * y;
    }
  }
  throw Error();
}

// WHY THE FUCK IS THIS NOT A BUILT IN FUNCTION/METHOD I HATE THIS LANGUAGE
bool listEqual(List? a, List? b) {
  if (a == b) {
    return true;
  }

  if (a == null || b == null || a.length != b.length) {
    return false;
  }

  for (int i = 0; i < a.length; i++) {
    if (a[i] != b[i]) {
      return false;
    }
  }

  return true;
}
