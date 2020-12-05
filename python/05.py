import heapq


class Seat:
    def __init__(self, code):
        self.row = self._parse_row(code[:-3])
        self.col = self._parse_col(code[-3:])
        self.column = 0
        self.seat_id = self.row * 8 + self.col

    def _parse_row(self, code):
        """
        Parse first 7 characters to row number

        :param code: in binary encoded form
        :return: the code in decimal

        >>> assert(Seat("FBFBBFFRLR").row == 44)
        """
        result = 0
        for ch in code:
            curr = 1 if ch == 'B' else 0
            result = (2 * result + curr)
        return result

    def _parse_col(self, code):
        """
        Parse last 3 characters to col number

        :param code: in binary encoded form
        :return: the code in decimal

        >>> assert(Seat("FBFBBFFRLR").col == 5)
        """
        result = 0
        for ch in code:
            curr = 1 if ch == 'R' else 0
            result = (2 * result + curr)
        return result


if __name__ == '__main__':
    f = open('../input/05.txt', 'r')

    heap = []
    for line in f:
        seat = Seat(line.strip())
        heapq.heappush(heap, -seat.seat_id)
    largest = -heapq.heappop(heap)
    print(f"Largest seat ID: {largest}")

    while len(heap) > 0:
        curr = -heapq.heappop(heap)
        if abs(largest - curr) > 1:
            break
        largest = curr

    print(f"My seat ID: {largest - 1}")
