import sys
from collections import defaultdict
from copy import deepcopy
from dataclasses import astuple, dataclass
from enum import Enum
from functools import reduce
from typing import Iterable, List, Optional, Tuple


class Orientation(Enum):
    ROTATE000 = (0, False)
    ROTATE090 = (1, False)
    ROTATE180 = (2, False)
    ROTATE270 = (3, False)
    ROTATE000_FLIPPED = (0, True)
    ROTATE090_FLIPPED = (1, True)
    ROTATE180_FLIPPED = (2, True)
    ROTATE270_FLIPPED = (3, True)


@dataclass
class Borders:
    top: str
    left: str
    bottom: str
    right: str


class Tile:
    def __init__(self, tile_id: int, lines: List[str]) -> None:
        self.id = tile_id
        self.data = [list(s) for s in lines]

    def borders(self, orientation: Orientation) -> Borders:
        data = self.translated(orientation)
        return Borders(
            *[
                "".join(x)
                for x in [
                    data[0],
                    [data[i][0] for i in range(len(data))],
                    data[-1],
                    [data[i][-1] for i in range(len(data))],
                ]
            ]
        )

    def translated(self, orientation: Orientation) -> List[List[str]]:
        rotate, flip = orientation.value
        data = deepcopy(self.data)
        for _ in range(rotate):
            next_data = deepcopy(data)
            for i, row in enumerate(data):
                for j, col in enumerate(row):
                    next_data[j][len(row) - 1 - i] = data[i][j]
            data = next_data
        if flip:
            for row in data:
                row.reverse()
        return data


@dataclass
class Arrangement:
    tile: Tile
    orientation: Orientation

    def borders(self) -> Borders:
        return self.tile.borders(self.orientation)

    def data_without_border(self) -> List[List[str]]:
        data = self.tile.translated(self.orientation)
        return [row[1:-1] for row in data[1:-1]]


class Solution:
    def __init__(self, inputs: List[str]) -> None:
        def split_at_empty() -> Iterable[List[str]]:
            indices = [idx for idx, x in enumerate(inputs) if not x]
            for start, end in zip([-1, *indices], [*indices, len(inputs)]):
                yield inputs[start + 1 : end]

        tiles = []
        for lines in split_at_empty():
            if not lines:
                continue
            tile_id = int(lines[0].split(" ")[1].rstrip(":"))
            tiles.append(Tile(tile_id, lines[1:]))

        self.reassembled = self.__reassemble(tiles)

    def part_1(self) -> int:
        return reduce(
            lambda x, y: x * y,
            [
                self.reassembled[0][0].tile.id,
                self.reassembled[0][-1].tile.id,
                self.reassembled[-1][0].tile.id,
                self.reassembled[-1][-1].tile.id,
            ],
        )

    def part_2(self) -> int:
        image_data = []
        for row in self.reassembled:
            data_row = [col.data_without_border() for col in row]
            for i in range(len(data_row[0])):
                image_data.append(
                    "".join(["".join(data_col[i]) for data_col in data_row])
                )
        image = Tile(0, image_data)

        sea_monster = [
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ]
        monster_height = len(sea_monster)
        monster_width = len(sea_monster[0])
        monster_positions = []
        for i in range(len(sea_monster)):
            for j in range(len(sea_monster[i])):
                if sea_monster[i][j] == "#":
                    monster_positions.append((i, j))

        def search_monsters(image: List[List[str]]) -> int:
            def found(i: int, j: int) -> bool:
                def match_monster(d: Tuple[int, int]) -> bool:
                    return image[i + d[0]][j + d[1]] == "#"

                return all(map(match_monster, monster_positions))

            count = 0
            for i in range(len(image) - monster_height):
                for j in range(len(image[i]) - monster_width):
                    if found(i, j):
                        count += 1
            return count

        for o in Orientation:
            translated = image.translated(o)
            num_monsters = search_monsters(translated)
            if num_monsters > 0:
                return len(
                    list(filter(lambda x: x == "#", sum(image.data, [])))
                ) - num_monsters * len(monster_positions)
        return 0

    def __reassemble(self, tiles: List[Tile]) -> List[List[Arrangement]]:
        border_ids = defaultdict(list)
        for t in tiles:
            for o in [Orientation.ROTATE000, Orientation.ROTATE180]:
                for border in astuple(t.borders(o)):
                    border_ids[border].append(t.id)

        def first_row() -> List[Arrangement]:
            def first_col() -> Arrangement:
                def is_corner(tile: Tile) -> bool:
                    def has_adjacent(border: str) -> bool:
                        return len(border_ids[border]) > 1

                    borders = astuple(tile.borders(Orientation.ROTATE000))
                    return len(list(filter(has_adjacent, borders))) < 3

                corner = next(filter(is_corner, tiles))

                def is_top_left(orientation: Orientation) -> bool:
                    borders = corner.borders(orientation)
                    return (
                        len(border_ids[borders.top]) == 1
                        and len(border_ids[borders.left]) == 1
                    )

                return Arrangement(corner, next(filter(is_top_left, Orientation)))

            row = [first_col()]

            def find_right(curr: Arrangement) -> Optional[Arrangement]:
                right = curr.borders().right
                ids = border_ids[right]
                if len(ids) != 2:
                    return None
                right_id = ids[0] if ids[1] == curr.tile.id else ids[1]
                t = next(filter(lambda t: t.id == right_id, tiles))
                for o in Orientation:
                    if t.borders(o).left == right:
                        return Arrangement(t, o)
                return None

            while True:
                right = find_right(row[-1])
                if right:
                    row.append(right)
                else:
                    break
            return row

        rows = [first_row()]

        def find_bottom(curr: Arrangement) -> Optional[Arrangement]:
            bottom = curr.borders().bottom
            ids = border_ids[bottom]
            if len(ids) != 2:
                return None
            bottom_id = ids[0] if ids[1] == curr.tile.id else ids[1]
            t = next(filter(lambda t: t.id == bottom_id, tiles))
            for o in Orientation:
                if t.borders(o).top == bottom:
                    return Arrangement(t, o)
            return None

        while True:
            row = []
            for col in rows[-1]:
                bottom = find_bottom(col)
                if bottom:
                    row.append(bottom)
            if row:
                rows.append(row)
            else:
                break
        return rows


if __name__ == "__main__":
    solution = Solution(sys.stdin.read().splitlines())
    print(f"Part 1: {solution.part_1()}")
    print(f"Part 2: {solution.part_2()}")
