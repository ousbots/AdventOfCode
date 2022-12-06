package main

// Standard libraries
import "bufio"
import "log"
import "os"

// parseFile(filename) parses the given file into a list of strings. Returns any errors.
func parseFile(filename string) ([][]rune, error) {
	file, err := os.Open(filename)
	if err != nil {
		log.Printf("Error opening file: %s", err)
		return [][]rune{}, err
	}
	defer file.Close()

	var boxIDs [][]rune

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		boxIDs = append(boxIDs, []rune(line))
	}

	return boxIDs, nil
}

// checksum(boxIDs) iterates over boxIDs and calculates the checksum based on the rules
// from the Advent of Code Day 2 event: https://adventofcode.com/2018/day/2
func checksum(boxIDs [][]rune) uint {
	var twice, thrice uint
	var letterMap map[rune]uint

	for _, boxID := range boxIDs {
		letterMap = map[rune]uint{}

		for _, letter := range boxID {
			count, _ := letterMap[letter]
			letterMap[letter] = count + 1
		}

		var twiceBool, thriceBool bool
		for _, count := range letterMap {
			switch {
			case count == 2 && !twiceBool:
				twice = twice + 1
				twiceBool = true

			case count == 3 && !thriceBool:
				thrice = thrice + 1
				thriceBool = true
			}
		}
	}

	return twice * thrice
}

// commonBoxIDLetters(boxIDs) parses boxIDs and returns the common letters between the
// two correct boxes based on the rules from the Advent of Code Day 2 event:
// https://adventofcode.com/2018/day/2
func commonBoxIDLetters(boxIDs [][]rune) string {
	for index, boxID := range boxIDs {
		for compIndex, compBoxID := range boxIDs {
			if index == compIndex {
				continue
			}

			var badCount uint
			var badIndex int

			for letterIndex, _ := range boxID {
				if boxID[letterIndex] != compBoxID[letterIndex] {
					badCount++
					badIndex = letterIndex

					if badCount > 1 {
						break
					}
				}
			}

			if badCount <= 1 {
				return string(append(boxID[:badIndex], boxID[badIndex+1:]...))
			}
		}

		boxIDs = append(boxIDs[:index], boxIDs[index+1:]...)
	}

	return ""
}

// run(filename) calls the functions to parse the input and calculate the checksum and
// the correct boxIDs
func run(filename string) error {
	boxIDs, err := parseFile(filename)
	if err != nil {
		return err
	}

	checksum := checksum(boxIDs)
	log.Printf("List checksum: %d", checksum)

	commonLetters := commonBoxIDLetters(boxIDs)
	log.Printf("Common letters between correct IDs: %s", commonLetters)

	return nil
}

// main() sets up logging, parses the command line arguments and calls run().
func main() {
	log.SetFlags(log.Lshortfile)

	args := os.Args[1:]
	if len(args) != 1 {
		log.Printf("Error: expected only a filename")
		return
	}

	err := run(args[0])
	if err != nil {
		log.Fatalf("Exiting due to error: %s", err)
	}
}
