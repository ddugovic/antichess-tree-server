extern {
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn abs(__x : i32) -> i32;
    fn printf(__format : *const u8, ...) -> i32;
    fn sprintf(__s : *mut u8, __format : *const u8, ...) -> i32;
    fn strlen(__s : *const u8) -> usize;
}

static mut PCHR : *const u8 = (*b"\0PNBRQK\0").as_ptr();

static mut ROOK_DELTAS : *const i32 = 8i32 as (*const i32);

static mut BISHOP_DELTAS : *const i32 = 9i32 as (*const i32);

static mut KING_DELTAS : *const i32 = 8i32 as (*const i32);

static mut KNIGHT_DELTAS : *const i32 = 17i32 as (*const i32);

unsafe extern fn mov_from(mut mov : u16) -> u8 {
    (mov as (i32) >> 6i32 & 0o77i32) as (u8)
}

unsafe extern fn mov_to(mut mov : u16) -> u8 {
    (mov as (i32) & 0o77i32) as (u8)
}

unsafe extern fn square_file(mut square : u8) -> i32 {
    square as (i32) & 7i32
}

unsafe extern fn square_rank(mut square : u8) -> i32 {
    square as (i32) >> 3i32
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum1 {
    kAll,
    kPawn,
    kKnight,
    kBishop,
    kRook,
    kQueen,
    kKing,
    kNone = 0i32,
}

unsafe extern fn mov_promotion(mut mov : u16) -> Enum1 {
    static mut promotions
        : *const Enum1
        = Enum1::kNone as (*const Enum1);
    *promotions.offset((mov as (i32) >> 12i32) as (isize))
}

#[no_mangle]
pub unsafe extern fn mov_uci(mut mov : u16, mut uci : *mut u8) {
    if mov == 0 {
        sprintf(uci,(*b"(none)\0").as_ptr());
    } else {
        let mut from : i32 = mov_from(mov) as (i32);
        let mut to : i32 = mov_to(mov) as (i32);
        let mut promotions : *const u8 = (*b"\0pnbrqk\0").as_ptr();
        sprintf(
            uci,
            (*b"%c%c%c%c%c\0").as_ptr(),
            b'a' as (i32) + square_file(from as (u8)),
            b'1' as (i32) + square_rank(from as (u8)),
            b'a' as (i32) + square_file(to as (u8)),
            b'1' as (i32) + square_rank(to as (u8)),
            *promotions.offset(mov_promotion(mov) as (isize)) as (i32)
        );
    }
}

#[no_mangle]
pub unsafe extern fn mov_parse(mut uci : *const u8) -> u16 {
    let mut _currentBlock;
    let mut promotions : *const u8 = (*b"\0pnbrqk\0").as_ptr();
    if strlen(uci) > 5usize || strlen(uci) < 4usize {
        0u16
    } else {
        let mut mov
            : u16
            = (*uci.offset(2isize) as (i32) - b'a' as (i32) + (*uci.offset(
                                                                    3isize
                                                                ) as (i32) - b'1' as (i32) << 3i32) + (*uci.offset(
                                                                                                            0isize
                                                                                                        ) as (i32) - b'a' as (i32) << 6i32) + (*uci.offset(
                                                                                                                                                    1isize
                                                                                                                                                ) as (i32) - b'1' as (i32) << 9i32)) as (u16);
        (if *uci.offset(4isize) != 0 {
             let mut k : i32 = 2i32;
             'loop4: loop {
                 if !(k <= 6i32) {
                     _currentBlock = 5;
                     break;
                 }
                 if *uci.offset(4isize) as (i32) == *promotions.offset(
                                                         k as (isize)
                                                     ) as (i32) {
                     _currentBlock = 8;
                     break;
                 }
                 k = k + 1;
             }
             (if _currentBlock == 5 {
                  0u16
              } else {
                  mov = (mov as (i32) | k << 12i32) as (u16);
                  mov
              })
         } else {
             mov
         })
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum2 {
    kBlack,
    kWhite,
}

#[derive(Copy)]
#[repr(C)]
pub struct board {
    pub occupied_co : [usize; 2],
    pub occupied : [usize; 7],
    pub turn : Enum2,
    pub ep_square : u8,
}

impl Clone for board {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn board_reset(mut board : *mut board) {
    (*board).occupied_co[Enum2::kWhite as (usize)] = 0xffffusize;
    (*board).occupied_co[
        Enum2::kBlack as (usize)
    ] = 0xffff000000000000usize;
    (*board).occupied[
        Enum1::kAll as (usize)
    ] = 0xffff00000000ffffusize;
    (*board).occupied[Enum1::kPawn as (usize)] = 0xff00000000ff00usize;
    (*board).occupied[
        Enum1::kKnight as (usize)
    ] = 0x4200000000000042usize;
    (*board).occupied[
        Enum1::kBishop as (usize)
    ] = 0x2400000000000024usize;
    (*board).occupied[
        Enum1::kRook as (usize)
    ] = 0x8100000000000081usize;
    (*board).occupied[
        Enum1::kQueen as (usize)
    ] = 0x800000000000008usize;
    (*board).occupied[
        Enum1::kKing as (usize)
    ] = 0x1000000000000010usize;
    (*board).turn = Enum2::kWhite;
    (*board).ep_square = 0u8;
}

#[no_mangle]
pub unsafe extern fn bb_debug(mut bb : usize) {
    let mut rank : i32 = 7i32;
    'loop1: loop {
        if !(rank >= 0i32) {
            break;
        }
        let mut file : i32 = 0i32;
        'loop4: loop {
            if !(file < 8i32) {
                break;
            }
            let mut square : u8 = (rank << 3i32 | file) as (u8);
            if bb & 1usize << square as (i32) != 0 {
                printf((*b"1\0").as_ptr());
            } else {
                printf((*b".\0").as_ptr());
            }
            if file < 7i32 {
                printf((*b" \0").as_ptr());
            } else {
                printf((*b"\n\0").as_ptr());
            }
            file = file + 1;
        }
        rank = rank - 1;
    }
}

unsafe extern fn board_piece_type_at(
    mut board : *const board, mut square : u8
) -> Enum1 {
    let mut _currentBlock;
    let mut bb : usize = 1usize << square as (i32);
    let mut pt : Enum1 = Enum1::kPawn;
    'loop1: loop {
        if !(pt as (i32) <= Enum1::kKing as (i32)) {
            _currentBlock = 2;
            break;
        }
        if (*board).occupied[pt as (usize)] & bb != 0 {
            _currentBlock = 5;
            break;
        }
        pt = (pt as (i32) + 1) as (Enum1);
    }
    if _currentBlock == 2 { Enum1::kNone } else { pt }
}

