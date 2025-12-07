// var data = File.ReadAllLines("../../../ExampleData.txt").ToList(); // 3263827
var data = File.ReadAllLines("../../../Data.txt").ToList(); // 9029931401920

var math = data.Last().ToList().Where(c => c != ' ').ToList();
data.RemoveAt(data.Count() - 1);

var flipData = new List<string>();
for (var i = 0; i < data[0].Count(); i++)
{
    flipData.Add(string.Join("", data.Select(row => row[i])));
}

long sum = 0;
var j = 0;
for (var i = 0; i < math.Count(); i++)
{
    long total = math[i] == '+' ? 0: 1;
    while (long.TryParse(flipData[j], out long number))
    {
        if (math[i] == '+')
            total += number;
        else
            total *= number;
        j++;
        if (j == flipData.Count())
            break;
    }
    j++;
    sum += total;
    Console.WriteLine($"{sum}, {total}");
}

