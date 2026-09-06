CREATE PROCEDURE dbo.usp_ListEmployees
AS
BEGIN
    SET NOCOUNT ON;

    SELECT
        id       AS Id,
        Nombre   AS Name,
        Salario  AS Salary
    FROM dbo.Empleado
    ORDER BY Nombre ASC;
END;
GO