#[no_mangle]
pub unsafe extern fn board_debug(mut board : *const board) {
    let mut rank : i32 = 7i32;
    'loop1: loop {
        if !(rank >= 0i32) {
            break;
        }
        let mut file : i32 = 0i32;
        'loop4: loop {
            if !(file < 8i32) {
                break;
            }
            let mut square : u8 = (rank << 3i32 | file) as (u8);
            let mut pt : Enum1 = board_piece_type_at(board,square);
            if pt == 0 {
                printf((*b".\0").as_ptr());
            } else if (*board).occupied_co[
                          Enum2::kWhite as (usize)
                      ] & 1usize << square as (i32) != 0 {
                printf((*b"%c\0").as_ptr(),*PCHR.offset(pt as (isize)) as (i32));
            } else {
                printf(
                    (*b"%c\0").as_ptr(),
                    *PCHR.offset(
                         pt as (isize)
                     ) as (i32) - b'A' as (i32) + b'a' as (i32)
                );
            }
            if file < 7i32 {
                printf((*b" \0").as_ptr());
            } else {
                printf((*b"\n\0").as_ptr());
            }
            file = file + 1;
        }
        rank = rank - 1;
    }
}

unsafe extern fn square_distance(mut a : u8, mut b : u8) -> i32 {
    let mut rd : i32 = abs(square_rank(a) - square_rank(b));
    let mut fd : i32 = abs(square_file(a) - square_file(b));
    if rd > fd { rd } else { fd }
}

