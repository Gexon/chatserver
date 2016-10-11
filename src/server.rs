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

    /// Старт сервака собственно.
    /// тут же бесконечный цикл обработки событйи.
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

    /// Регистрация серверного опросника событий.
    ///
    /// This keeps the registration details neatly tucked away inside of our implementation.
    /// Это хранит регистрационные данные аккуратно спрятан внутри нашей реализации.
    pub fn register(&mut self, poll: &mut Poll) -> io::Result<()> {
        poll.register(
            &self.sock,
            self.token,
            Ready::readable(),
            PollOpt::edge()
        ).or_else(|e| {
            error!("Ошибка регистрации опросника событий {:?}, {:?}", self.token, e);
            Err(e)
        })
    }

    /// обработчик события
    fn ready(&mut self, poll: &mut Poll, token: Token, event: Ready) {
        debug!("токен {:?} событие = {:?}", token, event);

        if event.is_error() {
            warn!("Ошибка события токена{:?}", token);
            self.find_connection_by_token(token).mark_reset(); // пометить на сброс соединения
            return;
        }

        if event.is_hup() {
            trace!("Событие обрыва соединения(Hup event for) токена {:?}", token);
            self.find_connection_by_token(token).mark_reset();
            return;
        }

        // Мы не обнаружили ошибок записи событий для токена нашего сервера.
        // Запись события для прочих токенов, должны передаваться этому подключению.
        if event.is_writable() {
            trace!("Записываем событие для токена {:?}", token);
            assert!(self.token != token, "Получение записанного события для Сервера");

            let conn = self.find_connection_by_token(token);

            if conn.is_reset() {
                info!("{:?} соединение сброшено", token);
                return;
            }

            conn.writable()
                .unwrap_or_else(|e| {
                    warn!("Ошибка записи события для токена {:?}, {:?}", token, e);
                    conn.mark_reset();
                });
        }

        // A read event for our `Server` token means we are establishing a new connection.
        // Событие чтения для токена нашего сервера означает, что мы устанавливаем новое соединение.
        // A read event for any other token should be handed off to that connection.
        // Событие чтения для любого другого токена должны быть передан этому соединению.
        if event.is_readable() {
            trace!("Читаем событие для токена {:?}", token);
            if self.token == token {
                self.accept(poll);
            } else {

                if self.find_connection_by_token(token).is_reset() {
                    info!("{:?} соединение сброшено", token);
                    return;
                }

                self.readable(token)
                    .unwrap_or_else(|e| {
                        warn!("Ошибка чтения события для токена {:?}: {:?}", token, e);
                        self.find_connection_by_token(token).mark_reset();
                    });
            }
        }

        if self.token != token {
            self.find_connection_by_token(token).mark_idle();
        }
    }

}