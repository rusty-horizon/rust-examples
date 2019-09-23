extern crate sdl2_sys;
extern crate nx;

fn main() {

    unsafe {

        sdl2_sys::SDL_Init(sdl2_sys::SDL_INIT_EVERYTHING as u32);
        let window = sdl2_sys::SDL_CreateWindow(b"SDL2-simple" as *const _ as *const u8, 0, 0, 1280, 720, 0);
        let renderer = sdl2_sys::SDL_CreateRenderer(window, 0, 2 | 4);
        sdl2_sys::SDL_SetRenderDrawBlendMode(renderer, 1);
        sdl2_sys::SDL_SetHint(b"SDL_RENDER_SCALE_QUALITY" as *const _ as *const u8, b"2" as *const _ as *const u8);

        loop {

            // Press A to exit the demo.
            let ipt = nx::hid::input_down(nx::hid::Controller::Auto);
            if nx::input_any!(ipt, nx::hid::Key::A) {
                break;
            }

            sdl2_sys::SDL_SetRenderDrawColor(renderer, 235, 235, 235, 255);
            sdl2_sys::SDL_RenderClear(renderer);

            // Draw a dummy cyan rectangle
            let rect = sdl2_sys::SDL_Rect {
                x: 10,
                y: 10,
                w: 200,
                h: 200
            };
            sdl2_sys::SDL_SetRenderDrawColor(renderer, 0, 255, 255, 255);
            sdl2_sys::SDL_RenderFillRect(renderer, &rect);
            
            sdl2_sys::SDL_RenderPresent(renderer);
        }

        sdl2_sys::SDL_DestroyRenderer(renderer);
        sdl2_sys::SDL_DestroyWindow(window);
        sdl2_sys::SDL_Quit();
    }
}