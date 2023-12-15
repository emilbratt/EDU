using System;

namespace ConsoleStars
{
    public class MovableStar 
    {
        protected int _x;
        protected int _y;
        private int _speedX;
        private int _speedY;

        public MovableStar(Random random) 
        {
            _x = random.Next(1, Console.WindowWidth - 1);
            _y = random.Next(1, Console.WindowHeight - 1);
            _speedX = random.Next(-1, 2);
            _speedY = random.Next(-1, 2);
        }

        public void Update()
        {
            var windowWidth = Console.WindowWidth;
            var windowHeight = Console.WindowHeight;

            if (_x <= 1)_speedX = 1;
            if (_x >= windowWidth - 1)_speedX = -1;
            if (_y <= 1)_speedY = 1;
            if (_y >= windowHeight - 1)_speedY = -1;
            _x += _speedX;
            _y += _speedY;
        }

        public void Show()
        {
            Console.CursorLeft = _x;
            Console.CursorTop = _y;
            Console.Write('*');
        }
    }
}
