# horrible code
import itertools
perms = set()
for x in itertools.permutations(range(8), 8):
    p1 = sorted([x[0], x[1]])
    p2 = sorted([x[2], x[3]])
    p3 = sorted([x[4], x[5]])
    p4 = sorted([x[6], x[7]])
    pairs = [(p1[0], p1[1]), (p2[0], p2[1]), (p3[0], p3[1]), (p4[0], p4[1])]
    pairs.sort()
    perms.add((pairs[0][0], pairs[0][1], pairs[1][0], pairs[1][1], pairs[2][0], pairs[2][1], pairs[3][0], pairs[3][1]))

print(sorted(perms))