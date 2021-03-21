Sub prepare_pivot()
    Dim row As Integer
    Dim col As Integer

    Dim startrow As Integer
    Dim endrow As Integer

    Dim startcol As Integer
    Dim endcol As Integer

    Dim colcount As Integer
    Dim rowcount As Integer

    Dim torow As Integer
    Dim fromrow As Integer

    Dim tocol As Integer
    Dim fromcol As Integer

    Dim yearcount As Integer

    Dim N As Integer
    Dim rowdel As Integer
    Dim endtable As Integer


    'REMOVE ROWS TOP
    rowdel = 2
    For row = 1 To rowdel
        Rows(1).EntireRow.Delete
    Next row


    'REMOVE ROWS BOTTOM
    endrow = Cells(Rows.Count, 1).End(xlUp).row
    endtable = Cells(Rows.Count, 2).End(xlUp).row + 1

    For row = endtable To endrow
        Rows(endtable).EntireRow.Delete
    Next row


    'INSERT 2 NEW COLUMNS AND TITLES
    Range("C:C").Insert
    Range("C:C").Insert
    Cells(2, 1).Value = "Flyplass"
    Cells(2, 2).Value = "Flytype"
    Cells(2, 3).Value = "År"
    Cells(2, 4).Value = "Passasjerer"



    'PREPARE NAMES IN COL A AND B
    endcol = Cells(2, Columns.Count).End(xlToLeft).Column ' = column N
    startcol = 5 ' = column C

    startrow = 3
    endrow = Cells(Rows.Count, 1).End(xlUp).row 'the if statement checks if row B is longer than current endrow
        If endrow < Cells(Rows.Count, 2).End(xlUp).row Then
            endrow = Cells(Rows.Count, 2).End(xlUp).row
        End If

    colcount = endcol - startcol
    torow = endrow + 1

    For yearcount = 1 To colcount
        For fromrow = startrow To endrow
            For N = 1 To 2
                Cells(torow, N).Value = Cells(fromrow, N).Value
            Next N
            torow = torow + 1
        Next fromrow
    Next yearcount

    'etterfylle tomme celler i A kolonnen
    endrow = Cells(Rows.Count, 1).End(xlUp).row
    For row = 2 To endrow + 1
        If Cells(row, 1).Value = "" Then
            Cells(row, 1).Value = Cells(row - 1, 1).Value
        End If
    Next row


    'POPULATE YEAR COLUMN
    startrow = 3
    endrow = Cells(Rows.Count, 5).End(xlUp).row 'last row in col E

    startcol = 5 'col E
    endcol = Cells(2, Columns.Count).End(xlToLeft).Column ' = column N

    torow = 3

    For fromcol = startcol To endcol
        For fromrow = startrow To endrow
            Cells(torow, 3).Value = Cells(2, fromcol).Value
            torow = torow + 1
        Next fromrow
    Next fromcol


    'POPULATE PASSANGER DATA
    startrow = 3
    endrow = Cells(Rows.Count, 5).End(xlUp).row 'last row in col E

    startcol = 5 'col E
    endcol = Cells(2, Columns.Count).End(xlToLeft).Column ' = column N

    torow = 3

    For fromcol = startcol To endcol
        For fromrow = startrow To endrow
            Cells(torow, 4).Value = Cells(fromrow, fromcol).Value
        torow = torow + 1
        Next fromrow
    Next fromcol


    'REMOVE LEFTOVER DATA
    startrow = 1
    endrow = Cells(Rows.Count, 5).End(xlUp).row 'last row in col E

    startcol = 5 'col E
    endcol = Cells(2, Columns.Count).End(xlToLeft).Column ' = column N

    For fromcol = startcol To endcol
        For fromrow = startrow To endrow
            Cells(fromrow, fromcol).Value = ""
        Next fromrow
    Next fromcol


    'REMOVE LAST PART OF YEAR (2009MO1 -> 2009)
    Dim tempname As String
    startrow = 2
    endrow = Cells(Rows.Count, 1).End(xlUp).row 'last row in col A

    For row = startrow To endrow
        tempname = Cells(row, 3).Value
        tempname = Left(tempname, 4)
        Cells(row, 3).Value = tempname
    Next row

End Sub
