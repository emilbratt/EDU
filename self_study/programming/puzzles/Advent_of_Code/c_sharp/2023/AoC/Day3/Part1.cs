namespace AoC.Day3;

class Part1
{
    private static string[]? _puzzle_input;
    private static int _last_row = 0;
    private static int _last_col = 0;

    public static string Run(string[] puzzle_input)
    {
        _puzzle_input = puzzle_input;

        // we treat the whole input as a matrix - each line is a row and each character is a column
        _last_row = _puzzle_input.Length - 1;
        _last_col = _puzzle_input[0].Length - 1; // each line has the exact same char count, we count first line

        return HandlePuzzleInput();
    }

    private static string HandlePuzzleInput()
    {
        if (_puzzle_input == null) Environment.Exit(1);

        int total = 0;
        for (int row = 0; row <= _last_row; row++)
        {
            int col = 0;
            while (col <= _last_col)
            {
                int total_digits = 0;
                int start_col = col;

                // this triggers on the first char that is a digit and ends on the last
                while (char.IsNumber(_puzzle_input[row][col]))
                {
                    total_digits++;
                    if (col == _last_col) break; // do not go to next col if this is the last col
                    col++;
                }

                // having any digits means we have a number, lets see if it is a valid part number before adding it
                if (total_digits > 0)
                {
                    string number = _puzzle_input[row].Substring(start_col, total_digits);
                    if (ValidPartNumber(row, start_col, start_col + total_digits)) total += int.Parse(number);
                } 

                col++;
            }
        }

        return total.ToString();
    }

    private static bool ValidPartNumber(int current_row, int from_col, int to_col)
    {
        if (_puzzle_input == null) Environment.Exit(1);

        for (int row = current_row - 1; row <= current_row + 1; row++)
        {
            if (row < 0 || row > _last_row) continue;

            for (int col = from_col - 1; col < to_col + 1; col++)
            {
                if (col < 0 || col > _last_col) continue;

                if (IsValidPart(_puzzle_input[row][col])) return true;
            }
        }

        return false;
    }

    private static bool IsValidPart(char c)
    {
        if (c == '.') return false; // periods makes invalid
        if (char.IsNumber(c)) return false; // if by chance, there is a number adjacent; thats invalid too

        // any other symbol makes it valid
        return true;
    }
}
