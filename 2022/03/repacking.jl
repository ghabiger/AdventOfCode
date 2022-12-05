# Julia REPL commands to solve AoC Task 02

f = open("03/input.txt")
lines = readlines(f)

# convert: Char-array -> 2 compartments of equal length
as_items = [map(x -> only(x), s) for s in split.(lines, "")]
compartments = map(x -> collect(Iterators.partition(x, Int(size(x)[1]/2))), as_items)

# find intersecting items and convert to their priority
intersections = [intersect(x[1], x[2]) for x in compartments]
priorities = [(Int(x[1]) < 97 ? Int(x[1] - 38) : Int(x[1] - 96)) for x in intersections]

# Solution for Task 03.1
sum(priorities)

# Solution for Task 03.2
# group by groups of 3, then again find intersecting items, etc.
groups = collect(Iterators.partition(as_items, 3))
badges = [intersect(x...) for x in groups]
sum([assign_priority(x[1]) for x in badges])