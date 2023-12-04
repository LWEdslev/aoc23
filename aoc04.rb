def part1
    file = File.read("aoc04.txt")
    lines = file.split("\n")

    sum = 0

    lines.each do |line|
        cards = line.split(":")[1].split("|")
        card1 = cards[0].split(" ").map(&:to_i)
        card2 = cards[1].split(" ").map(&:to_i)
        intersection = card1 & card2
        if intersection.length >= 1 then
            sum += 2**(intersection.length-1)
        end
    end

    puts sum
end

def part2
    file = File.read("aoc04.txt")
    lines = file.split("\n")

    ownedcards = []
    totalowned = 0

    for i in 0...lines.length
        ownedcards[i] = 1
        totalowned += 1
    end

    for i in 0...lines.length
        cards = lines[i].split(":")[1].split("|")
        card1 = cards[0].split(" ").map(&:to_i)
        card2 = cards[1].split(" ").map(&:to_i)

        intersection_length = (card1 & card2).length
        for addOneTo in i+1..(i+intersection_length)
            ownedcards[addOneTo] += ownedcards[i]
            totalowned += ownedcards[i]
        end
    end

    puts totalowned
end

part1
part2