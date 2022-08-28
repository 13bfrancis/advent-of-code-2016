
List<string> input = "L3, R2, L5, R1, L1, L2, L2, R1, R5, R1, L1, L2, R2, R4, L4, L3, L3, R5, L1, R3, L5, L2, R4, L5, R4, R2, L2, L1, R1, L3, L3, R2, R1, L4, L1, L1, R4, R5, R1, L2, L1, R188, R4, L3, R54, L4, R4, R74, R2, L4, R185, R1, R3, R5, L2, L3, R1, L1, L3, R3, R2, L3, L4, R1, L3, L5, L2, R2, L1, R2, R1, L4, R5, R4, L5, L5, L4, R5, R4, L5, L3, R4, R1, L5, L4, L3, R5, L5, L2, L4, R4, R4, R2, L1, L3, L2, R5, R4, L5, R1, R2, R5, L2, R4, R5, L2, L3, R3, L4, R3, L2, R1, R4, L5, R1, L5, L3, R4, L2, L2, L5, L5, R5, R2, L5, R1, L3, L2, L2, R3, L3, L4, R2, R3, L1, R2, L5, L3, R4, L4, R4, R3, L3, R1, L3, R5, L5, R1, R5, R3, L1".Split(",").ToList();
//List<string> input = "R2, R2, R2".Split(",").ToList();

int x = 0;
int y = 0;

int ChangeDirection(char rotation, int currentDirection)
{
    int change = rotation == 'R' ? 1 : 3;
    return ((currentDirection) + change) % 4;
}

Direction CurrentDirection = Direction.North;

foreach (var item in input)
{
    CurrentDirection = (Direction)(ChangeDirection(item.Trim().First(), (int)CurrentDirection));
    int steps = int.Parse(item.Trim().Remove(0, 1));

    switch (CurrentDirection)
    {
        case Direction.North:
            y += steps;
            break;
        case Direction.East:
            x += steps;
            break;
        case Direction.South:
            y -= steps;
            break;
        case Direction.West:
            x -= steps;
            break;
    }
}

var blocksAway = Math.Abs(x) + Math.Abs(y);

Console.Write($"Blocks Away: {blocksAway}");