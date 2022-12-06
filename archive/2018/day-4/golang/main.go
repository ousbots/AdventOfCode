package main

// Standard libraries
import "bufio"
import "fmt"
import "log"
import "os"
import "sort"
import "strconv"
import "strings"

// "enum" for describing the event type
type eventType uint

const (
	start eventType = iota
	sleep
	wake
)

// convenience type
type sleepMapType map[uint64]map[uint64]uint64

// record struct represents a single record from the input
type record struct {
	id       uint64
	fullDate string
	date     string
	hour     uint64
	minute   uint64
	event    eventType
}

// Constants
const dateAfterSep = "]"
const dateCutSel = "[]"
const dateSplitLen = 2
const space = " "
const timeSplit = ":"
const wakeEvent = "wakes up"
const sleepEvent = "falls asleep"
const idEvent = "Guard"
const idCutSel = "Guard#beginshft "
const base = 10
const bitSize = 64
const watchHour = 0

// sorter struct is used to fulfill the sort.Interface
type sorter struct {
	records []record
	sort    func(first, second *record) bool
}

// Len() is a sorter method to fulfill the sort.Interface
func (self sorter) Len() int {
	return len(self.records)
}

// Swap() is a sorter method to fulfill the sort.Interface
func (self sorter) Swap(first, second int) {
	self.records[first], self.records[second] = self.records[second], self.records[first]
}

// Less() is a sorter method to fulfill the sort.Interface
func (self sorter) Less(first, second int) bool {
	return self.sort(&self.records[first], &self.records[second])
}

// sortByDate(first, second) compares the date of two records and returns a bool
// describing the sort relation between the two
func sortByDate(first, second *record) bool {
	return first.fullDate < second.fullDate
}

// parseFile(filename) parses the given file into a list of records. Returns any errors.
func parseFile(filename string) ([]record, error) {
	file, err := os.Open(filename)
	if err != nil {
		log.Printf("Error opening file: %s", err)
		return nil, err
	}
	defer file.Close()

	var records []record

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		var record record

		dateSplit := strings.SplitAfter(line, dateAfterSep)
		if len(dateSplit) != dateSplitLen {
			log.Printf("Strange record found: %s", line)
			continue
		}

		dates := strings.TrimSpace(strings.Trim(dateSplit[0], dateCutSel))
		record.fullDate = dates

		dateElems := strings.Split(dates, space)
		if len(dateElems) != dateSplitLen {
			log.Printf("Strange date found: %s", dates)
			continue
		}

		record.date = strings.TrimSpace(dateElems[0])

		time := strings.Split(dateElems[1], timeSplit)
		if len(time) != dateSplitLen {
			log.Printf("Strange time found: %s", time)
			continue
		}

		record.hour, err = strconv.ParseUint(strings.TrimSpace(time[0]), base, bitSize)
		if err != nil {
			log.Printf("Error converting hour: %s", err)
			return nil, err
		}

		record.minute, err = strconv.ParseUint(strings.TrimSpace(time[1]), base, bitSize)
		if err != nil {
			log.Printf("Error converting minute: %s", err)
			return nil, err
		}

		event := strings.TrimSpace(dateSplit[1])

		switch event {
		case wakeEvent:
			record.event = wake

		case sleepEvent:
			record.event = sleep

		default:
			record.event = start
			idString := strings.Trim(event, idCutSel)

			record.id, err = strconv.ParseUint(strings.TrimSpace(idString), base, bitSize)
			if err != nil {
				log.Printf("Error converting ID: %s", err)
				return nil, err
			}
		}

		records = append(records, record)
	}

	sorter := sorter{
		records: records,
		sort:    sortByDate,
	}

	sort.Sort(sorter)

	return records, nil
}

