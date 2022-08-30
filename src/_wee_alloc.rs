// Source: https://yew.rs/docs/advanced-topics/optimizations
// Source: https://github.com/rustwasm/wee_alloc
// Use `wee_alloc` as the global allocator.

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;
