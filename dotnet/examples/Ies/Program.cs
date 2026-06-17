using Kawaiifi.Net;

using var defaultInterface = Interface.Default();
using var scan = defaultInterface?.Scan();
PrintScanIes(scan);

static void PrintScanIes(Scan? scan)
{
    if (scan is null)
    {
        return;
    }

    foreach (var bss in scan.BssList)
    {
        foreach (var ie in bss.Ies)
        {
            Console.WriteLine($"{ie.Name} ({ie.Id}) - {ie.Summary}");

            using FieldList fields = ie.Fields;
            foreach (Field field in fields)
            {
                PrintField(field, "  ");
            }
        }
        Console.WriteLine();
    }
}

static void PrintField(Field field, string indent)
{
    Console.WriteLine($"{indent}{field.Title}: {field.Value}");
    foreach (Field subfield in field.Subfields)
    {
        PrintField(subfield, indent + "  ");
    }
}
