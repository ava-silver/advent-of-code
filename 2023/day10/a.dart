import 'dart:io';

enum Dir {
  up,
  down,
  left,
  right,
}

Dir enterPipe(String pipe, Dir dir) {
  switch (pipe) {
    case "|":
    case "-":
      return dir;
    case "L":
      return (dir == Dir.down) ? Dir.right : Dir.up;
    case "J":
      return (dir == Dir.down) ? Dir.left : Dir.up;
    case "7":
      return (dir == Dir.up) ? Dir.left : Dir.down;
    case "F":
      return (dir == Dir.up) ? Dir.right : Dir.down;
    default:
      throw Error();
  }
}

(int, int) move(Dir dir, int x, int y) {
  switch (dir) {
    case Dir.left:
      return (x - 1, y);
    case Dir.right:
      return (x + 1, y);
    case Dir.up:
      return (x, y - 1);
    case Dir.down:
      return (x, y + 1);
  }
}

void main() {
  final map =
      File('input.txt').readAsLinesSync().map((l) => l.split('')).toList();
  final sY = map.indexWhere((y) => y.contains("S"));
  final sX = map[sY].indexOf("S");
  // we arbitrarily start by going left
  var dir = Dir.left;
  var (curX, curY) = (sX - 1, sY);
  // var dir = Dir.right;
  // var (curX, curY) = (sX + 1, sY);
  var steps = 1;
  // print("$sX, $sY, $curX, $curY");
  while ((curX, curY) != (sX, sY)) {
    // figure out new dir
    dir = enterPipe(map[curY][curX], dir);
    // move
    (curX, curY) = move(dir, curX, curY);
    steps++;
    // print("$dir, $curX, $curY");
  }
  print(steps ~/ 2);
}
