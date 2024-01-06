namespace Recipes.Ingredients
{
    class Flour(float amount) : IIngredient
    {
        public float Amount => amount;
        public string Name => "Flour";
        public float Price => 10 * amount;
        public string Unit => "kg.";

        public string Summary(float Qty) => $"{Name} {Math.Round(amount * Qty, 2)} {Unit} {Math.Round(Price * Qty, 2)},-";
    }
}
