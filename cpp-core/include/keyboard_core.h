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

#ifdef __cplusplus
}
#endif