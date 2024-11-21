from utils import get_input_lines, read_input


def get_dimensions(present: str) -> list[int]:
    return [int(d) for d in present.split("x")]


def get_areas(present_dimensions: list[int]) -> list[int]:
    return [
        (present_dimensions[0] * present_dimensions[1]),
        (present_dimensions[1] * present_dimensions[2]),
        (present_dimensions[0] * present_dimensions[2]),
    ]


def get_perimeters(present_dimensions: list[int]) -> list[int]:
    return [
        (2 * present_dimensions[0]) + (2 * present_dimensions[1]),
        (2 * present_dimensions[1]) + (2 * present_dimensions[2]),
        (2 * present_dimensions[0]) + (2 * present_dimensions[2]),
    ]


def get_volume(present_dimensions: list[int]) -> int:
    return present_dimensions[0] * present_dimensions[1] * present_dimensions[2]


def part_1(input: list[str]) -> int:
    paper_needed: int = 0
    for present in input:
        if present != "":
            present_areas = get_areas(get_dimensions(present))
            paper_needed += (
                (2 * present_areas[0])
                + (2 * present_areas[1])
                + (2 * present_areas[2])
                + min(present_areas)
            )
    return paper_needed


def part_2(input: str) -> int:
    ribbon_needed: int = 0
    for present in input:
        if present != "":
            present_dimensions = get_dimensions(present)
            present_perimeters = get_perimeters(present_dimensions)
            ribbon_needed += min(present_perimeters) + get_volume(present_dimensions)
    return ribbon_needed


def solve() -> None:
    input = get_input_lines(read_input("02"))
    return part_1(input), part_2(input)


if __name__ == "__main__":
    paper, ribbon = solve()
    print(f"Wrapping Paper Needed: {paper}")
    print(f"Ribbon Needed: {ribbon}")
