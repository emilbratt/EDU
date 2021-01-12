
def countLetter(char,wordList: list):
    '''
        takes a character and a list of words as input
        returns how many times the character is found in the words in the list
        words with one letter is not concidered
    '''

    # we declare a counter starting from zero
    counter = 0


    # well iterate through each word
    for word in wordList:
        # skip any word with 1 character
        if len(word) == 1:
            continue

        # if word passed the first check, lets add each word as +1 to counter
        for c in word:
            if c == char:
                counter += 1
                
    return counter

sampleText = [
'Joseph', 'Frank', 'Keaton,', 'known',
'professionally', 'as', 'Buster', 'Keaton,',
'was', 'an', 'American', 'actor,', 'comedian,',
'film', 'director,', 'producer,', 'screenwriter,',
'and', 'stunt', 'performer.', 'He', 'is', 'best',
'known', 'for', 'his', 'silent', 'films,', 'in',
'which', 'his', 'trademark', 'was', 'physical',
'comedy', 'with', 'a', 'consistently', 'stoic,',
'deadpan', 'expression', 'that', 'earned', 'him',
'the', 'nickname', "'The", 'Great', 'Stone', "Face'."
]

print(countLetter('s',sampleText))
