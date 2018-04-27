#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[no_mangle]
pub extern "C" fn UnityPluginLoad(unityInterfaces: *mut IUnityInterfaces) {
    println!("sup, dude");

unsafe {
    let fn_ptr = (*unityInterfaces).GetInterface.unwrap();
}
    
}

#[no_mangle]
pub extern "C" fn UnityPluginUnload(){
    println!("see ya");
}

#[no_mangle]
pub extern "C" fn hello() {
    println!("hello world");
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct UnityInterfaceGUID {
    pub m_GUIDHigh: ::std::os::raw::c_ulonglong,
    pub m_GUIDLow: ::std::os::raw::c_ulonglong,
}
#[test]
fn bindgen_test_layout_UnityInterfaceGUID() {
    assert_eq!(::std::mem::size_of::<UnityInterfaceGUID>() , 16usize , concat
               ! ( "Size of: " , stringify ! ( UnityInterfaceGUID ) ));
    assert_eq! (::std::mem::align_of::<UnityInterfaceGUID>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( UnityInterfaceGUID ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const UnityInterfaceGUID ) ) . m_GUIDHigh as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( UnityInterfaceGUID ) ,
                "::" , stringify ! ( m_GUIDHigh ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const UnityInterfaceGUID ) ) . m_GUIDLow as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( UnityInterfaceGUID ) ,
                "::" , stringify ! ( m_GUIDLow ) ));
}

impl Clone for UnityInterfaceGUID {
    fn clone(&self) -> Self { *self }
}

pub type IUnityInterface = ::std::os::raw::c_void;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct IUnityInterfaces {
    pub GetInterface: ::std::option::Option<unsafe extern "C" fn(guid:
                                                                     UnityInterfaceGUID)
                                                -> *mut IUnityInterface>,
    pub RegisterInterface: ::std::option::Option<unsafe extern "C" fn(guid:
                                                                          UnityInterfaceGUID,
                                                                      ptr:
                                                                          *mut IUnityInterface)>,
    pub GetInterfaceSplit: ::std::option::Option<unsafe extern "C" fn(guidHigh:
                                                                          ::std::os::raw::c_ulonglong,
                                                                      guidLow:
                                                                          ::std::os::raw::c_ulonglong)
                                                     -> *mut IUnityInterface>,
    pub RegisterInterfaceSplit: ::std::option::Option<unsafe extern "C" fn(guidHigh:
                                                                               ::std::os::raw::c_ulonglong,
                                                                           guidLow:
                                                                               ::std::os::raw::c_ulonglong,
                                                                           ptr:
                                                                               *mut IUnityInterface)>,
}

#[test]
fn bindgen_test_layout_IUnityInterfaces() {
    assert_eq!(::std::mem::size_of::<IUnityInterfaces>() , 32usize , concat !
               ( "Size of: " , stringify ! ( IUnityInterfaces ) ));
    assert_eq! (::std::mem::align_of::<IUnityInterfaces>() , 8usize , concat !
                ( "Alignment of " , stringify ! ( IUnityInterfaces ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const IUnityInterfaces ) ) . GetInterface as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( IUnityInterfaces ) ,
                "::" , stringify ! ( GetInterface ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const IUnityInterfaces ) ) . RegisterInterface
                as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( IUnityInterfaces ) ,
                "::" , stringify ! ( RegisterInterface ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const IUnityInterfaces ) ) . GetInterfaceSplit
                as * const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( IUnityInterfaces ) ,
                "::" , stringify ! ( GetInterfaceSplit ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const IUnityInterfaces ) ) .
                RegisterInterfaceSplit as * const _ as usize } , 24usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( IUnityInterfaces ) ,
                "::" , stringify ! ( RegisterInterfaceSplit ) ));
}

