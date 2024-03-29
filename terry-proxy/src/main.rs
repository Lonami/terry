use once_cell::sync::OnceCell;
use std::fs::File;
use std::io::{BufRead as _, BufWriter, Write as _};
use std::sync::Mutex;
use std::thread;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _, Result};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

#[derive(Debug)]
struct Config {
    server_parser: terraria_protocol::Parser,
    client_parser: terraria_protocol::Parser,
    server_traffic: Option<BufWriter<File>>,
    client_traffic: Option<BufWriter<File>>,
    flush_traffic: [bool; 2],
    dbg_in_tags: [bool; 256],
    dbg_out_tags: [bool; 256],
}

static CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();
const SERVER_TRAFFIC: &str = "server-traffic.bin";
const CLIENT_TRAFFIC: &str = "client-traffic.bin";

const SERVER_ADDR: &str = "127.0.0.1:7777";
const BIND_ADDR: &str = "127.0.0.1:8888";
const BUFFER_SIZE: usize = 4 * 1024;

fn toggle_cfg_tags(config: &mut Config, dir: &str, tag: &str, value: bool) -> &'static str {
    let (in_, out) = match dir {
        "in" => (true, false),
        "out" => (false, true),
        "both" => (true, true),
        _ => return "Invalid direction",
    };

    if tag == "all" {
        if in_ {
            config.dbg_in_tags.iter_mut().for_each(|t| *t = value);
        }
        if out {
            config.dbg_out_tags.iter_mut().for_each(|t| *t = value);
        }
        return "Success";
    }

    match tag.parse::<u8>() {
        Ok(t) => {
            if in_ {
                config.dbg_in_tags[t as usize] = value;
            }
            if out {
                config.dbg_out_tags[t as usize] = value;
            }
            "Success"
        }
        Err(_) => "Failed to parse tag number",
    }
}

fn user_input() {
    println!("Now handling user input. Type help for help");
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    loop {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(_) => {}
            Err(_) => break,
        }

        let mut argv = line.split_whitespace();

        match argv.next().unwrap_or("") {
            "" => {}
            "help" => {
                println!("* help: show this message");
                println!("* quit: stop processing stdin");
                println!("* show <in|out|both> <all|TAG>: show the dbg repr of in, out or both messages matching tag");
                println!("* hide <in|out|both> <all|TAG>: hide the dbg repr of in, out or both messages matching tag");
                println!("* list: list all tags along with the name");
                println!("* flush: flush network traffic writes to disk");
                println!("* nosave: stop saving network traffic to disk");
            }
            "quit" => break,
            "show" => {
                if let Some((dir, tag)) = argv.next().zip(argv.next()) {
                    let mut config = CONFIG.get().unwrap().lock().unwrap();
                    toggle_cfg_tags(&mut config, dir, tag, true);
                } else {
                    println!("Both dir and tag must be provided");
                }
            }
            "hide" => {
                if let Some((dir, tag)) = argv.next().zip(argv.next()) {
                    let mut config = CONFIG.get().unwrap().lock().unwrap();
                    toggle_cfg_tags(&mut config, dir, tag, false);
                } else {
                    println!("Both dir and tag must be provided");
                }
            }
            "list" => {
                todo!()
            }
            "flush" => {
                let mut config = CONFIG.get().unwrap().lock().unwrap();
                config.flush_traffic[0] = true;
                config.flush_traffic[1] = true;
            }
            "nosave" => {
                let mut config = CONFIG.get().unwrap().lock().unwrap();
                config.server_traffic.take();
                config.client_traffic.take();
                println!("Dropped open files");
            }
            cmd => {
                println!("Could not understand \"{}\". Type help for help", cmd);
            }
        }
    }
}

