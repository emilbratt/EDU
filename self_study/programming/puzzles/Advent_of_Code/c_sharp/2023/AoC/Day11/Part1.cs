namespace AoC.Day11;

class Part1
{

    public static string Run(string puzzle_input)
    {
        // System.Diagnostics.Debug.Assert(false);
        string[] lines = puzzle_input.Split('\n', StringSplitOptions.RemoveEmptyEntries);

        (int row, int col)[] galaxy_coordinates = RegisterGalaxies(lines);

        // any rows or columns containing no galaxies count as two rows or columns respectively
        (int[] double_row_width, int[] double_col_width) = RegisterDoubleWidth(lines);

        long res = SumOfLengthForEveryPair(galaxy_coordinates, double_row_width, double_col_width);
        return res.ToString();
    }

    private static (int[] rows, int[] cols) RegisterDoubleWidth(string[] matrix)
    {
        // for empty rows and empty collumns (no galaxies),
        // ..the space is twice as large so we preserve these rows+collumns for use later

        List<int> double_row_width = [];
        List<int> double_col_width = [];

        int row_count = matrix.Length;

        // i and j are swapped with row and col respectively when used inside the while blocks
        int i = 0;
        while (i < row_count)
        {
            int col_count = matrix[i].Length;

            // this algorithm expects an equal amount of rows and collumns
            // and we should be provided (as far as I know) with an "n x n" square-grid of input data
            System.Diagnostics.Debug.Assert(row_count == col_count);

            bool no_galaxy_in_vertical_space = true;
            bool no_galaxy_in_horizontal_space = true;

            int j = 0;
            while (j < col_count)
            {
                // we check the horizontal lane using i as row and j as collumn
                if (matrix[i][j] == '#') no_galaxy_in_horizontal_space = false;

                // we check the vertical lane using j as row and i as collumn
                if (matrix[j][i] == '#') no_galaxy_in_vertical_space = false;

                j++;
            }

            // add the row represented as 'i'
            if (no_galaxy_in_horizontal_space) double_row_width.Add(i);

            // add the column represented as ..also 'i'
            if (no_galaxy_in_vertical_space) double_col_width.Add(i);

            i++;
        }

        return (double_row_width.ToArray(), double_col_width.ToArray());
    }

    private static (int row, int col)[] RegisterGalaxies(string[] matrix)
    {
        List<(int row, int col)> galaxies = [];

        int row = 0;
        while (row < matrix.Length)
        {
            int col = 0;
            while (col < matrix[row].Length)
            {
                if (matrix[row][col] == '#') galaxies.Add( (row, col) );
                col++;
            }
            row++;
        }

        return galaxies.ToArray();
    }

    private static long SumOfLengthForEveryPair((int row, int col)[] galaxies, int[] dble_w_row, int[] dble_w_col)
    {
        // given galxies { a, b, c, d, e } we iterate over every pair like so
        // -> (ab), (ac), (ad), (ae)
        // -> (bc), (bd), (be)
        // -> (cd), (ce)
        // -> (de)

        long sum = 0;

        int i = 0;
        while (i < galaxies.Length)
        {
            int j = i+1;
            while (j < galaxies.Length)
            {
                (int row, int col) g1 = galaxies[i];
                (int row, int col) g2 = galaxies[j];

                int distance = 0;

                // g2 will only be on the same row or the one below (+1 row)
                while (g1.row < g2.row)
                {
                    distance += dble_w_row.Contains(g1.row) ? 2 : 1;
                    g1.row++;
                }
                // ..however, it may be on a column that is either before (-1) or after (+1)
                while (g1.col < g2.col)
                {
                    distance += dble_w_col.Contains(g1.col) ? 2 : 1;
                    g1.col++;
                }
                while (g1.col > g2.col)
                {
                    distance += dble_w_col.Contains(g1.col) ? 2 : 1;
                    g1.col--;
                }

                sum += distance;
                j++;
            }

            i++;
        }

        return sum;
    }
}
