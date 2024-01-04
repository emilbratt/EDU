namespace Recipes.Ingredients
{
    class Cheese(float amount) : IIngredient
    {
        private float _amount = amount;
        private readonly string _name = "Cheese";
        private readonly float _price = 110;
        private readonly string _unit = "kg.";

        public void PrintSummary() => Console.WriteLine($"{_name}: {_amount} {_unit}.");

        public float Amount() => _amount;

        public string Unit() => _unit;

        public string Name() => _name;

        public float Price() => _price * _amount;
    }
}
