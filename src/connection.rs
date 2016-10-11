use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::rc::Rc;

use byteorder::{ByteOrder, BigEndian};

use mio::*;
use mio::tcp::*;

/// Обертка вокруг неблокирующих сокетов.
/// Это Connection не соединяется с Сервером.
/// Это Connection представляет клиентские подключения,
/// принимаемых Серверными подключениями.
pub struct Connection {
    // handle подключенного сокета
    sock: TcpStream,

    // токен для регистрации в опроснике событий
    pub token: Token,

    // интересующий набор событий
    interest: Ready,

    // очередь отправляемых сообщений
    send_queue: Vec<Rc<Vec<u8>>>,

    // отслеживать ли необходимость в перерегистрации
    is_idle: bool,

    // отслеживать ли сброс соединения
    is_reset: bool,

    // отслеживать ли при чтении получение `WouldBlock`
    // и хранит количество байт что мы должны читать.
    // track whether a read received `WouldBlock` and store the number of
    // byte we are supposed to read
    read_continuation: Option<u64>,

    // отслеживать ли запись получил `WouldBlock`
    write_continuation: bool,
}