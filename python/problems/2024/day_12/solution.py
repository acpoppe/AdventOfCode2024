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
    self.map = self.set_map()

  def part1(self):
    visited = set()
    return self.get_region_prices(visited)

  def part2(self):
    visited = set()
    return self.get_bulk_region_prices(visited)

  def set_map(self):
    return list(list(line) for line in self.lines)

  def get_region_prices(self, visited: set[tuple[int, int]]) -> int:
    region_prices = 0
    for y in range(len(self.map)):
      for x in range(len(self.map[0])):
        if (x, y) in visited:
          continue
        region = self.get_coords_in_region(x, y, visited)
        area = len(region)
        perimeter = self.get_perimeter_of_region(region)
        region_prices += area * perimeter
    return region_prices

  def get_bulk_region_prices(self, visited: set[tuple[int, int]]) -> int:
    region_prices = 0
    for y in range(len(self.map)):
      for x in range(len(self.map[0])):
        if (x, y) in visited:
          continue
        region = self.get_coords_in_region(x, y, visited)
        area = len(region)
        perimeter = self.get_region_corner_counts(region)
        region_prices += area * perimeter
    return region_prices

  def get_coords_in_region(
    self, x: int, y: int, visited: set[tuple[int, int]]
  ) -> set[tuple[int, int]]:
    region = set()
    to_visit = [(x, y)]
    target_value = self.map[y][x]
    while to_visit:
      cx, cy = to_visit.pop()
      if (cx, cy) in visited:
        continue
      visited.add((cx, cy))
      region.add((cx, cy))
      for nx, ny in self.get_neighbors(cx, cy):
        if (nx, ny) not in visited and self.map[ny][nx] == target_value:
          to_visit.append((nx, ny))
    return region

  def get_perimeter_of_region(self, region: set[tuple[int, int]]) -> int:
    perimeter = 0
    for x, y in region:
      for nx, ny in self.get_neighbors(x, y):
        if (nx, ny) not in region:
          perimeter += 1
      if x == 0:
        perimeter += 1
      if x == len(self.map[0]) - 1:
        perimeter += 1
      if y == 0:
        perimeter += 1
      if y == len(self.map) - 1:
        perimeter += 1
    return perimeter

  def get_region_corner_counts(self, region: set[tuple[int, int]]) -> int:
    corner_count = 0
    for x, y in region:
      if x == 0 and y == 0:
        corner_count += 1
      if x == 0 and y == len(self.map) - 1:
        corner_count += 1
      if x == len(self.map[0]) - 1 and y == 0:
        corner_count += 1
      if x == len(self.map[0]) - 1 and y == len(self.map) - 1:
        corner_count += 1

      # top left outside corner
      if (x > 0 and y > 0 and ((x-1, y) not in region) and ((x, y-1) not in region)) or (x > 0 and y == 0 and (x-1, y) not in region) or (x == 0 and y > 0 and (x, y-1) not in region):
        corner_count += 1

      # top left inside corner
      if (x < len(self.map[0]) -1 and y < len(self.map) -1 and ((x+1, y) in region) and ((x, y+1) in region)) and ((x+1, y+1) not in region):
        corner_count += 1

      # top right outside corner
      if (x < len(self.map[0]) -1 and y > 0 and ((x+1, y) not in region) and ((x, y-1) not in region)) or (x < len(self.map[0]) -1 and y == 0 and (x+1, y) not in region) or (x == len(self.map[0]) -1 and y > 0 and (x, y-1) not in region):
        corner_count += 1

      # top right inside corner
      if (x > 0 and y < len(self.map) -1 and ((x-1, y) in region) and ((x, y+1) in region)) and ((x-1, y+1) not in region):
        corner_count += 1

      # bottom left outside corner
      if (x > 0 and y < len(self.map) -1 and ((x-1, y) not in region) and ((x, y+1) not in region)) or (x > 0 and y == len(self.map) -1 and (x-1, y) not in region) or (x == 0 and y < len(self.map) -1 and (x, y+1) not in region):
        corner_count += 1

      # bottom left inside corner
      if (x < len(self.map[0]) -1 and y > 0 and ((x+1, y) in region) and ((x, y-1) in region)) and ((x+1, y-1) not in region):
        corner_count += 1

      # bottom right outside corner
      if (x < len(self.map[0]) -1 and y < len(self.map) -1 and ((x+1, y) not in region) and ((x, y+1) not in region)) or (x < len(self.map[0]) -1 and y == len(self.map) -1 and (x+1, y) not in region) or (x == len(self.map[0]) -1 and y < len(self.map) -1 and (x, y+1) not in region):
        corner_count += 1

      # bottom right inside corner
      if (x > 0 and y > 0 and ((x-1, y) in region) and ((x, y-1) in region)) and ((x-1, y-1) not in region):
        corner_count += 1

    return corner_count

  def get_neighbors(self, x: int, y: int) -> list[tuple[int, int]]:
    neighbors = []
    directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]
    for dx, dy in directions:
      nx, ny = x + dx, y + dy
      if 0 <= nx < len(self.map[0]) and 0 <= ny < len(self.map):
        neighbors.append((nx, ny))
    return neighbors


if __name__ == "__main__":
  parser = argparse.ArgumentParser("Solution file")
  parser.add_argument("-part", required=True, type=int, help="Part (1/2)")
  parser.add_argument("-test", required=True, type=str, help="Test mode (True/False)")
  args = parser.parse_args()
  test = True if args.test in ["True", "true"] else False
  solution = Solution(test=test)
  result = solution.part1() if args.part == 1 else solution.part2()
  print(f"Result for Part=={args.part} & Test=={test} : {result}")
