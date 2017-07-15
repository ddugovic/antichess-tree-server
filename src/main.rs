extern {
    fn __errno_location() -> *mut i32;
    fn mov_parse(uci : *const u8) -> u16;
    fn mov_uci(mov : u16, uci : *mut u8);
    fn printf(__format : *const u8, ...) -> i32;
    fn query_result_clear(result : *mut query_result);
    fn query_result_sort(result : *mut query_result);
    fn strerror(__errnum : i32) -> *mut u8;
    fn tree_open(tree : *mut tree, filename : *const u8) -> bool;
    fn tree_query(
        tree : *mut tree,
        movs : *const u16,
        movs_len : usize,
        result : *mut query_result
    ) -> bool;
}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[derive(Copy)]
#[repr(C)]
pub struct node {
    pub data : u32,
    pub mov : u16,
}

impl Clone for node {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct hash_entry {
    pub index : u32,
    pub size : u32,
}

impl Clone for hash_entry {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct tree {
    pub fd : i32,
    pub prolog_len : u32,
    pub prolog : *mut u16,
    pub root : *mut node,
    pub num_pages : usize,
    pub size : u32,
    pub nodes : *mut node,
    pub hashtable : *mut hash_entry,
    pub num_hash_entries : usize,
    pub arr : *mut usize,
}

impl Clone for tree {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct query_result {
    pub movs : [u16; 256],
    pub sizes : [u32; 256],
    pub num_children : usize,
}

impl Clone for query_result {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn _c_main() -> i32 {
    let mut tree : tree;
    if !tree_open(
            &mut tree as (*mut tree),
            (*b"easy18.done\0").as_ptr()
        ) {
        printf(
            (*b"could not open tree: %s\n\0").as_ptr(),
            strerror(*__errno_location())
        );
        1i32
    } else {
        let mut result : query_result;
        query_result_clear(&mut result as (*mut query_result));
        let mut movs
            : *mut u16
            = mov_parse((*b"e2e3\0").as_ptr()) as (*mut u16);
        let mut movs_len : usize = 1usize;
        (if !tree_query(
                 &mut tree as (*mut tree),
                 movs as (*const u16),
                 movs_len,
                 &mut result as (*mut query_result)
             ) {
             printf((*b"query failed\n\0").as_ptr());
             1i32
         } else {
             query_result_sort(&mut result as (*mut query_result));
             let mut i : usize = 0usize;
             'loop3: loop {
                 if !(i < result.num_children) {
                     break;
                 }
                 let mut uci : [u8; 8];
                 mov_uci(result.movs[i],uci.as_mut_ptr());
                 printf((*b"%s %d\n\0").as_ptr(),uci.as_mut_ptr(),result.sizes[i]);
                 i = i.wrapping_add(1usize);
             }
             0i32
         })
    }
}
