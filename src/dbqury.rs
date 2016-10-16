extern crate mysql;

///// получаем токен из БД
//pub fn get_token(name: &str) -> i64 {
//    let pool = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();
//    let mut stmt0 = pool.prepare("  SELECT token \
//                                    FROM dotakiller.accounts \
//                                    WHERE name=?", ).unwrap();
//    for row in stmt0.execute((name, )).unwrap() {
//        let ret_token: i64 = mysql::from_row::<i64>(row.unwrap());
//        return ret_token;
//    }
//    0
//}

/// получаем имя из БД, по токену.
pub fn get_name(auth_token: &i64) -> String {
    let pool = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();
    let mut stmt0 = pool.prepare("  SELECT name \
                                    FROM dotakiller.accounts \
                                    WHERE token=?", ).unwrap();
    for row in stmt0.execute((auth_token, )).unwrap() {
        let ret_name: String = mysql::from_row::<String>(row.unwrap());
        return ret_name;
    }
    String::from("")
}