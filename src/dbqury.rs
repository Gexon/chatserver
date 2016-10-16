extern crate mysql;

/// получаем токен из БД
pub fn get_token(name: &str) -> i64 {
    let pool = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();
    let mut stmt0 = pool.prepare("  SELECT token \
                                    FROM dotakiller.accounts \
                                    WHERE name=?", ).unwrap();
    for row in stmt0.execute((name, )).unwrap() {
        let ret_token: i64 = mysql::from_row::<i64>(row.unwrap());
        return ret_token;
    }
    0
}