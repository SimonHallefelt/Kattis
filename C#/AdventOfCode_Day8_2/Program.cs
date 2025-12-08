// var data = File.ReadAllLines("../../../ExampleData.txt").ToList(); // 25272
var data = File.ReadAllLines("../../../Data.txt"); // 8465902405

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

var pointsTo = Enumerable.Range(0, coordinates.Count()).ToList();
var pointsToRev = Enumerable.Range(0, coordinates.Count()).Select(v => new HashSet<int>{}).ToList();
var num = 0;
while (pointsToRev[0].Count() != coordinates.Count())
{
    var d = distances[num++];
    AddToTree(d.Item2, d.Item3);
}
void AddToTree(int a, int b)
{
    var c = GetLowest(a);
    var d = GetLowest(b);

    var lowest = GetLowest(Math.Min(c, d));
    GoingUp(a, lowest, pointsTo[a]);
    GoingUp(b, lowest, pointsTo[b]);
    GoingUp(c, lowest, pointsTo[c]);
    GoingUp(d, lowest, pointsTo[d]);
}
int GetLowest(int a)
{
    if (a != pointsTo[a])
    {
        var b = GetLowest(pointsTo[a]);
        GoingUp(a, b, pointsTo[a]);
    }
    return pointsTo[a];
}
void GoingUp(int pos, int newValue, int oldValue)
{
    pointsTo[pos] = newValue;
    pointsToRev[newValue].Add(pos);
    if (newValue == pos || newValue == oldValue)
        return;
    pointsToRev[oldValue].Remove(pos);
    foreach (var p in pointsToRev[pos])
    {
        GoingUp(p, newValue, pos);
    }
    pointsToRev[pos].Clear();
}

var e = distances[num-1];
var result = coordinates[e.Item2].Item1 * coordinates[e.Item3].Item1;
Console.WriteLine($"{result}");
