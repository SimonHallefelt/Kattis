// var data = File.ReadAllLines("../../../ExampleData.txt"); // 33
var data = File.ReadAllLines("../../../Data.txt"); // 14677

var lightTargets = new List<List<bool>>();
var buttons = new List<List<List<int>>>();
var requirements = new List<List<int>>();

foreach (var line in data)
{
    var split = line.Replace("[", "").Replace("]", "").Replace("(", "").Replace(")", "").Replace("{", "").Replace("}", "").Split(" ").ToList();
    lightTargets.Add(split[0].Select(c => c == '#').ToList());
    buttons.Add(split.GetRange(1, split.Count() - 2).Select(s => s.Split(",").Select(int.Parse).ToList()).ToList());
    requirements.Add(split.Last().Split(",").Select(int.Parse).ToList());
}

var result = 0L;
for (int i = 0; i < lightTargets.Count(); i++)
{
    var button = buttons[i];
    var requirement = requirements[i];
    var value = bfs(requirement, button.ToHashSet().ToList().OrderBy(x => x.Count).ToList());
    if (value == -1)
        Console.WriteLine($"bfs failed on {i}");
    result += value;
    Console.WriteLine($"{result}, {i}");
    Console.WriteLine($"-----------------------");
}
long bfs(List<int> startValue, List<List<int>> paths)
{
    var (order, alternatives) = GetOrder(startValue, paths);
    var sortest = long.MaxValue;
    var visited = new Dictionary<string, long> { {string.Join(" ", startValue), 0L} };
    var target = string.Join(" ", Enumerable.Repeat(0, startValue.Count).ToList());
    var queue = new Queue<(List<int>, long)>();
    queue.Enqueue((startValue, 0L));

    while (queue.Count() > 0)
    {
        var q = queue.Dequeue();
        var counter = q.Item1;
        var steps = q.Item2;
        if (steps + counter.Max() >= sortest)
            continue;
        if (string.Join(" ", counter).Equals(target))
            sortest = steps;
        var i = 0;
        foreach(var o in order)
        {
            if (counter[o] == 0)
                continue;
            i = o;
            break;
        }
        foreach(var alternative in alternatives[i])
        {
            var newCounter = counter.ToList();
            var newSteps = steps;
            var skip = false;
            do
            {
                newSteps++;
                foreach(var button in paths[alternative])
                {
                    newCounter[button]--;
                    if (newCounter[button] < 0)
                    {
                        skip = true;
                        break;
                    }
                }
                if (skip)
                    break;
            } while(alternatives[i].Last() == alternative && newCounter[i] > 0);
            if (skip)
                continue;
            var newCounterString = string.Join($" ", newCounter);
            if (visited.ContainsKey(newCounterString) && visited[newCounterString] <= newSteps)
                continue;
            visited[newCounterString] = newSteps;
            queue.Enqueue((newCounter, newSteps));
        }
    }

    Console.WriteLine($"visited.Count = {visited.Count()}, target = {string.Join(" ", startValue)}");
    return sortest;
}
(List<int>, List<List<int>>) GetOrder(List<int> target, List<List<int>> paths) 
{
    var alternatives = Enumerable.Range(0, target.Count).Select(n => (n, new List<int>())).ToList();
    for(var i = 0; i < paths.Count; i++)
    {
        var path = paths[i];
        foreach (var button in path)
        {
            alternatives[button].Item2.Add(i);
        }
    }

    var visited = new HashSet<int>();
    var order = new List<int>();
    while (visited.Count < target.Count)
    {
        alternatives = alternatives.OrderBy(x => x.Item2.Sum(p => paths[p].Count()) * -1).ToList();
        alternatives = alternatives.OrderBy(x => Math.Pow(target[x.Item1], x.Item2.Count)).ToList();

        for(var i = 0; i < alternatives.Count; i++)
        {
            var alt = alternatives[i];
            if (!visited.Add(alt.Item1))
                continue;
            order.Add(alt.Item1);
            for(var j = i+1; j < alternatives.Count; j++)
            {
                alternatives[j] = (alternatives[j].Item1, alternatives[j].Item2.Where(p => !alt.Item2.Contains(p)).ToList());
            }
            break;
        }
    }

    var finalAlternatives = alternatives.OrderBy(x => x.Item1).Select(x => x.Item2).ToList();
    Console.WriteLine($"order = {string.Join(" ", order)}, finalAlternatives = {string.Join(" | ", finalAlternatives.Select(x => string.Join(" ", x)))}");
    return (order, finalAlternatives);
}

Console.WriteLine($"{result}");