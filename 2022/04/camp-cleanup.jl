# Julia REPL commands to solve AoC Task 04

f = open("04/input.txt")
lines = readlines(f)

# convert to simple arrays to simplify work
pair_boundaries = map(x -> [parse(Int, y.match) for y in eachmatch(r"([0-9]+)", x)], lines)

# swap those pairs where the latter pair begins with a smaller section than the first, to simplify later steps
# if the latter pair's start is equal, compare for length and swap only if longer
pair_boundaries_sorted = map(x -> (x[3] < x[1] || (x[3] == x[1] && x[4]-x[3] > x[2]-x[1])) ? [x[3], x[4], x[1], x[2]] : x, pair_boundaries)

# Solution for Task 04.1 -- test whether the second pair is within the first, count all matches
count(x -> x[3] >= x[1] && x[4] <= x[2], pair_boundaries_sorted)

# Solution for Task 04.2 -- test whether pairs overlap and count
count(x -> x[2] >= x[3], pair_boundaries_sorted)