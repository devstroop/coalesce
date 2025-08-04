// C networking code with platform-specific socket handling
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
    #include <winsock2.h>
    #include <ws2tcpip.h>
    #pragma comment(lib, "ws2_32.lib")
    typedef SOCKET socket_t;
    #define INVALID_SOCK INVALID_SOCKET
    #define close_socket closesocket
#else
    #include <sys/socket.h>
    #include <arpa/inet.h>
    #include <unistd.h>
    typedef int socket_t;
    #define INVALID_SOCK -1
    #define close_socket close
#endif

typedef struct {
    socket_t sock;
    struct sockaddr_in addr;
    int connected;
} tcp_client_t;

tcp_client_t* tcp_client_create(const char* host, int port) {
    tcp_client_t* client = malloc(sizeof(tcp_client_t));
    if (!client) return NULL;
    
    // Initialize Winsock on Windows
    #ifdef _WIN32
    WSADATA wsaData;
    if (WSAStartup(MAKEWORD(2, 2), &wsaData) != 0) {
        free(client);
        return NULL;
    }
    #endif
    
    // Create socket
    client->sock = socket(AF_INET, SOCK_STREAM, 0);
    if (client->sock == INVALID_SOCK) {
        free(client);
        return NULL;
    }
    
    // Setup address
    memset(&client->addr, 0, sizeof(client->addr));
    client->addr.sin_family = AF_INET;
    client->addr.sin_port = htons(port);
    inet_pton(AF_INET, host, &client->addr.sin_addr);
    
    client->connected = 0;
    return client;
}

int tcp_client_connect(tcp_client_t* client) {
    if (!client) return -1;
    
    if (connect(client->sock, (struct sockaddr*)&client->addr, sizeof(client->addr)) < 0) {
        return -1;
    }
    
    client->connected = 1;
    return 0;
}

int tcp_client_send(tcp_client_t* client, const char* data, size_t len) {
    if (!client || !client->connected) return -1;
    return send(client->sock, data, len, 0);
}

int tcp_client_recv(tcp_client_t* client, char* buffer, size_t len) {
    if (!client || !client->connected) return -1;
    return recv(client->sock, buffer, len, 0);
}

void tcp_client_destroy(tcp_client_t* client) {
    if (client) {
        if (client->connected) {
            close_socket(client->sock);
        }
        free(client);
    }
    
    #ifdef _WIN32
    WSACleanup();
    #endif
}
