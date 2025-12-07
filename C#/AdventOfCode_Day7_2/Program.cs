// var map = File.ReadAllLines("../../../ExampleData.txt").ToList(); // 40
var map = File.ReadAllLines("../../../Data.txt"); // 80158285728929

var visited = new List<Dictionary<int, long>>
{
    new() { { map[0].IndexOf('S'), 1 } }
};
for (var i = 0; i + 1 < map.Count(); i++)
{
    var visit = new Dictionary<int, long>();
    foreach (var v in visited[i])
    {
        var col = v.Key;
        if (map[i][col] == '^')
        {
            visit[col - 1] = visit.GetValueOrDefault(col - 1, 0) + v.Value;
            visit[col + 1] = visit.GetValueOrDefault(col + 1, 0) + v.Value;
        }
        else
            visit[col] = visit.GetValueOrDefault(col, 0) + v.Value;
    }
    visited.Add(visit);
    Console.WriteLine($"{visit.Values.Sum()}");
}

Console.WriteLine($"{visited[map.Count()-1].Values.Sum()}");