extern {
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn calloc(
        __nmemb : usize, __size : usize
    ) -> *mut ::std::os::raw::c_void;
    fn close(__fd : i32) -> i32;
    fn free(__ptr : *mut ::std::os::raw::c_void);
    fn fstat(__fd : i32, __buf : *mut stat) -> i32;
    fn getpagesize() -> i32;
    fn memset(
        __s : *mut ::std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn mmap(
        __addr : *mut ::std::os::raw::c_void,
        __len : usize,
        __prot : i32,
        __flags : i32,
        __fd : i32,
        __offset : isize
    ) -> *mut ::std::os::raw::c_void;
    fn mov_uci(mov : u16, uci : *mut u8);
    fn munmap(
        __addr : *mut ::std::os::raw::c_void, __len : usize
    ) -> i32;
    fn open(__file : *const u8, __oflag : i32, ...) -> i32;
    fn printf(__format : *const u8, ...) -> i32;
}

#[no_mangle]
pub unsafe extern fn arr_get_bit(
    mut arr : *const usize, mut n : usize
) -> bool {
    !(*arr.offset(
           (n >> 6i32) as (isize)
       ) & 1usize << (n & 0x3fusize) == 0)
}

#[no_mangle]
pub unsafe extern fn arr_set_bit(mut arr : *mut usize, mut n : usize) {
    let _rhs = 1usize << (n & 0x3fusize);
    let _lhs = &mut *arr.offset((n >> 6i32) as (isize));
    *_lhs = *_lhs | _rhs;
}

#[no_mangle]
pub static HASHTABLE_MASK : u32 = 0xfffffu32;

#[no_mangle]
pub unsafe extern fn compute_hash(mut n : u32) -> u32 {
    let mut k : u32 = n.wrapping_mul(n);
    k = k.wrapping_add(n >> 10i32 ^ !n & 0x3ffu32);
    k = k.wrapping_add((n >> 5i32 ^ !n & 0x7fffu32) << 5i32);
    k = k.wrapping_add((!n >> 15i32 ^ n & 0x1fu32) << 5i32);
    k = k.wrapping_add(n >> 4i32 & 0x55aa55u32);
    k = k.wrapping_add(!n >> 8i32 & 0xaa55aau32);
    k & HASHTABLE_MASK
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

#[no_mangle]
pub unsafe extern fn tree_next(
    mut tree : *const tree, mut node : *const node
) -> *const node {
    if (*tree).root as (*const node) == node {
        (*tree).nodes as (*const node)
    } else {
        node.offset(1isize)
    }
}

#[no_mangle]
pub unsafe extern fn tree_from_index(
    mut tree : *const tree, mut index : u32
) -> *mut node {
    if index == 0 {
        0i32 as (*mut ::std::os::raw::c_void) as (*mut node)
    } else {
        (*tree).nodes.offset(index as (isize)).offset(-1isize)
    }
}

#[no_mangle]
pub unsafe extern fn tree_index(
    mut tree : *const tree, mut node : *const node
) -> u32 {
    if (*tree).root as (*const node) == node {
        0u32
    } else {
        ((node as (isize)).wrapping_sub(
             (*tree).nodes as (isize)
         ) / ::std::mem::size_of::<node>() as (isize) + 1isize) as (u32)
    }
}

#[no_mangle]
pub unsafe extern fn node_mov(mut node : *const node) -> u16 {
    let mut mov : u16 = (*node).mov;
    if mov as (i32) == 0xfedci32 || mov as (i32) == 0xedci32 {
        0u16
    } else {
        if mov as (i32) >> 12i32 > 8i32 || mov as (i32) >> 12i32 == 7i32 {
            mov = (mov as (i32) ^ 0xf000i32) as (u16);
        }
        mov = (mov as (i32) & 0x7fffi32) as (u16);
        mov
    }
}

#[no_mangle]
pub unsafe extern fn node_is_trans(mut node : *const node) -> bool {
    (*node).data & 1u32 << 31i32 == 1u32 << 31i32
}

#[no_mangle]
pub unsafe extern fn node_trans_and_sibling(
    mut node : *const node
) -> bool {
    (*node).data & 3u32 << 30i32 == 3u32 << 30i32
}

#[no_mangle]
pub unsafe extern fn node_trans_index(mut node : *const node) -> u32 {
    (*node).data & 0x3fffffffu32
}

#[no_mangle]
pub unsafe extern fn node_has_child(mut node : *const node) -> bool {
    (*node).data & 3u32 << 30i32 == 1u32 << 30i32 && ((*node).data & 0x3fffffffu32 != 0x3fffffffu32)
}

#[no_mangle]
pub unsafe extern fn tree_trans(
    mut tree : *const tree, mut node : *const node
) -> *const node {
    tree_from_index(tree,node_trans_index(node)) as (*const node)
}

#[no_mangle]
pub unsafe extern fn tree_trans_ns(
    mut tree : *const tree, mut node : *const node
) -> *const node {
    if (*node).data & 0x3fffffffu32 != 0x3fffffffu32 {
        tree_from_index(tree,(*node).data & 0x3fffffffu32) as (*const node)
    } else if (*node).data & 1u32 << 30i32 == 1u32 << 30i32 {
        tree_next(tree,node)
    } else {
        0i32 as (*mut ::std::os::raw::c_void) as (*const node)
    }
}

#[no_mangle]
pub unsafe extern fn tree_next_sibling(
    mut tree : *const tree, mut node : *const node
) -> *const node {
    if node_trans_and_sibling(node) {
        tree_next(tree,node)
    } else if node_is_trans(node) {
        0i32 as (*mut ::std::os::raw::c_void) as (*const node)
    } else {
        tree_trans_ns(tree,node)
    }
}

#[no_mangle]
pub unsafe extern fn tree_lookup_subtree_size(
    mut tree : *const tree, mut node : *const node
) -> u32 {
    let mut _currentBlock;
    'loop0: loop {
        if !node_is_trans(node) {
            break;
        }
        node = tree_trans(tree,node);
    }
    let mut index : u32 = tree_index(tree,node);
    let mut bucket : u32 = compute_hash(index);
    'loop2: loop {
        if (*(*tree).hashtable.offset(bucket as (isize))).index == 0 {
            _currentBlock = 3;
            break;
        }
        if index == (*(*tree).hashtable.offset(bucket as (isize))).index {
            _currentBlock = 6;
            break;
        }
        bucket = bucket.wrapping_add(1u32) & HASHTABLE_MASK;
    }
    if _currentBlock == 3 {
        0u32
    } else {
        (*(*tree).hashtable.offset(bucket as (isize))).size
    }
}

