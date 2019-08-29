/* generated by rust_qt_binding_generator */
use libc::{c_char, c_ushort, c_int};
use std::slice;
use std::char::decode_utf16;

use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr::null;

use implementation::*;


#[repr(C)]
pub struct COption<T> {
    data: T,
    some: bool,
}

impl<T> COption<T> {
    #![allow(dead_code)]
    fn into(self) -> Option<T> {
        if self.some {
            Some(self.data)
        } else {
            None
        }
    }
}

impl<T> From<Option<T>> for COption<T>
where
    T: Default,
{
    fn from(t: Option<T>) -> COption<T> {
        if let Some(v) = t {
            COption {
                data: v,
                some: true,
            }
        } else {
            COption {
                data: T::default(),
                some: false,
            }
        }
    }
}


pub enum QString {}

fn set_string_from_utf16(s: &mut String, str: *const c_ushort, len: c_int) {
    let utf16 = unsafe { slice::from_raw_parts(str, to_usize(len)) };
    let characters = decode_utf16(utf16.iter().cloned())
        .map(|r| r.unwrap());
    s.clear();
    s.extend(characters);
}



pub enum QByteArray {}


#[repr(C)]
#[derive(PartialEq, Eq, Debug)]
pub enum SortOrder {
    Ascending = 0,
    Descending = 1,
}

#[repr(C)]
pub struct QModelIndex {
    row: c_int,
    internal_id: usize,
}


fn to_usize(n: c_int) -> usize {
    if n < 0 {
        panic!("Cannot cast {} to usize", n);
    }
    n as usize
}


fn to_c_int(n: usize) -> c_int {
    if n > c_int::max_value() as usize {
        panic!("Cannot cast {} to c_int", n);
    }
    n as c_int
}


pub struct ListQObject {}

pub struct ListEmitter {
    qobject: Arc<AtomicPtr<ListQObject>>,
    new_data_ready: fn(*mut ListQObject),
}

unsafe impl Send for ListEmitter {}

impl ListEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> ListEmitter {
        ListEmitter {
            qobject: self.qobject.clone(),
            new_data_ready: self.new_data_ready,
        }
    }
    fn clear(&self) {
        let n: *const ListQObject = null();
        self.qobject.store(n as *mut ListQObject, Ordering::SeqCst);
    }
    pub fn new_data_ready(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.new_data_ready)(ptr);
        }
    }
}

#[derive(Clone)]
pub struct ListList {
    qobject: *mut ListQObject,
    layout_about_to_be_changed: fn(*mut ListQObject),
    layout_changed: fn(*mut ListQObject),
    data_changed: fn(*mut ListQObject, usize, usize),
    begin_reset_model: fn(*mut ListQObject),
    end_reset_model: fn(*mut ListQObject),
    begin_insert_rows: fn(*mut ListQObject, usize, usize),
    end_insert_rows: fn(*mut ListQObject),
    begin_move_rows: fn(*mut ListQObject, usize, usize, usize),
    end_move_rows: fn(*mut ListQObject),
    begin_remove_rows: fn(*mut ListQObject, usize, usize),
    end_remove_rows: fn(*mut ListQObject),
}

