// var data = File.ReadAllLines("../../../ExampleData.txt").ToList(); // 40
var data = File.ReadAllLines("../../../Data.txt"); // 181584

var coordinates = data.Select(l => l.Split(",").Select(long.Parse).ToList()).Select(l => (l[0], l[1], l[2])).ToList();
var distances = new List<(long, int, int)>();
for (var i = 0; i + 1 < coordinates.Count(); i++)
{
    for (var j = i + 1; j < coordinates.Count(); j++)
    {
        var pos1 = coordinates[i];
        var pos2 = coordinates[j];
        var dif = (pos1.Item1 - pos2.Item1, pos1.Item2 - pos2.Item2, pos1.Item3 - pos2.Item3);
        var distance = dif.Item1 * dif.Item1 + dif.Item2 * dif.Item2 + dif.Item3 * dif.Item3;
        distances.Add((distance, i, j));
    }
}
distances.Sort();

List<int> pointsTo = Enumerable.Range(0, coordinates.Count()).ToList();
var max = data.Count() < 100 ? 10 : 1000;
for (var i = 0; i < max; i++)
{
    var d = distances[i];
    AddToTree(d.Item2, d.Item3);
}
void AddToTree(int a, int b)
{
    var c = GetLowest(a);
    var d = GetLowest(b);

    pointsTo[d] = GetLowest(Math.Min(c, d));
    pointsTo[c] = pointsTo[d];
    pointsTo[b] = pointsTo[d];
    pointsTo[a] = pointsTo[d];
}
int GetLowest(int a)
{
    if (a != pointsTo[a])
        pointsTo[a] = GetLowest(pointsTo[a]);
    return pointsTo[a];
}

for (var i = 0; i < pointsTo.Count(); i++)
{
    GetLowest(i);
}

var counts = pointsTo.GroupBy(n => n).Select(n => (n.Count(), n.Key)).OrderBy(x => x.Item1).ToList();
var result = counts[counts.Count() - 3].Item1 * counts[counts.Count() - 2].Item1 * counts[counts.Count() - 1].Item1;
Console.WriteLine($"{result}");
