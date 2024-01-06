namespace Recipes.Ingredients
{
    class Water(float amount) : IIngredient
    {
        public float Amount => amount;
        public string Name => "Water";
        public float Price => 0 * amount;
        public string Unit => "l.";

        public string Summary(float Qty) => $"{Name} {Math.Round(amount * Qty, 2)} {Unit} {Math.Round(Price * Qty, 2)},-";
    }
}
