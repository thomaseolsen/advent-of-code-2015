def read_input(day: str) -> str:
    with open(f"../input/{day}") as file:
        return file.read()
    
def get_input_lines(input: str) -> list[str]:
    return input.split("\n")