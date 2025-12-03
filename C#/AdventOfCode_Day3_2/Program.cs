// var data = File.ReadAllLines("../../../ExampleData.txt"); // 3121910778619
var data = File.ReadAllLines("../../../Data.txt"); // 171297349921310

long sum = 0;
foreach(var line in data)
{
    var largest = Enumerable.Repeat(0, 12).ToList();
    var lineLength = line.Count();
    for (var i = 0; i < lineLength; i++)
    {
        var value = int.Parse(line[i].ToString());
        var increased = false;
        for (var j = 0; j < 12; j++)
        {
            if (increased)
                largest[j] = 0;
            else if (i - j + 11 < lineLength && value > largest[j])
            {
                largest[j] = value;
                increased = true;
            }
        }
    }
    sum += long.Parse(string.Join("", largest));
    Console.WriteLine($"{sum}, {long.Parse(string.Join("", largest))}");
}

