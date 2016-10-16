use time;

use dbqury as db;

// принимаем чатик.
pub fn chat(args: &str, auth_token: &i64, reset_conn: &bool) -> (String, i64, bool) {
    // инициализация возвращаемых значений.
    let return_msg: String;
    let mut return_token: i64 = *auth_token;
    let mut return_reset: bool = *reset_conn;

    // для отладки.
    println!("recv>{}", args);

    // вынимаем из пришедшей строки имя
    let vec_msg: Vec<&str> = args.splitn(2, ' ').collect();
    // vec_msg[0] - имя, vec_msg[1] - текст сообщения

    // проверяем наличие авторизационного токена
    let zero: i64 = 0;

    if auth_token.eq(&zero) {
        // берем из БД а-токен.
        return_token = db::get_token(vec_msg[0]);
        //println!("token from db {}", return_token);
    }

    // проверяем актуальность токена
    if check_token(&return_token) {
        let head_msg: &str = "chat_all";
        return_msg = format!("{} {}", head_msg, args);
    } else {
        // а-токен не актуален, порвать соединение.
        return_reset = true;
        return_msg = "Токен авторизации просрочен.".to_string();
    }

        (return_msg, return_token, return_reset)

    //    let merged: String = args.iter()
    //        .flat_map(|s| s.chars().chain(" ".chars()))
    //        .collect();

    /*let mut owned_string: String = "hello ".to_owned();
    let borrowed_string: &str = "world";
    owned_string.push_str(borrowed_string);
    println!("{}", owned_string);*/
}

// проверяем актуальность токена
pub fn check_token(auth_token: &i64) -> bool {
    // расшифровываем
    // получаем текущую дату и сравниваем не истек ли токен.
    let current_time = time::get_time();
    //println!("current_time {}", current_time.sec);
    if auth_token > &current_time.sec {
        return true
    }

    false
}