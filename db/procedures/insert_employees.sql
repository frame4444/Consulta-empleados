CREATE PROCEDURE dbo.usp_InsertEmployee @NameVARCHAR(128),
@Salary MONEY AS BEGIN
SET NOCOUNT
ON ;
IF EXISTS(
  SELECT
    1
  FROM
    dbo.Empleado
  WHERE
    Nombre = @Name
) BEGIN SELECT
  0 AS Success,
  'Employee name already exists.' AS Message;
RETURN;

ENDINSERT INTO dbo.Empleado(
  Nombre,
  Salario
)
VALUES(
  @Name,
  @Salary
);
SELECT
  1 AS Success,
  'Employee inserted successfully.' AS Message;

END;
GO
