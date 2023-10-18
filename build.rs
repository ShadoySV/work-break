fn main() {
    #[cfg(windows)]
    embed_resource::compile("./windows/work-break.rc", embed_resource::NONE);
}
