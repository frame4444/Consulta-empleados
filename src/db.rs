use crate::models::Employee;
use crate::models::InsertResult;

use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub type DbClient = Client<Compat<TcpStream>>;

pub async fn connect_db() -> DbClient {
    let password = std::env::var("DB_PASSWORD").expect("missing DB_PASSWORD environment variable");

    let mut config = Config::new();
    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("sa", &password));
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr())
        .await
        .expect("could not connect to the database server");

    Client::connect(config, tcp.compat_write())
        .await
        .expect("connection to SQL Server failed")
}

pub async fn list_employees(client: &mut DbClient) -> Vec<Employee> {
    let rows = client
        .simple_query("EXEC dbo.usp_ListEmployees")
        .await
        .expect("stored procedure execution failed")
        .into_first_result()
        .await
        .expect("could not read the stored procedure result");

    let mut employees = Vec::new();
    for row in rows {
        let employee = Employee {
            id: row.get::<i32, _>("Id").expect("missing Id column"),
            name: row
                .get::<&str, _>("Name")
                .expect("missing Name column")
                .to_string(),
            salary: row.get::<f64, _>("Salary").expect("missing Salary column"),
        };
        employees.push(employee);
    }

    employees
}

pub async fn insert_employee(client: &mut DbClient, name: &str, salary: f64) -> InsertResult {
    let row = client
        .query("EXEC dbo.usp_InsertEmployee @P1, @P2", &[&name, &salary])
        .await
        .expect("stored procedure execution failed")
        .into_row()
        .await
        .expect("could not read the stored procedure result")
        .expect("stored procedure returned no rows");

    let success_flag: i32 = row.get("Success").expect("missing Success column");

    InsertResult {
        success: success_flag == 1,
        message: row
            .get::<&str, _>("Message")
            .expect("missing Message column")
            .to_string(),
    }
}
