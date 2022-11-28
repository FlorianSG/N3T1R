#ifndef N3T1R_H
#define N3T1R_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#define N3T1R_MAXIMUM_DATA_LEN 255

typedef struct IRCommunicationHandler IRCommunicationHandler;

typedef struct Result_String Result_String;

typedef struct Vec_String Vec_String;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void n3t1r_str_free(char *str_ptr);

bool n3t1r_result_is_error(struct Result_String *result_ptr);

char *n3t1r_result_get_error_message(struct Result_String *result_ptr);

void n3t1r_result_free(struct Result_String *result_ptr);

struct Vec_String *n3t1r_vec_string_new(void);

void n3t1r_vec_string_free(struct Vec_String *vector_ptr);

size_t n3t1r_vec_string_len(struct Vec_String *vector_ptr);

char *n3t1r_vec_string_get(struct Vec_String *vector_ptr, size_t index);

struct Result_String *n3t1r_get_available_serial_ports(struct Vec_String *names_vector_ptr, struct Vec_String *descriptions_vector_ptr);

struct Result_String *n3t1r_get_available_rooms(struct Vec_String *vector_ptr);

struct IRCommunicationHandler *n3t1r_irch_new(void);

void n3t1r_irch_free(struct IRCommunicationHandler *instance_ptr);

void n3t1r_irch_select_serial_backend(struct IRCommunicationHandler *instance_ptr, const char *port_name_ptr);

void n3t1r_irch_select_rendezvous_backend(struct IRCommunicationHandler *instance_ptr, const char *room_name_ptr);

void n3t1r_irch_select_network_backend(struct IRCommunicationHandler *instance_ptr, uint16_t source_port, const char *destination_host_ptr, uint16_t destination_port);

struct Result_String *n3t1r_irch_enable(struct IRCommunicationHandler *instance_ptr);

void n3t1r_irch_disable(struct IRCommunicationHandler *instance_ptr);

struct Result_String *n3t1r_irch_send(struct IRCommunicationHandler *instance_ptr, const uint8_t *data_ptr, size_t data_len);

struct Result_String *n3t1r_irch_receive(struct IRCommunicationHandler *instance_ptr, uint8_t *data_ptr, size_t *data_len_ptr);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* N3T1R_H */
