// var data = File.ReadAllText("../../../ExampleData.txt"); // 1227775554
var data = File.ReadAllText("../../../Data.txt"); // 12850231731

var ranges = data.Split(",");
long sum = 0;
foreach(var range in ranges)
{
    var startValue = long.Parse(range.Split("-")[0]);
    var endValue = long.Parse(range.Split("-")[1]);
    for (long i = startValue; i <= endValue; i++)
    {
        var length = i.ToString().Count();
        if (length != length / 2 * 2)
        {
            i = long.Parse(new String('9', i.ToString().Count()));
            continue;
        }

        var startNumber = i.ToString().Substring(0, length / 2);
        var endNumber = i.ToString().Substring(length / 2);

        if (startNumber.Count() != endNumber.Count())
        {
            Console.WriteLine($"f {i}, {startNumber}, {endNumber}");
            throw new Exception();
        }

        if (startNumber.Equals(endNumber))
            sum += i;
    }

    Console.WriteLine($"{sum}");
}