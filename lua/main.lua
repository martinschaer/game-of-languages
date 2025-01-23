-- Lua code for reference

-- Three girls' ages product is 36, sum is the same as the house number in the
-- street in front. Only one clue is required to guess their ages: the oldest
-- plays piano.

local desired_product = 36
local solutions = {}

for x = 1, desired_product do
    for y = 1, desired_product do
        for z = 1, desired_product do
            local product = x * y * z
            if product > desired_product then
                break
            end
            if product == desired_product then
                local sorted = { x, y, z }
                table.sort(sorted)
                local key = table.concat(sorted, ',')
                solutions[key] = sorted
            end
        end
        if x * y > desired_product then
            break
        end
    end
end

for _, solution in pairs(solutions) do
    print(
        string.format('Ages: %s - House %d',
            table.concat(solution, ', '),
            solution[1] + solution[2] + solution[3]
        )
    )
end

-- =============================================================================
-- Now, let's play the game!

local instructions = {
    [1] = function() print('Walking') end,
    [2] = function() print('Running') end,
    [0] = function() print('Thank you for playing') end
}

::Play::
print('1: Walk, 2: Run, 0: End')
io.write('> ')
local input = io.read()
local instr = instructions[tonumber(input)]
if instr then
    instr()
else
    print('Invalid input')
end
if tonumber(input) ~= 0 then
    goto Play
end