#[no_mangle]
pub unsafe extern fn tree_save_subtree_size(
    mut tree : *mut tree, mut node : *const node, mut size : u32
) -> bool {
    if (*tree).num_hash_entries > HASHTABLE_MASK.wrapping_div(
                                      8u32
                                  ) as (usize) {
        false
    } else {
        'loop1: loop {
            if !node_is_trans(node) {
                break;
            }
            node = tree_trans(tree as (*const tree),node);
        }
        let mut bucket
            : u32
            = compute_hash(tree_index(tree as (*const tree),node));
        'loop3: loop {
            if (*(*tree).hashtable.offset(bucket as (isize))).index == 0 {
                break;
            }
            bucket = bucket.wrapping_add(1u32) & HASHTABLE_MASK;
        }
        (*(*tree).hashtable.offset(bucket as (isize))).index = tree_index(
                                                                   tree as (*const tree),
                                                                   node
                                                               );
        (*(*tree).hashtable.offset(bucket as (isize))).size = size;
        (*tree).num_hash_entries = (*tree).num_hash_entries.wrapping_add(
                                       1usize
                                   );
        (if !node_has_child(node) {
             true
         } else if tree_next_sibling(
                       tree as (*const tree),
                       tree_next(tree as (*const tree),node)
                   ).is_null(
                   ) {
             tree_save_subtree_size(
                 tree,
                 tree_next(tree as (*const tree),node),
                 if size > 0u32 { size.wrapping_sub(1u32) } else { 0u32 }
             )
         } else {
             true
         })
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct timespec {
    pub tv_sec : isize,
    pub tv_nsec : isize,
}

