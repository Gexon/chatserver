use std::io::{self, ErrorKind};
use std::rc::Rc;

use mio::*;
use mio::tcp::*;
use slab;

use connection::Connection;

type Slab<T> = slab::Slab<T, Token>;

pub struct Server {
    // главное гнездо нашего сервера
    sock: TcpListener,

    // токен нашего сервера. мы отслеживаем его здесь вместо `const SERVER = Token(0)`.
    token: Token,

    // список подключений нашего сервера _accepted_ by
    conns: Slab<Connection>,

    // список событий для обработки
    events: Events,
}

impl Server {
    pub fn new(sock: TcpListener) -> Server {
        Server {
            sock: sock,

            // Даем нашему серверу токен с номером большим чем может поместиться в нашей плите 'Slab'.
            // Политы 'Slab' используються только для внутреннего смещения.
            token: Token(10_000_000),

            // SERVER is Token(1), после запуска такого сервака
            // мы сможет подключить не больше 128 клиентов.
            conns: Slab::with_capacity(128),

            // Список событий что сервер должен обработать.
            events: Events::with_capacity(1024),
        }
    }

    pub fn run(&mut self, poll: &mut Poll) -> io::Result<()> {
        try!(self.register(poll));

        info!("Запуск сервера, запуск цикла...");
        loop {
            let cnt = try!(poll.poll(&mut self.events, None));

            let mut i = 0;

            trace!("обработка событий... cnt={}; len={}", cnt, self.events.len());

            // Перебираем уведомления.
            // Каждое из этих событий дает token для регистрации
            // (который обычно представляет собой, handle события),
            // а также информацию о том, какие события происходили (чтение, запись, сигнал, и т. д.)
            while i < cnt {
                // TODO this would be nice if it would turn a Result type. trying to convert this
                // into a io::Result runs into a problem because .ok_or() expects std::Result and
                // not io::Result
                let event = self.events.get(i).expect("Ошибка получения события");

                trace!("event={:?}; idx={:?}", event, i);
                self.ready(poll, event.token(), event.kind());

                i += 1;
            }

            self.tick(poll);
        }
    }
}