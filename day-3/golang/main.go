package main

// Standard libraries
import "bufio"
import "fmt"
import "log"
import "os"
import "strings"
import "strconv"

type claim struct {
	number uint64
	x      uint64
	y      uint64
	width  uint64
	height uint64
}

// Constants
const claimSep = "@"
const claimTrim = "# "
const dimSep = ":"
const xySep = ","
const sizeSep = "x"
const splitSize = 2
const base = 10
const bitSize = 64

// parseFile(filename) parses the given file into a list of claims. Returns any errors.
func parseFile(filename string) ([]claim, error) {
	file, err := os.Open(filename)
	if err != nil {
		log.Printf("Error opening file: %s", err)
		return nil, err
	}
	defer file.Close()

	var claims []claim

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		var claim claim

		claimSplit := strings.Split(line, claimSep)
		if len(claimSplit) != splitSize {
			log.Printf("Strange claim found: %s", line)
			continue
		}

		claim.number, err = strconv.ParseUint(strings.Trim(claimSplit[0], claimTrim),
			base, bitSize)
		if err != nil {
			log.Printf("Error converting string to number: %s", err)
			return nil, err
		}

		dimSplit := strings.Split(claimSplit[1], dimSep)
		if len(dimSplit) != splitSize {
			log.Printf("Strange dimensions found: %s", claimSplit[1])
			continue
		}

		xy := strings.Split(dimSplit[0], xySep)
		if len(xy) != splitSize {
			log.Printf("Strange XY found: %s", dimSplit[0])
		}

		claim.x, err = strconv.ParseUint(strings.TrimSpace(xy[0]), base, bitSize)
		if err != nil {
			log.Printf("Error converting string to number: %s", err)
			return nil, err
		}

		claim.y, err = strconv.ParseUint(strings.TrimSpace(xy[1]), base, bitSize)
		if err != nil {
			log.Printf("Error converting string to number: %s", err)
			return nil, err
		}

		sizes := strings.Split(dimSplit[1], sizeSep)
		if len(sizes) != splitSize {
			log.Printf("Strange sizes found: %s", dimSplit[1])
			continue
		}

		claim.width, err = strconv.ParseUint(strings.TrimSpace(sizes[0]), base, bitSize)
		if err != nil {
			log.Printf("Error converting string to number: %s", err)
			return nil, err
		}

		claim.height, err = strconv.ParseUint(strings.TrimSpace(sizes[1]), base, bitSize)
		if err != nil {
			log.Printf("Error converting string to number: %s", err)
			return nil, err
		}

		claims = append(claims, claim)
	}

	return claims, nil
}

// overlapMap(claims) process the claim slice and generates a map of the fabric
// corresponding to the number of claims present on a given coordinate.
func overlapMap(claims []claim) map[string]uint {
	// Use a string of "x,y" as the position "hash"
	overlapMap := make(map[string]uint)

	posWriter := func(x uint64, y uint64) {
		posString := fmt.Sprintf("%d,%d", x, y)
		value, _ := overlapMap[posString]
		overlapMap[posString] = value + 1
	}

	for _, claim := range claims {
		for xInc := uint64(0); xInc < claim.width; xInc++ {
			for yInc := uint64(0); yInc < claim.height; yInc++ {
				posWriter(claim.x+xInc, claim.y+yInc)
			}
		}
	}

	return overlapMap
}

// overlapArea(overlapMap) parses the overlapMap and returns the number of overlapping
// coordinates based on the rules from the Advent of Code Day 3 event:
// https://adventofcode.com/2018/day/2
func overlapArea(overlapMap map[string]uint) uint64 {
	var overlapCount uint64
	for _, value := range overlapMap {
		if value > 1 {
			overlapCount++
		}
	}

	return overlapCount
}

// claimFree(claim, overlapMap) uses the overlapMap to determine if a claim does not
// overlap any other claims.
//
// This loops through the claim list, then checks if every coordinate in a claim does
// not overlap with any other claims.
func freeClaims(claims []claim, overlapMap map[string]uint) []uint64 {
	posString := func(x uint64, y uint64) string {
		return fmt.Sprintf("%d,%d", x, y)
	}

	var freeClaims []uint64

	for _, claim := range claims {
		free := true

	claimLoop:
		for xInc := uint64(0); xInc < claim.width; xInc++ {
			for yInc := uint64(0); yInc < claim.height; yInc++ {
				value, _ := overlapMap[posString(claim.x+xInc, claim.y+yInc)]
				if value != 1 {
					free = false
					break claimLoop
				}
			}
		}

		if free {
			freeClaims = append(freeClaims, claim.number)
		}
	}

	return freeClaims
}

// run(filename) calls the functions to parse the input and ...
func run(filename string) error {
	claims, err := parseFile(filename)
	if err != nil {
		return err
	}

	overlapMap := overlapMap(claims)
	overlap := overlapArea(overlapMap)
	log.Printf("Overlapping coordinates: %d", overlap)

	freeClaims := freeClaims(claims, overlapMap)
	if len(freeClaims) == 0 {
		log.Printf("No claims free")
	}

	for _, claim := range freeClaims {
		log.Printf("Claim %d is free", claim)
	}

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
