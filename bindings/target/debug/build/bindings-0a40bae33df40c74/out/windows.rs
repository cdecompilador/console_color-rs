#[allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Windows {
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Win32 {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod SystemServices {
            #[repr(transparent)]
            #[derive(
                :: std :: default :: Default,
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct BOOL(pub i32);
            unsafe impl ::windows::Abi for BOOL {
                type Abi = Self;
            }
            impl BOOL {
                #[inline]
                pub fn as_bool(self) -> bool {
                    !(self.0 == 0)
                }
                #[inline]
                pub fn ok(self) -> ::windows::Result<()> {
                    if self.as_bool() {
                        Ok(())
                    } else {
                        Err(::windows::HRESULT::from_thread().into())
                    }
                }
                #[inline]
                pub fn unwrap(self) {
                    self.ok().unwrap();
                }
                #[inline]
                pub fn expect(self, msg: &str) {
                    self.ok().expect(msg);
                }
            }
            impl ::std::convert::From<BOOL> for bool {
                fn from(value: BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<&BOOL> for bool {
                fn from(value: &BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<bool> for BOOL {
                fn from(value: bool) -> Self {
                    if value {
                        BOOL(1)
                    } else {
                        BOOL(0)
                    }
                }
            }
            impl ::std::convert::From<&bool> for BOOL {
                fn from(value: &bool) -> Self {
                    (*value).into()
                }
            }
            impl ::std::cmp::PartialEq<bool> for BOOL {
                fn eq(&self, other: &bool) -> bool {
                    self.as_bool() == *other
                }
            }
            impl ::std::cmp::PartialEq<BOOL> for bool {
                fn eq(&self, other: &BOOL) -> bool {
                    *self == other.as_bool()
                }
            }
            impl std::ops::Not for BOOL {
                type Output = Self;
                fn not(self) -> Self::Output {
                    if self.as_bool() {
                        BOOL(0)
                    } else {
                        BOOL(1)
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, BOOL> for bool {
                fn into_param(self) -> ::windows::Param<'a, BOOL> {
                    ::windows::Param::Owned(self.into())
                }
            }
            #[repr(transparent)]
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            pub struct HANDLE(pub isize);
            impl HANDLE {}
            impl ::std::default::Default for HANDLE {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl HANDLE {
                pub const NULL: Self = Self(0);
                pub fn is_null(&self) -> bool {
                    self.0 == 0
                }
            }
            impl ::std::fmt::Debug for HANDLE {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("HANDLE")
                        .field("Value", &format_args!("{:?}", self.0))
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for HANDLE {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for HANDLE {}
            unsafe impl ::windows::Abi for HANDLE {
                type Abi = Self;
            }
            impl HANDLE {
                pub const INVALID: Self = Self(-1);
                pub fn is_invalid(&self) -> bool {
                    self.0 == -1
                }
            }
            #[repr(C)]
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            pub struct COORD {
                pub X: i16,
                pub Y: i16,
            }
            impl COORD {}
            impl ::std::default::Default for COORD {
                fn default() -> Self {
                    Self { X: 0, Y: 0 }
                }
            }
            impl ::std::fmt::Debug for COORD {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("COORD")
                        .field("X", &format_args!("{:?}", self.X))
                        .field("Y", &format_args!("{:?}", self.Y))
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for COORD {
                fn eq(&self, other: &Self) -> bool {
                    self.X == other.X && self.Y == other.Y
                }
            }
            impl ::std::cmp::Eq for COORD {}
            unsafe impl ::windows::Abi for COORD {
                type Abi = Self;
            }
            #[repr(C)]
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            pub struct SMALL_RECT {
                pub Left: i16,
                pub Top: i16,
                pub Right: i16,
                pub Bottom: i16,
            }
            impl SMALL_RECT {}
            impl ::std::default::Default for SMALL_RECT {
                fn default() -> Self {
                    Self {
                        Left: 0,
                        Top: 0,
                        Right: 0,
                        Bottom: 0,
                    }
                }
            }
            impl ::std::fmt::Debug for SMALL_RECT {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("SMALL_RECT")
                        .field("Left", &format_args!("{:?}", self.Left))
                        .field("Top", &format_args!("{:?}", self.Top))
                        .field("Right", &format_args!("{:?}", self.Right))
                        .field("Bottom", &format_args!("{:?}", self.Bottom))
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for SMALL_RECT {
                fn eq(&self, other: &Self) -> bool {
                    self.Left == other.Left
                        && self.Top == other.Top
                        && self.Right == other.Right
                        && self.Bottom == other.Bottom
                }
            }
            impl ::std::cmp::Eq for SMALL_RECT {}
            unsafe impl ::windows::Abi for SMALL_RECT {
                type Abi = Self;
            }
            #[repr(C)]
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            pub struct CONSOLE_SCREEN_BUFFER_INFO {
                pub dwSize: COORD,
                pub dwCursorPosition: COORD,
                pub wAttributes: u16,
                pub srWindow: SMALL_RECT,
                pub dwMaximumWindowSize: COORD,
            }
            impl CONSOLE_SCREEN_BUFFER_INFO {}
            impl ::std::default::Default for CONSOLE_SCREEN_BUFFER_INFO {
                fn default() -> Self {
                    Self {
                        dwSize: ::std::default::Default::default(),
                        dwCursorPosition: ::std::default::Default::default(),
                        wAttributes: 0,
                        srWindow: ::std::default::Default::default(),
                        dwMaximumWindowSize: ::std::default::Default::default(),
                    }
                }
            }
            impl ::std::fmt::Debug for CONSOLE_SCREEN_BUFFER_INFO {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("CONSOLE_SCREEN_BUFFER_INFO")
                        .field("dwSize", &format_args!("{:?}", self.dwSize))
                        .field(
                            "dwCursorPosition",
                            &format_args!("{:?}", self.dwCursorPosition),
                        )
                        .field("wAttributes", &format_args!("{:?}", self.wAttributes))
                        .field("srWindow", &format_args!("{:?}", self.srWindow))
                        .field(
                            "dwMaximumWindowSize",
                            &format_args!("{:?}", self.dwMaximumWindowSize),
                        )
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for CONSOLE_SCREEN_BUFFER_INFO {
                fn eq(&self, other: &Self) -> bool {
                    self.dwSize == other.dwSize
                        && self.dwCursorPosition == other.dwCursorPosition
                        && self.wAttributes == other.wAttributes
                        && self.srWindow == other.srWindow
                        && self.dwMaximumWindowSize == other.dwMaximumWindowSize
                }
            }
            impl ::std::cmp::Eq for CONSOLE_SCREEN_BUFFER_INFO {}
            unsafe impl ::windows::Abi for CONSOLE_SCREEN_BUFFER_INFO {
                type Abi = Self;
            }
            pub unsafe fn GetConsoleScreenBufferInfo<'a>(
                hconsoleoutput: impl ::windows::IntoParam<'a, HANDLE>,
                lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO,
            ) -> BOOL {
                #[link(name = "KERNEL32")]
                extern "system" {
                    pub fn GetConsoleScreenBufferInfo(
                        hconsoleoutput: HANDLE,
                        lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO,
                    ) -> BOOL;
                }
                GetConsoleScreenBufferInfo(
                    hconsoleoutput.into_param().abi(),
                    ::std::mem::transmute(lpconsolescreenbufferinfo),
                )
            }
            pub unsafe fn SetConsoleTextAttribute<'a>(
                hconsoleoutput: impl ::windows::IntoParam<'a, HANDLE>,
                wattributes: u16,
            ) -> BOOL {
                #[link(name = "KERNEL32")]
                extern "system" {
                    pub fn SetConsoleTextAttribute(
                        hconsoleoutput: HANDLE,
                        wattributes: u16,
                    ) -> BOOL;
                }
                SetConsoleTextAttribute(
                    hconsoleoutput.into_param().abi(),
                    ::std::mem::transmute(wattributes),
                )
            }
            pub unsafe fn WriteConsoleA<'a>(
                hconsoleoutput: impl ::windows::IntoParam<'a, HANDLE>,
                lpbuffer: *const ::std::ffi::c_void,
                nnumberofcharstowrite: u32,
                lpnumberofcharswritten: *mut u32,
                lpreserved: *mut ::std::ffi::c_void,
            ) -> BOOL {
                #[link(name = "KERNEL32")]
                extern "system" {
                    pub fn WriteConsoleA(
                        hconsoleoutput: HANDLE,
                        lpbuffer: *const ::std::ffi::c_void,
                        nnumberofcharstowrite: u32,
                        lpnumberofcharswritten: *mut u32,
                        lpreserved: *mut ::std::ffi::c_void,
                    ) -> BOOL;
                }
                WriteConsoleA(
                    hconsoleoutput.into_param().abi(),
                    ::std::mem::transmute(lpbuffer),
                    ::std::mem::transmute(nnumberofcharstowrite),
                    ::std::mem::transmute(lpnumberofcharswritten),
                    ::std::mem::transmute(lpreserved),
                )
            }
            #[repr(transparent)]
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            pub struct NonClosableHandle(pub isize);
            impl NonClosableHandle {}
            impl ::std::default::Default for NonClosableHandle {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl NonClosableHandle {
                pub const NULL: Self = Self(0);
                pub fn is_null(&self) -> bool {
                    self.0 == 0
                }
            }
            impl ::std::fmt::Debug for NonClosableHandle {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("NonClosableHandle")
                        .field("Value", &format_args!("{:?}", self.0))
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for NonClosableHandle {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for NonClosableHandle {}
            unsafe impl ::windows::Abi for NonClosableHandle {
                type Abi = Self;
            }
            impl<'a> ::windows::IntoParam<'a, HANDLE> for NonClosableHandle {
                fn into_param(self) -> ::windows::Param<'a, HANDLE> {
                    ::windows::Param::Owned(HANDLE(self.0))
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod WindowsProgramming {
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: marker :: Copy,
                :: std :: clone :: Clone,
                :: std :: default :: Default,
                :: std :: fmt :: Debug,
            )]
            #[repr(transparent)]
            pub struct STD_HANDLE_TYPE(pub u32);
            impl STD_HANDLE_TYPE {
                pub const STD_INPUT_HANDLE: Self = Self(4294967286u32);
                pub const STD_OUTPUT_HANDLE: Self = Self(4294967285u32);
                pub const STD_ERROR_HANDLE: Self = Self(4294967284u32);
            }
            impl ::std::convert::From<u32> for STD_HANDLE_TYPE {
                fn from(value: u32) -> Self {
                    Self(value)
                }
            }
            unsafe impl ::windows::Abi for STD_HANDLE_TYPE {
                type Abi = Self;
            }
            impl ::std::ops::BitOr for STD_HANDLE_TYPE {
                type Output = Self;
                fn bitor(self, rhs: Self) -> Self {
                    Self(self.0 | rhs.0)
                }
            }
            impl ::std::ops::BitAnd for STD_HANDLE_TYPE {
                type Output = Self;
                fn bitand(self, rhs: Self) -> Self {
                    Self(self.0 & rhs.0)
                }
            }
            impl ::std::ops::BitOrAssign for STD_HANDLE_TYPE {
                fn bitor_assign(&mut self, rhs: Self) {
                    self.0.bitor_assign(rhs.0)
                }
            }
            impl ::std::ops::BitAndAssign for STD_HANDLE_TYPE {
                fn bitand_assign(&mut self, rhs: Self) {
                    self.0.bitand_assign(rhs.0)
                }
            }
            pub unsafe fn GetStdHandle(
                nstdhandle: STD_HANDLE_TYPE,
            ) -> super::SystemServices::NonClosableHandle {
                #[link(name = "KERNEL32")]
                extern "system" {
                    pub fn GetStdHandle(
                        nstdhandle: STD_HANDLE_TYPE,
                    ) -> super::SystemServices::NonClosableHandle;
                }
                GetStdHandle(::std::mem::transmute(nstdhandle))
            }
        }
    }
}