impl Clone for timespec {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct stat {
    pub st_dev : usize,
    pub st_ino : usize,
    pub st_nlink : usize,
    pub st_mode : u32,
    pub st_uid : u32,
    pub st_gid : u32,
    pub __pad0 : i32,
    pub st_rdev : usize,
    pub st_size : isize,
    pub st_blksize : isize,
    pub st_blocks : isize,
    pub st_atim : timespec,
    pub st_mtim : timespec,
    pub st_ctim : timespec,
    pub __glibc_reserved : [isize; 3],
}

impl Clone for stat {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn tree_open(
    mut tree : *mut tree, mut filename : *const u8
) -> bool {
    (*tree).fd = open(filename,0o0i32);
    if (*tree).fd == -1i32 {
        false
    } else {
        let mut sb : stat;
        (if fstat((*tree).fd,&mut sb as (*mut stat)) == -1i32 {
             false
         } else {
             (*tree).root = mmap(
                                0i32 as (*mut ::std::os::raw::c_void),
                                sb.st_size as (usize),
                                0x1i32,
                                0x1i32,
                                (*tree).fd,
                                0isize
                            ) as (*mut node);
             (if (*tree).root == -1i32 as (*mut ::std::os::raw::c_void) as (*mut node) {
                  false
              } else {
                  let page_size : usize = getpagesize() as (usize);
                  (*tree).num_pages = (sb.st_size as (usize)).wrapping_div(
                                          page_size
                                      );
                  if (sb.st_size as (usize)).wrapping_rem(page_size) > 0usize {
                      (*tree).num_pages = (*tree).num_pages.wrapping_add(1usize);
                  }
                  (*tree).prolog_len = (*(*tree).root).mov as (u32);
                  (*tree).prolog = (*tree).root.offset(1isize) as (*mut u16);
                  (*tree).nodes = (*tree).prolog.offset(
                                      (*tree).prolog_len as (isize)
                                  ) as (*mut node);
                  (*tree).size = node_trans_index((*tree).root as (*const node));
                  (if (*tree).size == 0 {
                       false
                   } else {
                       (*tree).arr = calloc(
                                         (*tree).size.wrapping_div(8u32).wrapping_add(
                                             64u32
                                         ) as (usize),
                                         1usize
                                     ) as (*mut usize);
                       (if (*tree).arr.is_null() {
                            false
                        } else {
                            (*tree).num_hash_entries = 0usize;
                            (*tree).hashtable = calloc(
                                                    HASHTABLE_MASK.wrapping_add(1u32) as (usize),
                                                    ::std::mem::size_of::<hash_entry>()
                                                ) as (*mut hash_entry);
                            (if (*tree).hashtable.is_null() {
                                 false
                             } else {
                                 let mut data
                                     : *mut u32
                                     = (*tree).nodes.offset((*tree).size as (isize)).offset(
                                           -1isize
                                       ) as (*mut u32);
                                 'loop9: loop {
                                     if !(data as (*mut u8) < ((*tree).root as (*mut u8)).offset(
                                                                  sb.st_size
                                                              )) {
                                         break;
                                     }
                                     let mut node
                                         : *mut node
                                         = tree_from_index(
                                               tree as (*const tree),
                                               *{
                                                    let _old = data;
                                                    data = data.offset(1isize);
                                                    _old
                                                }
                                           );
                                     if node.is_null() {
                                         node = (*tree).root;
                                     }
                                     let mut size
                                         : u32
                                         = *{
                                                let _old = data;
                                                data = data.offset(1isize);
                                                _old
                                            };
                                     if !(tree_lookup_subtree_size(
                                              tree as (*const tree),
                                              node as (*const node)
                                          ) == 0) {
                                         continue;
                                     }
                                     let mut success
                                         : bool
                                         = tree_save_subtree_size(tree,node as (*const node),size);
                                     if success {
                                         0i32;
                                     } else {
                                         __assert_fail(
                                             (*b"success\0").as_ptr(),
                                             file!().as_ptr(),
                                             line!(),
                                             (*b"tree_open\0").as_ptr()
                                         );
                                     }
                                 }
                                 true
                             })
                        })
                   })
              })
         })
    }
}

#[no_mangle]
pub unsafe extern fn tree_close(mut tree : *mut tree) {
    if !(*tree).hashtable.is_null() {
        free((*tree).hashtable as (*mut ::std::os::raw::c_void));
    }
    if !(*tree).arr.is_null() {
        free((*tree).arr as (*mut ::std::os::raw::c_void));
    }
    munmap(
        (*tree).root as (*mut ::std::os::raw::c_void),
        (*tree).num_pages.wrapping_mul(getpagesize() as (usize))
    );
    close((*tree).fd);
    memset(
        tree as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<tree>()
    );
}

#[no_mangle]
pub unsafe extern fn tree_mov(
    mut tree : *const tree, mut mov : u16, mut node : *const node
) -> *const node {
    let mut _currentBlock;
    if node.is_null() {
        0i32 as (*mut ::std::os::raw::c_void) as (*const node)
    } else if !node_has_child(node) {
        0i32 as (*mut ::std::os::raw::c_void) as (*const node)
    } else {
        let mut child : *const node = tree_next(tree,node);
        'loop3: loop {
            if node_mov(child) as (i32) == mov as (i32) {
                _currentBlock = 6;
                break;
            }
            if {
                   child = tree_next_sibling(tree,child);
                   child
               }.is_null(
               ) {
                _currentBlock = 5;
                break;
            }
        }
        (if _currentBlock == 5 {
             0i32 as (*mut ::std::os::raw::c_void) as (*const node)
         } else {
             'loop6: loop {
                 if !node_is_trans(child) {
                     break;
                 }
                 child = tree_trans(tree,child);
             }
             child
         })
    }
}

