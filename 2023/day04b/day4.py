import re

with open('input.txt') as file:
  data = file.readlines()

points = 0 # For part 1
cards = 0 # For part 2

# Finds the matching numbers between numbers and winning numbers
def findMatchingNumbers(line):
  winningNumbers = set(re.findall('\d+', re.search(':(.*)[|]', line).group()))
  numbers = set(re.findall('\d+', re.search('[|](.*)', line).group()))
  return winningNumbers.intersection(numbers)

cachedCopies = dict()
# Finds the number of copies of scratchcards for a given scratch card
# Caches findings which significantly increases speed of algorithm
def findCopies(id, number):
  if number == 0:
    return 0

  total = number
  for copiedIndex,copiedLine in enumerate(data[id:id+number]):
    if copiedLine in cachedCopies:
      total += cachedCopies[copiedLine]
      continue

    matchingNumbers = findMatchingNumbers(copiedLine)
    copies = findCopies(id + copiedIndex + 1, len(matchingNumbers))
    cachedCopies[copiedLine] = copies
    total += copies

  return total

for index,line in enumerate(data):
  matchingNumbers = findMatchingNumbers(line)
  cards += 1
  if len(matchingNumbers) > 0:
    points += 2 ** (len(matchingNumbers) - 1)
    cards += findCopies(index + 1, len(matchingNumbers))

print(points)
print(cards)