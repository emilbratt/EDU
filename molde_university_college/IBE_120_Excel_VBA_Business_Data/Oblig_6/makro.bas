Private Sub CommandButton1_Click()

    'SIMPLE DATABASE CONNECTION WRITTEN BY Emil B. Boersting

    '..prepare excel for connecting to Access
    '..1. Go to Tools in the VBA editor
    '..2. Chose "References"
    '..3. Include references by checking:
        '->Microsoft ActiveX Data Objects 2.0 Library
        '->Microsoft Excel 16.0 Object Library
        '->Microsoft Office 16.0 Object Library
        '->Microsoft Forms 2.0 Object Library


    'DECLARE VARIABLES & DATABASE CONNECTION

    '..for fieldcolumn in recordset used in loop
    Dim field As Integer
    Dim fieldcount As Integer

    '..for path
    Dim pathRoot As String
    Dim pathDatabase As String

    '..for connection to Access
    Dim cnxn As ADODB.Connection
    Dim records As ADODB.Recordset
    Dim cnxnExc As ADODB.Command
    Dim fields As ADODB.fields

    '..for tablename to be concatenated into query -> "SELECT * FROM <table>"
    Dim table As String

    '..for iterating
    Dim N As Integer

    '..automatically fetch filename and build path to database
    pathRoot = Application.ActiveWorkbook.path
    pathDatabase = pathRoot & "\" & Cells(4, 2) & ".accdb"

    '..fetch table by user input from cell B2 and B3
    table = Cells(2, 2) & Cells(3, 2)


    'CREATE NEW SHEET AFTER CURRENT SHEET "oversikt"

    '..make new sheet and set that sheet as object -> Data
    Sheets.Add(After:=Sheets("oversikt")).Name = "data"
    Set Data = Worksheets("data")
    Sheets("oversikt").Select

    'CONNECT TO DATABASE

    '..apply libraries for connection to Access
    Set cnxn = New ADODB.Connection
    Set cnxnExc = New ADODB.Command
    Set records = New ADODB.Recordset

    '..establish connection
    With cnxn
        .Provider = "Microsoft.ACE.OLEDB.12.0"
        .ConnectionString = pathDatabase
        .Open
    End With


    'RUN QUERY AND INSERT RESULT INTO SPREADSHEET

    '..execute query
    cnxnExc.ActiveConnection = cnxn
    cnxnExc.CommandText = "SELECT * FROM " & table
    cnxnExc.CommandType = adCmdText

    '..fetch results
    Set records = cnxnExc.Execute

    '..count columns in results
    Set fields = records.fields


    'POPULATE SPREADSHEET WITH DATA IN RESULTSET

    '..populate field names into row
    For N = 1 To fields.Count
        Data.Cells(1, N).Value = fields.Item(N - 1).Name
    Next N

    '..populate each field with records
    Data.Range("A2").CopyFromRecordset Data:=records
    cnxn.Close


    'POPULATE FIELD NAMES IN "oversikt"

    '.. prepare iteration values used in for loop
    fieldcount = Data.Cells(1, Columns.Count).End(xlToLeft).Column

    '..set first insert row number
    insert = 8

    '..fetch each field name and paste
    For field = 2 To fieldcount
        Cells(insert, 1).Value = Data.Cells(1, field)
        insert = insert + 1
    Next field


End Sub

Private Sub CommandButton2_Click()

    'DECLARE VARIABLES

    '..for counting loop iterations
    Dim field As Integer
    Dim fieldcount As Integer
    Dim recordcount As Integer

    '..for use inside loop
    Dim insert As Integer 'sets insert row number
    Dim sum As Double 'to be used when adding values

    '..make new sheet and set that sheet as object -> Data
    Set Data = Worksheets("data")


    'CALCULATE TOTAL FOR EACH RECORD AND PASTE

    '.. prepare iteration values used in for loop
    fieldcount = Data.Cells(1, Columns.Count).End(xlToLeft).Column

    '.. prepare iteration values used in for loop
    recordcount = Data.Cells(Rows.Count, 1).End(xlUp).row - 1 '(minus the field row)
    insert = 8

    '..fetch each total for each field and paste
    For field = 2 To fieldcount
        sum = 0
        For row = 2 To recordcount
            sum = sum + Data.Cells(row, field).Value
        Next row
        Cells(insert, 2).Value = sum
        insert = insert + 1
    Next field

End Sub

Private Sub CommandButton3_Click()

    'DECLARE VARIABLES

    '..for counting loop iterations
    Dim field As Integer
    Dim fieldcount As Integer
    Dim recordcount As Integer

    '..for use inside loop
    Dim custoemrcount As Integer
    Dim insert As Integer 'sets insert row number

    '..make new sheet and set that sheet as object -> Data
    Set Data = Worksheets("data")


    'COUNT THE AMOUNT OF CUSTOMERS THAT HAS DELIVERED EACH ANIMAL SLAUGHTER

    '.. prepare iteration values used in for loop
    recordcount = Data.Cells(Rows.Count, 1).End(xlUp).row - 1 '(minus the field row)
    fieldcount = Data.Cells(1, Columns.Count).End(xlToLeft).Column
    insert = 8

    '..check each row for value over 0 and add to count
    For field = 2 To fieldcount
        customercount = 0
        For row = 2 To recordcount
            If Data.Cells(row, field).Value > 0 Then
                customercount = customercount + 1
            End If
        Next row
        Cells(insert, 3).Value = customercount
        insert = insert + 1
    Next field

End Sub

Private Sub CommandButton4_Click()

    'DECLARE VARIABLES

    '..for counting loop iterations
    Dim field As Integer
    Dim fieldcount As Integer
    Dim recordcount As Integer

    '..for use inside loop
    Dim insert As Integer 'sets insert row number
    Dim sum As Double 'to be used when adding values
    Dim avgcount As Integer 'used as divisor for calculating average
    Dim avg As Double 'used to store average result

    '..make new sheet and set that sheet as object -> Data
    Set Data = Worksheets("data")


    'CALCULATE AVERAGE FOR EACH ANIMAL

    '.. prepare iteration values used in for loop
    recordcount = Data.Cells(Rows.Count, 1).End(xlUp).row - 1 '(minus the field row)
    fieldcount = Data.Cells(1, Columns.Count).End(xlToLeft).Column
    insert = 8

    '..add total for each field and divide by customers (skip customers with 0)
    For field = 2 To fieldcount
        sum = 0
        avgcount = 0
        For row = 2 To recordcount
            If Data.Cells(row, field).Value > 0 Then
                sum = sum + Data.Cells(row, field).Value
                avgcount = avgcount + 1
            End If
        Next row
        avg = sum / avgcount
        Cells(insert, 4).Value = avg
        Cells(insert, 4).NumberFormat = "0.0" 'force format one decimal
        insert = insert + 1
    Next field

End Sub
