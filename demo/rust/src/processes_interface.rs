/* generated by rust_qt_binding_generator */
#![allow(unknown_lints)]
#![allow(mutex_atomic, needless_pass_by_value)]
use libc::{c_int, c_void, uint8_t, uint16_t};
use std::slice;

use std::sync::{Arc, Mutex};
use std::ptr::null;

use processes_implementation::*;


#[repr(C)]
pub struct COption<T> {
    data: T,
    some: bool,
}

impl<T> From<Option<T>> for COption<T> where T: Default {
    fn from(t: Option<T>) -> COption <T> {
        if let Some(v) = t {
            COption {
                data: v,
                some: true
            }
        } else {
            COption {
                data: T::default(),
                some: false
            }
        }
    }
}


#[repr(C)]
pub struct QString {
    data: *const uint8_t,
    len: c_int,
}

#[repr(C)]
pub struct QStringIn {
    data: *const uint16_t,
    len: c_int,
}

impl QStringIn {
    fn convert(&self) -> String {
        let data = unsafe { slice::from_raw_parts(self.data, self.len as usize) };
        String::from_utf16_lossy(data)
    }
}

impl<'a> From<&'a String> for QString {
    fn from(string: &'a String) -> QString {
        QString {
            len: string.len() as c_int,
            data: string.as_ptr(),
        }
    }
}


#[repr(C)]
pub enum SortOrder {
    Ascending = 0,
    Descending = 1
}

#[repr(C)]
pub struct QModelIndex {
    row: c_int,
    internal_id: usize,
}

pub struct ProcessesQObject {}

#[derive (Clone)]
pub struct ProcessesEmitter {
    qobject: Arc<Mutex<*const ProcessesQObject>>,
    active_changed: fn(*const ProcessesQObject),
    new_data_ready: fn(*const ProcessesQObject, item: usize, valid: bool),
}

unsafe impl Send for ProcessesEmitter {}

impl ProcessesEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
    pub fn active_changed(&self) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.active_changed)(ptr);
        }
    }
    pub fn new_data_ready(&self, item: Option<usize>) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
             (self.new_data_ready)(ptr, item.unwrap_or(13), item.is_some());
        }
    }
}

pub struct ProcessesUniformTree {
    qobject: *const ProcessesQObject,
    data_changed: fn(*const ProcessesQObject, usize, usize),
    begin_reset_model: fn(*const ProcessesQObject),
    end_reset_model: fn(*const ProcessesQObject),
    begin_insert_rows: fn(*const ProcessesQObject, item: usize, valid: bool, usize, usize),
    end_insert_rows: fn(*const ProcessesQObject),
    begin_remove_rows: fn(*const ProcessesQObject, item: usize, valid: bool, usize, usize),
    end_remove_rows: fn(*const ProcessesQObject),
}

