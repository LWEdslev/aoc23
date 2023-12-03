function ReadAll(file)
    local f = assert(io.open(file, "rb"))
    local content = f:read("*all")
    f:close()
    return content
end

File = ReadAll("aoc03.txt")
Lines = {}
for line in string.gmatch(File, '([^\n]+)') do
    table.insert(Lines, line)
end

Matrix = {}

for i = 1, #(Lines[1]) do
    Matrix[i] = {}
    for j = 1, #Lines do
        Matrix[i][j] = '.'
    end
end

for y = 1, #Lines do
    for x = 1, #(Lines[y]) do
        if Lines[y]:sub(x, x) == '*' then
            Matrix[x][y] = '*'
        end
    end
end


local currentNumber = 0
local prev_was_digit = false
local number_start_pos = nil
local counter = 0
-- set all write numbers to matrix
for y = 1, #Lines do
    for x = 1, #(Lines[y]) do
        local lookup = Lines[y]:sub(x, x)
        if lookup:match("%d") then
            counter = counter + 1
            currentNumber = currentNumber*10 + tonumber(lookup)
            if not prev_was_digit then
                prev_was_digit = true
                number_start_pos = x
            end
        else if prev_was_digit then
            prev_was_digit = false
            for i = number_start_pos, number_start_pos + counter-1 do Matrix[i][y] = currentNumber end
            counter = 0
            currentNumber = 0
            end
        end
    end
end

function ProdIffOnlyTwo(matrix, x, y)
    local prod = 1
    local found = 0
    local positions = {{-1,-1}, {-1, 0}, {-1, 1}, {0, 1}, {0,-1}, {1, 1}, {1, 0}, {1, -1}}
    -- checking each edge/corner for a number
    for _, pos in ipairs(positions) do
        local posx = x + pos[1]
        local posy = y + pos[2]
        local in_bounds = (0 < posx and posx <= #matrix) and (0 < posy and posy <= #matrix[1])
        if in_bounds and type(matrix[posx][posy]) == 'number' then
            found = found + 1
            prod = prod * matrix[posx][posy]
            local offset = 0
            -- once we have found a number we remove the other instances of this number (so that we don't use the duplicates)
            while (posx + offset <= #matrix) and (type(matrix[posx+offset][posy]) == 'number') do
                matrix[posx+offset][posy] = '.'
                offset = offset + 1
            end
        end
    end

    if found == 2 then
        return prod
    else
        return 0
    end
end


Total = 0
for x = 1, #Matrix do
    for y = 1, #Matrix[x] do
        if Matrix[x][y] == '*' then -- if a star is found we check if there are two numbers around it 
           Total = Total + ProdIffOnlyTwo(Matrix, x, y)
        end
    end
end

print(Total)