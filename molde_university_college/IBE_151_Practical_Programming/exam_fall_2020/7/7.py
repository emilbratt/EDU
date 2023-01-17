

def magPies(all: tuple, new: str, size: int):

    '''
    function receives a list of tuples,
    the name of a new magpie and its size
    function updates the list by placing the
    name and size of a tuple in the correct position

    In case of a tie, the last magpie to arrive
    must be placed last among those with the same size.
    '''

    # we need to append name and the size of new magpie into a tuple
    new = (new,size)

    # lets append the tuple to the list
    all.append(new)


    size = 1 # we set the magpie that we want to compare

    for magpie in range(1,len(all)):
        # lets grab the size from current iteration starting on the 2nd
        # magpie and moving incrementally 1 by 1
        curVal = all[magpie][size]

        # compare the size with all the values currently on its left
        while all[magpie-1][size] > curVal and magpie > 0:
            # swap the place while magpie is larger
            all[magpie], all[magpie-1] = all[magpie-1], all[magpie]
            magpie -= 1

    return all


# all = [("Timon", 5.6), ("Bluetooth", 7)]
# new = ("Nono", 3.6)

# all = [("Timon", 5.6), ("Bluetooth", 7)]
# new = "Macbeth"
# size = 6


# all = [("Timon", 5.6), ("Bluetooth", 7)]
# new = "Dumbo"
# size = 8

all = [("Timon", 5.6), ("Bluetooth", 7)]
new =  "Pumba"
size = 5.6

print(magPies(all,new, size))
