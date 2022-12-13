# Julia REPL commands to solve AoC Task 06

f = open("06/input.txt")
lines = readlines(f)

chars = split(lines[1], "")

# Task 06.1
for c in 1:length(chars)
    v = Set(@view chars[c:c+3])
    if length(v) == 4
        println(c+3)
        break
    end
end

# Task 06.2
for c in 1:length(chars)
    v = Set(@view chars[c:c+13])
    if length(v) == 14
        println(c+13)
        break
    end
end