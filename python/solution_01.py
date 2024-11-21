from utils import read_input

def part_1(input: str) -> int:
    floor: int = 0
    for c in input:
        if c == '(':
            floor += 1
        else:
            floor += -1
    return floor

def part_2(input: str) -> int:
    floor: int = 0
    count: int = 0
    for c in input:
        count += 1
        if c == '(':
            floor += 1
        else:
            floor += -1

        if floor < 0:
            return count

def solve() -> None:
    input = read_input("01").replace("\n", "")
    return part_1(input), part_2(input)


if __name__ == "__main__":
    floor, basement = solve()
    print(f"Floor: {floor}")
    print(f"Basement: {basement}")