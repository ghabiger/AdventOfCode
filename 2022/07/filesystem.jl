# Julia REPL commands to solve AoC Task 07

f = open("07/input.txt")
lines = readlines(f)

currentpath = String[]

function parsecommand!(line, currentpath)
    if startswith(line, "\$ cd")
        dir = SubString(line, 6)
        if dir == ".."
            pop!(currentpath)
        elseif dir == "/"
            currentpath = ["/"]
        else
            push!(currentpath, dir)
        end
    end
end

map(x -> parsecommand!(x, currentpath), lines)

currentpath