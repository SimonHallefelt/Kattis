// var data = File.ReadAllLines("../../../ExampleData.txt"); // 43
var data = File.ReadAllLines("../../../Data.txt"); // 8616


var map = new List<List<char>>();
foreach (var line in data)
{
    map.Add(line.ToList());
}

var colLength = map.Count();
var rowLength = map[0].Count();

var sum = 0;

for(var i = 0; i < rowLength; i++)
{
    for(var j = 0; j < colLength; j++)
    {
        if (map[i][j] != '@')
            continue;
        if (LookAtNeighbors(i, j) < 4)
        {
            sum++;
            map[i][j] = '.';
            i = Math.Max(0, i - 1);
            j = Math.Max(-1, j-2);
        }
    }
}
Console.WriteLine($"{sum}");

int LookAtNeighbors(int r, int c)
{
    var rolls = 0;
    for(var i = Math.Max(0, r-1); i < Math.Min(r+2, rowLength); i++)
    {
        for(var j = Math.Max(0, c-1); j < Math.Min(c+2, colLength); j++)
        {
            if (i == r && j == c)
                continue;
            if (map[i][j] == '@')
                rolls++;
        }
    }
    return rolls;
}