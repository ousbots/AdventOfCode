package main

// Standard libraries
import "bufio"
import "errors"
import "log"
import "os"
import "strconv"

const base10 = 10
const bitSize = 64

// parseFile(filename) parses the given file into a list of int64. The list reflects
// the change in frequency in the given file (i.e. +1 -> 1, -1 -> -1). Returns any
// generated errors.
func parseFile(filename string) ([]int64, error) {
	file, err := os.Open(filename)
	if err != nil {
		log.Printf("Error opening file: %s", err)
		return []int64{}, err
	}
	defer file.Close()

	var changes []int64

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		oper := string(line[0])
		num, err := strconv.ParseUint(line[1:], base10, bitSize)
		if err != nil {
			log.Printf("Error converting number: %s", err)
			return []int64{}, err
		}

		switch oper {
		case ("+"):
			changes = append(changes, int64(num))

		case "-":
			changes = append(changes, -int64(num))

		default:
			log.Printf("Bad line encountered: %s", line)
			return []int64{}, errors.New("Bad line encountered")
		}
	}

	return changes, nil
}

// run(filename) calls parseFile() then iterates over the results calculating the final
// frequency as well as the calibration frequency based on the rules of Day 1 of the
// Advent of Code event: https://adventofcode.com/2018/day/1
func run(filename string) error {
	changes, err := parseFile(filename)
	if err != nil {
		return err
	}

	var freq int64
	var finalFreq int64
	var foundFreq bool
	var calibFreq int64
	var foundCalib bool
	var freqMap = map[int64]struct{}{0: struct{}{}}

	var loopCount uint64
	for !foundCalib {
		loopCount = loopCount + 1

		for _, change := range changes {
			freq = freq + change

			_, exists := freqMap[freq]
			if !exists {
				freqMap[freq] = struct{}{}
			} else {
				if !foundCalib {
					calibFreq = freq
					foundCalib = true

					if foundFreq {
						break
					}
				}
			}
		}

		if !foundFreq {
			finalFreq = freq
			foundFreq = true
		}
	}

	log.Printf("Final frequency: %d", finalFreq)
	log.Printf("Calibration frequency: %d", calibFreq)
	log.Printf("Calibration required %d loops", loopCount)

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
