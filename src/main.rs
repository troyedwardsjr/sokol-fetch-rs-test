use toxoid_sokol::sokol::{app as sapp, gfx as sg};
use toxoid_sokol::bindings::*;
use toxoid_sokol::sokol::*;
use toxoid_ffi::*;

extern "C" fn init_cb() {
    println!("Sokol initialized");
    let mut sfetch_desc: sfetch_desc_t = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    sfetch_desc.max_requests = 3;
    sfetch_desc.num_channels = 1;
    sfetch_desc.num_lanes = 1;
    sfetch_desc.logger.func = Some(log::slog_func);     
    unsafe {
        sfetch_setup(&mut sfetch_desc);
        if !sfetch_valid() {
            panic!("sfetch is not valid");
        }
    }   
    // 2008 kb
    let buffer = vec![0u8; 2008 * 1024];
    // Box buffer
    let buffer = Box::into_raw(Box::new(buffer));
    unsafe {
        let mut sfetch_request: sfetch_request_t = core::mem::MaybeUninit::zeroed().assume_init();
        sfetch_request.path = "kitten.jpg\0".as_ptr() as *const i8;
        sfetch_request.channel = 0;
        sfetch_request.buffer = sfetch_range_t {
            ptr: buffer as *const u8 as *const core::ffi::c_void,
            size: 512 * 1024 
        };
        sfetch_request.callback = Some(data_loaded);
        sfetch_send(&sfetch_request);
    }
}

extern "C" fn frame_cb() {
    unsafe { sfetch_dowork() }
}


extern "C" fn cleanup_cb() {
    println!("Sokol cleanup");
    sg::shutdown()
}

extern "C" fn data_loaded(response: *const sfetch_response_t) {
    println!("Data loaded: {:#?}", unsafe { (*response).buffer });
}

fn main() {
    let window_title = b"Sokol Fetch Test\0".as_ptr() as _;
    // Initialize renderer
    sapp::run(&sapp::Desc {
        init_cb: Some(init_cb),
        cleanup_cb: Some(cleanup_cb),
        frame_cb: Some(frame_cb),
        window_title,
        width: 100,
        height: 100,
        sample_count: 1,
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}