#[no_mangle]
pub unsafe extern fn board_mov(
    mut board : *mut board, mut mov : u16
) { if mov == 0 {
    } else {
        (*board).ep_square = 0u8;
        let mut from : u8 = mov_from(mov);
        let mut to : u8 = mov_to(mov);
        let mut pt
            : Enum1
            = board_piece_type_at(board as (*const board),from);
        (if pt == 0 {
         } else {
             let _rhs = !(1usize << from as (i32));
             let _lhs = &mut (*board).occupied_co[(*board).turn as (usize)];
             *_lhs = *_lhs & _rhs;
             let _rhs = !(1usize << from as (i32));
             let _lhs = &mut (*board).occupied[Enum1::kAll as (usize)];
             *_lhs = *_lhs & _rhs;
             let _rhs = !(1usize << from as (i32));
             let _lhs = &mut (*board).occupied[pt as (usize)];
             *_lhs = *_lhs & _rhs;
             let capture
                 : Enum1
                 = board_piece_type_at(board as (*const board),to);
             if capture != 0 {
                 let _rhs = !(1usize << to as (i32));
                 let _lhs
                     = &mut (*board).occupied_co[((*board).turn == 0) as (usize)];
                 *_lhs = *_lhs & _rhs;
                 let _rhs = !(1usize << to as (i32));
                 let _lhs = &mut (*board).occupied[capture as (usize)];
                 *_lhs = *_lhs & _rhs;
             }
             if pt as (i32) == Enum1::kPawn as (i32) {
                 if square_file(from) != square_file(to) && (capture == 0) {
                     let mut ep_mask
                         : usize
                         = 1usize << to as (i32) + if (*board).turn != 0 {
                                                       -8i32
                                                   } else {
                                                       8i32
                                                   };
                     let _rhs = !ep_mask;
                     let _lhs
                         = &mut (*board).occupied_co[((*board).turn == 0) as (usize)];
                     *_lhs = *_lhs & _rhs;
                     let _rhs = !ep_mask;
                     let _lhs = &mut (*board).occupied[Enum1::kAll as (usize)];
                     *_lhs = *_lhs & _rhs;
                     let _rhs = !ep_mask;
                     let _lhs = &mut (*board).occupied[Enum1::kPawn as (usize)];
                     *_lhs = *_lhs & _rhs;
                 } else if square_distance(from,to) == 2i32 {
                     (*board).ep_square = (from as (i32) + if (*board).turn != 0 {
                                                               8i32
                                                           } else {
                                                               -8i32
                                                           }) as (u8);
                 }
             }
             if mov_promotion(mov) != 0 {
                 pt = mov_promotion(mov);
             }
             let _rhs = 1usize << to as (i32);
             let _lhs = &mut (*board).occupied_co[(*board).turn as (usize)];
             *_lhs = *_lhs | _rhs;
             let _rhs = 1usize << to as (i32);
             let _lhs = &mut (*board).occupied[Enum1::kAll as (usize)];
             *_lhs = *_lhs | _rhs;
             let _rhs = 1usize << to as (i32);
             let _lhs = &mut (*board).occupied[pt as (usize)];
             *_lhs = *_lhs | _rhs;
             (*board).turn = ((*board).turn == 0) as (Enum2);
         })
    }
}

unsafe extern fn attacks_sliding(
    mut deltas : *const i32, mut square : u8, mut occupied : usize
) -> usize {
    let mut attack : usize = 0usize;
    let mut i : i32 = 0i32;
    'loop1: loop {
        if *deltas.offset(i as (isize)) == 0 {
            break;
        }
        let mut s : i32 = square as (i32) + *deltas.offset(i as (isize));
        'loop4: loop {
            if !(s >= 0i32 && (s < 64i32) && (square_distance(
                                                  s as (u8),
                                                  (s - *deltas.offset(i as (isize))) as (u8)
                                              ) <= 2i32)) {
                break;
            }
            attack = attack | 1usize << s;
            if occupied & 1usize << s != 0 {
                break;
            }
            s = s + *deltas.offset(i as (isize));
        }
        i = i + 1;
    }
    attack
}

