/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

/* automatically generated by rust-bindgen */

use std::libc;
use std::libc::*;
use jsapi::*;
use jsfriendapi::JSJitInfo;
use jsval::JSVal;

type c_bool = libc::c_int;

pub struct ProxyTraps {
    getPropertyDescriptor: extern "C" fn(*JSContext, *JSObject, jsid, c_bool, *mut JSPropertyDescriptor) -> c_bool,
    getOwnPropertyDescriptor: extern "C" fn(*JSContext, *JSObject, jsid, JSBool, *mut JSPropertyDescriptor) -> JSBool,
    defineProperty: extern "C" fn(*JSContext, *JSObject, jsid, *JSPropertyDescriptor) -> JSBool,
    getOwnPropertyNames: *u8,
    delete_: *u8,
    enumerate: *u8,

    has: *u8,
    hasOwn: extern "C" fn(*JSContext, *JSObject, jsid, *mut JSBool) -> JSBool,
    get: extern "C" fn(*JSContext, *JSObject, *JSObject, jsid, *mut JSVal) -> JSBool,
    set: *u8,
    keys: *u8,
    iterate: *u8,

    call: *u8,
    construct: *u8,
    nativeCall: *u8,
    hasInstance: *u8,
    typeOf: *u8,
    objectClassIs: *u8,
    obj_toString: extern "C" fn(*JSContext, *JSObject) -> *JSString,
    fun_toString: *u8,
    //regexp_toShared: *u8,
    defaultValue: *u8,
    iteratorNext: *u8,
    finalize: extern "C" fn(*JSFreeOp, *JSObject),
    getElementIfPresent: *u8,
    getPrototypeOf: *u8,
    trace: Option<extern "C" fn(*mut JSTracer, *JSObject)>
}

#[link(name = "jsglue")]
extern { }


#[cfg(target_os = "android")]
#[link_args = "-ljsglue -lstdc++ -lgcc -rdynamic"]
extern { }

extern {

//#[rust_stack]
pub fn RUST_JS_NumberValue(d: f64) -> JSVal;

//#[rust_stack]
pub fn CallJitPropertyOp(info: *JSJitInfo, cx: *JSContext, thisObj: *JSObject, specializedThis: *libc::c_void, vp: *JSVal) -> JSBool;

//#[rust_stack]
pub fn CallJitMethodOp(info: *JSJitInfo, cx: *JSContext, thisObj: *JSObject, specializedThis: *libc::c_void, argc: libc::c_uint, vp: *JSVal) -> JSBool;

//#[rust_stack]
pub fn RUST_FUNCTION_VALUE_TO_JITINFO(v: JSVal) -> *JSJitInfo;

pub fn SetFunctionNativeReserved(fun: *JSObject, which: libc::size_t, val: *JSVal);
pub fn GetFunctionNativeReserved(fun: *JSObject, which: libc::size_t) -> *JSVal;

pub fn CreateProxyHandler(traps: *ProxyTraps, extra: *libc::c_void) -> *libc::c_void;
pub fn NewProxyObject(cx: *JSContext, handler: *libc::c_void, priv_: *JSVal,
                      proto: *JSObject, parent: *JSObject, call: *JSObject,
                      construct: *JSObject) -> *JSObject;

pub fn GetProxyExtra(obj: *JSObject, slot: c_uint) -> JSVal;
pub fn GetProxyPrivate(obj: *JSObject) -> JSVal;
pub fn SetProxyExtra(obj: *JSObject, slot: c_uint, val: JSVal);

pub fn GetObjectProto(obj: *JSObject) -> *JSObject;
pub fn GetObjectParent(obj: *JSObject) -> *JSObject;

pub fn RUST_JSID_IS_INT(id: jsid) -> JSBool;
pub fn RUST_JSID_TO_INT(id: jsid) -> libc::c_int;
pub fn RUST_JSID_IS_STRING(id: jsid) -> JSBool;
pub fn RUST_JSID_TO_STRING(id: jsid) -> *JSString;

pub fn RUST_SET_JITINFO(func: *JSFunction, info: *JSJitInfo);

pub fn RUST_INTERNED_STRING_TO_JSID(cx: *JSContext, str: *JSString) -> jsid;

pub fn DefineFunctionWithReserved(cx: *JSContext, obj: *JSObject,
                                  name: *libc::c_char, call: JSNative, nargs: libc::c_uint,
                                  attrs: libc::c_uint) -> *JSObject;
pub fn GetObjectJSClass(obj: *JSObject) -> *JSClass;
pub fn js_GetErrorMessage(userRef: *libc::c_void, locale: *libc::c_char,
                          errorNumber: libc::c_uint) -> *JSErrorFormatString;
pub fn js_IsObjectProxyClass(obj: *JSObject) -> bool;
pub fn js_IsFunctionProxyClass(obj: *JSObject) -> bool;
pub fn IsProxyHandlerFamily(obj: *JSObject) -> bool;
pub fn GetProxyHandlerExtra(obj: *JSObject) -> *libc::c_void;
pub fn GetProxyHandler(obj: *JSObject) -> *libc::c_void;
pub fn InvokeGetOwnPropertyDescriptor(handler: *libc::c_void, cx: *JSContext, proxy: *JSObject, id: jsid, set: JSBool, desc: *mut JSPropertyDescriptor) -> JSBool;
pub fn GetGlobalForObjectCrossCompartment(obj: *JSObject) -> *JSObject;
pub fn ReportError(cx: *JSContext, error: *c_char);
}
