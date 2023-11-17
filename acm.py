# Works on samples but on actual test cases for some reason.
# It's ok because this is not elegant. Time for haskell.
time, prob, grade = 0, 0, 0

def read():
    xs = input().split()
    if len(xs) != 3:
        return False
    global time
    global prob
    global grade
    time = int(xs[0])
    prob = xs[1]
    grade = xs[2]
    return True

times = {}
solved = set()

while read():
    print(grade)
    try:
        times[prob] = (times[prob][0] + 1, time)
    except:
        times[prob] = (1, time)

    if grade == "right":
        solved.add(prob)

score = 0
for prob in solved:
    score += times[prob][1] + 20 * (times[prob][0] - 1)
    
print(len(solved), score)
