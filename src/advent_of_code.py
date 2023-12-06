import re

pairings = {
	"one": 1,
	"two": 2,
	"three": 3,
	"four": 4,
	"five": 5,
	"six": 6, 
	"seven": 7,
	"eight": 8,
	"nine": 9
}

def sum_calibration():
	with open("input.txt") as file:
		s = 0
		for line in file: 
			written_nums = {}
			for key in pairings.keys():
				written_num_indices = [m.start() for m in re.finditer(key, line)]
				for i in written_num_indices:
					written_nums[i] = key
			for i in sorted(written_nums.keys(), reverse=True):
				line = line[:i] + str(pairings[written_nums[i]]) + line[i:]
			nums = re.findall(r'\d', line)
			s += int(nums[0] + nums[len(nums)-1])
		return s

s = sum_calibration()
print(s)