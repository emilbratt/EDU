def returnLongest():
    '''
        reads a file and returns the longest word in the file
        assumes that the file contains no punctuation marks
        in case of a tie, the function returns the longest word that appears last in the file
    '''
    try:
        loadFile = open('nytimes.txt',encoding='utf-8')

    except FileNotFoundError:
        print('sorry, we got ourself a FileNotFoundError')
        return None


    # lets extract each line and store as a list-object
    allLines = []
    for line in loadFile:
        allLines.append(line[:-1]) # [:-1] removes '\n' in every line


    # lets filter separate each word from each line
    # create an empty list
    allWords = []
    for line in allLines:
        word = '' # assign an empty string variable

        # loop through each character and add character to the word
        for i in range(len(line)):
            if line[i] != ' ':
                word += line[i]

            # when whitespace or end of the list, append to list
            if i+1 == len(line) or line[i] == ' ':
                 allWords.append(word)
                 word = ''


    # now we count letters in each word
    counter = 0
    longestWord = None
    for word in allWords:
        # if a word with greater or same length
        # replace the previous word
        if len(word) >= counter:
            counter = len(word)
            longestWord = word


    return longestWord


print(returnLongest())
