# Julia REPL commands to solve AoC Task 05

f = open("05/input.txt")
lines = readlines(f)

# separate inital stack setup from moves
lines_stacks, lines_moves = [lines[1:findfirst(isempty, lines)-2], lines[11:end]]

# transpose into array of arrays that each represent a stack and remove empty spaces
tmp = lines_stacks .|> (x -> x[2:2:end]) .|> (x -> x[1:2:end]) .|> (x -> only.(split(x, "")))
cratestacks = mapslices(x -> [x], hcat(tmp...), dims = 2)[:] .|> (x -> filter!(y -> y != ' ', x))

# also decode the moves
moves = map(x -> parse.(Int, match(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)", x).captures), lines_moves)

# define a function to push around crates according to an input move vector
mover!(movevec) = (splice!(cratestacks[movevec[2]], 1:movevec[1]) |> reverse |> (x -> pushfirst!(cratestacks[movevec[3]], x...)))

# Solution for Task 05.1 -- apply all moves, then print out the top crates on each stack
# but first save cratestacks in case we need it again
cratestacks_snapshot = deepcopy(cratestacks)
mover!.(moves);
string(first.(cratestacks)...)

# Solution for Task 05.2 -- modify mover! ... simply remove the reverse to implement the CraveMover9001
# restore original cratestacks, then apply moves again
cratestacks = cratestacks_snapshot
mover9001!(movevec) = (splice!(cratestacks[movevec[2]], 1:movevec[1]) |> (x -> pushfirst!(cratestacks[movevec[3]], x...)))
mover9001!.(moves);
string(first.(cratestacks)...)