with open('data/input') as f:
    inp = f.read().split("\n\n")
    state = inp[0].rstrip().split("\n")
    instructions = inp[-1].strip().split("\n")

for y in range(int(state[-1][-1])):
    for x in range(len(state)-1):
        
