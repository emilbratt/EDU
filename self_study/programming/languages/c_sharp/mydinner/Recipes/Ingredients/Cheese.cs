namespace Recipes.Ingredients
{
    class Cheese(float amount) : IIngredient
    {
        public float Amount => amount;
        public string Name => "Cheese";
        public float Price => 110 * amount;
        public string Unit => "kg.";

        public string Summary(float Qty) => $"{Name} {Math.Round(amount * Qty, 2)} {Unit} {Math.Round(Price * Qty, 2)},-";
    }
}