impl ListList {
    pub fn layout_about_to_be_changed(&mut self) {
        (self.layout_about_to_be_changed)(self.qobject);
    }
    pub fn layout_changed(&mut self) {
        (self.layout_changed)(self.qobject);
    }
    pub fn data_changed(&mut self, first: usize, last: usize) {
        (self.data_changed)(self.qobject, first, last);
    }
    pub fn begin_reset_model(&mut self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&mut self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&mut self, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, first, last);
    }
    pub fn end_insert_rows(&mut self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_move_rows(&mut self, first: usize, last: usize, destination: usize) {
        (self.begin_move_rows)(self.qobject, first, last, destination);
    }
    pub fn end_move_rows(&mut self) {
        (self.end_move_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&mut self, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, first, last);
    }
    pub fn end_remove_rows(&mut self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait ListTrait {
    fn new(emit: ListEmitter, model: ListList) -> Self;
    fn emit(&mut self) -> &mut ListEmitter;
    fn row_count(&self) -> usize;
    fn insert_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn remove_rows(&mut self, _row: usize, _count: usize) -> bool { false }
    fn can_fetch_more(&self) -> bool {
        false
    }
    fn fetch_more(&mut self) {}
    fn sort(&mut self, _: u8, _: SortOrder) {}
    fn boolean(&self, index: usize) -> bool;
    fn set_boolean(&mut self, index: usize, _: bool) -> bool;
    fn bytearray(&self, index: usize) -> &[u8];
    fn set_bytearray(&mut self, index: usize, _: &[u8]) -> bool;
    fn f32(&self, index: usize) -> f32;
    fn set_f32(&mut self, index: usize, _: f32) -> bool;
    fn f64(&self, index: usize) -> f64;
    fn set_f64(&mut self, index: usize, _: f64) -> bool;
    fn i16(&self, index: usize) -> i16;
    fn set_i16(&mut self, index: usize, _: i16) -> bool;
    fn i32(&self, index: usize) -> i32;
    fn set_i32(&mut self, index: usize, _: i32) -> bool;
    fn i64(&self, index: usize) -> i64;
    fn set_i64(&mut self, index: usize, _: i64) -> bool;
    fn i8(&self, index: usize) -> i8;
    fn set_i8(&mut self, index: usize, _: i8) -> bool;
    fn optional_boolean(&self, index: usize) -> Option<bool>;
    fn set_optional_boolean(&mut self, index: usize, _: Option<bool>) -> bool;
    fn optional_bytearray(&self, index: usize) -> Option<&[u8]>;
    fn set_optional_bytearray(&mut self, index: usize, _: Option<&[u8]>) -> bool;
    fn optional_string(&self, index: usize) -> Option<&str>;
    fn set_optional_string(&mut self, index: usize, _: Option<String>) -> bool;
    fn string(&self, index: usize) -> &str;
    fn set_string(&mut self, index: usize, _: String) -> bool;
    fn u16(&self, index: usize) -> u16;
    fn set_u16(&mut self, index: usize, _: u16) -> bool;
    fn u32(&self, index: usize) -> u32;
    fn set_u32(&mut self, index: usize, _: u32) -> bool;
    fn u64(&self, index: usize) -> u64;
    fn set_u64(&mut self, index: usize, _: u64) -> bool;
    fn u8(&self, index: usize) -> u8;
    fn set_u8(&mut self, index: usize, _: u8) -> bool;
}

#[no_mangle]
pub extern "C" fn list_new(
    list: *mut ListQObject,
    list_new_data_ready: fn(*mut ListQObject),
    list_layout_about_to_be_changed: fn(*mut ListQObject),
    list_layout_changed: fn(*mut ListQObject),
    list_data_changed: fn(*mut ListQObject, usize, usize),
    list_begin_reset_model: fn(*mut ListQObject),
    list_end_reset_model: fn(*mut ListQObject),
    list_begin_insert_rows: fn(*mut ListQObject, usize, usize),
    list_end_insert_rows: fn(*mut ListQObject),
    list_begin_move_rows: fn(*mut ListQObject, usize, usize, usize),
    list_end_move_rows: fn(*mut ListQObject),
    list_begin_remove_rows: fn(*mut ListQObject, usize, usize),
    list_end_remove_rows: fn(*mut ListQObject),
) -> *mut List {
    let list_emit = ListEmitter {
        qobject: Arc::new(AtomicPtr::new(list)),
        new_data_ready: list_new_data_ready,
    };
    let model = ListList {
        qobject: list,
        layout_about_to_be_changed: list_layout_about_to_be_changed,
        layout_changed: list_layout_changed,
        data_changed: list_data_changed,
        begin_reset_model: list_begin_reset_model,
        end_reset_model: list_end_reset_model,
        begin_insert_rows: list_begin_insert_rows,
        end_insert_rows: list_end_insert_rows,
        begin_move_rows: list_begin_move_rows,
        end_move_rows: list_end_move_rows,
        begin_remove_rows: list_begin_remove_rows,
        end_remove_rows: list_end_remove_rows,
    };
    let d_list = List::new(list_emit, model);
    Box::into_raw(Box::new(d_list))
}

#[no_mangle]
pub unsafe extern "C" fn list_free(ptr: *mut List) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn list_row_count(ptr: *const List) -> c_int {
    to_c_int((&*ptr).row_count())
}
#[no_mangle]
pub unsafe extern "C" fn list_insert_rows(ptr: *mut List, row: c_int, count: c_int) -> bool {
    (&mut *ptr).insert_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn list_remove_rows(ptr: *mut List, row: c_int, count: c_int) -> bool {
    (&mut *ptr).remove_rows(to_usize(row), to_usize(count))
}
#[no_mangle]
pub unsafe extern "C" fn list_can_fetch_more(ptr: *const List) -> bool {
    (&*ptr).can_fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn list_fetch_more(ptr: *mut List) {
    (&mut *ptr).fetch_more()
}
#[no_mangle]
pub unsafe extern "C" fn list_sort(
    ptr: *mut List,
    column: u8,
    order: SortOrder,
) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_boolean(ptr: *const List, row: c_int) -> bool {
    let o = &*ptr;
    o.boolean(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_boolean(
    ptr: *mut List, row: c_int,
    v: bool,
) -> bool {
    (&mut *ptr).set_boolean(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_bytearray(
    ptr: *const List, row: c_int,
    d: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.bytearray(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_bytearray(
    ptr: *mut List, row: c_int,
    s: *const c_char, len: c_int,
) -> bool {
    let o = &mut *ptr;
    let slice = ::std::slice::from_raw_parts(s as *const u8, to_usize(len));
    o.set_bytearray(to_usize(row), slice)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_f32(ptr: *const List, row: c_int) -> f32 {
    let o = &*ptr;
    o.f32(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_f32(
    ptr: *mut List, row: c_int,
    v: f32,
) -> bool {
    (&mut *ptr).set_f32(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_f64(ptr: *const List, row: c_int) -> f64 {
    let o = &*ptr;
    o.f64(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_f64(
    ptr: *mut List, row: c_int,
    v: f64,
) -> bool {
    (&mut *ptr).set_f64(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_i16(ptr: *const List, row: c_int) -> i16 {
    let o = &*ptr;
    o.i16(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_i16(
    ptr: *mut List, row: c_int,
    v: i16,
) -> bool {
    (&mut *ptr).set_i16(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_i32(ptr: *const List, row: c_int) -> i32 {
    let o = &*ptr;
    o.i32(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_i32(
    ptr: *mut List, row: c_int,
    v: i32,
) -> bool {
    (&mut *ptr).set_i32(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_i64(ptr: *const List, row: c_int) -> i64 {
    let o = &*ptr;
    o.i64(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_i64(
    ptr: *mut List, row: c_int,
    v: i64,
) -> bool {
    (&mut *ptr).set_i64(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_i8(ptr: *const List, row: c_int) -> i8 {
    let o = &*ptr;
    o.i8(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_i8(
    ptr: *mut List, row: c_int,
    v: i8,
) -> bool {
    (&mut *ptr).set_i8(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_optional_boolean(ptr: *const List, row: c_int) -> COption<bool> {
    let o = &*ptr;
    o.optional_boolean(to_usize(row)).into()
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_optional_boolean(
    ptr: *mut List, row: c_int,
    v: bool,
) -> bool {
    (&mut *ptr).set_optional_boolean(to_usize(row), Some(v))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_optional_boolean_none(ptr: *mut List, row: c_int) -> bool {
    (&mut *ptr).set_optional_boolean(to_usize(row), None)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_optional_bytearray(
    ptr: *const List, row: c_int,
    d: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.optional_bytearray(to_usize(row));
    if let Some(data) = data {
        let s: *const c_char = data.as_ptr() as (*const c_char);
        set(d, s, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_optional_bytearray(
    ptr: *mut List, row: c_int,
    s: *const c_char, len: c_int,
) -> bool {
    let o = &mut *ptr;
    let slice = ::std::slice::from_raw_parts(s as *const u8, to_usize(len));
    o.set_optional_bytearray(to_usize(row), Some(slice))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_optional_bytearray_none(ptr: *mut List, row: c_int) -> bool {
    (&mut *ptr).set_optional_bytearray(to_usize(row), None)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_optional_string(
    ptr: *const List, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.optional_string(to_usize(row));
    if let Some(data) = data {
        let s: *const c_char = data.as_ptr() as (*const c_char);
        set(d, s, to_c_int(data.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_optional_string(
    ptr: *mut List, row: c_int,
    s: *const c_ushort, len: c_int,
) -> bool {
    let o = &mut *ptr;
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_optional_string(to_usize(row), Some(v))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_optional_string_none(ptr: *mut List, row: c_int) -> bool {
    (&mut *ptr).set_optional_string(to_usize(row), None)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_string(
    ptr: *const List, row: c_int,
    d: *mut QString,
    set: fn(*mut QString, *const c_char, len: c_int),
) {
    let o = &*ptr;
    let data = o.string(to_usize(row));
    let s: *const c_char = data.as_ptr() as (*const c_char);
    set(d, s, to_c_int(data.len()));
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_string(
    ptr: *mut List, row: c_int,
    s: *const c_ushort, len: c_int,
) -> bool {
    let o = &mut *ptr;
    let mut v = String::new();
    set_string_from_utf16(&mut v, s, len);
    o.set_string(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_u16(ptr: *const List, row: c_int) -> u16 {
    let o = &*ptr;
    o.u16(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_u16(
    ptr: *mut List, row: c_int,
    v: u16,
) -> bool {
    (&mut *ptr).set_u16(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_u32(ptr: *const List, row: c_int) -> u32 {
    let o = &*ptr;
    o.u32(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_u32(
    ptr: *mut List, row: c_int,
    v: u32,
) -> bool {
    (&mut *ptr).set_u32(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_u64(ptr: *const List, row: c_int) -> u64 {
    let o = &*ptr;
    o.u64(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_u64(
    ptr: *mut List, row: c_int,
    v: u64,
) -> bool {
    (&mut *ptr).set_u64(to_usize(row), v)
}

#[no_mangle]
pub unsafe extern "C" fn list_data_u8(ptr: *const List, row: c_int) -> u8 {
    let o = &*ptr;
    o.u8(to_usize(row))
}

#[no_mangle]
pub unsafe extern "C" fn list_set_data_u8(
    ptr: *mut List, row: c_int,
    v: u8,
) -> bool {
    (&mut *ptr).set_u8(to_usize(row), v)
}
