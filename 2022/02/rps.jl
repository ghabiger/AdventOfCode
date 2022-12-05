# Julia REPL commands to solve AoC Task 02

# check the score of a given vector of 2x Char representing a RPS round
function rpschecker(vec)
    check = vec[2] - vec[1] - 23
    if check == 1 || check == -2
        res = 6
    elseif check == 0
        res = 3
    else
        res = 0
    end
    res + Int(vec[2]) - 87
end

# Solve the RPS round for a given challenge and strategic choice (as a vector of 2x Char)
function rpssolver(vec)
    if vec[2] == 'Y'
        newvec = [vec; vec[1] + 23]
    elseif vec[2] == 'Z'
        newvec = [vec; Char((Int((vec[1]) - 65) + 1) % 3 + 88)]
    else
        newvec = [vec; Char((Int((vec[1]) - 65) + 2) % 3 + 88)]
    end
    newvec
end

f = open("02\\input.txt")
lines = readlines(f)

# convert to array of Char-arrays
strat_raw = map(x -> split(x, " "), lines)
strat_chars = map(x -> [only(x[1]), only(x[2])], strat_raw)

# Solution for Task 02.1
sum(map(rpschecker, strat_chars))

# Solution for Task 02.2
sum(map(rpschecker, map(x -> [x[1], x[3]], map(rpssolver, strat_chars))))