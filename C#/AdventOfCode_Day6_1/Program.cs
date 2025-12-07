// var data = File.ReadAllLines("../../../ExampleData.txt"); // 4277556
var data = File.ReadAllLines("../../../Data.txt"); // 4693419406682

var sheet = new List<List<long>>();
var math = new List<char>();
foreach(var line in data)
{
    var values = line.Split(" ", StringSplitOptions.RemoveEmptyEntries);
    if (long.TryParse(values[0], out long notUsed))
        sheet.Add(values.Select(long.Parse).ToList());
    else
        math = line.ToList().Where(c => c != ' ').ToList();
}

long sum = 0;
for (var i = 0; i < math.Count(); i++)
{
    var row = sheet.Select(row => row[i]).ToList();
    long total = 0;
    if (math[i] == '+')
        total = row.Sum();
    else
        total = row.Aggregate(1L, (accumulator, n) => accumulator * n);

    sum += total;
    Console.WriteLine($"{sum}, {total}");
}

