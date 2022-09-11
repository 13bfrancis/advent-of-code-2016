public class DayTwo
{
    public static string GetCode(List<string> inputList, string[,] numberPad)
    {
        int x = numberPad.GetLength(0) / 2;
        int y = numberPad.GetLength(1) / 2;

        string currentNumber = "";
        string results = "";
        foreach (var item in inputList)
        {
            foreach (char c in item)
            {
                Direction curDirection;
                Enum.TryParse(c.ToString(), out curDirection);

                switch (curDirection)
                {
                    case Direction.U:
                        if (x > 0 && !string.IsNullOrEmpty(numberPad[x - 1, y].ToString())) x -= 1;
                        currentNumber = numberPad[x, y];
                        break;
                    case Direction.D:
                        if (x < (numberPad.GetLength(0) - 1) && !string.IsNullOrEmpty(numberPad[x + 1, y].ToString())) x += 1;
                        currentNumber = numberPad[x, y];
                        break;
                    case Direction.L:
                        if (y > 0 && !string.IsNullOrEmpty(numberPad[x, y - 1].ToString())) y -= 1;
                        currentNumber = numberPad[x, y];
                        break;
                    case Direction.R:
                        if (y < (numberPad.GetLength(1) - 1) && !string.IsNullOrEmpty(numberPad[x, y + 1].ToString())) y += 1;
                        currentNumber = numberPad[x, y];
                        break;
                }

            }

            results = results + currentNumber;

        }

        return results;
    }
}