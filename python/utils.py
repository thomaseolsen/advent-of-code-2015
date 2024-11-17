def read_input(day: str) -> str:
    with open(f"../input/{day}_input.txt") as file:
        return file.read().replace("\n", "")