// sleepMap(records) parses the list of records and generates a map of IDs to a map of
// minutes to sleep frequency.  See the Advent of Code day-4 event for more details:
// https://adventofcode.com/2018/day/4
func sleepMap(records []record) sleepMapType {
	sleepMap := make(sleepMapType)

	if len(records) == 0 {
		return sleepMap
	}

	currentID := records[0].id
	lastRecord := records[0]

	for _, record := range records {
		switch record.event {
		case start:
			if lastRecord.event != wake {
				minuteMap, exists := sleepMap[currentID]
				if !exists {
					minuteMap = make(map[uint64]uint64)
					sleepMap[currentID] = minuteMap
				}

				for minute := lastRecord.minute; minute < record.minute; minute++ {
					minuteMap[minute] = minuteMap[minute] + uint64(1)
				}

				sleepMap[currentID] = minuteMap
			}

			currentID = record.id
			lastRecord = record

		case wake:
			// XXX: only covers easy situation
			if record.hour == watchHour {
				if lastRecord.hour != watchHour {
					lastRecord.minute = 0
				}

				minuteMap, exists := sleepMap[currentID]
				if !exists {
					minuteMap = make(map[uint64]uint64)
					sleepMap[currentID] = minuteMap
				}

				for minute := lastRecord.minute; minute < record.minute; minute++ {
					minuteMap[minute] = minuteMap[minute] + uint64(1)
				}

				sleepMap[currentID] = minuteMap
			}

			lastRecord = record

		case sleep:
			lastRecord = record

		default:
			log.Printf("Bad event type encountered: %v", record)
		}
	}

	return sleepMap
}

// mostSleep(sleepMap) parses the sleep map and solves for Strategy 1 of the Advent of
// Code day 4 puzzle: https://adventofcode.com/2018/day/4
func mostSleep(sleepMap sleepMapType) (uint64, uint64) {
	var mostID, mostCount uint64

	for ID, minuteMap := range sleepMap {
		var totalMinutes uint64

		for _, count := range minuteMap {
			totalMinutes += count
		}

		if totalMinutes > mostCount {
			mostCount = totalMinutes
			mostID = ID
		}
	}

	minuteMap, _ := sleepMap[mostID]

	var mostMinute uint64
	mostCount = 0

	for minute, count := range minuteMap {
		if count > mostCount {
			mostCount = count
			mostMinute = minute
		}
	}

	return mostID, mostMinute
}

// mostLikelyAsleep(sleepMap) parses the sleep map and solves for Strategy 1 of the
// Advent of Code day 4 puzzle: https://adventofcode.com/2018/day/4
func mostLikelyAsleep(sleepMap sleepMapType) (uint64, uint64) {
	type likelySleep struct {
		minute uint64
		count  uint64
	}

	likelySleepMap := make(map[uint64]likelySleep)

	for ID, minuteMap := range sleepMap {
		likelySleepMap[ID] = likelySleep{}

		for minute, count := range minuteMap {
			sleep := likelySleepMap[ID]
			if count > sleep.count {
				sleep.count = count
				sleep.minute = minute
				likelySleepMap[ID] = sleep
			}
		}
	}

	var bestID uint64

	for ID, sleep := range likelySleepMap {
		if sleep.count > likelySleepMap[bestID].count {
			bestID = ID
		}
	}

	return bestID, likelySleepMap[bestID].minute
}

// visualize(sleepMap) prints a visualization of the sleep map
func visualize(sleepMap sleepMapType) {
	var topLine, bottomLine string
	topLine = "\t\t"
	bottomLine = "ID#\t"

	for tenCount := 0; tenCount < 6; tenCount++ {
		for count := 0; count < 10; count++ {
			topLine += fmt.Sprintf("%d  ", tenCount)
			bottomLine += fmt.Sprintf("%d  ", count)
		}
	}

	log.Printf(topLine)
	log.Printf(bottomLine)
	log.Printf("\n")

	for ID, minuteMap := range sleepMap {
		line := fmt.Sprintf("#%d\t", ID)
		var totalMinutes uint64

		for minute := uint64(0); minute < 60; minute++ {
			value, exists := minuteMap[minute]
			if exists {
				if value > 9 {
					line += fmt.Sprintf("%d ", value)
				} else {
					line += fmt.Sprintf("%d  ", value)
				}

				totalMinutes += value

			} else {
				line += "0  "
			}
		}

		line += fmt.Sprintf("\t%d", totalMinutes)

		log.Println(line)
	}
}

// run(filename) calls the functions to parse the input and calculate the checksum and
// the correct boxIDs
func run(filename string) error {
	records, err := parseFile(filename)
	if err != nil {
		return err
	}

	sleepMap := sleepMap(records)
	visualize(sleepMap)

	ID, minute := mostSleep(sleepMap)
	log.Printf("Strategy 1 most likely to be asleep: Employee #%d at minute %d (%d)", ID,
		minute, ID*minute)

	ID, minute = mostLikelyAsleep(sleepMap)
	log.Printf("Strategy 2 most likely to be asleep: Employee #%d at minute %d (%d)", ID,
		minute, ID*minute)

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
