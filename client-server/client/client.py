import socket


def main():
    sock = socket.socket()
    sock.connect(('127.0.0.1', 3000))
    sock.sendall(b'3')
    data = sock.recv(4096)
    print(data)
    sock.close()


main()
