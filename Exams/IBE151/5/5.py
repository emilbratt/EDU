def fortuneWheel(prizes: list, spins: int):

    # do not allow a negative number
    if spins < 0:
        print('No negative numbers allowed')
        return None
    '''
        feed this function with a list of prizes and a number
        that represents the total "spins"
        the function will loop through the list of prizes
        and each time the loop reaches the last prize,
        the loop starts from the first prize

        the function returns the prize that the wheels
        position ends up on
    '''


    # we`ll set a position indicator for the wheel
    position = 1

    # we start spinning and subtract one from spins for each loop
    # until we have no more spins left
    while spins != 0:
        spins -= 1

        # if the position exceeds the length of the list
        # we have to reset the position to one
        if position == len(prizes):
            position = 1
        else:
            position += 1


    # we will have to subtract 1 from the position
    # because the list indexes starts from 0 and not 1
    return prizes[position-1]



prizes = ["Candy", "Chocolate","Apple","Carrot"]
# prizes = ["Trip", "Car","House"]
print(fortuneWheel(prizes,4))
