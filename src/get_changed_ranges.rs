use crate::*;

use std::os;

// #define DEBUG_GET_CHANGED_RANGES
unsafe extern "C" fn ts_range_array_add(
    mut self_0: *mut TSRangeArray,
    mut start: Length,
    mut end: Length,
) {
    if (*self_0).size > 0 as os::raw::c_int as os::raw::c_uint {
        if (*self_0)
            .size
            .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint)
            < (*self_0).size
        {
        } else {
            __assert_fail(
                b"(u32)(self)->size - 1 < (self)->size\x00" as *const u8 as *const os::raw::c_char,
                b"lib/src/get_changed_ranges.c\x00" as *const u8 as *const os::raw::c_char,
                12 as os::raw::c_int as os::raw::c_uint,
                (*::std::mem::transmute::<&[u8; 56], &[os::raw::c_char; 56]>(
                    b"void ts_range_array_add(TSRangeArray *, Length, Length)\x00",
                ))
                .as_ptr(),
            );
        }
        let mut last_range: *mut TSRange = &mut *(*self_0).contents.offset(
            (*self_0)
                .size
                .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint) as isize,
        ) as *mut TSRange;
        if start.bytes <= (*last_range).end_byte {
            (*last_range).end_byte = end.bytes;
            (*last_range).end_point = end.extent;
            return;
        }
    }
    if start.bytes < end.bytes {
        let mut range: TSRange = {
            let mut init = TSRange {
                start_point: start.extent,
                end_point: end.extent,
                start_byte: start.bytes,
                end_byte: end.bytes,
            };
            init
        };
        array__grow(
            self_0 as *mut VoidArray,
            1 as os::raw::c_int as size_t,
            ::std::mem::size_of::<TSRange>() as os::raw::c_ulong,
        );
        let fresh0 = (*self_0).size;
        (*self_0).size = (*self_0).size.wrapping_add(1);
        *(*self_0).contents.offset(fresh0 as isize) = range
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_range_array_intersects(
    mut self_0: *const TSRangeArray,
    mut start_index: os::raw::c_uint,
    mut start_byte: u32,
    mut end_byte: u32,
) -> bool {
    let mut i: os::raw::c_uint = start_index;
    while i < (*self_0).size {
        let mut range: *mut TSRange = &mut *(*self_0).contents.offset(i as isize) as *mut TSRange;
        if (*range).end_byte > start_byte {
            if (*range).start_byte >= end_byte {
                break;
            }
            return 1 as os::raw::c_int != 0;
        } else {
            i = i.wrapping_add(1)
        }
    }
    return 0 as os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_range_array_get_changed_ranges(
    mut old_ranges: *const TSRange,
    mut old_range_count: os::raw::c_uint,
    mut new_ranges: *const TSRange,
    mut new_range_count: os::raw::c_uint,
    mut differences: *mut TSRangeArray,
) {
    let mut new_index: os::raw::c_uint = 0 as os::raw::c_int as os::raw::c_uint;
    let mut old_index: os::raw::c_uint = 0 as os::raw::c_int as os::raw::c_uint;
    let mut current_position: Length = length_zero();
    let mut in_old_range: bool = 0 as os::raw::c_int != 0;
    let mut in_new_range: bool = 0 as os::raw::c_int != 0;
    while old_index < old_range_count || new_index < new_range_count {
        let mut old_range: *const TSRange =
            &*old_ranges.offset(old_index as isize) as *const TSRange;
        let mut new_range: *const TSRange =
            &*new_ranges.offset(new_index as isize) as *const TSRange;
        let mut next_old_position: Length = Length {
            bytes: 0,
            extent: TSPoint { row: 0, column: 0 },
        };
        if in_old_range {
            next_old_position = {
                let mut init = Length {
                    bytes: (*old_range).end_byte,
                    extent: (*old_range).end_point,
                };
                init
            }
        } else if old_index < old_range_count {
            next_old_position = {
                let mut init = Length {
                    bytes: (*old_range).start_byte,
                    extent: (*old_range).start_point,
                };
                init
            }
        } else {
            next_old_position = LENGTH_MAX
        }
        let mut next_new_position: Length = Length {
            bytes: 0,
            extent: TSPoint { row: 0, column: 0 },
        };
        if in_new_range {
            next_new_position = {
                let mut init = Length {
                    bytes: (*new_range).end_byte,
                    extent: (*new_range).end_point,
                };
                init
            }
        } else if new_index < new_range_count {
            next_new_position = {
                let mut init = Length {
                    bytes: (*new_range).start_byte,
                    extent: (*new_range).start_point,
                };
                init
            }
        } else {
            next_new_position = LENGTH_MAX
        }
        if next_old_position.bytes < next_new_position.bytes {
            if in_old_range as os::raw::c_int != in_new_range as os::raw::c_int {
                ts_range_array_add(differences, current_position, next_old_position);
            }
            if in_old_range {
                old_index = old_index.wrapping_add(1)
            }
            current_position = next_old_position;
            in_old_range = !in_old_range
        } else if next_new_position.bytes < next_old_position.bytes {
            if in_old_range as os::raw::c_int != in_new_range as os::raw::c_int {
                ts_range_array_add(differences, current_position, next_new_position);
            }
            if in_new_range {
                new_index = new_index.wrapping_add(1)
            }
            current_position = next_new_position;
            in_new_range = !in_new_range
        } else {
            if in_old_range as os::raw::c_int != in_new_range as os::raw::c_int {
                ts_range_array_add(differences, current_position, next_new_position);
            }
            if in_old_range {
                old_index = old_index.wrapping_add(1)
            }
            if in_new_range {
                new_index = new_index.wrapping_add(1)
            }
            in_old_range = !in_old_range;
            in_new_range = !in_new_range;
            current_position = next_new_position
        }
    }
}
unsafe extern "C" fn iterator_new(
    mut cursor: *mut TreeCursor,
    mut tree: *const Subtree,
    mut language: *const TSLanguage,
) -> Iterator_0 {
    (*cursor).stack.size = 0 as os::raw::c_int as u32;
    array__grow(
        &mut (*cursor).stack as *mut TreeCursorEntryArray as *mut VoidArray,
        1 as os::raw::c_int as size_t,
        ::std::mem::size_of::<TreeCursorEntry>() as os::raw::c_ulong,
    );
    let fresh1 = (*cursor).stack.size;
    (*cursor).stack.size = (*cursor).stack.size.wrapping_add(1);
    *(*cursor).stack.contents.offset(fresh1 as isize) = {
        let mut init = TreeCursorEntry {
            subtree: tree,
            position: length_zero(),
            child_index: 0 as os::raw::c_int as u32,
            structural_child_index: 0 as os::raw::c_int as u32,
        };
        init
    };
    return {
        let mut init = Iterator_0 {
            cursor: *cursor,
            language: language,
            visible_depth: 1 as os::raw::c_int as os::raw::c_uint,
            in_padding: 0 as os::raw::c_int != 0,
        };
        init
    };
}
unsafe extern "C" fn iterator_done(mut self_0: *mut Iterator_0) -> bool {
    return (*self_0).cursor.stack.size == 0 as os::raw::c_int as os::raw::c_uint;
}
unsafe extern "C" fn iterator_start_position(mut self_0: *mut Iterator_0) -> Length {
    if (*self_0)
        .cursor
        .stack
        .size
        .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint)
        < (*self_0).cursor.stack.size
    {
    } else {
        __assert_fail(
            b"(u32)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00" as *const u8
                as *const os::raw::c_char,
            b"lib/src/get_changed_ranges.c\x00" as *const u8 as *const os::raw::c_char,
            126 as os::raw::c_int as os::raw::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[os::raw::c_char; 43]>(
                b"Length iterator_start_position(Iterator *)\x00",
            ))
            .as_ptr(),
        );
    }
    let mut entry: TreeCursorEntry = *(&mut *(*self_0).cursor.stack.contents.offset(
        (*self_0)
            .cursor
            .stack
            .size
            .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint) as isize,
    ) as *mut TreeCursorEntry);
    if (*self_0).in_padding {
        return entry.position;
    } else {
        return length_add(entry.position, ts_subtree_padding(*entry.subtree));
    };
}
unsafe extern "C" fn iterator_end_position(mut self_0: *mut Iterator_0) -> Length {
    if (*self_0)
        .cursor
        .stack
        .size
        .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint)
        < (*self_0).cursor.stack.size
    {
    } else {
        __assert_fail(
            b"(u32)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00" as *const u8
                as *const os::raw::c_char,
            b"lib/src/get_changed_ranges.c\x00" as *const u8 as *const os::raw::c_char,
            135 as os::raw::c_int as os::raw::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[os::raw::c_char; 41]>(
                b"Length iterator_end_position(Iterator *)\x00",
            ))
            .as_ptr(),
        );
    }
    let mut entry: TreeCursorEntry = *(&mut *(*self_0).cursor.stack.contents.offset(
        (*self_0)
            .cursor
            .stack
            .size
            .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint) as isize,
    ) as *mut TreeCursorEntry);
    let mut result: Length = length_add(entry.position, ts_subtree_padding(*entry.subtree));
    if (*self_0).in_padding {
        return result;
    } else {
        return length_add(result, ts_subtree_size(*entry.subtree));
    };
}
unsafe extern "C" fn iterator_tree_is_visible(mut self_0: *const Iterator_0) -> bool {
    if (*self_0)
        .cursor
        .stack
        .size
        .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint)
        < (*self_0).cursor.stack.size
    {
    } else {
        __assert_fail(
            b"(u32)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00" as *const u8
                as *const os::raw::c_char,
            b"lib/src/get_changed_ranges.c\x00" as *const u8 as *const os::raw::c_char,
            145 as os::raw::c_int as os::raw::c_uint,
            (*::std::mem::transmute::<&[u8; 49], &[os::raw::c_char; 49]>(
                b"_Bool iterator_tree_is_visible(const Iterator *)\x00",
            ))
            .as_ptr(),
        );
    }
    let mut entry: TreeCursorEntry = *(&mut *(*self_0).cursor.stack.contents.offset(
        (*self_0)
            .cursor
            .stack
            .size
            .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint) as isize,
    ) as *mut TreeCursorEntry);
    if ts_subtree_visible(*entry.subtree) {
        return 1 as os::raw::c_int != 0;
    }
    if (*self_0).cursor.stack.size > 1 as os::raw::c_int as os::raw::c_uint {
        let mut parent: Subtree = *(*(*self_0).cursor.stack.contents.offset(
            (*self_0)
                .cursor
                .stack
                .size
                .wrapping_sub(2 as os::raw::c_int as os::raw::c_uint) as isize,
        ))
        .subtree;
        let mut alias_sequence: *const TSSymbol = ts_language_alias_sequence(
            (*self_0).language,
            (*parent.ptr).c2rust_unnamed.c2rust_unnamed.production_id as u32,
        );
        return !alias_sequence.is_null()
            && *alias_sequence.offset(entry.structural_child_index as isize) as os::raw::c_int
                != 0 as os::raw::c_int;
    }
    return 0 as os::raw::c_int != 0;
}
unsafe extern "C" fn iterator_get_visible_state(
    mut self_0: *const Iterator_0,
    mut tree: *mut Subtree,
    mut alias_symbol: *mut TSSymbol,
    mut start_byte: *mut u32,
) {
    let mut i: u32 = (*self_0)
        .cursor
        .stack
        .size
        .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint);
    if (*self_0).in_padding {
        if i == 0 as os::raw::c_int as os::raw::c_uint {
            return;
        }
        i = i.wrapping_sub(1)
    }
    while i.wrapping_add(1 as os::raw::c_int as os::raw::c_uint) > 0 as os::raw::c_int as os::raw::c_uint {
        let mut entry: TreeCursorEntry = *(*self_0).cursor.stack.contents.offset(i as isize);
        if i > 0 as os::raw::c_int as os::raw::c_uint {
            let mut parent: *const Subtree = (*(*self_0)
                .cursor
                .stack
                .contents
                .offset(i.wrapping_sub(1 as os::raw::c_int as os::raw::c_uint) as isize))
            .subtree;
            let mut alias_sequence: *const TSSymbol = ts_language_alias_sequence(
                (*self_0).language,
                (*(*parent).ptr).c2rust_unnamed.c2rust_unnamed.production_id as u32,
            );
            if !alias_sequence.is_null() {
                *alias_symbol = *alias_sequence.offset(entry.structural_child_index as isize)
            }
        }
        if ts_subtree_visible(*entry.subtree) as os::raw::c_int != 0
            || *alias_symbol as os::raw::c_int != 0
        {
            *tree = *entry.subtree;
            *start_byte = entry.position.bytes;
            break;
        } else {
            i = i.wrapping_sub(1)
        }
    }
}
unsafe extern "C" fn iterator_ascend(mut self_0: *mut Iterator_0) {
    if iterator_done(self_0) {
        return;
    }
    if iterator_tree_is_visible(self_0) as os::raw::c_int != 0 && !(*self_0).in_padding {
        (*self_0).visible_depth = (*self_0).visible_depth.wrapping_sub(1)
    }
    if (*self_0)
        .cursor
        .stack
        .size
        .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint)
        < (*self_0).cursor.stack.size
    {
    } else {
        __assert_fail(
            b"(u32)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00" as *const u8
                as *const os::raw::c_char,
            b"lib/src/get_changed_ranges.c\x00" as *const u8 as *const os::raw::c_char,
            192 as os::raw::c_int as os::raw::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[os::raw::c_char; 33]>(
                b"void iterator_ascend(Iterator *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*(&mut *(*self_0).cursor.stack.contents.offset(
        (*self_0)
            .cursor
            .stack
            .size
            .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint) as isize,
    ) as *mut TreeCursorEntry))
        .child_index
        > 0 as os::raw::c_int as os::raw::c_uint
    {
        (*self_0).in_padding = 0 as os::raw::c_int != 0
    }
    (*self_0).cursor.stack.size = (*self_0).cursor.stack.size.wrapping_sub(1);
}
unsafe extern "C" fn iterator_descend(mut self_0: *mut Iterator_0, mut goal_position: u32) -> bool {
    if (*self_0).in_padding {
        return 0 as os::raw::c_int != 0;
    }
    let mut did_descend: bool = false;
    loop {
        did_descend = 0 as os::raw::c_int != 0;
        if (*self_0)
            .cursor
            .stack
            .size
            .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint)
            < (*self_0).cursor.stack.size
        {
        } else {
            __assert_fail(
                b"(u32)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00"
                    as *const u8 as *const os::raw::c_char,
                b"lib/src/get_changed_ranges.c\x00" as *const u8 as *const os::raw::c_char,
                202 as os::raw::c_int as os::raw::c_uint,
                (*::std::mem::transmute::<&[u8; 45], &[os::raw::c_char; 45]>(
                    b"_Bool iterator_descend(Iterator *, uint32_t)\x00",
                ))
                .as_ptr(),
            );
        }
        let mut entry: TreeCursorEntry = *(&mut *(*self_0).cursor.stack.contents.offset(
            (*self_0)
                .cursor
                .stack
                .size
                .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint) as isize,
        ) as *mut TreeCursorEntry);
        let mut position: Length = entry.position;
        let mut structural_child_index: u32 = 0 as os::raw::c_int as u32;
        let mut i: u32 = 0 as os::raw::c_int as u32;
        let mut n: u32 = ts_subtree_child_count(*entry.subtree);
        while i < n {
            let mut child: *const Subtree = &mut *(*(*entry.subtree).ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .children
                .offset(i as isize) as *mut Subtree;
            let mut child_left: Length = length_add(position, ts_subtree_padding(*child));
            let mut child_right: Length = length_add(child_left, ts_subtree_size(*child));
            if child_right.bytes > goal_position {
                array__grow(
                    &mut (*self_0).cursor.stack as *mut TreeCursorEntryArray as *mut VoidArray,
                    1 as os::raw::c_int as size_t,
                    ::std::mem::size_of::<TreeCursorEntry>() as os::raw::c_ulong,
                );
                let fresh2 = (*self_0).cursor.stack.size;
                (*self_0).cursor.stack.size = (*self_0).cursor.stack.size.wrapping_add(1);
                *(*self_0).cursor.stack.contents.offset(fresh2 as isize) = {
                    let mut init = TreeCursorEntry {
                        subtree: child,
                        position: position,
                        child_index: i,
                        structural_child_index: structural_child_index,
                    };
                    init
                };
                if iterator_tree_is_visible(self_0) {
                    if child_left.bytes > goal_position {
                        (*self_0).in_padding = 1 as os::raw::c_int != 0
                    } else {
                        (*self_0).visible_depth = (*self_0).visible_depth.wrapping_add(1)
                    }
                    return 1 as os::raw::c_int != 0;
                }
                did_descend = 1 as os::raw::c_int != 0;
                break;
            } else {
                position = child_right;
                if !ts_subtree_extra(*child) {
                    structural_child_index = structural_child_index.wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
        }
        if !did_descend {
            break;
        }
    }
    return 0 as os::raw::c_int != 0;
}
unsafe extern "C" fn iterator_advance(mut self_0: *mut Iterator_0) {
    if (*self_0).in_padding {
        (*self_0).in_padding = 0 as os::raw::c_int != 0;
        if iterator_tree_is_visible(self_0) {
            (*self_0).visible_depth = (*self_0).visible_depth.wrapping_add(1)
        } else {
            iterator_descend(self_0, 0 as os::raw::c_int as u32);
        }
        return;
    }
    loop {
        if iterator_tree_is_visible(self_0) {
            (*self_0).visible_depth = (*self_0).visible_depth.wrapping_sub(1)
        }
        (*self_0).cursor.stack.size = (*self_0).cursor.stack.size.wrapping_sub(1);
        let mut entry: TreeCursorEntry = *(*self_0)
            .cursor
            .stack
            .contents
            .offset((*self_0).cursor.stack.size as isize);
        if iterator_done(self_0) {
            return;
        }
        if (*self_0)
            .cursor
            .stack
            .size
            .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint)
            < (*self_0).cursor.stack.size
        {
        } else {
            __assert_fail(
                b"(u32)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00"
                    as *const u8 as *const os::raw::c_char,
                b"lib/src/get_changed_ranges.c\x00" as *const u8 as *const os::raw::c_char,
                255 as os::raw::c_int as os::raw::c_uint,
                (*::std::mem::transmute::<&[u8; 34], &[os::raw::c_char; 34]>(
                    b"void iterator_advance(Iterator *)\x00",
                ))
                .as_ptr(),
            );
        }
        let mut parent: *const Subtree = (*(&mut *(*self_0).cursor.stack.contents.offset(
            (*self_0)
                .cursor
                .stack
                .size
                .wrapping_sub(1 as os::raw::c_int as os::raw::c_uint) as isize,
        ) as *mut TreeCursorEntry))
            .subtree;
        let mut child_index: u32 = entry
            .child_index
            .wrapping_add(1 as os::raw::c_int as os::raw::c_uint);
        if !(ts_subtree_child_count(*parent) > child_index) {
            continue;
        }
        let mut position: Length =
            length_add(entry.position, ts_subtree_total_size(*entry.subtree));
        let mut structural_child_index: u32 = entry.structural_child_index;
        if !ts_subtree_extra(*entry.subtree) {
            structural_child_index = structural_child_index.wrapping_add(1)
        }
        let mut next_child: *const Subtree = &mut *(*(*parent).ptr)
            .c2rust_unnamed
            .c2rust_unnamed
            .children
            .offset(child_index as isize)
            as *mut Subtree;
        array__grow(
            &mut (*self_0).cursor.stack as *mut TreeCursorEntryArray as *mut VoidArray,
            1 as os::raw::c_int as size_t,
            ::std::mem::size_of::<TreeCursorEntry>() as os::raw::c_ulong,
        );
        let fresh3 = (*self_0).cursor.stack.size;
        (*self_0).cursor.stack.size = (*self_0).cursor.stack.size.wrapping_add(1);
        *(*self_0).cursor.stack.contents.offset(fresh3 as isize) = {
            let mut init = TreeCursorEntry {
                subtree: next_child,
                position: position,
                child_index: child_index,
                structural_child_index: structural_child_index,
            };
            init
        };
        if iterator_tree_is_visible(self_0) {
            if ts_subtree_padding(*next_child).bytes > 0 as os::raw::c_int as os::raw::c_uint {
                (*self_0).in_padding = 1 as os::raw::c_int != 0
            } else {
                (*self_0).visible_depth = (*self_0).visible_depth.wrapping_add(1)
            }
        } else {
            iterator_descend(self_0, 0 as os::raw::c_int as u32);
        }
        break;
    }
}
unsafe extern "C" fn iterator_compare(
    mut old_iter: *const Iterator_0,
    mut new_iter: *const Iterator_0,
) -> IteratorComparison {
    let mut old_tree: Subtree = Subtree {
        ptr: 0 as *const SubtreeHeapData,
    };
    let mut new_tree: Subtree = Subtree {
        ptr: 0 as *const SubtreeHeapData,
    };
    let mut old_start: u32 = 0 as os::raw::c_int as u32;
    let mut new_start: u32 = 0 as os::raw::c_int as u32;
    let mut old_alias_symbol: TSSymbol = 0 as os::raw::c_int as TSSymbol;
    let mut new_alias_symbol: TSSymbol = 0 as os::raw::c_int as TSSymbol;
    iterator_get_visible_state(
        old_iter,
        &mut old_tree,
        &mut old_alias_symbol,
        &mut old_start,
    );
    iterator_get_visible_state(
        new_iter,
        &mut new_tree,
        &mut new_alias_symbol,
        &mut new_start,
    );
    if old_tree.ptr.is_null() && new_tree.ptr.is_null() {
        return IteratorMatches;
    }
    if old_tree.ptr.is_null() || new_tree.ptr.is_null() {
        return IteratorDiffers;
    }
    if old_alias_symbol as os::raw::c_int == new_alias_symbol as os::raw::c_int
        && ts_subtree_symbol(old_tree) as os::raw::c_int == ts_subtree_symbol(new_tree) as os::raw::c_int
    {
        if old_start == new_start
            && !ts_subtree_has_changes(old_tree)
            && ts_subtree_symbol(old_tree) as os::raw::c_int
                != -(1 as os::raw::c_int) as TSSymbol as os::raw::c_int
            && ts_subtree_size(old_tree).bytes == ts_subtree_size(new_tree).bytes
            && ts_subtree_parse_state(old_tree) as os::raw::c_int != TS_TREE_STATE_NONE as os::raw::c_int
            && ts_subtree_parse_state(new_tree) as os::raw::c_int != TS_TREE_STATE_NONE as os::raw::c_int
            && (ts_subtree_parse_state(old_tree) as os::raw::c_int == 0 as os::raw::c_int) as os::raw::c_int
                == (ts_subtree_parse_state(new_tree) as os::raw::c_int == 0 as os::raw::c_int)
                    as os::raw::c_int
        {
            return IteratorMatches;
        } else {
            return IteratorMayDiffer;
        }
    }
    return IteratorDiffers;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_get_changed_ranges(
    mut old_tree: *const Subtree,
    mut new_tree: *const Subtree,
    mut cursor1: *mut TreeCursor,
    mut cursor2: *mut TreeCursor,
    mut language: *const TSLanguage,
    mut included_range_differences: *const TSRangeArray,
    mut ranges: *mut *mut TSRange,
) -> os::raw::c_uint {
    let mut results: TSRangeArray = {
        let mut init = TSRangeArray {
            contents: 0 as *mut TSRange,
            size: 0 as os::raw::c_int as u32,
            capacity: 0 as os::raw::c_int as u32,
        };
        init
    };
    let mut old_iter: Iterator_0 = iterator_new(cursor1, old_tree, language);
    let mut new_iter: Iterator_0 = iterator_new(cursor2, new_tree, language);
    let mut included_range_difference_index: os::raw::c_uint = 0 as os::raw::c_int as os::raw::c_uint;
    let mut position: Length = iterator_start_position(&mut old_iter);
    let mut next_position: Length = iterator_start_position(&mut new_iter);
    if position.bytes < next_position.bytes {
        ts_range_array_add(&mut results, position, next_position);
        position = next_position
    } else if position.bytes > next_position.bytes {
        ts_range_array_add(&mut results, next_position, position);
        next_position = position
    }
    loop {
        // Compare the old and new subtrees.
        let mut comparison: IteratorComparison = iterator_compare(&mut old_iter, &mut new_iter);
        // Even if the two subtrees appear to be identical, they could differ
        // internally if they contain a range of text that was previously
        // excluded from the parse, and is now included, or vice-versa.
        if comparison as os::raw::c_uint == IteratorMatches as os::raw::c_int as os::raw::c_uint
            && ts_range_array_intersects(
                included_range_differences,
                included_range_difference_index,
                position.bytes,
                iterator_end_position(&mut old_iter).bytes,
            ) as os::raw::c_int
                != 0
        {
            comparison = IteratorMayDiffer
        }
        let mut is_changed: bool = 0 as os::raw::c_int != 0;
        match comparison as os::raw::c_uint {
            2 => {
                // If the subtrees are definitely identical, move to the end
                // of both subtrees.
                next_position = iterator_end_position(&mut old_iter)
            }
            1 => {
                // If the subtrees might differ internally, descend into both
                // subtrees, finding the first child that spans the current position.
                if iterator_descend(&mut old_iter, position.bytes) {
                    if !iterator_descend(&mut new_iter, position.bytes) {
                        is_changed = 1 as os::raw::c_int != 0;
                        next_position = iterator_end_position(&mut old_iter)
                    }
                } else if iterator_descend(&mut new_iter, position.bytes) {
                    is_changed = 1 as os::raw::c_int != 0;
                    next_position = iterator_end_position(&mut new_iter)
                } else {
                    next_position = length_min(
                        iterator_end_position(&mut old_iter),
                        iterator_end_position(&mut new_iter),
                    )
                }
            }
            0 => {
                // If the subtrees are different, record a change and then move
                // to the end of both subtrees.
                is_changed = 1 as os::raw::c_int != 0;
                next_position = length_min(
                    iterator_end_position(&mut old_iter),
                    iterator_end_position(&mut new_iter),
                )
            }
            _ => {}
        }
        // Ensure that both iterators are caught up to the current position.
        while !iterator_done(&mut old_iter)
            && iterator_end_position(&mut old_iter).bytes <= next_position.bytes
        {
            iterator_advance(&mut old_iter);
        }
        while !iterator_done(&mut new_iter)
            && iterator_end_position(&mut new_iter).bytes <= next_position.bytes
        {
            iterator_advance(&mut new_iter);
        }
        // Ensure that both iterators are at the same depth in the tree.
        while old_iter.visible_depth > new_iter.visible_depth {
            iterator_ascend(&mut old_iter);
        }
        while new_iter.visible_depth > old_iter.visible_depth {
            iterator_ascend(&mut new_iter);
        }
        if is_changed {
            ts_range_array_add(&mut results, position, next_position);
        }
        position = next_position;
        // Keep track of the current position in the included range differences
        // array in order to avoid scanning the entire array on each iteration.
        while included_range_difference_index < (*included_range_differences).size {
            let mut range: *const TSRange = &mut *(*included_range_differences)
                .contents
                .offset(included_range_difference_index as isize)
                as *mut TSRange;
            if !((*range).end_byte <= position.bytes) {
                break;
            }
            included_range_difference_index = included_range_difference_index.wrapping_add(1)
        }
        if !(!iterator_done(&mut old_iter) && !iterator_done(&mut new_iter)) {
            break;
        }
    }
    let mut old_size: Length = ts_subtree_total_size(*old_tree);
    let mut new_size: Length = ts_subtree_total_size(*new_tree);
    if old_size.bytes < new_size.bytes {
        ts_range_array_add(&mut results, old_size, new_size);
    } else if new_size.bytes < old_size.bytes {
        ts_range_array_add(&mut results, new_size, old_size);
    }
    *cursor1 = old_iter.cursor;
    *cursor2 = new_iter.cursor;
    *ranges = results.contents;
    return results.size;
}
