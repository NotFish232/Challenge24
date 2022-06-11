using System.Data;
using System.Diagnostics;

namespace Challenge24
{

    class Init
    {
        public static void Main(String[] args)
        {
            while (true) { new Game(); }
        }
    }

    class Game
    {
        private (List<Cards>, string) set;
        private List<Cards> cards;
        private string solution;
        private Stopwatch timer;

        public Game()
        {
            set = Generator.generate_cards();
            cards = set.Item1.ConvertAll(card => card.copy());
            solution = set.Item2;

            timer = new Stopwatch();
            timer.Start();

            run();
        }

        private void run()
        {
            while (cards.Count(card => card.used) != cards.Count - 1 || cards.Find(card => !card.used)?.val != 24)
            {
                int first_card_index = get_card_index(cards);
                cards[first_card_index].used = true;

                string op = Cards.operators[get_operator_index()];

                int second_card_index = get_card_index(cards);
                cards[second_card_index].used = true;

                cards[first_card_index].operation(cards[second_card_index], op);
                cards[first_card_index].used = false;
            }
            timer.Stop();

            Console.WriteLine(String.Join(" ", cards));
            log_results(set.Item1, timer.Elapsed, cards.FindAll(card => !card.used)[0].equation, solution);
        }

        private void log_results(List<Cards> cards, TimeSpan time, string user_solution, string computer_solution)
        {
            log($"You solved [{String.Join(", ", cards)}] in {time.Seconds}.{time.Milliseconds} seconds!");
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
            Console.WriteLine(String.Join(" ", cards));

            int inpt;
            do
            {
                Console.Write($"Pick a card (1 - {cards.Count}): ");
                inpt = input();

            } while (!(inpt >= 1 && inpt <= cards.Count) || cards[inpt - 1].used);

            return inpt - 1;
        }

        private int get_operator_index()
        {
            Console.WriteLine(String.Join(" ", Cards.operators));

            int inpt;
            do
            {
                Console.Write($"Pick a operator (1 - {Cards.operators.Length}): ");
                inpt = input();

            } while (!(inpt >= 1 && inpt <= Cards.operators.Length));

            return inpt - 1;
        }

        private int input()
        {
            int result = 0;
            var input = Console.ReadLine();

            if (!string.IsNullOrWhiteSpace(input))
            {
                if (input.ToLower() == "q") Environment.Exit(0);
                if (input.ToLower() == "s") Console.WriteLine($"Solution: {solution}");
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

        public double val { get => Cards.eval(equation); }

        public string equation { get => _equation; set => _equation = value; }

        public bool used { get => _used; set => _used = value; }

        public override string ToString() => used ? "*" : val.ToString();

        public Cards copy() => (Cards)this.MemberwiseClone();

        public static double eval(string expression) => Convert.ToDouble(new DataTable().Compute(expression, ""));

        public Cards operation(Cards card, string op)
        {
            equation = $"({equation} {op} {card.equation})";
            return this;
        }

    }

    static class Generator
    {
        private static int cards_count = 4, min_num = 1, max_num = 15, target_num = 24;
        private static Random rnd = new Random();

        public static (List<Cards>, string) generate_cards()
        {
            List<Cards> cards = new List<Cards>();
            (bool, string) result = (false, "");

            while (!result.Item1)
            {
                cards = Enumerable.Range(1, cards_count).Select(s => new Cards(rnd.Next(min_num, max_num))).ToList();
                result = is_solvable(cards);
            }

            return (cards, result.Item2);
        }

        private static (bool, string) is_solvable(List<Cards> cards)
        {
            if (cards.Count == 1) return (cards[0].val == target_num, cards[0].equation);
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
                        if (op != "/" || cards[j].val != 0)
                        {
                            newCards.Add(cards[i].copy().operation(cards[j], op));
                            (bool, string) result = Generator.is_solvable(newCards);
                            newCards.RemoveAt(newCards.Count - 1);
                            if (result.Item1) return result;

                        }
                    }
                }
            }

            return (false, "");
        }

    }

}
