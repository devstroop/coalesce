#ifndef SESSION_H
#define SESSION_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <pthread.h>
#include <stdbool.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <openssl/ssl.h>

#define MAX_BUFFER_SIZE 8192
#define MAX_CONNECTIONS 1000
#define SESSION_TIMEOUT 3600

// Forward declarations
typedef struct BUFFER BUFFER;
typedef struct SESSION SESSION;
typedef struct QUEUE QUEUE;

// Buffer structure for network data
typedef struct BUFFER {
    char* data;
    size_t size;
    size_t capacity;
    size_t read_pos;
    size_t write_pos;
} BUFFER;

// Simple queue for message passing
typedef struct QUEUE {
    void** items;
    size_t capacity;
    size_t size;
    size_t head;
    size_t tail;
    pthread_mutex_t mutex;
    pthread_cond_t not_empty;
    pthread_cond_t not_full;
} QUEUE;

// Network session structure (inspired by SoftEtherVPN)
typedef struct SESSION {
    int socket_fd;
    SSL* ssl_context;
    struct sockaddr_in client_addr;
    
    // Threading
    pthread_t recv_thread;
    pthread_t send_thread;
    bool threads_started;
    
    // Buffers
    BUFFER* send_buffer;
    BUFFER* recv_buffer;
    QUEUE* message_queue;
    
    // State management
    bool halt_flag;
    bool is_authenticated;
    bool is_connected;
    
    // Session info
    char client_ip[16];
    time_t created_time;
    time_t last_activity;
    
    // Synchronization
    pthread_mutex_t session_lock;
    
    // Statistics
    unsigned long bytes_sent;
    unsigned long bytes_received;
    unsigned int packets_sent;
    unsigned int packets_received;
} SESSION;

// Function prototypes
SESSION* session_create(int socket_fd);
void session_destroy(SESSION* session);
bool session_start(SESSION* session);
void session_stop(SESSION* session);
bool session_send_data(SESSION* session, const void* data, size_t size);
int session_receive_data(SESSION* session, void* buffer, size_t buffer_size);
bool session_is_active(SESSION* session);
void session_update_activity(SESSION* session);

// Buffer functions
BUFFER* buffer_create(size_t capacity);
void buffer_destroy(BUFFER* buffer);
bool buffer_write(BUFFER* buffer, const void* data, size_t size);
size_t buffer_read(BUFFER* buffer, void* data, size_t size);
void buffer_clear(BUFFER* buffer);
size_t buffer_available_read(BUFFER* buffer);
size_t buffer_available_write(BUFFER* buffer);

// Queue functions
QUEUE* queue_create(size_t capacity);
void queue_destroy(QUEUE* queue);
bool queue_push(QUEUE* queue, void* item);
void* queue_pop(QUEUE* queue);
size_t queue_size(QUEUE* queue);

#endif // SESSION_H
