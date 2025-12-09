// var data = File.ReadAllLines("../../../ExampleData.txt").ToList(); // 50
var data = File.ReadAllLines("../../../Data.txt"); // 4769758290

var positions = new List<(long, long)>();
foreach (var line in data)
{
    var split = line.Split(",").Select(long.Parse).ToList();
    positions.Add((split[0], split[1]));
}

var distances = new List<long>();
for(var i = 0; i+1 < positions.Count(); i++)
{
    for(var j = i+1; j < positions.Count(); j++)
    {
        var pos1 = positions[i];
        var pos2 = positions[j];
        distances.Add((Math.Abs(pos1.Item1 - pos2.Item1) + 1) * (Math.Abs(pos1.Item2 - pos2.Item2) + 1));
    }
}
distances.Sort();

Console.WriteLine($"{distances.Last()}");
