// lar brukeren skrive inn en verdi - og så vises alle objekter som inneholder denne verdien i et av feltene, som du velger selv.

class Program
{
    static int Main()
    {
        // Creating an array of Country objects
        Country[] countries_array = new Country[]
        {
            new Country("USA", 331000000),
            new Country("China", 1444216107),
            new Country("India", 1393409038),
        };

        bool found_country;
        while (true)
        {
            found_country = false;

            Console.Write("Type in the name of a country or leave blank to exit: ");
            string input_country_name = Console.ReadLine()?? "";
            if (input_country_name == "") return 0;

            input_country_name = input_country_name.ToLower();
            foreach (Country country in countries_array)
            {
                if (found_country) break;

                if (input_country_name == country.Name.ToLower())
                {
                    Console.WriteLine($"{country.Name} has a population of {country.Population}");
                    found_country = true;
                }
            }

            if (!found_country) Console.WriteLine($"{input_country_name} not found in our database");
        }
    }
}
