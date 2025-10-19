import argparse
from collections.abc import Set
import re


class Solution:
  filename_real_input = "real_input.txt"
  filename_test_input = "test_input.txt"

  @staticmethod
  def get_nums(string: str) -> list[int]:
    return list(map(int, re.findall("[-+]?\\d+", string)))

  def __init__(self, test=False):
    self.filename = self.filename_test_input if test else self.filename_real_input
    self.file = open(self.filename, "r").read()
    self.lines = self.file.splitlines()

  def part1(self):
    self.map = list(list(map(lambda x: int(x), list(line))) for line in self.lines)
    starting_points = self.collect_starting_points()
    scores = {}
    for point in starting_points.keys():
      x, y = point
      visited = set()
      reached_nine = set()
      self.progress_from_point(x, y, visited, reached_nine)
      scores[point] = len(reached_nine)
    return sum(scores.values())

  def part2(self):
    self.map = list(list(map(lambda x: int(x), list(line))) for line in self.lines)
    starting_points = self.collect_starting_points()
    scores = {}
    for point in starting_points.keys():
      x, y = point
      visited = set()
      reached_nine = []
      self.progress_from_point(x, y, visited, reached_nine)
      scores[point] = len(reached_nine)
    return sum(scores.values())

  def collect_starting_points(self):
    starting_points = {}
    for y in range(len(self.map)):
      for x in range(len(self.map[0])):
        if self.map[y][x] == 0:
          starting_points[(x, y)] = True
    return starting_points

  def neighboring_points(self, x, y):
    neighbors = []
    if x > 0:
      neighbors.append((x - 1, y))
    if x < len(self.map[0]) - 1:
      neighbors.append((x + 1, y))
    if y > 0:
      neighbors.append((x, y - 1))
    if y < len(self.map) - 1:
      neighbors.append((x, y + 1))
    return neighbors

  def progress_from_point(self, x, y, visited, reached_nine):
    current_value = self.map[y][x]
    if current_value == 9:
      if isinstance(reached_nine, set):
        reached_nine.add((x, y))
      else:
        reached_nine.append((x, y))
      return
    for neighbor in self.neighboring_points(x, y):
      nx, ny = neighbor
      neighbor_value = self.map[ny][nx]
      if neighbor_value == current_value + 1:
        self.progress_from_point(nx, ny, visited, reached_nine)

if __name__ == "__main__":
  parser = argparse.ArgumentParser("Solution file")
  parser.add_argument("-part", required=True, type=int, help="Part (1/2)")
  parser.add_argument("-test", required=True, type=str, help="Test mode (True/False)")
  args = parser.parse_args()
  test = True if args.test in ["True", "true"] else False
  solution = Solution(test=test)
  result = solution.part1() if args.part == 1 else solution.part2()
  print(f"Result for Part=={args.part} & Test=={test} : {result}")