async fn start() -> Result<()> {
    CONFIG
        .set(Mutex::new(Config {
            server_parser: terraria_protocol::Parser::new(),
            client_parser: terraria_protocol::Parser::new(),
            server_traffic: Some(BufWriter::new(File::create(SERVER_TRAFFIC)?)),
            client_traffic: Some(BufWriter::new(File::create(CLIENT_TRAFFIC)?)),
            flush_traffic: [false; 2],
            dbg_in_tags: [true; 256],
            dbg_out_tags: [true; 256],
        }))
        .unwrap();

    println!("Binding socket to {}...", BIND_ADDR);
    let listener = TcpListener::bind(BIND_ADDR).await?;
    println!(
        "Socket bound to {}. Accepting incoming client connection...",
        BIND_ADDR
    );
    let (client, client_addr) = listener.accept().await?;
    println!("Accepted client {}!", client_addr);

    println!("Connecting to the server {}...", SERVER_ADDR);
    let server = TcpStream::connect(SERVER_ADDR).await?;
    println!("Connected to the server {}!", SERVER_ADDR);

    let (mut client_rd, mut client_wr) = client.into_split();
    let (mut server_rd, mut server_wr) = server.into_split();

    println!("Launching input thread (UIT)...");
    let input_thread = thread::spawn(|| user_input());

    println!("Launching Server-to-Client task (STC)...");
    let sv_to_cl = task::spawn(async move {
        let mut buffer = vec![0; BUFFER_SIZE];
        loop {
            let n = server_rd.read(&mut buffer).await?;
            {
                let mut config = CONFIG.get().unwrap().lock().unwrap();
                config.server_parser.feed(&buffer[..n]);
                while let Some(packet) = config.server_parser.next() {
                    match packet {
                        Ok(packet) => {
                            if config.dbg_out_tags[packet.tag() as usize] {
                                eprintln!("STC< {} {:#?}", packet.tag(), packet);
                            }
                        }
                        Err(err) => {
                            eprintln!("STC! bad packet: {}", err);
                        }
                    }
                }
                let flush = config.flush_traffic[0];
                if let Some(fd) = &mut config.server_traffic {
                    fd.write_all(&buffer[..n])?;
                    if flush {
                        fd.flush()?;
                        config.flush_traffic[0] = false;
                    }
                }
            }
            client_wr.write_all(&buffer[..n]).await?;
        }
        #[allow(unreachable_code)]
        Result::Ok(())
    });

    println!("Launching Client-to-Server task (CTS)...");
    let cl_to_sv = task::spawn(async move {
        let mut buffer = vec![0; BUFFER_SIZE];
        loop {
            let n = client_rd.read(&mut buffer).await?;
            {
                let mut config = CONFIG.get().unwrap().lock().unwrap();
                config.client_parser.feed(&buffer[..n]);
                while let Some(packet) = config.client_parser.next() {
                    match packet {
                        Ok(packet) => {
                            if config.dbg_out_tags[packet.tag() as usize] {
                                eprintln!("CTS> {} {:#?}", packet.tag(), packet);
                            }
                        }
                        Err(err) => {
                            eprintln!("CTS! bad packet: {}", err);
                        }
                    }
                }
                let flush = config.flush_traffic[1];
                if let Some(fd) = &mut config.client_traffic {
                    fd.write_all(&buffer[..n])?;
                    if flush {
                        fd.flush()?;
                        config.flush_traffic[1] = false;
                    }
                }
            }
            server_wr.write_all(&buffer[..n]).await?;
        }
        #[allow(unreachable_code)]
        Result::Ok(())
    });

    if let Err(err) = sv_to_cl.await.unwrap() {
        eprintln!("Server-to-Client task exited: {}", err);
    }
    if let Err(err) = cl_to_sv.await.unwrap() {
        eprintln!("Client-to-Server task exited: {}", err);
    }
    if let Err(err) = input_thread.join() {
        eprintln!("Input thread exited: {:?}", err);
    }

    Ok(())
}

fn main() -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();

    rt.block_on(start())
}
