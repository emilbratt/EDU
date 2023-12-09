namespace PasswordGenerator;
class Program
{
    private readonly static Random _random = new();
    private static string _generated_password = "";
    private static readonly string _allowed_letters = "lLsd";
    private static readonly string _lower_case_alphabet = "abcdefghijklmopqrstuvwxyz";
    private static readonly string _upper_case_alphabet = "ABCDEFGHIJKLMOPQRSTUVWXYZ";
    private static readonly string _special_characters = "(!\"#\u00a4%&/(){}[]";

    static void Main(string[] args)
    {
        if (args.Length != 2) AppErrExit();

        int pwd_length = ValidateFirstArgument(args[0]);
        string pwd_options = ValidateSecondArgument(args[1]);

        ValidateLengthToOptions(pwd_length, pwd_options);

        Console.WriteLine($"Length: {pwd_length}");
        Console.WriteLine($"Options: {pwd_options}");

        bool pwd_opt_satisfied = false;

        while (!pwd_opt_satisfied)
        {
            pwd_opt_satisfied = GeneratePassword(pwd_length, pwd_options);
        }

        Console.WriteLine($"\nGenerated password: {_generated_password}");
    }

    private static bool GeneratePassword(int pwd_length, string pwd_options)
    {
        _generated_password = "";
        bool[] arr_options_satisfied = new bool[pwd_options.Length];
        int round = 1;

        while (pwd_length > 0)
        {
            // pick a random option from the 2nd agrument
            int char_index = _random.Next(0, pwd_options.Length);
            char pwd_option = pwd_options[char_index];

            // set the option as satisfied
            arr_options_satisfied[char_index] = true;

            _generated_password += GetCharacter(pwd_option);
            round ++;
            pwd_length--;
        }

        foreach (bool pwd_opt_satisfied in arr_options_satisfied)
        {
            if (!pwd_opt_satisfied) return false;
        }

        // reaching this far means we successfully generated a password
        return true;
    }

    private static char GetCharacter(char pwd_option)
    {
        return pwd_option switch
        {
            'L' => GetRandomUpperCaseCharacter(),
            'l' => GetRandomLowerCaseCharacter(),
            's' => GetRandomSpecialCharacter(),
            'd' => GetRandomDigitCharacter(),

            // if none above -> default (..just for runtime safety)
            _ => GetRandomLowerCaseCharacter(),
        };
    }

    private static char GetRandomLowerCaseCharacter()
    {
        int index = _random.Next(0, _lower_case_alphabet.Length);
        return _lower_case_alphabet[index];
    }

    private static char GetRandomUpperCaseCharacter()
    {
        int index = _random.Next(0, _upper_case_alphabet.Length);
        return _upper_case_alphabet[index];
    }

    private static char GetRandomSpecialCharacter()
    {
        int index = _random.Next(0, _special_characters.Length);
        return _special_characters[index];
    }

    private static char GetRandomDigitCharacter()
    {
        int random_number = _random.Next(0, 10);
        // NOTE: we have to add the random number to the ASCII value of '0' to convert to char
        return (char)('0' + random_number);
    }

    private static int ValidateFirstArgument(string argument)
    {
        foreach (char c in argument)
        {
            if (!char.IsDigit(c)) AppErrExit();
        }
        return Convert.ToInt32(argument);
    }

    private static string ValidateSecondArgument(string argument)
    {
        foreach (var c in argument)
        {
            if (!_allowed_letters.Contains(c)) AppErrExit();;
        }
        return argument;
    }

    private static void ValidateLengthToOptions(int length, string word)
    {
        if (length < word.Length) AppErrExit();
    }

    private static void AppErrExit()
    {
        Console.WriteLine("ERROR: Arguments");
        Console.WriteLine("\n1st argument sets password length:");
        Console.WriteLine("  <N>");
        Console.WriteLine("\n2nd argument sets password option(s) (options must not exceed the password length):");
        Console.WriteLine("  l = lower case letter");
        Console.WriteLine("  L = upper case letter");
        Console.WriteLine("  d = digit");
        Console.WriteLine("  s = special character");
        Console.WriteLine("\nExample:");
        Console.WriteLine("  <command> 8 lLssdd");
        Console.WriteLine("\nThe example above will generate a password with 8 characters applying");
        Console.WriteLine("  minimum 1 lower case letter [a-z]");
        Console.WriteLine("  minimum 1 upper case letter [A-Z]");
        Console.WriteLine("  minimum 2 special characters [\"!#¤%&/(){}[]]");
        Console.WriteLine("  minimum 2 digits [0-9]");
        Environment.Exit(1);
    }
}