unsafe extern fn bb_lsb(mut bb : usize) -> u8 {
    if bb != 0 {
        bb.trailing_zeros();
    } else {
        __assert_fail(
            (*b"bb\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"bb_lsb\0").as_ptr()
        );
    }
    0u8
}

unsafe extern fn bb_poplsb(mut bb : *mut usize) -> u8 {
    let mut sq : u8 = bb_lsb(*bb);
    *bb = *bb & (*bb).wrapping_sub(1usize);
    sq
}

#[no_mangle]
pub unsafe extern fn board_is_game_over(
    mut board : *const board
) -> bool {
    let mut _currentBlock;
    if (*board).occupied_co[
           Enum2::kBlack as (usize)
       ] == 0 || (*board).occupied_co[Enum2::kWhite as (usize)] == 0 {
        true
    } else {
        let mut us
            : usize
            = (*board).occupied_co[(*board).turn as (usize)];
        let mut pawn_attacks : usize = 0usize;
        let mut pawn_movs : usize;
        if (*board).turn as (i32) == Enum2::kWhite as (i32) {
            pawn_attacks = (pawn_attacks as (u64) | (((*board).occupied[
                                                          Enum1::kPawn as (usize)
                                                      ] & us) << 7i32) as (u64) & !0x8080808080808080u64) as (usize);
            pawn_attacks = (pawn_attacks as (u64) | (((*board).occupied[
                                                          Enum1::kPawn as (usize)
                                                      ] & us) << 9i32) as (u64) & !0x101010101010101u64) as (usize);
            pawn_movs = ((*board).occupied[
                              Enum1::kPawn as (usize)
                          ] & us) << 8i32;
        } else {
            pawn_attacks = (pawn_attacks as (u64) | (((*board).occupied[
                                                          Enum1::kPawn as (usize)
                                                      ] & us) >> 9i32) as (u64) & !0x8080808080808080u64) as (usize);
            pawn_attacks = (pawn_attacks as (u64) | (((*board).occupied[
                                                          Enum1::kPawn as (usize)
                                                      ] & us) >> 7i32) as (u64) & !0x101010101010101u64) as (usize);
            pawn_movs = ((*board).occupied[
                              Enum1::kPawn as (usize)
                          ] & us) >> 8i32;
        }
        (if pawn_attacks & (*board).occupied_co[
                               ((*board).turn == 0) as (usize)
                           ] != 0 {
             false
         } else if (*board).ep_square != 0 && (pawn_attacks & 1usize << (*board).ep_square as (i32) != 0) {
             false
         } else if pawn_movs & !(*board).occupied[
                                     Enum1::kAll as (usize)
                                 ] != 0 {
             false
         } else {
             let mut knights
                 : usize
                 = (*board).occupied[Enum1::kKnight as (usize)] & us;
             'loop8: loop {
                 if knights == 0 {
                     _currentBlock = 9;
                     break;
                 }
                 if attacks_sliding(
                        KNIGHT_DELTAS,
                        bb_poplsb(&mut knights as (*mut usize)),
                        0xffffffffffffffffusize
                    ) & !us != 0 {
                     _currentBlock = 19;
                     break;
                 }
             }
             (if _currentBlock == 9 {
                  let mut diagonal
                      : usize
                      = ((*board).occupied[
                             Enum1::kBishop as (usize)
                         ] | (*board).occupied[
                                 Enum1::kQueen as (usize)
                             ] | (*board).occupied[Enum1::kKing as (usize)]) & us;
                  'loop10: loop {
                      if diagonal == 0 {
                          _currentBlock = 11;
                          break;
                      }
                      if attacks_sliding(
                             BISHOP_DELTAS,
                             bb_poplsb(&mut diagonal as (*mut usize)),
                             (*board).occupied[Enum1::kAll as (usize)]
                         ) & !us != 0 {
                          _currentBlock = 17;
                          break;
                      }
                  }
                  (if _currentBlock == 11 {
                       let mut straight
                           : usize
                           = ((*board).occupied[Enum1::kRook as (usize)] | (*board).occupied[
                                                                               Enum1::kQueen as (usize)
                                                                           ] | (*board).occupied[
                                                                                   Enum1::kKing as (usize)
                                                                               ]) & us;
                       'loop12: loop {
                           if straight == 0 {
                               _currentBlock = 13;
                               break;
                           }
                           if attacks_sliding(
                                  ROOK_DELTAS,
                                  bb_poplsb(&mut straight as (*mut usize)),
                                  (*board).occupied[Enum1::kAll as (usize)]
                              ) & !us != 0 {
                               _currentBlock = 15;
                               break;
                           }
                       }
                       (if _currentBlock == 13 { true } else { false })
                   } else {
                       false
                   })
              } else {
                  false
              })
         })
    }
}

