extern {
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn board_mov(board : *mut board, mov : u16);
    fn board_reset(board : *mut board);
    fn board_san(board : *mut board, mov : u16, san : *mut u8);
    fn mov_parse(uci : *const u8) -> u16;
    fn strcmp(__s1 : *const u8, __s2 : *const u8) -> i32;
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum1 {
    kBlack,
    kWhite,
}

#[derive(Copy)]
#[repr(C)]
pub struct board {
    pub occupied_co : [usize; 2],
    pub occupied : [usize; 7],
    pub turn : Enum1,
    pub ep_square : u8,
}

impl Clone for board {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn test_san() {
    let mut san : [u8; 8];
    let mut board : board;
    board_reset(&mut board as (*mut board));
    board_mov(
        &mut board as (*mut board),
        mov_parse((*b"e2e3\0").as_ptr())
    );
    board_mov(
        &mut board as (*mut board),
        mov_parse((*b"b7b6\0").as_ptr())
    );
    board_mov(
        &mut board as (*mut board),
        mov_parse((*b"a2a4\0").as_ptr())
    );
    board_mov(
        &mut board as (*mut board),
        mov_parse((*b"c8a6\0").as_ptr())
    );
    let mut Bxa6 : u16 = mov_parse((*b"f1a6\0").as_ptr());
    board_san(&mut board as (*mut board),Bxa6,san.as_mut_ptr());
    if strcmp(
           san.as_mut_ptr() as (*const u8),
           (*b"Bxa6\0").as_ptr()
       ) == 0i32 {
        0i32;
    } else {
        __assert_fail(
            (*b"strcmp(san, \"Bxa6\") == 0\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"test_san\0").as_ptr()
        );
    }
    board_mov(&mut board as (*mut board),Bxa6);
    board_mov(
        &mut board as (*mut board),
        mov_parse((*b"b8a6\0").as_ptr())
    );
    board_mov(
        &mut board as (*mut board),
        mov_parse((*b"b2b4\0").as_ptr())
    );
    board_mov(
        &mut board as (*mut board),
        mov_parse((*b"a6b4\0").as_ptr())
    );
    board_mov(
        &mut board as (*mut board),
        mov_parse((*b"d1h5\0").as_ptr())
    );
    let mut Nxc2 : u16 = mov_parse((*b"b4c2\0").as_ptr());
    board_san(&mut board as (*mut board),Nxc2,san.as_mut_ptr());
    if strcmp(
           san.as_mut_ptr() as (*const u8),
           (*b"Nxc2\0").as_ptr()
       ) == 0i32 {
        0i32;
    } else {
        __assert_fail(
            (*b"strcmp(san, \"Nxc2\") == 0\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"test_san\0").as_ptr()
        );
    }
}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern fn _c_main() -> i32 {
    test_san();
    0i32
}
