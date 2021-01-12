# queen can reach all indexes in x row
# index queen can reach all y rows wih queens row index position

# since we only allow an odd number of x and y, we can assume that
# queen can always take on chessBoard[0][0] and on chessBoard[-1][-1]
# queen can reach first square and last square on row 1
# queen can reach 2nd position and 2nd last position on row 2
# queen can reach 3nd position and 3rd last position on row 3
# and so on.......



def queenReach(boardSize: int,opX: int,opY: int):
    '''
        receives:
        board length and height (N) must be odd number
        row and column where the opponent is placed
        function implies that queen resides on the center square of x and y
        returns True if the piece is reachable, False if not
    '''
    queenPos = [int((boardSize+0)/2),int((boardSize+0)/2)]
    opponentPos = [opX,opY]

    if boardSize%2 != 1:
        input('board must have an odd number of rows and collumns')


    # this checks if opponent is located to queens x or y directions
    # no more calculations needed if this is the case
    if int((boardSize+0)/2) == opY:
        return True

    if int((boardSize+0)/2) == opX:
        return True


    # create and check chess board to make things a bit easier
    chessBoard = []

    for row in range(boardSize):

        newRow = []
        for square in range(boardSize):

            newRow.append(square)

        chessBoard.append(newRow)

    # declare an index for last square in each row
    # just to make it easy to slice
    l = boardSize-1

    canTake = []
    for i,row in enumerate(chessBoard):

        canTake.append([row[i],row[i]])
        # canTake.append([row[i]],row[(len(row)-1)]])
        canTake.append([row[i],row[l-i]])

    # now that we have a list over queens reachable positions
    # we can easily check if there is a position
    # in opponents position that matches
    for take in canTake:
        if take == opponentPos:
            return True

    return False




print(queenReach(5,4,1,))
