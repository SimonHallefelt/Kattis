using System.Reflection.PortableExecutable;

// var map = File.ReadAllLines("../../../ExampleData.txt").ToList(); // 21
var map = File.ReadAllLines("../../../Data.txt"); // 1541

var visited = new List<HashSet<int>>();
visited.Add(new HashSet<int> { map[0].IndexOf('S') });
var splits = 0;
for (var i = 0; i + 1 < map.Count(); i++)
{
    var visit = new HashSet<int>();
    foreach (var col in visited[i])
    {
        if (map[i][col] == '^')
        {
            visit.Add(col - 1);
            visit.Add(col + 1);
            splits++;
        }
        else
            visit.Add(col);
    }
    visited.Add(visit);
    Console.WriteLine($"{splits}, {visit.Count()}");
}

Console.WriteLine($"{splits}, {visited[map.Count()-1].Count()}");