impl ProcessesUniformTree {
    pub fn data_changed(&self, first: usize, last: usize) {
        (self.data_changed)(self.qobject, first, last);
    }
    pub fn begin_reset_model(&self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&self, item: Option<usize>, first: usize, last: usize) {
        (self.begin_insert_rows)(self.qobject, item.unwrap_or(13), item.is_some(), first, last);
    }
    pub fn end_insert_rows(&self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&self, item: Option<usize>, first: usize, last: usize) {
        (self.begin_remove_rows)(self.qobject, item.unwrap_or(13), item.is_some(), first, last);
    }
    pub fn end_remove_rows(&self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait ProcessesTrait {
    fn create(emit: ProcessesEmitter, model: ProcessesUniformTree) -> Self;
    fn emit(&self) -> &ProcessesEmitter;
    fn get_active(&self) -> bool;
    fn set_active(&mut self, value: bool);
    fn row_count(&self, Option<usize>) -> usize;
    fn can_fetch_more(&self, Option<usize>) -> bool { false }
    fn fetch_more(&mut self, Option<usize>) {}
    fn sort(&mut self, u8, SortOrder) {}
    fn index(&self, item: Option<usize>, row: usize) -> usize;
    fn parent(&self, item: usize) -> Option<usize>;
    fn row(&self, item: usize) -> usize;
    fn cmd(&self, item: usize) -> String;
    fn cpu_percentage(&self, item: usize) -> u8;
    fn cpu_usage(&self, item: usize) -> f32;
    fn memory(&self, item: usize) -> u64;
    fn name(&self, item: usize) -> String;
    fn pid(&self, item: usize) -> u32;
    fn uid(&self, item: usize) -> u32;
}

#[no_mangle]
pub extern "C" fn processes_new(processes: *mut ProcessesQObject,
        active_changed: fn(*const ProcessesQObject),
        new_data_ready: fn(*const ProcessesQObject, item: usize, valid: bool),
        data_changed: fn(*const ProcessesQObject, usize, usize),
        begin_reset_model: fn(*const ProcessesQObject),
        end_reset_model: fn(*const ProcessesQObject),
        begin_insert_rows: fn(*const ProcessesQObject, item: usize, valid: bool,
            usize,
            usize),
        end_insert_rows: fn(*const ProcessesQObject),
        begin_remove_rows: fn(*const ProcessesQObject, item: usize, valid: bool,
            usize,
            usize),
        end_remove_rows: fn(*const ProcessesQObject))
        -> *mut Processes {
    let processes_emit = ProcessesEmitter {
        qobject: Arc::new(Mutex::new(processes)),
        active_changed: active_changed,
        new_data_ready: new_data_ready,
    };
    let model = ProcessesUniformTree {
        qobject: processes,
        data_changed: data_changed,
        begin_reset_model: begin_reset_model,
        end_reset_model: end_reset_model,
        begin_insert_rows: begin_insert_rows,
        end_insert_rows: end_insert_rows,
        begin_remove_rows: begin_remove_rows,
        end_remove_rows: end_remove_rows,
    };
    let d_processes = Processes::create(processes_emit, model);
    Box::into_raw(Box::new(d_processes))
}

#[no_mangle]
pub unsafe extern "C" fn processes_free(ptr: *mut Processes) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn processes_active_get(ptr: *const Processes) -> bool {
    (&*ptr).get_active()
}

#[no_mangle]
pub unsafe extern "C" fn processes_active_set(ptr: *mut Processes, v: bool) {
    (&mut *ptr).set_active(v);
}

#[no_mangle]
pub unsafe extern "C" fn processes_row_count(ptr: *const Processes, item: usize, valid: bool) -> c_int {
    if valid {
        (&*ptr).row_count(Some(item)) as c_int
    } else {
        (&*ptr).row_count(None) as c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn processes_can_fetch_more(ptr: *const Processes, item: usize, valid: bool) -> bool {
    if valid {
        (&*ptr).can_fetch_more(Some(item))
    } else {
        (&*ptr).can_fetch_more(None)
    }
}
#[no_mangle]
pub unsafe extern "C" fn processes_fetch_more(ptr: *mut Processes, item: usize, valid: bool) {
    if valid {
        (&mut *ptr).fetch_more(Some(item))
    } else {
        (&mut *ptr).fetch_more(None)
    }
}
#[no_mangle]
pub unsafe extern "C" fn processes_sort(ptr: *mut Processes, column: u8, order: SortOrder) {
    (&mut *ptr).sort(column, order)
}
#[no_mangle]
pub unsafe extern "C" fn processes_index(ptr: *const Processes, item: usize, valid: bool, row: c_int) -> usize {
    if !valid {
        (&*ptr).index(None, row as usize)
    } else {
        (&*ptr).index(Some(item), row as usize)
    }
}
#[no_mangle]
pub unsafe extern "C" fn processes_parent(ptr: *const Processes, index: usize) -> QModelIndex {
    if let Some(parent) = (&*ptr).parent(index) {
        QModelIndex{row: (&*ptr).row(parent) as c_int, internal_id: parent}
    } else {
        QModelIndex{row: -1, internal_id: 0}
    }
}
#[no_mangle]
pub unsafe extern "C" fn processes_row(ptr: *const Processes, item: usize) -> c_int {
    (&*ptr).row(item) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_cmd(ptr: *const Processes, item: usize,
        d: *mut c_void,
        set: fn(*mut c_void, QString)) {
    let data = (&*ptr).cmd(item);
    set(d, QString::from(&data));
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_cpu_percentage(ptr: *const Processes, item: usize) -> u8 {
    (&*ptr).cpu_percentage(item).into()
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_cpu_usage(ptr: *const Processes, item: usize) -> f32 {
    (&*ptr).cpu_usage(item).into()
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_memory(ptr: *const Processes, item: usize) -> u64 {
    (&*ptr).memory(item).into()
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_name(ptr: *const Processes, item: usize,
        d: *mut c_void,
        set: fn(*mut c_void, QString)) {
    let data = (&*ptr).name(item);
    set(d, QString::from(&data));
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_pid(ptr: *const Processes, item: usize) -> u32 {
    (&*ptr).pid(item).into()
}

#[no_mangle]
pub unsafe extern "C" fn processes_data_uid(ptr: *const Processes, item: usize) -> u32 {
    (&*ptr).uid(item).into()
}
