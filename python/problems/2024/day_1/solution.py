import argparse
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
    lists = list(li.split('  ') for li in self.lines)
    l1 = sorted(int(v[0]) for v in lists)
    r1 = sorted(int(v[1]) for v in lists)
    return sum(abs(l - r) for l, r in (zip(l1, r1)))

  def part2(self):
    lists = list(li.split('  ') for li in self.lines)
    l1 = sorted(int(v[0]) for v in lists)
    r1 = sorted(int(v[1]) for v in lists)
    return sum(l * sum((1 if r == l else 0) for r in r1) for l in l1)


if __name__ == "__main__":
  parser = argparse.ArgumentParser("Solution file")
  parser.add_argument("-part", required=True, type=int, help="Part (1/2)")
  parser.add_argument("-test", required=True, type=str, help="Test mode (True/False)")
  args = parser.parse_args()
  test = True if args.test in ["True", "true"] else False
  solution = Solution(test=test)
  result = solution.part1() if args.part == 1 else solution.part2()
  print(f"Result for Part=={args.part} & Test=={test} : {result}")
