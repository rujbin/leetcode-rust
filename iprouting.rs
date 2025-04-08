enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {
        x: i32, y: i32
     },
        Write(String),
        ChangeColor(i32, i32, i32),
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("Routing für IPv4"),
        IpAddrKind::V6 => println!("Routing für IPv6"),
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Nachricht: Quit erhalten.");
        }
        Message::Move { x, y } =>  {
            println!("Nachrihct: Move erhalten. Neue Position: x = {}, y = {}", x, y);
        }
        Message::Write(text) => {
            println!("Nachricht: Write erhalten. Text: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Nachricht: ChangeColor erhalten. Neue Farbe: R = {}, G = {}, B = {}", r, g, b);
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Aufruf der route Funktion: ");
    route(four);
    route(six);
    println!("-----");

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Beispiel IP-Adressen erstellt.");

    match home {
        IpAddr::V4(a, b, c, d) => println!("Home ist eine IPv4-Adresse: {} {} {} {}", a, b, c, d),
        IpAddr::V6(addr) => println!("Loopback ist eine IPv6-Adresse: {}", addr),
    }
    match loopback {
        IpAddr::V4(a,b,c,d) => println!("Loopback ist eine IPv4-Adresse Adresse: {} {} {} {}", a, b, c, d),
        IpAddr::V6(addr) => println!("Loopback ist eine IPv6-Adresse: {}", addr),
    }
    println!("----");

    let m_quit = Message::Quit;
    let m_move = Message::Move { x: 5, y: 20 };
    let m_write = Message::Write(String::from("Hallo Feride!"));
    let m_color = Message::ChangeColor(255, 0, 128);
    
    println!("Verarbeitung Nachrichten: ");
    process_message(m_quit);
    process_message(m_move);
    process_message(m_write);
    process_message(m_color);
    println!("------");
}

