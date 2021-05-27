
def PosDiv(N: int):
    '''
        this function takes an integer input
        outputs True or False
        depending on if the input integer is
        divisable with 10 and divisable with 2
    '''
    # using the modulo operator to count remainder
    # a remainder of 0 means the number is divisable
    # so if number is divisable with 10 and divisable with 2 we
    # express this with modulo 10 and modulo 2 will
    return N % 10 == 0 and N % 2 == 0

print(PosDiv(10))
