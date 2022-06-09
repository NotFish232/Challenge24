using System.Data;
using System.Diagnostics;

namespace Challenge24
{

    class Init
    {
        public static void Main(String[] args)
        {
            while (true)
            {
                new Game();
            }
        }
    }

    class Game
    {
        private Tuple<List<Cards>, string> set;
        private List<Cards> cards;
        private string solution;
        private Stopwatch timer;

        public Game()
        {
            set = Generator.generate_cards();
            cards = new List<Cards>();
            foreach (Cards card in set.Item1)
            {
                cards.Add(card.copy());
            }
            solution = set.Item2;

            timer = new Stopwatch();
            timer.Start();

            run();
        }

        private void run()
        {
            while (!(cards.Find(card => card.val() == 24 && !card.used()) != null && cards.Count(card => card.used()) == 3))
            {
                int first_index = get_card_index(cards);
                cards[first_index].set_used(true);

                string op = get_operator();

                int second_index = get_card_index(cards);
                cards[second_index].set_used(true);

                cards[first_index].set_used(false);
                cards[first_index].operation(cards[second_index], op);
            }

            timer.Stop();

            var card = cards.Find(card => !card.used());
            if (card != null)
            {
                log_results(set.Item1, timer.Elapsed, card.equation(), solution);
            }
        }

        private void log_results(List<Cards> cards, TimeSpan time, string user_solution, string computer_solution)
        {
            print_cards(cards);
            log($"\nYou solved [{String.Join(" ", cards)}] in {time.Seconds}.{time.Milliseconds} seconds!");
            log($"Your solution: {user_solution}");
            log($"Computer solution: {computer_solution}\n");
        }

        private void log(string str)
        {
            using (StreamWriter sw = File.AppendText("results.txt"))
            {
                sw.WriteLine(str);
                Console.WriteLine(str);
            }
        }

        private int get_card_index(List<Cards> cards)
        {
            print_cards(cards);

            int inpt;
            do
            {
                Console.Write($"Pick a card (1 - {cards.Count}): ");
                inpt = input();

            } while (!(inpt >= 1 && inpt <= cards.Count) || cards[inpt - 1].used());

            return inpt - 1;
        }

        private void print_cards(List<Cards> cards)
        {
            foreach (Cards card in cards)
            {
                Console.Write($"{(card.used() ? "*" : Math.Round(card.val(), 2))} ");
            }
            Console.WriteLine();
        }

        private string get_operator()
        {
            foreach (string op in Cards.operators)
            {
                Console.Write(op + " ");
            }
            Console.WriteLine();

            int inpt;
            do
            {
                Console.Write($"Pick a operator (1 - {Cards.operators.Length}): ");
                inpt = input();

            } while (!(inpt >= 1 && inpt <= Cards.operators.Length));

            return Cards.operators[inpt - 1];
        }

        private int input()
        {
            int result = 0;
            var input = Console.ReadLine();

            if (!string.IsNullOrWhiteSpace(input))
            {
                if (input.ToLower() == "q")
                {
                    Console.WriteLine($"Solution: {solution}");
                    Environment.Exit(0);
                }
                if (input.ToLower() == "s")
                {
                    Console.WriteLine($"Solution: {solution}");
                }
                Int32.TryParse(input, out result);
            }

            return result;
        }

    }

    class Cards
    {
        public static string[] operators = new string[] { "+", "-", "*", "/" };
        private string _equation;
        private bool _used;

        public Cards(double val)
        {
            _equation = "" + val;
            _used = false;
        }

        public double val()
        {
            return Cards.eval(equation());
        }

        public override string ToString()
        {
            return val().ToString();
        }

        public string equation()
        {
            return _equation;
        }

        public void operation(Cards card, string op)
        {
            _equation = $"({equation()} {op} {card.equation()})";
        }

        public bool used()
        {
            return _used;
        }

        public void set_used(bool used)
        {
            _used = used;
        }

        public Cards copy()
        {
            return (Cards)this.MemberwiseClone();
        }

        public static double eval(string expression)
        {
            return Convert.ToDouble(new DataTable().Compute(expression, ""));
        }

    }

    static class Generator
    {
        private static int cards_count = 4;
        private static int min = 1;
        private static int max = 12;
        private static int target = 24;
        private static Random rnd = new Random();

        public static Tuple<List<Cards>, string> generate_cards()
        {
            List<Cards> cards = new List<Cards>();
            Tuple<bool, string> result = Tuple.Create(false, "");

            do
            {
                cards.Clear();
                for (int i = 0; i < Generator.cards_count; i++)
                {
                    cards.Add(new Cards((int)rnd.NextInt64(Generator.min, Generator.max)));
                    result = is_solvable(cards);
                }
            } while (!result.Item1);

            return Tuple.Create(cards, result.Item2);
        }

        private static Tuple<bool, string> is_solvable(List<Cards> cards)
        {
            if (cards.Count == 1) return Tuple.Create(cards[0].val() == target, cards[0].equation());

            for (int i = 0; i < cards.Count; i++)
            {
                for (int j = 0; j < cards.Count; j++)
                {
                    if (i == j) continue;
                    List<Cards> newCards = new List<Cards>(cards);
                    newCards.Remove(cards[i]);
                    newCards.Remove(cards[j]);

                    foreach (string op in Cards.operators)
                    {
                        if (!(op == "/" && cards[j].val() == 0))
                        {
                            Cards card = cards[i].copy();
                            card.operation(cards[j], op);
                            newCards.Add(card);

                            Tuple<bool, string> result = Generator.is_solvable(newCards);

                            newCards.RemoveAt(newCards.Count - 1);
                            if (result.Item1) return result;

                        }
                    }
                }
            }

            return Tuple.Create(false, "");
        }

    }

}
