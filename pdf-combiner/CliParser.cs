namespace pdf_combiner;

/**
 * Parser for parsing CLI input
 */
public static class CliParser
{
    /**
     * Parses input (junior comments, baby)
     */
    public static (string?, string?) ParseInput(string[] args)
    {
        var inFolder = "";
        var outFolder = "";
        switch (args.Length)
        {
            case 0:
                Console.WriteLine("Need in-folder path");
                break;
            case 1:
                Console.WriteLine("Need out-folder path");
                break;
            default:
                inFolder = args[0];
                outFolder = args[1];
                break;
        }

        return (inFolder == "" ? null : inFolder, outFolder == "" ? null : outFolder);
    }
}