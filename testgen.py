import random
import subprocess

board_len = 10
random.seed(20)

def gen_numbers():
    numbers = []
    for _ in range(board_len):
        numbers.append(random.randrange(-board_len // 3, board_len // 3 + 1))
    return numbers
    
def check(name: str, number_in) -> int:
    p = subprocess.run([f"target/{name}"], stdout=subprocess.PIPE, input=f"\n{number_in}", encoding="ascii")
    return int(p.stdout)

for i in range(10):
    number_in = ' '.join(map(str, gen_numbers()))
    old = check("froggerhard_old", number_in)
    new = check("froggerhard", number_in)
    if new != old:
        print(f"DIFFERENCE (i = {i})")
        print(f"in: {number_in}")
        print(f"old: {old}")
        print(f"new: {new}")
        print("="*30)
