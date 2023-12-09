namespace pdf_combiner;

class Program
{
    public static void Main(string[] args)
    {
        var (inFolder, outFolder) = CliParser.ParseInput(args);

        Console.WriteLine(inFolder);
        Console.WriteLine(outFolder);
    }
}