#[no_mangle]
pub unsafe extern fn board_san(
    mut board : *mut board, mut mov : u16, mut san : *mut u8
) {
    let mut from : u8 = mov_from(mov);
    let mut to : u8 = mov_to(mov);
    let mut pt
        : Enum1
        = board_piece_type_at(board as (*const board),from);
    if mov == 0 || pt == 0 {
        sprintf(san,(*b"--\0").as_ptr());
    } else {
        if pt as (i32) == Enum1::kPawn as (i32) {
            if square_file(from) != square_file(to) {
                *{
                     let _old = san;
                     san = san.offset(1isize);
                     _old
                 } = (b'a' as (i32) + square_file(from)) as (u8);
                *{
                     let _old = san;
                     san = san.offset(1isize);
                     _old
                 } = b'x';
            }
        } else {
            *{
                 let _old = san;
                 san = san.offset(1isize);
                 _old
             } = *PCHR.offset(pt as (isize));
            let mut candidates : usize = 0usize;
            if pt as (i32) == Enum1::kKing as (i32) {
                candidates = attacks_sliding(
                                 KING_DELTAS,
                                 to,
                                 (*board).occupied[Enum1::kAll as (usize)]
                             );
            }
            if pt as (i32) == Enum1::kKnight as (i32) {
                candidates = attacks_sliding(
                                 KNIGHT_DELTAS,
                                 to,
                                 0xffffffffffffffffusize
                             );
            }
            if pt as (i32) == Enum1::kRook as (i32) || pt as (i32) == Enum1::kQueen as (i32) {
                candidates = candidates | attacks_sliding(
                                              ROOK_DELTAS,
                                              to,
                                              (*board).occupied[Enum1::kAll as (usize)]
                                          );
            }
            if pt as (i32) == Enum1::kBishop as (i32) || pt as (i32) == Enum1::kQueen as (i32) {
                candidates = candidates | attacks_sliding(
                                              BISHOP_DELTAS,
                                              to,
                                              (*board).occupied[Enum1::kAll as (usize)]
                                          );
            }
            candidates = candidates & ((*board).occupied[
                                           pt as (usize)
                                       ] & (*board).occupied_co[(*board).turn as (usize)]);
            let mut rank : bool = false;
            let mut file : bool = false;
            'loop11: loop {
                if candidates == 0 {
                    break;
                }
                let mut square : u8 = bb_poplsb(&mut candidates as (*mut usize));
                if square as (i32) == from as (i32) {
                    continue;
                }
                if square_rank(from) == square_rank(square) {
                    file = true;
                }
                if square_file(from) == square_file(square) {
                    rank = true;
                } else {
                    file = true;
                }
            }
            if file {
                *{
                     let _old = san;
                     san = san.offset(1isize);
                     _old
                 } = (b'a' as (i32) + square_file(from)) as (u8);
            }
            if rank {
                *{
                     let _old = san;
                     san = san.offset(1isize);
                     _old
                 } = (b'1' as (i32) + square_rank(from)) as (u8);
            }
            if (*board).occupied[
                   Enum1::kAll as (usize)
               ] & 1usize << to as (i32) != 0 {
                *{
                     let _old = san;
                     san = san.offset(1isize);
                     _old
                 } = b'x';
            }
        }
        *{
             let _old = san;
             san = san.offset(1isize);
             _old
         } = (b'a' as (i32) + square_file(to)) as (u8);
        *{
             let _old = san;
             san = san.offset(1isize);
             _old
         } = (b'1' as (i32) + square_rank(to)) as (u8);
        if mov_promotion(mov) != 0 {
            *{
                 let _old = san;
                 san = san.offset(1isize);
                 _old
             } = b'=';
            *{
                 let _old = san;
                 san = san.offset(1isize);
                 _old
             } = *PCHR.offset(mov_promotion(mov) as (isize));
        }
        let mut board_after : board = *board;
        board_mov(&mut board_after as (*mut board),mov);
        if board_is_game_over(
               &mut board_after as (*mut board) as (*const board)
           ) {
            *{
                 let _old = san;
                 san = san.offset(1isize);
                 _old
             } = b'#';
        }
        *{
             let _old = san;
             san = san.offset(1isize);
             _old
         } = 0u8;
    }
}
