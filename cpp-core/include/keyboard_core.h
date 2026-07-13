#pragma once 


#ifdef _WIN32 
    #define KB_API __declspec(dllexport)
#else
    #define KB_API
#endif


#ifdef __cplusplus
extern "C" {
#endif

    KB_API bool kb_init();
    KB_API void kb_shutdown();
    KB_API void kb_poll();
    KB_API bool kb_is_key_down(int virtual_key);
    KB_API bool kb_register_raw_input(void* hwnd);
    KB_API void kb_process_raw_input(void* hrawinput);
    KB_API bool kb_raw_input_enabled();
    KB_API int kb_device_count();
    KB_API int kb_device_name(int index, char* buffer, int buffer_size);

#ifdef __cplusplus
}
#endif