#[no_mangle]
pub unsafe extern fn tree_debug(
    mut tree : *const tree, mut dump_hashtable : bool
) {
    printf(
        (*b"tree size = %u (%zumb) \n\0").as_ptr(),
        (*tree).size,
        ::std::mem::size_of::<node>().wrapping_mul(
            (*tree).size as (usize)
        ) >> 20i32
    );
    let mut i : usize = 0usize;
    'loop1: loop {
        if !(i < (*tree).prolog_len as (usize)) {
            break;
        }
        let mut uci : [u8; 8];
        mov_uci(*(*tree).prolog.offset(i as (isize)),uci.as_mut_ptr());
        printf((*b"prolog[%zu] = %s\n\0").as_ptr(),i,uci.as_mut_ptr());
        i = i.wrapping_add(1usize);
    }
    if dump_hashtable {
        let mut i : usize = 0usize;
        'loop4: loop {
            if !(i <= HASHTABLE_MASK as (usize)) {
                break;
            }
            if (*(*tree).hashtable.offset(i as (isize))).index != 0 {
                printf(
                    (*b"hashtable[%zu] = <%d, %d>\n\0").as_ptr(),
                    i,
                    (*(*tree).hashtable.offset(i as (isize))).index,
                    (*(*tree).hashtable.offset(i as (isize))).size
                );
            }
            i = i.wrapping_add(1usize);
        }
    }
}

