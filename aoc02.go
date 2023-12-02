package main

import (
	"os"
	"strconv"
	"strings"
)

func part1() {
	sum := 0
	bytes, _ := os.ReadFile("aoc02.txt")
	lines := strings.Split(string(bytes), "\n")

	for _, line := range lines {
		split := strings.Split(line, ":")
		id, _ := strconv.Atoi(strings.Split(split[0], " ")[1])
		
		max_blue := 0
		max_red := 0
		max_green := 0

		sets := strings.Split(split[1], ";")
		for _, set := range sets {
			set_blue := 0
			set_red := 0
			set_green := 0
			cubes := strings.Split(set, ",")
			for _, cube := range cubes {
				cube_tokens := strings.Split(cube, " ")
				cube_count, _ := strconv.Atoi(cube_tokens[1])
				cube_color := cube_tokens[2]
				switch cube_color {
					case "blue": set_blue += cube_count
					case "red":	set_red += cube_count
					case "green": set_green += cube_count
					default: panic(cube_color)	
				}
			}
			if set_green > max_green { max_green = set_green }
			if set_red > max_red { max_red = set_red }
			if set_blue > max_blue { max_blue = set_blue }
		}
		if max_blue <= 14 && max_green <= 13 && max_red <= 12 {
			sum += id
		}
	}

	println(sum)
}

func part2() {
	sum := 0
	bytes, _ := os.ReadFile("aoc02.txt")
	lines := strings.Split(string(bytes), "\n")

	for _, line := range lines {
		split := strings.Split(line, ":")
		max_blue := 0
		max_red := 0
		max_green := 0

		sets := strings.Split(split[1], ";")
		for _, set := range sets {
			set_blue := 0
			set_red := 0
			set_green := 0
			cubes := strings.Split(set, ",")
			for _, cube := range cubes {
				cube_tokens := strings.Split(cube, " ")
				cube_count, _ := strconv.Atoi(cube_tokens[1])
				cube_color := cube_tokens[2]
				switch cube_color {
					case "blue": set_blue += cube_count
					case "red":	set_red += cube_count
					case "green": set_green += cube_count
					default: panic(cube_color)	
				}
			}
			if set_green > max_green { max_green = set_green }
			if set_red > max_red { max_red = set_red }
			if set_blue > max_blue { max_blue = set_blue }
		}
		sum += max_blue * max_green * max_red
	}

	println(sum)
}

func main() {
	part1()
	part2()
}