#ifndef BINDING_H
#define BINDING_H
#ifdef __cplusplus
extern "C" {
#endif

struct worker_s;
typedef struct worker_s worker;

struct buf_s {
  void* data;
  size_t len;
};
typedef struct buf_s buf;

const char* get_version();
void set_flags(int* argc, char** argv);

void v8_init();
worker* worker_new();
int worker_load(worker* w, char* name_s, char* source_s);
const char* worker_get_last_exception(worker* w);
void worker_dispose(worker* w);
void worker_terminate_execution(worker* w);
void worker_set_rust_callback(worker*, void*);
void worker_set_rust_object(worker*, void*);

buf_s* recv(void*, int, void*);
int worker_send_bytes(worker* w, void* data, size_t len);

//call js function from rs
void rs2js_worker_run_task(worker* w, void* task_obj, size_t task_obj_size);



#ifdef __cplusplus
}  // extern "C"
#endif
#endif  // BINDING_H
