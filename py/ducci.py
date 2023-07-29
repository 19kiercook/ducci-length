from typing import List, Tuple

DUCCI_SIZE = 4

def next_ducci(ducci: Tuple[int]) -> Tuple[int]:
    """Compute the next Ducci sequence."""
    return (
        abs(ducci[0] - ducci[1]),
        abs(ducci[1] - ducci[2]),
        abs(ducci[2] - ducci[3]),
        abs(ducci[3] - ducci[0])
    )

def generate_swaps(ducci: Tuple[int]) -> Tuple[Tuple[int], Tuple[int]]:
    swap_one = (ducci[1], ducci[0], ducci[2], ducci[3])
    swap_two = (ducci[0], ducci[2], ducci[1], ducci[3])
    return swap_one, swap_two

def checker_convergence(ducci: Tuple[int]) -> bool:
    """Check if the Ducci sequence has converged in the checker format (0, a, 0, a)."""
    return ducci[0] == 0 and ducci[2] == 0 and ducci[1] == ducci[3]

def ladder_convergence(ducci: Tuple[int]) -> bool:
    """Check if the Ducci sequence has converged in the ladder format (0, a, 2a, a)."""
    return ducci[0] == 0 and ducci[2] == 2 * ducci[1] and ducci[3] == ducci[1]

def generate_rotations(ducci: Tuple[int]) -> List[Tuple[int]]:
    """Generate the rotations of the Ducci sequence."""
    
    # 90 degree rotation
    rotation_one = (ducci[3], ducci[0], ducci[1], ducci[2])
    # 180 degree rotation
    rotation_two = (ducci[2], ducci[3], ducci[0], ducci[1])
    # 270 degree rotation
    rotation_three = (ducci[1], ducci[2], ducci[3], ducci[0])

    return [rotation_one, rotation_two, rotation_three]

def is_terminal(ducci: Tuple[int]) -> bool:
    if all([x == 0 for x in ducci]):
        return True

    for rotation in generate_rotations(ducci):
        if checker_convergence(rotation):
            return True
        if ladder_convergence(rotation):
            return True
        
    return False

def calculate_paths(ducci: Tuple[int], lookup_table = None) -> List[Tuple[Tuple[int], int]]:
    """Calculate the paths for the Ducci sequence."""
    if not lookup_table is None:
        if ducci in lookup_table:
            return lookup_table[ducci]

    next_state = next_ducci(ducci)

    if is_terminal(next_state):
        return [(next_state, 0)]

    terminals = []
    swaps = generate_swaps(next_state)

    for swap in swaps:
        swap_results = calculate_paths(swap)
        for result in swap_results:
            terminals.append((result[0], result[1] + 1))

    if not lookup_table is None:
        lookup_table[ducci] = terminals

    return terminals

if __name__ == "__main__":
    from tqdm import tqdm

    MAX_INT = 30

    progress_bar = tqdm(total=MAX_INT ** 4)
    lookup_table = {}

    for a in range(MAX_INT):
        for b in range(MAX_INT):
            for c in range(MAX_INT):
                for d in range(MAX_INT):
                    ducci = (a, b, c, d)
                    calculate_paths(ducci, lookup_table)
                    progress_bar.update(1)

    # ducci = [2, 0, 0, 2]
    # print(is_terminal(next_ducci(ducci)))