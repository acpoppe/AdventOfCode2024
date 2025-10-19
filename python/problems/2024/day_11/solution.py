import argparse
import re

class Solution:
  filename_real_input = 'real_input.txt'
  filename_test_input = 'test_input.txt'
  
  @staticmethod
  def get_nums(string: str) -> list[int]:
    return list(map(int, re.findall('[-+]?\\d+', string)))
  
  def __init__(self, test=False):
    self.filename = self.filename_test_input if test else self.filename_real_input
    self.file = open(self.filename,'r').read()
    self.lines = self.file.splitlines()
    
  def part1(self):
    stones = self.file.strip().split(' ')
    memo = {}
    return sum(self.count_stones_growth(stone, 25, memo) for stone in stones)
  
  def part2(self):
    stones = self.file.strip().split(' ')
    memo = {}
    return sum(self.count_stones_growth(stone, 75, memo) for stone in stones)

  def count_stones_growth(self, stone: str, remaining_days: int, memo: dict) -> int:
    if remaining_days == 0:
      return 1
    if (stone, remaining_days) in memo:
      return memo[(stone, remaining_days)]
    if stone == '0':
      count = self.count_stones_growth('1', remaining_days - 1, memo)
      memo[(stone, remaining_days)] = count
      return count
    if len(stone) % 2 == 0:
      mid = len(stone) // 2
      left_stone = str(int(stone[:mid]))
      right_stone = str(int(stone[mid:]))
      count = (self.count_stones_growth(left_stone, remaining_days - 1, memo) +
              self.count_stones_growth(right_stone, remaining_days - 1, memo))
      memo[(stone, remaining_days)] = count
      return count
    new_stone = str(int(stone) * 2024)
    count = self.count_stones_growth(new_stone, remaining_days - 1, memo)
    memo[(stone, remaining_days)] = count
    return count
  
if __name__ == '__main__':
  parser = argparse.ArgumentParser('Solution file')
  parser.add_argument('-part', required=True, type=int, help='Part (1/2)')
  parser.add_argument('-test', required=True, type=str, help='Test mode (True/False)')
  args = parser.parse_args()
  test = True if args.test in ['True','true'] else False
  solution = Solution(test=test)
  result = solution.part1() if args.part == 1 else solution.part2()
  print(f'Result for Part=={args.part} & Test=={test} : {result}')
