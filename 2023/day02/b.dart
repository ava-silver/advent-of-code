import 'dart:io';
import 'dart:math';

Map<String, int> minRocks(List<String> pulls) {
  Map<String, int> rocks = {};
  for (final pull in pulls) {
    for (final colorPull in pull.split(',')) {
      final colorCount = colorPull.split(' ').where((e) => e != '').toList();
      rocks[colorCount[1]] =
          max(rocks[colorCount[1]] ?? 0, int.parse(colorCount[0]));
    }
  }
  return rocks;
}

void main() {
  final games = File("input.txt").readAsLinesSync();
  final sum = games.fold(0, (power, game) {
    final pulls = game.split(':')[1].split(';');
    return power +
        minRocks(pulls)
            .values
            .fold(1, (product, colorMin) => product * colorMin);
  });
  print(sum);
}
