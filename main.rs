use learning_wgpu::run;

fn main() {
    // std::env::set_var("WINIT_UNIX_BACKEND", "wayland");
    pollster::block_on(run());
}