#[no_mangle]
pub unsafe extern fn tree_walk(
    mut tree : *mut tree,
    mut node : *const node,
    mut transpositions : bool
) {
    let mut index : u32 = tree_index(tree as (*const tree),node);
    let mut k : u16 = ((*node).mov as (i32) >> 12i32) as (u16);
    if node_trans_index(node) != 0x3fffffffu32 {
        0i32;
    } else {
        __assert_fail(
            (*b"node_trans_index(node) != 0x3fffffff\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"tree_walk\0").as_ptr()
        );
    }
    if (*node).mov as (i32) == 0xfedci32 || k as (i32) != 7i32 && (k as (i32) < 9i32) {
        0i32;
    } else {
        __assert_fail(
            (*b"node->mov == 0xfedc || (k != 7 && k < 9)\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"tree_walk\0").as_ptr()
        );
    }
    if transpositions {
        if arr_get_bit((*tree).arr as (*const usize),index as (usize)) {
            return;
        } else {
            arr_set_bit((*tree).arr,index as (usize));
        }
    }
    if node_is_trans(node) {
        tree_walk(
            tree,
            tree_trans(tree as (*const tree),node),
            transpositions
        );
    } else {
        if !transpositions {
            if arr_get_bit((*tree).arr as (*const usize),index as (usize)) {
                return;
            } else {
                arr_set_bit((*tree).arr,index as (usize));
            }
        }
        (if !node_has_child(node) {
         } else {
             let mut child
                 : *const node
                 = tree_next(tree as (*const tree),node);
             'loop9: loop {
                 tree_walk(tree,child,transpositions);
                 if {
                        child = tree_next_sibling(tree as (*const tree),child);
                        child
                    }.is_null(
                    ) {
                     break;
                 }
             }
         })
    }
}

unsafe extern fn bb_popcount(mut bb : usize) -> i32 { bb.count_ones() }

#[no_mangle]
pub unsafe extern fn tree_subtree_size(
    mut tree : *mut tree, mut node : *const node
) -> u32 {
    if (*tree).root as (*const node) == node {
        (*tree).size
    } else {
        let mut k : u16 = ((*node).mov as (i32) >> 12i32) as (u16);
        if node_trans_index(node) != 0x3fffffffu32 {
            0i32;
        } else {
            __assert_fail(
                (*b"node_trans_index(node) != 0x3fffffff\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"tree_subtree_size\0").as_ptr()
            );
        }
        if (*node).mov as (i32) == 0xfedci32 || k as (i32) != 7i32 && (k as (i32) < 9i32) {
            0i32;
        } else {
            __assert_fail(
                (*b"node->mov == 0xfedc || (k != 7 && k < 9)\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"tree_subtree_size\0").as_ptr()
            );
        }
        'loop2: loop {
            if !node_is_trans(node) {
                break;
            }
            node = tree_trans(tree as (*const tree),node);
        }
        let mut subtree_size
            : u32
            = tree_lookup_subtree_size(tree as (*const tree),node);
        (if subtree_size != 0 {
             subtree_size
         } else {
             let mut size
                 : u32
                 = (*tree).size.wrapping_add(63u32).wrapping_div(64u32);
             memset(
                 (*tree).arr as (*mut ::std::os::raw::c_void),
                 0i32,
                 ::std::mem::size_of::<usize>().wrapping_mul(size as (usize))
             );
             tree_walk(tree,node,false);
             let mut i : u32 = 0u32;
             'loop5: loop {
                 if !(i < size) {
                     break;
                 }
                 subtree_size = subtree_size.wrapping_add(
                                    bb_popcount(*(*tree).arr.offset(i as (isize))) as (u32)
                                );
                 i = i.wrapping_add(1u32);
             }
             tree_save_subtree_size(tree,node,subtree_size);
             subtree_size
         })
    }
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
pub unsafe extern fn query_result_clear(
    mut result : *mut query_result
) {
    memset(
        result as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<query_result>()
    );
}

#[no_mangle]
pub unsafe extern fn query_result_add(
    mut result : *mut query_result, mut mov : u16, mut size : u32
) {
    let mut _currentBlock;
    let mut i : usize = 0usize;
    'loop1: loop {
        if !(i < (*result).num_children) {
            _currentBlock = 2;
            break;
        }
        if (*result).movs[i] as (i32) == mov as (i32) {
            _currentBlock = 5;
            break;
        }
        i = i.wrapping_add(1usize);
    }
    if _currentBlock == 2 {
        if (*result).num_children < 256usize {
            0i32;
        } else {
            __assert_fail(
                (*b"result->num_children < MAX_LEGAL_MOVES\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"query_result_add\0").as_ptr()
            );
        }
        (*result).movs[(*result).num_children] = mov;
        (*result).sizes[(*result).num_children] = size;
        (*result).num_children = (*result).num_children.wrapping_add(
                                     1usize
                                 );
    } else {
        let _rhs = size;
        let _lhs = &mut (*result).sizes[i];
        *_lhs = (*_lhs).wrapping_add(_rhs);
    }
}

#[no_mangle]
pub unsafe extern fn query_result_sort(
    mut result : *mut query_result
) {
}

#[no_mangle]
pub unsafe extern fn tree_query_children(
    mut tree : *mut tree,
    mut node : *const node,
    mut result : *mut query_result
) -> bool {
    if !node.is_null() {
        0i32;
    } else {
        __assert_fail(
            (*b"node\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"tree_query_children\0").as_ptr()
        );
    }
    if !node_has_child(node) {
        false
    } else {
        let mut child
            : *const node
            = tree_next(tree as (*const tree),node);
        'loop2: loop {
            query_result_add(
                result,
                node_mov(child),
                tree_subtree_size(tree,child)
            );
            if {
                   child = tree_next_sibling(tree as (*const tree),child);
                   child
               }.is_null(
               ) {
                break;
            }
        }
        true
    }
}

#[no_mangle]
pub unsafe extern fn tree_query(
    mut tree : *mut tree,
    mut movs : *const u16,
    mut movs_len : usize,
    mut result : *mut query_result
) -> bool {
    let mut _currentBlock;
    if (*tree).prolog_len as (usize) > movs_len {
        let mut i : usize = 0usize;
        'loop13: loop {
            if !(i < movs_len) {
                _currentBlock = 14;
                break;
            }
            if *(*tree).prolog.offset(i as (isize)) as (i32) != *movs.offset(
                                                                     i as (isize)
                                                                 ) as (i32) {
                _currentBlock = 17;
                break;
            }
            i = i.wrapping_add(1usize);
        }
        (if _currentBlock == 14 {
             query_result_add(
                 result,
                 *(*tree).prolog.offset(movs_len as (isize)),
                 ((*tree).size.wrapping_add(
                      (*tree).prolog_len
                  ) as (usize)).wrapping_sub(
                     movs_len
                 ) as (u32)
             );
             true
         } else {
             false
         })
    } else {
        let mut i : usize = 0usize;
        'loop2: loop {
            if !(i < (*tree).prolog_len as (usize)) {
                _currentBlock = 3;
                break;
            }
            if *(*tree).prolog.offset(i as (isize)) as (i32) != *movs.offset(
                                                                     i as (isize)
                                                                 ) as (i32) {
                _currentBlock = 11;
                break;
            }
            i = i.wrapping_add(1usize);
        }
        (if _currentBlock == 3 {
             let mut node : *const node = (*tree).root as (*const node);
             let mut i : usize = (*tree).prolog_len as (usize);
             'loop4: loop {
                 if !(i < movs_len) {
                     _currentBlock = 5;
                     break;
                 }
                 node = tree_mov(
                            tree as (*const tree),
                            *movs.offset(i as (isize)),
                            node
                        );
                 if node.is_null() {
                     _currentBlock = 8;
                     break;
                 }
                 i = i.wrapping_add(1usize);
             }
             (if _currentBlock == 5 {
                  tree_query_children(tree,node,result)
              } else {
                  false
              })
         } else {
             false
         })
    }
}