impl Clone for IUnityInterfaces {
    fn clone(&self) -> Self { *self }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderSurfaceBase {
    _unused: [u8; 0],
}

pub type UnityRenderBuffer = *mut RenderSurfaceBase;
pub type UnityTextureID = ::std::os::raw::c_uint;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UnityGfxRenderer {
    kUnityGfxRendererD3D11 = 2,
    kUnityGfxRendererGCM = 3,
    kUnityGfxRendererNull = 4,
    kUnityGfxRendererOpenGLES20 = 8,
    kUnityGfxRendererOpenGLES30 = 11,
    kUnityGfxRendererGXM = 12,
    kUnityGfxRendererPS4 = 13,
    kUnityGfxRendererXboxOne = 14,
    kUnityGfxRendererMetal = 16,
    kUnityGfxRendererOpenGLCore = 17,
    kUnityGfxRendererD3D12 = 18,
    kUnityGfxRendererVulkan = 21,
    kUnityGfxRendererNvn = 22,
    kUnityGfxRendererXboxOneD3D12 = 23,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UnityGfxDeviceEventType {
    kUnityGfxDeviceEventInitialize = 0,
    kUnityGfxDeviceEventShutdown = 1,
    kUnityGfxDeviceEventBeforeReset = 2,
    kUnityGfxDeviceEventAfterReset = 3,
}

pub type IUnityGraphicsDeviceEventCallback =
    ::std::option::Option<unsafe extern "C" fn(eventType:
                                                   UnityGfxDeviceEventType)>;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct IUnityGraphics {
    pub GetRenderer: ::std::option::Option<unsafe extern "C" fn()
                                               -> UnityGfxRenderer>,
    pub RegisterDeviceEventCallback: ::std::option::Option<unsafe extern "C" fn(callback:
                                                                                    IUnityGraphicsDeviceEventCallback)>,
    pub UnregisterDeviceEventCallback: ::std::option::Option<unsafe extern "C" fn(callback:
                                                                                      IUnityGraphicsDeviceEventCallback)>,
    pub ReserveEventIDRange: ::std::option::Option<unsafe extern "C" fn(count:
                                                                            ::std::os::raw::c_int)
                                                       ->
                                                           ::std::os::raw::c_int>,
}

#[test]
fn bindgen_test_layout_IUnityGraphics() {
    assert_eq!(::std::mem::size_of::<IUnityGraphics>() , 32usize , concat ! (
               "Size of: " , stringify ! ( IUnityGraphics ) ));
    assert_eq! (::std::mem::align_of::<IUnityGraphics>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( IUnityGraphics ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const IUnityGraphics ) ) . GetRenderer as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( IUnityGraphics ) , "::"
                , stringify ! ( GetRenderer ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const IUnityGraphics ) ) .
                RegisterDeviceEventCallback as * const _ as usize } , 8usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( IUnityGraphics ) , "::"
                , stringify ! ( RegisterDeviceEventCallback ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const IUnityGraphics ) ) .
                UnregisterDeviceEventCallback as * const _ as usize } ,
                16usize , concat ! (
                "Alignment of field: " , stringify ! ( IUnityGraphics ) , "::"
                , stringify ! ( UnregisterDeviceEventCallback ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const IUnityGraphics ) ) . ReserveEventIDRange
                as * const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( IUnityGraphics ) , "::"
                , stringify ! ( ReserveEventIDRange ) ));
}
impl Clone for IUnityGraphics {
    fn clone(&self) -> Self { *self }
}

//TODO: manually copy required guids into separate mod
// extern "C" {
//     #[link_name = "IUnityGraphics_GUID"]
//     pub static IUnityGraphics_GUID: UnityInterfaceGUID;
// }

pub type UnityRenderingEvent =
    ::std::option::Option<unsafe extern "C" fn(eventId:
                                                   ::std::os::raw::c_int)>;
pub type UnityRenderingEventAndData =
    ::std::option::Option<unsafe extern "C" fn(eventId: ::std::os::raw::c_int,
                                               data:
                                                   *mut ::std::os::raw::c_void)>;
