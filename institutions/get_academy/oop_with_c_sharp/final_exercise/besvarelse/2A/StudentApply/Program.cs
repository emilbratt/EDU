var ENG_NOR = new Dictionary<string, string>
{
    { "has_applied", "Har søkt" },
    { "received_documentation", "Levert dokumentasjon" },
    { "admission_granted", "Fått tilbud" },
    { "signed_contract", "Signert kontrakt" },
    { "enrolled", "Startet" },
    { "not_enrolled", "Ikke startet" },
    { "interrupted", "Brutt" },
    { "not_passed_no_tries_left", "Ikke bestått - ikke flere forsøk igjen" },
    { "passed", "Fullført og bestått" },
    { "complete_not_passed", "Fullført ikke bestått" },
    { "bogus_status", "Tullestatus" },
};

StudentStatus student_status = new();
string result;


// TRY UPDATING STATUS
Console.Write($"Oppdaterer fra '{ENG_NOR[student_status.CurrentStatus]}' til '{ENG_NOR["received_documentation"]}' | ");
result = student_status.Update("received_documentation") ? "OK" : "IKKE LOV";
Console.WriteLine(result);

Console.Write($"Oppdaterer fra '{ENG_NOR[student_status.CurrentStatus]}' til '{ENG_NOR["enrolled"]}' | ");
result = student_status.Update("enrolled") ? "OK" : "IKKE LOV";
Console.WriteLine(result);

Console.Write($"Oppdaterer fra '{ENG_NOR[student_status.CurrentStatus]}' til '{ENG_NOR["admission_granted"]}' | ");
result = student_status.Update("admission_granted") ? "OK" : "IKKE LOV";
Console.WriteLine(result);

Console.Write($"Oppdaterer fra '{ENG_NOR[student_status.CurrentStatus]}' til '{ENG_NOR["signed_contract"]}' | ");
result = student_status.Update("signed_contract") ? "OK" : "IKKE LOV";
Console.WriteLine(result);

Console.Write($"Oppdaterer fra '{ENG_NOR[student_status.CurrentStatus]}' til '{ENG_NOR["enrolled"]}' | ");
result = student_status.Update("enrolled") ? "OK" : "IKKE LOV";
Console.WriteLine(result);

Console.Write($"Oppdaterer fra '{ENG_NOR[student_status.CurrentStatus]}' til '{ENG_NOR["bogus_status"]}' | ");
result = student_status.Update("bogus_status") ? "OK" : "IKKE LOV";
Console.WriteLine(result);

Console.Write($"Oppdaterer fra '{ENG_NOR[student_status.CurrentStatus]}' til '{ENG_NOR["complete_not_passed"]}' | ");
result = student_status.Update("complete_not_passed") ? "OK" : "IKKE LOV";
Console.WriteLine(result);

Console.Write($"Oppdaterer fra '{ENG_NOR[student_status.CurrentStatus]}' til '{ENG_NOR["not_passed_no_tries_left"]}' | ");
result = student_status.Update("not_passed_no_tries_left") ? "OK" : "IKKE LOV";
Console.WriteLine(result);

Console.Write($"Oppdaterer fra '{ENG_NOR[student_status.CurrentStatus]}' til '{ENG_NOR["passed"]}' | ");
result = student_status.Update("passed") ? "OK" : "IKKE LOV";
Console.WriteLine(result);


// CHECK STATUS HISTORY
result = student_status.HasBeenSet("has_applied")? "OK" : "IKKE SATT";
Console.WriteLine($"Sjekker om status '{ENG_NOR["has_applied"]}' er tidligere satt | {result}");

result = student_status.HasBeenSet("bogus_status")? "OK" : "IKKE SATT";
Console.WriteLine($"Sjekker om status '{ENG_NOR["bogus_status"]}' er tidligere satt | {result}");

result = student_status.HasBeenSet("passed")? "OK" : "IKKE SATT";
Console.WriteLine($"Sjekker om status '{ENG_NOR["passed"]}' er tidligere satt | {result}");
