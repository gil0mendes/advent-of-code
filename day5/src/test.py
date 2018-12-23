from functools import reduce

def trigger(x, y):
    return False if not x else abs(ord(x[-1]) - ord(y)) == 32

def react(polymer):
    return reduce((lambda x, y: x[:-1] if trigger(x, y) else x+y), polymer)

polymer = open('input.txt').read()
print(len(react(polymer)))
print(min([len(react(polymer.replace(chr(i), '').replace(chr(i-32), ''))) for i in range(ord('a'), ord('z') + 1)]))

