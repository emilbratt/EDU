Private Sub connect_access()

    'SIMPLE DATABASE CONNECTION FOR EXCEL
    'WRITTEN BY Emil Bratt Boersting -> https://github.com/emilbratt

    '..INFO
    '..this makro will load the data into a new sheet called "data"
    '..to keep things as simpe as possible, only 2 values needs to be inserted by you
    '..see the guide below

    '..GUIDE
    '..complete these 4 steps to connect your excel spreadsheet to your Access database
    '..1. Go to Tools in the VBA editor
    '..2. Chose "References"
    '..3. Include references by checking:
        '-> Microsoft ActiveX Data Objects 2.0 Library
        '-> Microsoft Excel 16.0 Object Library
        '-> Microsoft Office 16.0 Object Library
        '-> Microsoft Forms 2.0 Object Library
    '..4. Apply your access database filename and a tablename
        '-> set filename for access database by uncommenting line
            '..uncomment (remove the leading ') before either
            '..line 52 or 53 depending on your preference
            '..(NB: omit the file extention for filename)
        '-> set tablename for query by uncommenting line
            '..uncomment (remove the leading ') before either
            '..line 56 or 57 depending on your preference

    '..you should now be ready to connect to Access..

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

    '..automatically fetch filename and build file path to the database using value from cell A1 or a hardcoded value
    pathRoot = Application.ActiveWorkbook.path
    'pathDatabase = pathRoot & "\" & Cells(1, 1) & ".accdb" '----> uncomment for loading databasename from cell A1
    'pathDatabase = pathRoot & "\" & "?" & ".accdb" '----> uncomment for hardcoding (replace ? with your database file name.

    '..fetch table name by using value from cell B1 or a hardcoded value
    'table = Cells(1, 2) '----> uncomment for loading tablename from cell B1
    'table = "?" '----> uncomment for hardcoding (replace ? with your table name)


    'CREATE NEW SHEET FOR DATASET

    '..make new sheet and set the sheet as object -> Data
    Sheets.Add.Name = "data"
    Set Data = Worksheets("data")


    'CONNECT TO DATABASE

    '..apply dataobjects for connection to Access
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

    '..extract the column titles from the records object
    Set fields = records.fields


    'POPULATE SPREADSHEET WITH DATA IN RESULTSET

    '..populate field names into its own row
    For field = 1 To fields.Count
        Data.Cells(1, field).Value = fields.Item(field - 1).Name
    Next field

    '..populate the rows under the field row with resultset from query
    Data.Range("A2").CopyFromRecordset Data:=records

    '..close connection to Access
    cnxn.Close


End Sub
