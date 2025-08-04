#include "session.h"

// Create a new session
SESSION* session_create(int socket_fd) {
    SESSION* session = (SESSION*)malloc(sizeof(SESSION));
    if (!session) {
        printf("Error: Failed to allocate memory for session\n");
        return NULL;
    }
    
    // Initialize basic fields
    memset(session, 0, sizeof(SESSION));
    session->socket_fd = socket_fd;
    session->ssl_context = NULL;
    session->halt_flag = false;
    session->is_authenticated = false;
    session->is_connected = true;
    session->threads_started = false;
    
    // Initialize timing
    session->created_time = time(NULL);
    session->last_activity = session->created_time;
    
    // Create buffers
    session->send_buffer = buffer_create(MAX_BUFFER_SIZE);
    session->recv_buffer = buffer_create(MAX_BUFFER_SIZE);
    session->message_queue = queue_create(100);
    
    if (!session->send_buffer || !session->recv_buffer || !session->message_queue) {
        printf("Error: Failed to create session buffers\n");
        session_destroy(session);
        return NULL;
    }
    
    // Initialize mutex
    if (pthread_mutex_init(&session->session_lock, NULL) != 0) {
        printf("Error: Failed to initialize session mutex\n");
        session_destroy(session);
        return NULL;
    }
    
    // Get client IP address
    socklen_t addr_len = sizeof(session->client_addr);
    if (getpeername(socket_fd, (struct sockaddr*)&session->client_addr, &addr_len) == 0) {
        inet_ntop(AF_INET, &session->client_addr.sin_addr, 
                 session->client_ip, sizeof(session->client_ip));
    } else {
        strcpy(session->client_ip, "unknown");
    }
    
    printf("Session created for client: %s\n", session->client_ip);
    return session;
}

// Destroy a session and free all resources
void session_destroy(SESSION* session) {
    if (!session) return;
    
    printf("Destroying session for client: %s\n", session->client_ip);
    
    // Stop session if still running
    session_stop(session);
    
    // Clean up buffers
    if (session->send_buffer) {
        buffer_destroy(session->send_buffer);
    }
    if (session->recv_buffer) {
        buffer_destroy(session->recv_buffer);
    }
    if (session->message_queue) {
        queue_destroy(session->message_queue);
    }
    
    // Clean up SSL
    if (session->ssl_context) {
        SSL_free(session->ssl_context);
    }
    
    // Close socket
    if (session->socket_fd > 0) {
        close(session->socket_fd);
    }
    
    // Destroy mutex
    pthread_mutex_destroy(&session->session_lock);
    
    // Free session memory
    free(session);
}

// Send data through the session
bool session_send_data(SESSION* session, const void* data, size_t size) {
    if (!session || !data || size == 0 || session->halt_flag) {
        return false;
    }
    
    pthread_mutex_lock(&session->session_lock);
    
    bool success = false;
    ssize_t bytes_sent = 0;
    
    if (session->ssl_context) {
        // SSL send
        bytes_sent = SSL_write(session->ssl_context, data, size);
        success = (bytes_sent > 0);
    } else {
        // Regular socket send
        bytes_sent = send(session->socket_fd, data, size, MSG_NOSIGNAL);
        success = (bytes_sent > 0);
    }
    
    if (success) {
        session->bytes_sent += bytes_sent;
        session->packets_sent++;
        session_update_activity(session);
        printf("Sent %zd bytes to %s\n", bytes_sent, session->client_ip);
    } else {
        printf("Error: Failed to send data to %s\n", session->client_ip);
        session->is_connected = false;
    }
    
    pthread_mutex_unlock(&session->session_lock);
    return success;
}

// Receive data from the session
int session_receive_data(SESSION* session, void* buffer, size_t buffer_size) {
    if (!session || !buffer || buffer_size == 0 || session->halt_flag) {
        return -1;
    }
    
    pthread_mutex_lock(&session->session_lock);
    
    ssize_t bytes_received = 0;
    
    if (session->ssl_context) {
        // SSL receive
        bytes_received = SSL_read(session->ssl_context, buffer, buffer_size);
    } else {
        // Regular socket receive
        bytes_received = recv(session->socket_fd, buffer, buffer_size, 0);
    }
    
    if (bytes_received > 0) {
        session->bytes_received += bytes_received;
        session->packets_received++;
        session_update_activity(session);
        printf("Received %zd bytes from %s\n", bytes_received, session->client_ip);
    } else if (bytes_received == 0) {
        printf("Client %s disconnected\n", session->client_ip);
        session->is_connected = false;
    } else {
        printf("Error: Failed to receive data from %s\n", session->client_ip);
        session->is_connected = false;
    }
    
    pthread_mutex_unlock(&session->session_lock);
    return bytes_received;
}

// Check if session is still active
bool session_is_active(SESSION* session) {
    if (!session) return false;
    
    pthread_mutex_lock(&session->session_lock);
    
    bool active = session->is_connected && !session->halt_flag;
    
    // Check for timeout
    time_t now = time(NULL);
    if (now - session->last_activity > SESSION_TIMEOUT) {
        printf("Session timeout for client: %s\n", session->client_ip);
        session->is_connected = false;
        active = false;
    }
    
    pthread_mutex_unlock(&session->session_lock);
    return active;
}

// Update last activity timestamp
void session_update_activity(SESSION* session) {
    if (session) {
        session->last_activity = time(NULL);
    }
}

// Stop the session
void session_stop(SESSION* session) {
    if (!session) return;
    
    printf("Stopping session for client: %s\n", session->client_ip);
    
    pthread_mutex_lock(&session->session_lock);
    session->halt_flag = true;
    session->is_connected = false;
    pthread_mutex_unlock(&session->session_lock);
    
    // Wait for threads to finish if they were started
    if (session->threads_started) {
        pthread_join(session->recv_thread, NULL);
        pthread_join(session->send_thread, NULL);
        session->threads_started = false;
    }
}
