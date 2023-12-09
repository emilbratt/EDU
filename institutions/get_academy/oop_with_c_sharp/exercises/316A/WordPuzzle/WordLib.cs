namespace WordPuzzle;

class WordLib
{
    private static bool CheckOverlap(int length, string first_word, string last_word)
    {
        // trying to implement an efficient algorithm to check if two words overlap

        // instead of loading a substring with N length for two words and compare these,
        // we instead index the characters at the overlapping point stargin from left

        // if the characters compare, move to the next character for both words and compare these
        // do this until we are at the last character of the first word

        // at any point if the characters do not compare, return false
        // return true if the opposite is the case

        // NOTE: the starting index for the first word is not the first character
        int index_shift_right = first_word.Length - length;

        int i = 0;
        while (i < length)
        {
            // this avoids potential "System.IndexOutOfRangeException"..
            if (index_shift_right < 0) return false;

            // characters do not match, no need to continue..
            if (last_word[i] != first_word[index_shift_right + i]) return false;

            i++;
        }

        // both words satisfies the length of the overlap
        return true;
    }

    public static int CountLongestOverlap(int max, int min, string first_word, string last_word)
    {
        // returns the length of the longest overlap between two words
        // if no overlap, return 0

        // max = start from here, this is the longest overlap that we try to find first
        // min = end here, this is the shortest overlap evaulated if we cant find any larger overlap

        while (max > min)
        {
            if (CheckOverlap(max, first_word, last_word)) return max;
            max--;
        }

        return 0;
    }

    public static int CountShortestOverlap(int min, int max, string first_word, string last_word)
    {
        // returns the length of the shortest overlap between two words
        // if no overlap, return 0

        // min = start from here, this is the shortest overlap that we try to find first
        // max = end here, this is the longest overlap evaulated, if we cant find any shorter overlap

        while (min <= max)
        {
            if (CheckOverlap(min, first_word, last_word)) return min;
            min++;
        }

        return 0;
    }

    public static string ExtractLongestOverlappingPart(string first_word, string last_word)
    {
        int shortest_word_length = last_word.Length;
        if (first_word.Length < last_word.Length) shortest_word_length = first_word.Length;

        int overlap_length = CountLongestOverlap(shortest_word_length, 1, first_word, last_word);

        return first_word.Substring(first_word.Length - overlap_length, overlap_length);
    }

    public static string ExtractShortestOverlappingPart(int min, string first_word, string last_word)
    {
        int shortest_word_length = last_word.Length;
        if (first_word.Length < last_word.Length) shortest_word_length = first_word.Length;

        int overlap_length = CountShortestOverlap(min, shortest_word_length, first_word, last_word);

        return first_word.Substring(first_word.Length - overlap_length, overlap_length);
    }
}
