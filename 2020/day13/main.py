#!/usr/bin/env python3


def main():
    with open("input") as f:
        timestamp = int(f.readline().strip())
        busses = f.readline().strip().split(",")
        filtered = [int(bus) for bus in busses if bus != "x"]
    part_one(timestamp, filtered)
    part_two(busses)


def part_one(timestamp, busses):
    closest_departures = {}
    for bus in busses:
        counter = 1
        while (departure := bus * counter) < timestamp:
            counter += 1
        else:
            closest_departures[departure] = bus
    closest_departure = min(closest_departures)
    print((closest_departure - timestamp) * closest_departures[closest_departure])


def part_two(busses):
    bus_time_with_offset = [(int(bus.strip()), n) for n, bus in enumerate(busses) if bus != "x"]
    timestamp = 0
    step = 1
    for bus, offset in bus_time_with_offset:
        while (timestamp + offset) % bus != 0:
            timestamp += step
        step *= bus
    print(timestamp)


if __name__ == '__main__':
    main()
