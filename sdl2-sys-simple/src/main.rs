extern crate sdl2;

fn main() {

    unsafe {

        sdl2::SDL_Init(sdl2::SDL_INIT_EVERYTHING as u32);
        let window = sdl2::SDL_CreateWindow(b"SDL2-demo" as *const u8, 0, 0, 1280, 720, 0);
        let renderer = sdl2::SDL_CreateRenderer(window, 0, 0x00000002 | 0x00000004);

        loop {

            sdl2::SDL_SetRenderDrawColor(renderer, 255, 255, 255, 255);
            sdl2::SDL_RenderClear(renderer);

            sdl2::SDL_SetRenderDrawColor(renderer, 0, 0, 255, 255);
            sdl2::SDL_RenderDrawLine(renderer, 10, 10, 500, 500);

            sdl2::SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);
            sdl2::SDL_RenderDrawLine(renderer, 10, 500, 500, 10);

            sdl2::SDL_RenderPresent(renderer);
        }
    }
}