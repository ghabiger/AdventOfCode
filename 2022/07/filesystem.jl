# Julia REPL commands to solve AoC Task 07

function updatedirmapsize!(currentpath, dirmap, size)
    path = join(currentpath, "/")
    if haskey(dirmap, path)
        dirmap[path] += size
    else
        dirmap[path] = size
    end
    pop!(currentpath)
    if !isempty(currentpath)
        updatedirmapsize!(currentpath, dirmap, size)
    end
end

function parsecommand!(line, currentpath, dirmap)
    if startswith(line, "\$ cd")
        dir = SubString(line, 6)
        if dir == ".."
            pop!(currentpath)
            if isempty(currentpath)
                currentpath = [" "]
            end
        elseif dir == "/"
            currentpath = [" "]
        else
            push!(currentpath, dir)
        end
    end
    if occursin(r"^\d+", line)
        size = parse(Int64, match(r"^(\d+) .*", line).captures[1])
        println("Trying to insert or add $size to path $currentpath")
        updatedirmapsize!(deepcopy(currentpath), dirmap, size)
    end
    return (currentpath, dirmap)
end

f = open("07/input.txt")
lines = readlines(f)

currentpath = String[]
dirmap = Dict{String, Int64}(())

for line in lines
    currentpath, dirmap = parsecommand!(line, currentpath, dirmap)
end

# Task 07.1
sum(values(filter(x -> last(x) <= 100000, dirmap)))

# Task 07.2
totalspace = 70000000
reqspace = 30000000
occspace = dirmap[" "]
unusedspace = totalspace - occspace
delsize = reqspace - unusedspace

possiblefolders = filter(x -> last(x) > delsize, dirmap)
possiblefolders[reduce((x, y) -> possiblefolders[x] ≤ possiblefolders[y] ? x : y, keys(possiblefolders))]