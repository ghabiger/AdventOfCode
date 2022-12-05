# Julia REPL commands to solve AoC Task 01

f = open("01/input.txt")
lines = readlines(f)

# separate elves by empty lines
emptylines = findall(isempty, lines)
elves = getindex.(Ref(lines), UnitRange.([1; emptylines .+ 1], [emptylines .- 1; length(lines)]))

# sum up all calories per elf
sum_calories = elves .|> (x -> parse.(Int, x)) .|> sum

# Lösung für AoC Task 01.1
maximum(sum_calories)

# Lösung für AoC Task 01.2
sum(sort(sum_calories, rev=true)[1:3])