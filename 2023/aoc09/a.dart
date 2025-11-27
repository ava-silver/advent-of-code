import 'dart:io';

void main() {
  final histories = File('input.txt')
      .readAsLinesSync()
      .map((l) => l.split(' ').map(int.parse).toList());
  final nextValues = histories.map((history) {
    final stack = <List<int>>[history];
    while (stack.last.any((e) => e != 0)) {
      stack.add(List.generate(stack.last.length - 1,
          (idx) => stack.last[idx + 1] - stack.last[idx]));
    }
    for (var idx = stack.length - 2; idx >= 0; idx--) {
      stack[idx].add(stack[idx].last + stack[idx + 1].last);
    }
    return stack[0].last;
  });

  final sum = nextValues.reduce((a, b) => a + b);
  print(sum);
}
