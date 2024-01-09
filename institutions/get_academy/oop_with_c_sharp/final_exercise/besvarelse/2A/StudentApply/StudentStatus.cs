public class StudentStatus
{
    // SAVES ALL STATUSES SET BY USER
    private List<string> _status_list = new() { "has_applied" };

    // GET CURRENT STATUS (last item is current status)
    public string CurrentStatus { get => _status_list.Last(); }

    // STATUS MAP - BASED ON CURRENT STATUS (REPRESENTED BY THE KEY), THE NEXT STATUS TO BE SET MUST EXIST IN THE CORRESPONDING ARRAY
    private readonly Dictionary <string, string[]> _NEXT_ALLOWED = new() {
        {
            "has_applied", new string[] { "received_documentation", "not_enrolled" }
        },
        {
            "received_documentation", new string[] { "admission_granted", "not_enrolled" }
        },
        {
            "admission_granted", new string[] { "signed_contract", "not_enrolled" }
        },
        {
            "signed_contract", new string[] { "enrolled", "not_enrolled" }
        },
        {
            "enrolled", new string[] { "interrupted", "complete_not_passed", "passed" }
        },
        {
            "complete_not_passed", new string[] { "not_passed_no_tries_left", "passed" }
        },
        {
            "not_enrolled", Array.Empty<string>() // final status, status can no longer be altered
        },
        {
            "interrupted", Array.Empty<string>() // final status, status can no longer be altered
        },
        {
            "not_passed_no_tries_left", Array.Empty<string>() // final status, status can no longer be altered
        },
        {
            "passed", Array.Empty<string>() // final status, status can no longer be altered
        },
    };

    // SET NEW STATUS ONLY IF ALLOWED
    public bool Update(string new_status)
    {
        if (_NEXT_ALLOWED[CurrentStatus].Contains(new_status)) _status_list.Add(new_status);
        return new_status == CurrentStatus;
    }

    // CHECK IF STATUS HAS BEEN SET BEFORE
    public bool HasBeenSet(string status) => _status_list.Contains(status);
}
