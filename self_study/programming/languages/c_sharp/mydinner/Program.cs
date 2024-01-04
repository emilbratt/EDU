namespace Recipes;

class Program
{
    static void Main(string[] args)
    {
        Pizza pizza_meal;

        pizza_meal = new Pizza(1);
        Console.WriteLine($"Price for 1 pizza = {pizza_meal.Price()}");
        Console.WriteLine("Summary for 1 pizza");
        pizza_meal.PrintIngredients();


        pizza_meal = new Pizza(3);
        Console.WriteLine($"Price for 3 pizza = {pizza_meal.Price()}");
        Console.WriteLine("Summary for 3 pizza");
        pizza_meal.PrintIngredients();

    }
}
