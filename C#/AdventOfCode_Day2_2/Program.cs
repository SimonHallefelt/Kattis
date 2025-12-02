// var data = File.ReadAllText("../../../ExampleData.txt"); // 4174379265
var data = File.ReadAllText("../../../Data.txt"); // 24774350322

var ranges = data.Split(",");
long sum = 0;
foreach(var range in ranges)
{
    var startValue = long.Parse(range.Split("-")[0]);
    var endValue = long.Parse(range.Split("-")[1]);
    for (long i = startValue; i <= endValue; i++)
    {
        var length = i.ToString().Count();

        for (int j = 1; j <= length / 2; j++)
        {
            if (length != length / j * j)
                continue;
            var hash = new HashSet<string>();
            for (int k = 0; k < length; k += j)
            {
                hash.Add(i.ToString().Substring(k, j));
            }
            if (hash.Count() == 1)
            {
                sum += i;
                Console.WriteLine($"{sum}, {i}");
                break;
            }
        }
    }

    Console.WriteLine($"{sum}");
}