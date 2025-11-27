import 'dart:io';

// only 12 red cubes, 13 green cubes, and 14 blue cubes
final maxRocks = {
  "red": 12,
  "green": 13,
  "blue": 14,
};

bool isValidPull(String pull) {
  for (final colorPull in pull.split(',')) {
    final color = colorPull.split(' ').where((e) => e != '').toList();
    if (int.parse(color[0]) > maxRocks[color[1]]!) {
      return false;
    }
  }
  return true;
}

void main() {
  final games = File("input.txt").readAsLinesSync();
  final sum = games.fold(0, (idSum, game) {
    final parts = game.split(':');
    final pulls = parts[1].split(';');
    if (pulls.every(isValidPull)) {
      final gameId = int.parse(parts[0].substring(5));
      return idSum + gameId;
    } else {
      return idSum;
    }
  });
  print(sum);
}
