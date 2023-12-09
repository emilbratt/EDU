using System;

class Program
{
    static void Main()
    {
        int start = 1000;
        int end = 1028;
        int step = 7;

        int[] numbers = GenerateIntegers(start, end, step);

        Console.WriteLine("Generated array:");
        foreach (int n in numbers)
        {
            bool is_last_number = end - step < n;
            if (is_last_number) Console.Write($"{n}\n");
            else Console.Write($"{n} ");
        }
    }

    static int[] GenerateIntegers(int start, int end, int step)
    {
        // we need to know the size of the array, this calculation does the trick
        int arr_length = (end - start) / step + 1;

        // then create it and setting all indexes to the value of 0
        int[] arr_result = new int[arr_length];

        // finally, we swap out all the 0 values with the intended value
        int current_number = start;
        for (int index = 0; index < arr_length; index++)
        {
            arr_result[index] = current_number;
            current_number += step;
        }

        return arr_result;
    }
}
