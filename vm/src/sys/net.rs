use std::collections::{HashMap, VecDeque};
use std::os::fd::RawFd;
use std::thread;
use std::net::{TcpListener, TcpStream};
use std::os::fd::AsRawFd;
use std::io::{self, Read};
use std::sync::{Arc, Weak, Mutex};
use crate::vm::{VM, Value, ExitReason};

// State associated with a socket
pub struct Socket
{
    fd: RawFd,

    /// Incoming connections
    incoming: VecDeque<TcpStream>,

    // TODO: read buffer
}

// State for the networking subsystem
pub struct NetState
{
    /// Next socket id to use
    next_id: u64,

    /// Map of open sockets
    sockets: HashMap<u64, Socket>,
}

impl Default for NetState
{
    fn default() -> Self
    {
        Self {
            // Start at FFFF so we can reserve the low values for error codes
            next_id: 0xFF_FF,
            sockets: HashMap::default(),
        }
    }
}

/// TCP listening thread
fn listen_thread(
    vm_mutex: Weak<Mutex<VM>>,
    listener: TcpListener,
    socket_id: u64,
    on_new_conn: u64
)
{
    // Block until a connection can be accepted
    for result in listener.incoming() {

        let arc = vm_mutex.upgrade().unwrap();
        let mut vm = arc.lock().unwrap();

        // TODO: note, accepting the connection may error,
        // for example if the socket was closed
        let stream = result.unwrap();

        // Add the new connection to the queue
        let mut net_state = &mut vm.sys_state.net_state;
        match net_state.sockets.get_mut(&socket_id) {
            Some(socket) => {
                socket.incoming.push_back(stream);
            }
            _ => panic!()
        }

        // Call on_new_conn to signal an incoming connection
        match vm.call(on_new_conn, &[Value::from(socket_id)]) {
            ExitReason::Return(val) => {}
            _ => panic!()
        }
    }
}

// Syscall to create a TCP listening socket to accept incoming connections
// u64 socket_id = net_listen_tcp(
//     u16 port_no,
//     ip_space, // IPV4 / IPV6
//     const char* address, // Network interface address to listen on, null for any address
//     callback on_new_connection, // Called on new incoming connection
//     u64 flags // optional flags, default 0
// )
pub fn net_listen_tcp(
    vm: &mut VM,
    port_no: Value,
    ip_space: Value,
    bind_address: Value,
    on_new_conn: Value,
) -> Value
{
    // TODO: accept input address and port
    // TODO: VM helper function to read UTF-8 string?

    // TODO: return 0 on failure
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let socket_fd = listener.as_raw_fd();

    // Assign a socket id to the socket
    let mut net_state = &mut vm.sys_state.net_state;
    let socket_id = net_state.next_id;
    net_state.next_id += 1;
    net_state.sockets.insert(
        socket_id,
        Socket {
            fd: socket_fd,
            incoming: VecDeque::default()
        }
    );

    // Create a listening thread to accept incoming connections
    let vm_mutex = vm.sys_state.mutex.clone();
    let on_new_conn = on_new_conn.as_u64();
    thread::spawn(move || {
        listen_thread(
            vm_mutex,
            listener,
            socket_id,
            on_new_conn,
        )
    });

    // Return the socket id
    Value::from(socket_id)
}









// Syscall to accept a new connection
// Writes the client address in the buffer you specify
// u64 socket_id = net_accept(u64 socket_id, client_addr_t *client_addr, callback on_incoming_data)
pub fn net_accept(
    vm: &mut VM,
    socket_id: Value,
    client_addr: Value,
    client_addr_len: Value,
    on_incoming_data: Value,
) -> Value
{
    let socket_id = socket_id.as_u64();
    let client_addr = client_addr.as_u64();
    let client_addr_len = client_addr_len.as_u64();
    let on_incoming_data = on_incoming_data.as_u64();

    let mut net_state = &mut vm.sys_state.net_state;

    // If there is a connection waiting
    match net_state.sockets.get_mut(&socket_id) {
        Some(socket) => {
            if socket.incoming.len() == 0 {
                panic!();
            }

            let stream = socket.incoming.pop_front().unwrap();
            let socket_fd = stream.as_raw_fd();

            //
            // TODO: we need to write the client address into the buffer
            //




            /*
            // Assign a socket id to the socket
            let mut net_state = &mut vm.sys_state.net_state;
            let socket_id = net_state.next_id;
            net_state.next_id += 1;
            net_state.sockets.insert(
                socket_id,
                Socket {
                    fd: socket_fd,
                    incoming: VecDeque::default()
                }
            );

            // Create a listening thread to accept incoming connections
            let vm_mutex = vm.sys_state.mutex.clone();
            let on_new_conn = on_new_conn.as_u64();
            thread::spawn(move || {
                listen_thread(
                    vm_mutex,
                    listener,
                    socket_id,
                    on_new_conn,
                )
            });
            */


            // Return the socket id
            //Value::from(socket_id)


            todo!();


        }
        _ => panic!()
    }
}

// Syscall to read data from a given socket into a buffer you specify
// u64 num_bytes_read = net_read(u64 socket_id, void* buffer, u64 buf_len)
pub fn net_read(
    vm: &mut VM,
    socket_id: Value,
    buffer: Value,
    buf_len: Value,
) -> Value
{
    todo!();
}

// Syscall to write data on a given socket
// void net_write(u64 socket_id, void* buffer, u64 buf_len);
pub fn net_write(
    vm: &mut VM,
    socket_id: Value,
    buffer: Value,
    buf_len: Value,
) -> Value
{
    todo!();
}

// Syscall to close a socket
// net_close(u64 socket_id)
pub fn net_close(
    vm: &mut VM,
    socked_id: Value,
)
{
    todo!();
}
