namespace Recipes.Ingredients
{
    class Water(float amount) : IIngredient
    {
        private float _amount = amount;
        private readonly string _name = "Water";
        private readonly float _price = 0;
        private readonly string _unit = "l.";

        public void PrintSummary() => Console.WriteLine($"{_name}: {_amount} {_unit}.");

        public float Amount() => _amount;

        public string Unit() => _unit;

        public string Name() => _name;

        public float Price() => _price * _amount;
    }
}
