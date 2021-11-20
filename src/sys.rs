#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]

pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0]
}

impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
}

impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }

    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }

    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}

pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 30;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const __TIMESIZE: u32 = 64;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const LIBAVUTIL_VERSION_MAJOR: u32 = 56;
pub const LIBAVUTIL_VERSION_MINOR: u32 = 31;
pub const LIBAVUTIL_VERSION_MICRO: u32 = 100;
pub const AV_DICT_MATCH_CASE: u32 = 1;
pub const AV_DICT_IGNORE_SUFFIX: u32 = 2;
pub const AV_DICT_DONT_STRDUP_KEY: u32 = 4;
pub const AV_DICT_DONT_STRDUP_VAL: u32 = 8;
pub const AV_DICT_DONT_OVERWRITE: u32 = 16;
pub const AV_DICT_APPEND: u32 = 32;
pub const AV_DICT_MULTIKEY: u32 = 64;
pub const _ERRNO_H: u32 = 1;
pub const _BITS_ERRNO_H: u32 = 1;
pub const EPERM: u32 = 1;
pub const ENOENT: u32 = 2;
pub const ESRCH: u32 = 3;
pub const EINTR: u32 = 4;
pub const EIO: u32 = 5;
pub const ENXIO: u32 = 6;
pub const E2BIG: u32 = 7;
pub const ENOEXEC: u32 = 8;
pub const EBADF: u32 = 9;
pub const ECHILD: u32 = 10;
pub const EAGAIN: u32 = 11;
pub const ENOMEM: u32 = 12;
pub const EACCES: u32 = 13;
pub const EFAULT: u32 = 14;
pub const ENOTBLK: u32 = 15;
pub const EBUSY: u32 = 16;
pub const EEXIST: u32 = 17;
pub const EXDEV: u32 = 18;
pub const ENODEV: u32 = 19;
pub const ENOTDIR: u32 = 20;
pub const EISDIR: u32 = 21;
pub const EINVAL: u32 = 22;
pub const ENFILE: u32 = 23;
pub const EMFILE: u32 = 24;
pub const ENOTTY: u32 = 25;
pub const ETXTBSY: u32 = 26;
pub const EFBIG: u32 = 27;
pub const ENOSPC: u32 = 28;
pub const ESPIPE: u32 = 29;
pub const EROFS: u32 = 30;
pub const EMLINK: u32 = 31;
pub const EPIPE: u32 = 32;
pub const EDOM: u32 = 33;
pub const ERANGE: u32 = 34;
pub const EDEADLK: u32 = 35;
pub const ENAMETOOLONG: u32 = 36;
pub const ENOLCK: u32 = 37;
pub const ENOSYS: u32 = 38;
pub const ENOTEMPTY: u32 = 39;
pub const ELOOP: u32 = 40;
pub const EWOULDBLOCK: u32 = 11;
pub const ENOMSG: u32 = 42;
pub const EIDRM: u32 = 43;
pub const ECHRNG: u32 = 44;
pub const EL2NSYNC: u32 = 45;
pub const EL3HLT: u32 = 46;
pub const EL3RST: u32 = 47;
pub const ELNRNG: u32 = 48;
pub const EUNATCH: u32 = 49;
pub const ENOCSI: u32 = 50;
pub const EL2HLT: u32 = 51;
pub const EBADE: u32 = 52;
pub const EBADR: u32 = 53;
pub const EXFULL: u32 = 54;
pub const ENOANO: u32 = 55;
pub const EBADRQC: u32 = 56;
pub const EBADSLT: u32 = 57;
pub const EDEADLOCK: u32 = 35;
pub const EBFONT: u32 = 59;
pub const ENOSTR: u32 = 60;
pub const ENODATA: u32 = 61;
pub const ETIME: u32 = 62;
pub const ENOSR: u32 = 63;
pub const ENONET: u32 = 64;
pub const ENOPKG: u32 = 65;
pub const EREMOTE: u32 = 66;
pub const ENOLINK: u32 = 67;
pub const EADV: u32 = 68;
pub const ESRMNT: u32 = 69;
pub const ECOMM: u32 = 70;
pub const EPROTO: u32 = 71;
pub const EMULTIHOP: u32 = 72;
pub const EDOTDOT: u32 = 73;
pub const EBADMSG: u32 = 74;
pub const EOVERFLOW: u32 = 75;
pub const ENOTUNIQ: u32 = 76;
pub const EBADFD: u32 = 77;
pub const EREMCHG: u32 = 78;
pub const ELIBACC: u32 = 79;
pub const ELIBBAD: u32 = 80;
pub const ELIBSCN: u32 = 81;
pub const ELIBMAX: u32 = 82;
pub const ELIBEXEC: u32 = 83;
pub const EILSEQ: u32 = 84;
pub const ERESTART: u32 = 85;
pub const ESTRPIPE: u32 = 86;
pub const EUSERS: u32 = 87;
pub const ENOTSOCK: u32 = 88;
pub const EDESTADDRREQ: u32 = 89;
pub const EMSGSIZE: u32 = 90;
pub const EPROTOTYPE: u32 = 91;
pub const ENOPROTOOPT: u32 = 92;
pub const EPROTONOSUPPORT: u32 = 93;
pub const ESOCKTNOSUPPORT: u32 = 94;
pub const EOPNOTSUPP: u32 = 95;
pub const EPFNOSUPPORT: u32 = 96;
pub const EAFNOSUPPORT: u32 = 97;
pub const EADDRINUSE: u32 = 98;
pub const EADDRNOTAVAIL: u32 = 99;
pub const ENETDOWN: u32 = 100;
pub const ENETUNREACH: u32 = 101;
pub const ENETRESET: u32 = 102;
pub const ECONNABORTED: u32 = 103;
pub const ECONNRESET: u32 = 104;
pub const ENOBUFS: u32 = 105;
pub const EISCONN: u32 = 106;
pub const ENOTCONN: u32 = 107;
pub const ESHUTDOWN: u32 = 108;
pub const ETOOMANYREFS: u32 = 109;
pub const ETIMEDOUT: u32 = 110;
pub const ECONNREFUSED: u32 = 111;
pub const EHOSTDOWN: u32 = 112;
pub const EHOSTUNREACH: u32 = 113;
pub const EALREADY: u32 = 114;
pub const EINPROGRESS: u32 = 115;
pub const ESTALE: u32 = 116;
pub const EUCLEAN: u32 = 117;
pub const ENOTNAM: u32 = 118;
pub const ENAVAIL: u32 = 119;
pub const EISNAM: u32 = 120;
pub const EREMOTEIO: u32 = 121;
pub const EDQUOT: u32 = 122;
pub const ENOMEDIUM: u32 = 123;
pub const EMEDIUMTYPE: u32 = 124;
pub const ECANCELED: u32 = 125;
pub const ENOKEY: u32 = 126;
pub const EKEYEXPIRED: u32 = 127;
pub const EKEYREVOKED: u32 = 128;
pub const EKEYREJECTED: u32 = 129;
pub const EOWNERDEAD: u32 = 130;
pub const ENOTRECOVERABLE: u32 = 131;
pub const ERFKILL: u32 = 132;
pub const EHWPOISON: u32 = 133;
pub const ENOTSUP: u32 = 95;
pub const _INTTYPES_H: u32 = 1;
pub const ____gwchar_t_defined: u32 = 1;
pub const __PRI64_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const __PRIPTR_PREFIX: &'static [u8; 2usize] = b"l\0";
pub const PRId8: &'static [u8; 2usize] = b"d\0";
pub const PRId16: &'static [u8; 2usize] = b"d\0";
pub const PRId32: &'static [u8; 2usize] = b"d\0";
pub const PRId64: &'static [u8; 3usize] = b"ld\0";
pub const PRIdLEAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST16: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const PRIdLEAST64: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST8: &'static [u8; 2usize] = b"d\0";
pub const PRIdFAST16: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST32: &'static [u8; 3usize] = b"ld\0";
pub const PRIdFAST64: &'static [u8; 3usize] = b"ld\0";
pub const PRIi8: &'static [u8; 2usize] = b"i\0";
pub const PRIi16: &'static [u8; 2usize] = b"i\0";
pub const PRIi32: &'static [u8; 2usize] = b"i\0";
pub const PRIi64: &'static [u8; 3usize] = b"li\0";
pub const PRIiLEAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST16: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const PRIiLEAST64: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST8: &'static [u8; 2usize] = b"i\0";
pub const PRIiFAST16: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST32: &'static [u8; 3usize] = b"li\0";
pub const PRIiFAST64: &'static [u8; 3usize] = b"li\0";
pub const PRIo8: &'static [u8; 2usize] = b"o\0";
pub const PRIo16: &'static [u8; 2usize] = b"o\0";
pub const PRIo32: &'static [u8; 2usize] = b"o\0";
pub const PRIo64: &'static [u8; 3usize] = b"lo\0";
pub const PRIoLEAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST16: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const PRIoLEAST64: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST8: &'static [u8; 2usize] = b"o\0";
pub const PRIoFAST16: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST32: &'static [u8; 3usize] = b"lo\0";
pub const PRIoFAST64: &'static [u8; 3usize] = b"lo\0";
pub const PRIu8: &'static [u8; 2usize] = b"u\0";
pub const PRIu16: &'static [u8; 2usize] = b"u\0";
pub const PRIu32: &'static [u8; 2usize] = b"u\0";
pub const PRIu64: &'static [u8; 3usize] = b"lu\0";
pub const PRIuLEAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST16: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const PRIuLEAST64: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST8: &'static [u8; 2usize] = b"u\0";
pub const PRIuFAST16: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST32: &'static [u8; 3usize] = b"lu\0";
pub const PRIuFAST64: &'static [u8; 3usize] = b"lu\0";
pub const PRIx8: &'static [u8; 2usize] = b"x\0";
pub const PRIx16: &'static [u8; 2usize] = b"x\0";
pub const PRIx32: &'static [u8; 2usize] = b"x\0";
pub const PRIx64: &'static [u8; 3usize] = b"lx\0";
pub const PRIxLEAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST16: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const PRIxLEAST64: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST8: &'static [u8; 2usize] = b"x\0";
pub const PRIxFAST16: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST32: &'static [u8; 3usize] = b"lx\0";
pub const PRIxFAST64: &'static [u8; 3usize] = b"lx\0";
pub const PRIX8: &'static [u8; 2usize] = b"X\0";
pub const PRIX16: &'static [u8; 2usize] = b"X\0";
pub const PRIX32: &'static [u8; 2usize] = b"X\0";
pub const PRIX64: &'static [u8; 3usize] = b"lX\0";
pub const PRIXLEAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST16: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST32: &'static [u8; 2usize] = b"X\0";
pub const PRIXLEAST64: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST8: &'static [u8; 2usize] = b"X\0";
pub const PRIXFAST16: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST32: &'static [u8; 3usize] = b"lX\0";
pub const PRIXFAST64: &'static [u8; 3usize] = b"lX\0";
pub const PRIdMAX: &'static [u8; 3usize] = b"ld\0";
pub const PRIiMAX: &'static [u8; 3usize] = b"li\0";
pub const PRIoMAX: &'static [u8; 3usize] = b"lo\0";
pub const PRIuMAX: &'static [u8; 3usize] = b"lu\0";
pub const PRIxMAX: &'static [u8; 3usize] = b"lx\0";
pub const PRIXMAX: &'static [u8; 3usize] = b"lX\0";
pub const PRIdPTR: &'static [u8; 3usize] = b"ld\0";
pub const PRIiPTR: &'static [u8; 3usize] = b"li\0";
pub const PRIoPTR: &'static [u8; 3usize] = b"lo\0";
pub const PRIuPTR: &'static [u8; 3usize] = b"lu\0";
pub const PRIxPTR: &'static [u8; 3usize] = b"lx\0";
pub const PRIXPTR: &'static [u8; 3usize] = b"lX\0";
pub const SCNd8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNd16: &'static [u8; 3usize] = b"hd\0";
pub const SCNd32: &'static [u8; 2usize] = b"d\0";
pub const SCNd64: &'static [u8; 3usize] = b"ld\0";
pub const SCNdLEAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdLEAST16: &'static [u8; 3usize] = b"hd\0";
pub const SCNdLEAST32: &'static [u8; 2usize] = b"d\0";
pub const SCNdLEAST64: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST8: &'static [u8; 4usize] = b"hhd\0";
pub const SCNdFAST16: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST32: &'static [u8; 3usize] = b"ld\0";
pub const SCNdFAST64: &'static [u8; 3usize] = b"ld\0";
pub const SCNi8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNi16: &'static [u8; 3usize] = b"hi\0";
pub const SCNi32: &'static [u8; 2usize] = b"i\0";
pub const SCNi64: &'static [u8; 3usize] = b"li\0";
pub const SCNiLEAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiLEAST16: &'static [u8; 3usize] = b"hi\0";
pub const SCNiLEAST32: &'static [u8; 2usize] = b"i\0";
pub const SCNiLEAST64: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST8: &'static [u8; 4usize] = b"hhi\0";
pub const SCNiFAST16: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST32: &'static [u8; 3usize] = b"li\0";
pub const SCNiFAST64: &'static [u8; 3usize] = b"li\0";
pub const SCNu8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNu16: &'static [u8; 3usize] = b"hu\0";
pub const SCNu32: &'static [u8; 2usize] = b"u\0";
pub const SCNu64: &'static [u8; 3usize] = b"lu\0";
pub const SCNuLEAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuLEAST16: &'static [u8; 3usize] = b"hu\0";
pub const SCNuLEAST32: &'static [u8; 2usize] = b"u\0";
pub const SCNuLEAST64: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST8: &'static [u8; 4usize] = b"hhu\0";
pub const SCNuFAST16: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST32: &'static [u8; 3usize] = b"lu\0";
pub const SCNuFAST64: &'static [u8; 3usize] = b"lu\0";
pub const SCNo8: &'static [u8; 4usize] = b"hho\0";
pub const SCNo16: &'static [u8; 3usize] = b"ho\0";
pub const SCNo32: &'static [u8; 2usize] = b"o\0";
pub const SCNo64: &'static [u8; 3usize] = b"lo\0";
pub const SCNoLEAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoLEAST16: &'static [u8; 3usize] = b"ho\0";
pub const SCNoLEAST32: &'static [u8; 2usize] = b"o\0";
pub const SCNoLEAST64: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST8: &'static [u8; 4usize] = b"hho\0";
pub const SCNoFAST16: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST32: &'static [u8; 3usize] = b"lo\0";
pub const SCNoFAST64: &'static [u8; 3usize] = b"lo\0";
pub const SCNx8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNx16: &'static [u8; 3usize] = b"hx\0";
pub const SCNx32: &'static [u8; 2usize] = b"x\0";
pub const SCNx64: &'static [u8; 3usize] = b"lx\0";
pub const SCNxLEAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxLEAST16: &'static [u8; 3usize] = b"hx\0";
pub const SCNxLEAST32: &'static [u8; 2usize] = b"x\0";
pub const SCNxLEAST64: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST8: &'static [u8; 4usize] = b"hhx\0";
pub const SCNxFAST16: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST32: &'static [u8; 3usize] = b"lx\0";
pub const SCNxFAST64: &'static [u8; 3usize] = b"lx\0";
pub const SCNdMAX: &'static [u8; 3usize] = b"ld\0";
pub const SCNiMAX: &'static [u8; 3usize] = b"li\0";
pub const SCNoMAX: &'static [u8; 3usize] = b"lo\0";
pub const SCNuMAX: &'static [u8; 3usize] = b"lu\0";
pub const SCNxMAX: &'static [u8; 3usize] = b"lx\0";
pub const SCNdPTR: &'static [u8; 3usize] = b"ld\0";
pub const SCNiPTR: &'static [u8; 3usize] = b"li\0";
pub const SCNoPTR: &'static [u8; 3usize] = b"lo\0";
pub const SCNuPTR: &'static [u8; 3usize] = b"lu\0";
pub const SCNxPTR: &'static [u8; 3usize] = b"lx\0";
pub const _LIBC_LIMITS_H_: u32 = 1;
pub const MB_LEN_MAX: u32 = 16;
pub const _BITS_POSIX1_LIM_H: u32 = 1;
pub const _POSIX_AIO_LISTIO_MAX: u32 = 2;
pub const _POSIX_AIO_MAX: u32 = 1;
pub const _POSIX_ARG_MAX: u32 = 4096;
pub const _POSIX_CHILD_MAX: u32 = 25;
pub const _POSIX_DELAYTIMER_MAX: u32 = 32;
pub const _POSIX_HOST_NAME_MAX: u32 = 255;
pub const _POSIX_LINK_MAX: u32 = 8;
pub const _POSIX_LOGIN_NAME_MAX: u32 = 9;
pub const _POSIX_MAX_CANON: u32 = 255;
pub const _POSIX_MAX_INPUT: u32 = 255;
pub const _POSIX_MQ_OPEN_MAX: u32 = 8;
pub const _POSIX_MQ_PRIO_MAX: u32 = 32;
pub const _POSIX_NAME_MAX: u32 = 14;
pub const _POSIX_NGROUPS_MAX: u32 = 8;
pub const _POSIX_OPEN_MAX: u32 = 20;
pub const _POSIX_PATH_MAX: u32 = 256;
pub const _POSIX_PIPE_BUF: u32 = 512;
pub const _POSIX_RE_DUP_MAX: u32 = 255;
pub const _POSIX_RTSIG_MAX: u32 = 8;
pub const _POSIX_SEM_NSEMS_MAX: u32 = 256;
pub const _POSIX_SEM_VALUE_MAX: u32 = 32767;
pub const _POSIX_SIGQUEUE_MAX: u32 = 32;
pub const _POSIX_SSIZE_MAX: u32 = 32767;
pub const _POSIX_STREAM_MAX: u32 = 8;
pub const _POSIX_SYMLINK_MAX: u32 = 255;
pub const _POSIX_SYMLOOP_MAX: u32 = 8;
pub const _POSIX_TIMER_MAX: u32 = 32;
pub const _POSIX_TTY_NAME_MAX: u32 = 9;
pub const _POSIX_TZNAME_MAX: u32 = 6;
pub const _POSIX_CLOCKRES_MIN: u32 = 20000000;
pub const NR_OPEN: u32 = 1024;
pub const NGROUPS_MAX: u32 = 65536;
pub const ARG_MAX: u32 = 131072;
pub const LINK_MAX: u32 = 127;
pub const MAX_CANON: u32 = 255;
pub const MAX_INPUT: u32 = 255;
pub const NAME_MAX: u32 = 255;
pub const PATH_MAX: u32 = 4096;
pub const PIPE_BUF: u32 = 4096;
pub const XATTR_NAME_MAX: u32 = 255;
pub const XATTR_SIZE_MAX: u32 = 65536;
pub const XATTR_LIST_MAX: u32 = 65536;
pub const RTSIG_MAX: u32 = 32;
pub const _POSIX_THREAD_KEYS_MAX: u32 = 128;
pub const PTHREAD_KEYS_MAX: u32 = 1024;
pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const _POSIX_THREAD_THREADS_MAX: u32 = 64;
pub const AIO_PRIO_DELTA_MAX: u32 = 20;
pub const PTHREAD_STACK_MIN: u32 = 16384;
pub const DELAYTIMER_MAX: u32 = 2147483647;
pub const TTY_NAME_MAX: u32 = 32;
pub const LOGIN_NAME_MAX: u32 = 256;
pub const HOST_NAME_MAX: u32 = 64;
pub const MQ_PRIO_MAX: u32 = 32768;
pub const SEM_VALUE_MAX: u32 = 2147483647;
pub const _BITS_POSIX2_LIM_H: u32 = 1;
pub const _POSIX2_BC_BASE_MAX: u32 = 99;
pub const _POSIX2_BC_DIM_MAX: u32 = 2048;
pub const _POSIX2_BC_SCALE_MAX: u32 = 99;
pub const _POSIX2_BC_STRING_MAX: u32 = 1000;
pub const _POSIX2_COLL_WEIGHTS_MAX: u32 = 2;
pub const _POSIX2_EXPR_NEST_MAX: u32 = 32;
pub const _POSIX2_LINE_MAX: u32 = 2048;
pub const _POSIX2_RE_DUP_MAX: u32 = 255;
pub const _POSIX2_CHARCLASS_NAME_MAX: u32 = 14;
pub const BC_BASE_MAX: u32 = 99;
pub const BC_DIM_MAX: u32 = 2048;
pub const BC_SCALE_MAX: u32 = 99;
pub const BC_STRING_MAX: u32 = 1000;
pub const COLL_WEIGHTS_MAX: u32 = 255;
pub const EXPR_NEST_MAX: u32 = 32;
pub const LINE_MAX: u32 = 2048;
pub const CHARCLASS_NAME_MAX: u32 = 2048;
pub const RE_DUP_MAX: u32 = 32767;
pub const _MATH_H: u32 = 1;
pub const _BITS_LIBM_SIMD_DECL_STUBS_H: u32 = 1;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const __FP_LOGB0_IS_MIN: u32 = 1;
pub const __FP_LOGBNAN_IS_MIN: u32 = 1;
pub const FP_ILOGB0: i32 = -2147483648;
pub const FP_ILOGBNAN: i32 = -2147483648;
pub const __MATH_DECLARING_DOUBLE: u32 = 1;
pub const __MATH_DECLARING_FLOATN: u32 = 0;
pub const __MATH_DECLARE_LDOUBLE: u32 = 1;
pub const MATH_ERRNO: u32 = 1;
pub const MATH_ERREXCEPT: u32 = 2;
pub const math_errhandling: u32 = 3;
pub const M_E: f64 = 2.718281828459045;
pub const M_LOG2E: f64 = 1.4426950408889634;
pub const M_LOG10E: f64 = 0.4342944819032518;
pub const M_LN2: f64 = 0.6931471805599453;
pub const M_LN10: f64 = 2.302585092994046;
pub const M_PI: f64 = 3.141592653589793;
pub const M_PI_2: f64 = 1.5707963267948966;
pub const M_PI_4: f64 = 0.7853981633974483;
pub const M_1_PI: f64 = 0.3183098861837907;
pub const M_2_PI: f64 = 0.6366197723675814;
pub const M_2_SQRTPI: f64 = 1.1283791670955126;
pub const M_SQRT2: f64 = 1.4142135623730951;
pub const M_SQRT1_2: f64 = 0.7071067811865476;
pub const _STDIO_H: u32 = 1;
pub const __GNUC_VA_LIST: u32 = 1;
pub const _____fpos_t_defined: u32 = 1;
pub const ____mbstate_t_defined: u32 = 1;
pub const _____fpos64_t_defined: u32 = 1;
pub const ____FILE_defined: u32 = 1;
pub const __FILE_defined: u32 = 1;
pub const __struct_FILE_defined: u32 = 1;
pub const _IO_EOF_SEEN: u32 = 16;
pub const _IO_ERR_SEEN: u32 = 32;
pub const _IO_USER_LOCK: u32 = 32768;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 8192;
pub const EOF: i32 = -1;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const P_tmpdir: &'static [u8; 5usize] = b"/tmp\0";
pub const _BITS_STDIO_LIM_H: u32 = 1;
pub const L_tmpnam: u32 = 20;
pub const TMP_MAX: u32 = 238328;
pub const FILENAME_MAX: u32 = 4096;
pub const L_ctermid: u32 = 9;
pub const FOPEN_MAX: u32 = 16;
pub const _STDLIB_H: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WSTOPPED: u32 = 2;
pub const WEXITED: u32 = 4;
pub const WCONTINUED: u32 = 8;
pub const WNOWAIT: u32 = 16777216;
pub const __WNOTHREAD: u32 = 536870912;
pub const __WALL: u32 = 1073741824;
pub const __WCLONE: u32 = 2147483648;
pub const __W_CONTINUED: u32 = 65535;
pub const __WCOREFLAG: u32 = 128;
pub const __ldiv_t_defined: u32 = 1;
pub const __lldiv_t_defined: u32 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const _SYS_TYPES_H: u32 = 1;
pub const __clock_t_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __FD_ZERO_STOS: &'static [u8; 6usize] = b"stosq\0";
pub const __sigset_t_defined: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const _STRUCT_TIMESPEC: u32 = 1;
pub const FD_SETSIZE: u32 = 1024;
pub const _BITS_PTHREADTYPES_COMMON_H: u32 = 1;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u32 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u32 = 40;
pub const __SIZEOF_PTHREAD_ATTR_T: u32 = 56;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u32 = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: u32 = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_COND_T: u32 = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u32 = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u32 = 4;
pub const __PTHREAD_MUTEX_LOCK_ELISION: u32 = 1;
pub const __PTHREAD_MUTEX_NUSERS_AFTER_KIND: u32 = 0;
pub const __PTHREAD_MUTEX_USE_UNION: u32 = 0;
pub const __PTHREAD_RWLOCK_INT_FLAGS_SHARED: u32 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u32 = 1;
pub const __have_pthread_attr_t: u32 = 1;
pub const _ALLOCA_H: u32 = 1;
pub const _STRING_H: u32 = 1;
pub const _BITS_TYPES_LOCALE_T_H: u32 = 1;
pub const _BITS_TYPES___LOCALE_T_H: u32 = 1;
pub const _STRINGS_H: u32 = 1;
pub const AV_HAVE_BIGENDIAN: u32 = 0;
pub const AV_HAVE_FAST_UNALIGNED: u32 = 1;
pub const AVERROR_EXPERIMENTAL: i32 = -733130664;
pub const AVERROR_INPUT_CHANGED: i32 = -1668179713;
pub const AVERROR_OUTPUT_CHANGED: i32 = -1668179714;
pub const AV_ERROR_MAX_STRING_SIZE: u32 = 64;
pub const FF_LAMBDA_SHIFT: u32 = 7;
pub const FF_LAMBDA_SCALE: u32 = 128;
pub const FF_QP2LAMBDA: u32 = 118;
pub const FF_LAMBDA_MAX: u32 = 32767;
pub const FF_QUALITY_SCALE: u32 = 128;
pub const AV_TIME_BASE: u32 = 1000000;
pub const M_LOG2_10: f64 = 3.321928094887362;
pub const M_PHI: f64 = 1.618033988749895;
pub const AV_LOG_QUIET: i32 = -8;
pub const AV_LOG_PANIC: u32 = 0;
pub const AV_LOG_FATAL: u32 = 8;
pub const AV_LOG_ERROR: u32 = 16;
pub const AV_LOG_WARNING: u32 = 24;
pub const AV_LOG_INFO: u32 = 32;
pub const AV_LOG_VERBOSE: u32 = 40;
pub const AV_LOG_DEBUG: u32 = 48;
pub const AV_LOG_TRACE: u32 = 56;
pub const AV_LOG_MAX_OFFSET: u32 = 64;
pub const AV_LOG_SKIP_REPEATED: u32 = 1;
pub const AV_LOG_PRINT_LEVEL: u32 = 2;
pub const AVPALETTE_SIZE: u32 = 1024;
pub const AVPALETTE_COUNT: u32 = 256;
pub const AV_FOURCC_MAX_STRING_SIZE: u32 = 32;
pub const AV_BUFFER_FLAG_READONLY: u32 = 1;
pub const AV_NUM_DATA_POINTERS: u32 = 8;
pub const AV_FRAME_FLAG_CORRUPT: u32 = 1;
pub const AV_FRAME_FLAG_DISCARD: u32 = 4;
pub const FF_DECODE_ERROR_INVALID_BITSTREAM: u32 = 1;
pub const FF_DECODE_ERROR_MISSING_REFERENCE: u32 = 2;
pub const FF_DECODE_ERROR_CONCEALMENT_ACTIVE: u32 = 4;
pub const FF_DECODE_ERROR_DECODE_SLICES: u32 = 8;
pub const AV_PIX_FMT_FLAG_BE: u32 = 1;
pub const AV_PIX_FMT_FLAG_PAL: u32 = 2;
pub const AV_PIX_FMT_FLAG_BITSTREAM: u32 = 4;
pub const AV_PIX_FMT_FLAG_HWACCEL: u32 = 8;
pub const AV_PIX_FMT_FLAG_PLANAR: u32 = 16;
pub const AV_PIX_FMT_FLAG_RGB: u32 = 32;
pub const AV_PIX_FMT_FLAG_PSEUDOPAL: u32 = 64;
pub const AV_PIX_FMT_FLAG_ALPHA: u32 = 128;
pub const AV_PIX_FMT_FLAG_BAYER: u32 = 256;
pub const AV_PIX_FMT_FLAG_FLOAT: u32 = 512;
pub const FF_LOSS_RESOLUTION: u32 = 1;
pub const FF_LOSS_DEPTH: u32 = 2;
pub const FF_LOSS_COLORSPACE: u32 = 4;
pub const FF_LOSS_ALPHA: u32 = 8;
pub const FF_LOSS_COLORQUANT: u32 = 16;
pub const FF_LOSS_CHROMA: u32 = 32;
pub const AV_OPT_FLAG_ENCODING_PARAM: u32 = 1;
pub const AV_OPT_FLAG_DECODING_PARAM: u32 = 2;
pub const AV_OPT_FLAG_AUDIO_PARAM: u32 = 8;
pub const AV_OPT_FLAG_VIDEO_PARAM: u32 = 16;
pub const AV_OPT_FLAG_SUBTITLE_PARAM: u32 = 32;
pub const AV_OPT_FLAG_EXPORT: u32 = 64;
pub const AV_OPT_FLAG_READONLY: u32 = 128;
pub const AV_OPT_FLAG_BSF_PARAM: u32 = 256;
pub const AV_OPT_FLAG_FILTERING_PARAM: u32 = 65536;
pub const AV_OPT_FLAG_DEPRECATED: u32 = 131072;
pub const AV_OPT_SEARCH_CHILDREN: u32 = 1;
pub const AV_OPT_SEARCH_FAKE_OBJ: u32 = 2;
pub const AV_OPT_ALLOW_NULL: u32 = 4;
pub const AV_OPT_MULTI_COMPONENT_RANGE: u32 = 4096;
pub const AV_OPT_SERIALIZE_SKIP_DEFAULTS: u32 = 1;
pub const AV_OPT_SERIALIZE_OPT_FLAGS_EXACT: u32 = 2;
pub const AV_TIMECODE_STR_SIZE: u32 = 23;
pub const AV_ESCAPE_FLAG_WHITESPACE: u32 = 1;
pub const AV_ESCAPE_FLAG_STRICT: u32 = 2;
pub const AV_UTF8_FLAG_ACCEPT_INVALID_BIG_CODES: u32 = 1;
pub const AV_UTF8_FLAG_ACCEPT_NON_CHARACTERS: u32 = 2;
pub const AV_UTF8_FLAG_ACCEPT_SURROGATES: u32 = 4;
pub const AV_UTF8_FLAG_EXCLUDE_XML_INVALID_CONTROL_CODES: u32 = 8;
pub const AV_UTF8_FLAG_ACCEPT_ALL: u32 = 7;
pub const AV_BF_ROUNDS: u32 = 16;
pub const AV_BPRINT_SIZE_AUTOMATIC: u32 = 1;
pub const AV_BPRINT_SIZE_COUNT_ONLY: u32 = 0;
pub const AV_CH_FRONT_LEFT: u32 = 1;
pub const AV_CH_FRONT_RIGHT: u32 = 2;
pub const AV_CH_FRONT_CENTER: u32 = 4;
pub const AV_CH_LOW_FREQUENCY: u32 = 8;
pub const AV_CH_BACK_LEFT: u32 = 16;
pub const AV_CH_BACK_RIGHT: u32 = 32;
pub const AV_CH_FRONT_LEFT_OF_CENTER: u32 = 64;
pub const AV_CH_FRONT_RIGHT_OF_CENTER: u32 = 128;
pub const AV_CH_BACK_CENTER: u32 = 256;
pub const AV_CH_SIDE_LEFT: u32 = 512;
pub const AV_CH_SIDE_RIGHT: u32 = 1024;
pub const AV_CH_TOP_CENTER: u32 = 2048;
pub const AV_CH_TOP_FRONT_LEFT: u32 = 4096;
pub const AV_CH_TOP_FRONT_CENTER: u32 = 8192;
pub const AV_CH_TOP_FRONT_RIGHT: u32 = 16384;
pub const AV_CH_TOP_BACK_LEFT: u32 = 32768;
pub const AV_CH_TOP_BACK_CENTER: u32 = 65536;
pub const AV_CH_TOP_BACK_RIGHT: u32 = 131072;
pub const AV_CH_STEREO_LEFT: u32 = 536870912;
pub const AV_CH_STEREO_RIGHT: u32 = 1073741824;
pub const AV_CH_WIDE_LEFT: u32 = 2147483648;
pub const AV_CH_WIDE_RIGHT: u64 = 4294967296;
pub const AV_CH_SURROUND_DIRECT_LEFT: u64 = 8589934592;
pub const AV_CH_SURROUND_DIRECT_RIGHT: u64 = 17179869184;
pub const AV_CH_LOW_FREQUENCY_2: u64 = 34359738368;
pub const AV_CH_LAYOUT_NATIVE: i64 = -9223372036854775808;
pub const AV_CH_LAYOUT_MONO: u32 = 4;
pub const AV_CH_LAYOUT_STEREO: u32 = 3;
pub const AV_CH_LAYOUT_2POINT1: u32 = 11;
pub const AV_CH_LAYOUT_2_1: u32 = 259;
pub const AV_CH_LAYOUT_SURROUND: u32 = 7;
pub const AV_CH_LAYOUT_3POINT1: u32 = 15;
pub const AV_CH_LAYOUT_4POINT0: u32 = 263;
pub const AV_CH_LAYOUT_4POINT1: u32 = 271;
pub const AV_CH_LAYOUT_2_2: u32 = 1539;
pub const AV_CH_LAYOUT_QUAD: u32 = 51;
pub const AV_CH_LAYOUT_5POINT0: u32 = 1543;
pub const AV_CH_LAYOUT_5POINT1: u32 = 1551;
pub const AV_CH_LAYOUT_5POINT0_BACK: u32 = 55;
pub const AV_CH_LAYOUT_5POINT1_BACK: u32 = 63;
pub const AV_CH_LAYOUT_6POINT0: u32 = 1799;
pub const AV_CH_LAYOUT_6POINT0_FRONT: u32 = 1731;
pub const AV_CH_LAYOUT_HEXAGONAL: u32 = 311;
pub const AV_CH_LAYOUT_6POINT1: u32 = 1807;
pub const AV_CH_LAYOUT_6POINT1_BACK: u32 = 319;
pub const AV_CH_LAYOUT_6POINT1_FRONT: u32 = 1739;
pub const AV_CH_LAYOUT_7POINT0: u32 = 1591;
pub const AV_CH_LAYOUT_7POINT0_FRONT: u32 = 1735;
pub const AV_CH_LAYOUT_7POINT1: u32 = 1599;
pub const AV_CH_LAYOUT_7POINT1_WIDE: u32 = 1743;
pub const AV_CH_LAYOUT_7POINT1_WIDE_BACK: u32 = 255;
pub const AV_CH_LAYOUT_OCTAGONAL: u32 = 1847;
pub const AV_CH_LAYOUT_HEXADECAGONAL: u64 = 6442710839;
pub const AV_CH_LAYOUT_STEREO_DOWNMIX: u32 = 1610612736;
pub const AV_TS_MAX_STRING_SIZE: u32 = 32;
pub const AV_CPU_FLAG_FORCE: u32 = 2147483648;
pub const AV_CPU_FLAG_MMX: u32 = 1;
pub const AV_CPU_FLAG_MMXEXT: u32 = 2;
pub const AV_CPU_FLAG_MMX2: u32 = 2;
pub const AV_CPU_FLAG_3DNOW: u32 = 4;
pub const AV_CPU_FLAG_SSE: u32 = 8;
pub const AV_CPU_FLAG_SSE2: u32 = 16;
pub const AV_CPU_FLAG_SSE2SLOW: u32 = 1073741824;
pub const AV_CPU_FLAG_3DNOWEXT: u32 = 32;
pub const AV_CPU_FLAG_SSE3: u32 = 64;
pub const AV_CPU_FLAG_SSE3SLOW: u32 = 536870912;
pub const AV_CPU_FLAG_SSSE3: u32 = 128;
pub const AV_CPU_FLAG_SSSE3SLOW: u32 = 67108864;
pub const AV_CPU_FLAG_ATOM: u32 = 268435456;
pub const AV_CPU_FLAG_SSE4: u32 = 256;
pub const AV_CPU_FLAG_SSE42: u32 = 512;
pub const AV_CPU_FLAG_AESNI: u32 = 524288;
pub const AV_CPU_FLAG_AVX: u32 = 16384;
pub const AV_CPU_FLAG_AVXSLOW: u32 = 134217728;
pub const AV_CPU_FLAG_XOP: u32 = 1024;
pub const AV_CPU_FLAG_FMA4: u32 = 2048;
pub const AV_CPU_FLAG_CMOV: u32 = 4096;
pub const AV_CPU_FLAG_AVX2: u32 = 32768;
pub const AV_CPU_FLAG_FMA3: u32 = 65536;
pub const AV_CPU_FLAG_BMI1: u32 = 131072;
pub const AV_CPU_FLAG_BMI2: u32 = 262144;
pub const AV_CPU_FLAG_AVX512: u32 = 1048576;
pub const AV_CPU_FLAG_ALTIVEC: u32 = 1;
pub const AV_CPU_FLAG_VSX: u32 = 2;
pub const AV_CPU_FLAG_POWER8: u32 = 4;
pub const AV_CPU_FLAG_ARMV5TE: u32 = 1;
pub const AV_CPU_FLAG_ARMV6: u32 = 2;
pub const AV_CPU_FLAG_ARMV6T2: u32 = 4;
pub const AV_CPU_FLAG_VFP: u32 = 8;
pub const AV_CPU_FLAG_VFPV3: u32 = 16;
pub const AV_CPU_FLAG_NEON: u32 = 32;
pub const AV_CPU_FLAG_ARMV8: u32 = 64;
pub const AV_CPU_FLAG_VFP_VM: u32 = 128;
pub const AV_CPU_FLAG_SETEND: u32 = 65536;
pub const LIBAVCODEC_VERSION_MAJOR: u32 = 58;
pub const LIBAVCODEC_VERSION_MINOR: u32 = 54;
pub const LIBAVCODEC_VERSION_MICRO: u32 = 100;
pub const AV_CODEC_PROP_INTRA_ONLY: u32 = 1;
pub const AV_CODEC_PROP_LOSSY: u32 = 2;
pub const AV_CODEC_PROP_LOSSLESS: u32 = 4;
pub const AV_CODEC_PROP_REORDER: u32 = 8;
pub const AV_CODEC_PROP_BITMAP_SUB: u32 = 65536;
pub const AV_CODEC_PROP_TEXT_SUB: u32 = 131072;
pub const AV_INPUT_BUFFER_PADDING_SIZE: u32 = 64;
pub const AV_INPUT_BUFFER_MIN_SIZE: u32 = 16384;
pub const AV_CODEC_FLAG_UNALIGNED: u32 = 1;
pub const AV_CODEC_FLAG_QSCALE: u32 = 2;
pub const AV_CODEC_FLAG_4MV: u32 = 4;
pub const AV_CODEC_FLAG_OUTPUT_CORRUPT: u32 = 8;
pub const AV_CODEC_FLAG_QPEL: u32 = 16;
pub const AV_CODEC_FLAG_DROPCHANGED: u32 = 32;
pub const AV_CODEC_FLAG_PASS1: u32 = 512;
pub const AV_CODEC_FLAG_PASS2: u32 = 1024;
pub const AV_CODEC_FLAG_LOOP_FILTER: u32 = 2048;
pub const AV_CODEC_FLAG_GRAY: u32 = 8192;
pub const AV_CODEC_FLAG_PSNR: u32 = 32768;
pub const AV_CODEC_FLAG_TRUNCATED: u32 = 65536;
pub const AV_CODEC_FLAG_INTERLACED_DCT: u32 = 262144;
pub const AV_CODEC_FLAG_LOW_DELAY: u32 = 524288;
pub const AV_CODEC_FLAG_GLOBAL_HEADER: u32 = 4194304;
pub const AV_CODEC_FLAG_BITEXACT: u32 = 8388608;
pub const AV_CODEC_FLAG_AC_PRED: u32 = 16777216;
pub const AV_CODEC_FLAG_INTERLACED_ME: u32 = 536870912;
pub const AV_CODEC_FLAG_CLOSED_GOP: u32 = 2147483648;
pub const AV_CODEC_FLAG2_FAST: u32 = 1;
pub const AV_CODEC_FLAG2_NO_OUTPUT: u32 = 4;
pub const AV_CODEC_FLAG2_LOCAL_HEADER: u32 = 8;
pub const AV_CODEC_FLAG2_DROP_FRAME_TIMECODE: u32 = 8192;
pub const AV_CODEC_FLAG2_CHUNKS: u32 = 32768;
pub const AV_CODEC_FLAG2_IGNORE_CROP: u32 = 65536;
pub const AV_CODEC_FLAG2_SHOW_ALL: u32 = 4194304;
pub const AV_CODEC_FLAG2_EXPORT_MVS: u32 = 268435456;
pub const AV_CODEC_FLAG2_SKIP_MANUAL: u32 = 536870912;
pub const AV_CODEC_FLAG2_RO_FLUSH_NOOP: u32 = 1073741824;
pub const AV_CODEC_CAP_DRAW_HORIZ_BAND: u32 = 1;
pub const AV_CODEC_CAP_DR1: u32 = 2;
pub const AV_CODEC_CAP_TRUNCATED: u32 = 8;
pub const AV_CODEC_CAP_DELAY: u32 = 32;
pub const AV_CODEC_CAP_SMALL_LAST_FRAME: u32 = 64;
pub const AV_CODEC_CAP_SUBFRAMES: u32 = 256;
pub const AV_CODEC_CAP_EXPERIMENTAL: u32 = 512;
pub const AV_CODEC_CAP_CHANNEL_CONF: u32 = 1024;
pub const AV_CODEC_CAP_FRAME_THREADS: u32 = 4096;
pub const AV_CODEC_CAP_SLICE_THREADS: u32 = 8192;
pub const AV_CODEC_CAP_PARAM_CHANGE: u32 = 16384;
pub const AV_CODEC_CAP_AUTO_THREADS: u32 = 32768;
pub const AV_CODEC_CAP_VARIABLE_FRAME_SIZE: u32 = 65536;
pub const AV_CODEC_CAP_AVOID_PROBING: u32 = 131072;
pub const AV_CODEC_CAP_INTRA_ONLY: u32 = 1073741824;
pub const AV_CODEC_CAP_LOSSLESS: u32 = 2147483648;
pub const AV_CODEC_CAP_HARDWARE: u32 = 262144;
pub const AV_CODEC_CAP_HYBRID: u32 = 524288;
pub const AV_CODEC_CAP_ENCODER_REORDERED_OPAQUE: u32 = 1048576;
pub const AV_GET_BUFFER_FLAG_REF: u32 = 1;
pub const AV_PKT_FLAG_KEY: u32 = 1;
pub const AV_PKT_FLAG_CORRUPT: u32 = 2;
pub const AV_PKT_FLAG_DISCARD: u32 = 4;
pub const AV_PKT_FLAG_TRUSTED: u32 = 8;
pub const AV_PKT_FLAG_DISPOSABLE: u32 = 16;
pub const FF_COMPRESSION_DEFAULT: i32 = -1;
pub const FF_PRED_LEFT: u32 = 0;
pub const FF_PRED_PLANE: u32 = 1;
pub const FF_PRED_MEDIAN: u32 = 2;
pub const FF_CMP_SAD: u32 = 0;
pub const FF_CMP_SSE: u32 = 1;
pub const FF_CMP_SATD: u32 = 2;
pub const FF_CMP_DCT: u32 = 3;
pub const FF_CMP_PSNR: u32 = 4;
pub const FF_CMP_BIT: u32 = 5;
pub const FF_CMP_RD: u32 = 6;
pub const FF_CMP_ZERO: u32 = 7;
pub const FF_CMP_VSAD: u32 = 8;
pub const FF_CMP_VSSE: u32 = 9;
pub const FF_CMP_NSSE: u32 = 10;
pub const FF_CMP_W53: u32 = 11;
pub const FF_CMP_W97: u32 = 12;
pub const FF_CMP_DCTMAX: u32 = 13;
pub const FF_CMP_DCT264: u32 = 14;
pub const FF_CMP_MEDIAN_SAD: u32 = 15;
pub const FF_CMP_CHROMA: u32 = 256;
pub const SLICE_FLAG_CODED_ORDER: u32 = 1;
pub const SLICE_FLAG_ALLOW_FIELD: u32 = 2;
pub const SLICE_FLAG_ALLOW_PLANE: u32 = 4;
pub const FF_MB_DECISION_SIMPLE: u32 = 0;
pub const FF_MB_DECISION_BITS: u32 = 1;
pub const FF_MB_DECISION_RD: u32 = 2;
pub const FF_CODER_TYPE_VLC: u32 = 0;
pub const FF_CODER_TYPE_AC: u32 = 1;
pub const FF_CODER_TYPE_RAW: u32 = 2;
pub const FF_CODER_TYPE_RLE: u32 = 3;
pub const FF_BUG_AUTODETECT: u32 = 1;
pub const FF_BUG_XVID_ILACE: u32 = 4;
pub const FF_BUG_UMP4: u32 = 8;
pub const FF_BUG_NO_PADDING: u32 = 16;
pub const FF_BUG_AMV: u32 = 32;
pub const FF_BUG_QPEL_CHROMA: u32 = 64;
pub const FF_BUG_STD_QPEL: u32 = 128;
pub const FF_BUG_QPEL_CHROMA2: u32 = 256;
pub const FF_BUG_DIRECT_BLOCKSIZE: u32 = 512;
pub const FF_BUG_EDGE: u32 = 1024;
pub const FF_BUG_HPEL_CHROMA: u32 = 2048;
pub const FF_BUG_DC_CLIP: u32 = 4096;
pub const FF_BUG_MS: u32 = 8192;
pub const FF_BUG_TRUNCATED: u32 = 16384;
pub const FF_BUG_IEDGE: u32 = 32768;
pub const FF_COMPLIANCE_VERY_STRICT: u32 = 2;
pub const FF_COMPLIANCE_STRICT: u32 = 1;
pub const FF_COMPLIANCE_NORMAL: u32 = 0;
pub const FF_COMPLIANCE_UNOFFICIAL: i32 = -1;
pub const FF_COMPLIANCE_EXPERIMENTAL: i32 = -2;
pub const FF_EC_GUESS_MVS: u32 = 1;
pub const FF_EC_DEBLOCK: u32 = 2;
pub const FF_EC_FAVOR_INTER: u32 = 256;
pub const FF_DEBUG_PICT_INFO: u32 = 1;
pub const FF_DEBUG_RC: u32 = 2;
pub const FF_DEBUG_BITSTREAM: u32 = 4;
pub const FF_DEBUG_MB_TYPE: u32 = 8;
pub const FF_DEBUG_QP: u32 = 16;
pub const FF_DEBUG_DCT_COEFF: u32 = 64;
pub const FF_DEBUG_SKIP: u32 = 128;
pub const FF_DEBUG_STARTCODE: u32 = 256;
pub const FF_DEBUG_ER: u32 = 1024;
pub const FF_DEBUG_MMCO: u32 = 2048;
pub const FF_DEBUG_BUGS: u32 = 4096;
pub const FF_DEBUG_BUFFERS: u32 = 32768;
pub const FF_DEBUG_THREADS: u32 = 65536;
pub const FF_DEBUG_GREEN_MD: u32 = 8388608;
pub const FF_DEBUG_NOMC: u32 = 16777216;
pub const AV_EF_CRCCHECK: u32 = 1;
pub const AV_EF_BITSTREAM: u32 = 2;
pub const AV_EF_BUFFER: u32 = 4;
pub const AV_EF_EXPLODE: u32 = 8;
pub const AV_EF_IGNORE_ERR: u32 = 32768;
pub const AV_EF_CAREFUL: u32 = 65536;
pub const AV_EF_COMPLIANT: u32 = 131072;
pub const AV_EF_AGGRESSIVE: u32 = 262144;
pub const FF_DCT_AUTO: u32 = 0;
pub const FF_DCT_FASTINT: u32 = 1;
pub const FF_DCT_INT: u32 = 2;
pub const FF_DCT_MMX: u32 = 3;
pub const FF_DCT_ALTIVEC: u32 = 5;
pub const FF_DCT_FAAN: u32 = 6;
pub const FF_IDCT_AUTO: u32 = 0;
pub const FF_IDCT_INT: u32 = 1;
pub const FF_IDCT_SIMPLE: u32 = 2;
pub const FF_IDCT_SIMPLEMMX: u32 = 3;
pub const FF_IDCT_ARM: u32 = 7;
pub const FF_IDCT_ALTIVEC: u32 = 8;
pub const FF_IDCT_SIMPLEARM: u32 = 10;
pub const FF_IDCT_XVID: u32 = 14;
pub const FF_IDCT_SIMPLEARMV5TE: u32 = 16;
pub const FF_IDCT_SIMPLEARMV6: u32 = 17;
pub const FF_IDCT_FAAN: u32 = 20;
pub const FF_IDCT_SIMPLENEON: u32 = 22;
pub const FF_IDCT_NONE: u32 = 24;
pub const FF_IDCT_SIMPLEAUTO: u32 = 128;
pub const FF_THREAD_FRAME: u32 = 1;
pub const FF_THREAD_SLICE: u32 = 2;
pub const FF_PROFILE_UNKNOWN: i32 = -99;
pub const FF_PROFILE_RESERVED: i32 = -100;
pub const FF_PROFILE_AAC_MAIN: u32 = 0;
pub const FF_PROFILE_AAC_LOW: u32 = 1;
pub const FF_PROFILE_AAC_SSR: u32 = 2;
pub const FF_PROFILE_AAC_LTP: u32 = 3;
pub const FF_PROFILE_AAC_HE: u32 = 4;
pub const FF_PROFILE_AAC_HE_V2: u32 = 28;
pub const FF_PROFILE_AAC_LD: u32 = 22;
pub const FF_PROFILE_AAC_ELD: u32 = 38;
pub const FF_PROFILE_MPEG2_AAC_LOW: u32 = 128;
pub const FF_PROFILE_MPEG2_AAC_HE: u32 = 131;
pub const FF_PROFILE_DNXHD: u32 = 0;
pub const FF_PROFILE_DNXHR_LB: u32 = 1;
pub const FF_PROFILE_DNXHR_SQ: u32 = 2;
pub const FF_PROFILE_DNXHR_HQ: u32 = 3;
pub const FF_PROFILE_DNXHR_HQX: u32 = 4;
pub const FF_PROFILE_DNXHR_444: u32 = 5;
pub const FF_PROFILE_DTS: u32 = 20;
pub const FF_PROFILE_DTS_ES: u32 = 30;
pub const FF_PROFILE_DTS_96_24: u32 = 40;
pub const FF_PROFILE_DTS_HD_HRA: u32 = 50;
pub const FF_PROFILE_DTS_HD_MA: u32 = 60;
pub const FF_PROFILE_DTS_EXPRESS: u32 = 70;
pub const FF_PROFILE_MPEG2_422: u32 = 0;
pub const FF_PROFILE_MPEG2_HIGH: u32 = 1;
pub const FF_PROFILE_MPEG2_SS: u32 = 2;
pub const FF_PROFILE_MPEG2_SNR_SCALABLE: u32 = 3;
pub const FF_PROFILE_MPEG2_MAIN: u32 = 4;
pub const FF_PROFILE_MPEG2_SIMPLE: u32 = 5;
pub const FF_PROFILE_H264_CONSTRAINED: u32 = 512;
pub const FF_PROFILE_H264_INTRA: u32 = 2048;
pub const FF_PROFILE_H264_BASELINE: u32 = 66;
pub const FF_PROFILE_H264_CONSTRAINED_BASELINE: u32 = 578;
pub const FF_PROFILE_H264_MAIN: u32 = 77;
pub const FF_PROFILE_H264_EXTENDED: u32 = 88;
pub const FF_PROFILE_H264_HIGH: u32 = 100;
pub const FF_PROFILE_H264_HIGH_10: u32 = 110;
pub const FF_PROFILE_H264_HIGH_10_INTRA: u32 = 2158;
pub const FF_PROFILE_H264_MULTIVIEW_HIGH: u32 = 118;
pub const FF_PROFILE_H264_HIGH_422: u32 = 122;
pub const FF_PROFILE_H264_HIGH_422_INTRA: u32 = 2170;
pub const FF_PROFILE_H264_STEREO_HIGH: u32 = 128;
pub const FF_PROFILE_H264_HIGH_444: u32 = 144;
pub const FF_PROFILE_H264_HIGH_444_PREDICTIVE: u32 = 244;
pub const FF_PROFILE_H264_HIGH_444_INTRA: u32 = 2292;
pub const FF_PROFILE_H264_CAVLC_444: u32 = 44;
pub const FF_PROFILE_VC1_SIMPLE: u32 = 0;
pub const FF_PROFILE_VC1_MAIN: u32 = 1;
pub const FF_PROFILE_VC1_COMPLEX: u32 = 2;
pub const FF_PROFILE_VC1_ADVANCED: u32 = 3;
pub const FF_PROFILE_MPEG4_SIMPLE: u32 = 0;
pub const FF_PROFILE_MPEG4_SIMPLE_SCALABLE: u32 = 1;
pub const FF_PROFILE_MPEG4_CORE: u32 = 2;
pub const FF_PROFILE_MPEG4_MAIN: u32 = 3;
pub const FF_PROFILE_MPEG4_N_BIT: u32 = 4;
pub const FF_PROFILE_MPEG4_SCALABLE_TEXTURE: u32 = 5;
pub const FF_PROFILE_MPEG4_SIMPLE_FACE_ANIMATION: u32 = 6;
pub const FF_PROFILE_MPEG4_BASIC_ANIMATED_TEXTURE: u32 = 7;
pub const FF_PROFILE_MPEG4_HYBRID: u32 = 8;
pub const FF_PROFILE_MPEG4_ADVANCED_REAL_TIME: u32 = 9;
pub const FF_PROFILE_MPEG4_CORE_SCALABLE: u32 = 10;
pub const FF_PROFILE_MPEG4_ADVANCED_CODING: u32 = 11;
pub const FF_PROFILE_MPEG4_ADVANCED_CORE: u32 = 12;
pub const FF_PROFILE_MPEG4_ADVANCED_SCALABLE_TEXTURE: u32 = 13;
pub const FF_PROFILE_MPEG4_SIMPLE_STUDIO: u32 = 14;
pub const FF_PROFILE_MPEG4_ADVANCED_SIMPLE: u32 = 15;
pub const FF_PROFILE_JPEG2000_CSTREAM_RESTRICTION_0: u32 = 1;
pub const FF_PROFILE_JPEG2000_CSTREAM_RESTRICTION_1: u32 = 2;
pub const FF_PROFILE_JPEG2000_CSTREAM_NO_RESTRICTION: u32 = 32768;
pub const FF_PROFILE_JPEG2000_DCINEMA_2K: u32 = 3;
pub const FF_PROFILE_JPEG2000_DCINEMA_4K: u32 = 4;
pub const FF_PROFILE_VP9_0: u32 = 0;
pub const FF_PROFILE_VP9_1: u32 = 1;
pub const FF_PROFILE_VP9_2: u32 = 2;
pub const FF_PROFILE_VP9_3: u32 = 3;
pub const FF_PROFILE_HEVC_MAIN: u32 = 1;
pub const FF_PROFILE_HEVC_MAIN_10: u32 = 2;
pub const FF_PROFILE_HEVC_MAIN_STILL_PICTURE: u32 = 3;
pub const FF_PROFILE_HEVC_REXT: u32 = 4;
pub const FF_PROFILE_AV1_MAIN: u32 = 0;
pub const FF_PROFILE_AV1_HIGH: u32 = 1;
pub const FF_PROFILE_AV1_PROFESSIONAL: u32 = 2;
pub const FF_PROFILE_MJPEG_HUFFMAN_BASELINE_DCT: u32 = 192;
pub const FF_PROFILE_MJPEG_HUFFMAN_EXTENDED_SEQUENTIAL_DCT: u32 = 193;
pub const FF_PROFILE_MJPEG_HUFFMAN_PROGRESSIVE_DCT: u32 = 194;
pub const FF_PROFILE_MJPEG_HUFFMAN_LOSSLESS: u32 = 195;
pub const FF_PROFILE_MJPEG_JPEG_LS: u32 = 247;
pub const FF_PROFILE_SBC_MSBC: u32 = 1;
pub const FF_PROFILE_PRORES_PROXY: u32 = 0;
pub const FF_PROFILE_PRORES_LT: u32 = 1;
pub const FF_PROFILE_PRORES_STANDARD: u32 = 2;
pub const FF_PROFILE_PRORES_HQ: u32 = 3;
pub const FF_PROFILE_PRORES_4444: u32 = 4;
pub const FF_PROFILE_PRORES_XQ: u32 = 5;
pub const FF_PROFILE_ARIB_PROFILE_A: u32 = 0;
pub const FF_PROFILE_ARIB_PROFILE_C: u32 = 1;
pub const FF_LEVEL_UNKNOWN: i32 = -99;
pub const FF_SUB_CHARENC_MODE_DO_NOTHING: i32 = -1;
pub const FF_SUB_CHARENC_MODE_AUTOMATIC: u32 = 0;
pub const FF_SUB_CHARENC_MODE_PRE_DECODER: u32 = 1;
pub const FF_SUB_CHARENC_MODE_IGNORE: u32 = 2;
pub const FF_DEBUG_VIS_MV_P_FOR: u32 = 1;
pub const FF_DEBUG_VIS_MV_B_FOR: u32 = 2;
pub const FF_DEBUG_VIS_MV_B_BACK: u32 = 4;
pub const FF_CODEC_PROPERTY_LOSSLESS: u32 = 1;
pub const FF_CODEC_PROPERTY_CLOSED_CAPTIONS: u32 = 2;
pub const FF_SUB_TEXT_FMT_ASS: u32 = 0;
pub const FF_SUB_TEXT_FMT_ASS_WITH_TIMINGS: u32 = 1;
pub const AV_HWACCEL_CODEC_CAP_EXPERIMENTAL: u32 = 512;
pub const AV_HWACCEL_FLAG_IGNORE_LEVEL: u32 = 1;
pub const AV_HWACCEL_FLAG_ALLOW_HIGH_DEPTH: u32 = 2;
pub const AV_HWACCEL_FLAG_ALLOW_PROFILE_MISMATCH: u32 = 4;
pub const AV_SUBTITLE_FLAG_FORCED: u32 = 1;
pub const AV_PARSER_PTS_NB: u32 = 4;
pub const PARSER_FLAG_COMPLETE_FRAMES: u32 = 1;
pub const PARSER_FLAG_ONCE: u32 = 2;
pub const PARSER_FLAG_FETCHED_OFFSET: u32 = 4;
pub const PARSER_FLAG_USE_CODEC_TS: u32 = 4096;
pub const LIBAVDEVICE_VERSION_MAJOR: u32 = 58;
pub const LIBAVDEVICE_VERSION_MINOR: u32 = 8;
pub const LIBAVDEVICE_VERSION_MICRO: u32 = 100;
pub const _TIME_H: u32 = 1;
pub const _BITS_TIME_H: u32 = 1;
pub const CLOCK_REALTIME: u32 = 0;
pub const CLOCK_MONOTONIC: u32 = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
pub const CLOCK_THREAD_CPUTIME_ID: u32 = 3;
pub const CLOCK_MONOTONIC_RAW: u32 = 4;
pub const CLOCK_REALTIME_COARSE: u32 = 5;
pub const CLOCK_MONOTONIC_COARSE: u32 = 6;
pub const CLOCK_BOOTTIME: u32 = 7;
pub const CLOCK_REALTIME_ALARM: u32 = 8;
pub const CLOCK_BOOTTIME_ALARM: u32 = 9;
pub const CLOCK_TAI: u32 = 11;
pub const TIMER_ABSTIME: u32 = 1;
pub const __struct_tm_defined: u32 = 1;
pub const __itimerspec_defined: u32 = 1;
pub const TIME_UTC: u32 = 1;
pub const LIBAVFORMAT_VERSION_MAJOR: u32 = 58;
pub const LIBAVFORMAT_VERSION_MINOR: u32 = 29;
pub const LIBAVFORMAT_VERSION_MICRO: u32 = 100;
pub const FF_API_R_FRAME_RATE: u32 = 1;
pub const AVIO_SEEKABLE_NORMAL: u32 = 1;
pub const AVIO_SEEKABLE_TIME: u32 = 2;
pub const AVSEEK_SIZE: u32 = 65536;
pub const AVSEEK_FORCE: u32 = 131072;
pub const AVIO_FLAG_READ: u32 = 1;
pub const AVIO_FLAG_WRITE: u32 = 2;
pub const AVIO_FLAG_READ_WRITE: u32 = 3;
pub const AVIO_FLAG_NONBLOCK: u32 = 8;
pub const AVIO_FLAG_DIRECT: u32 = 32768;
pub const AVPROBE_SCORE_EXTENSION: u32 = 50;
pub const AVPROBE_SCORE_MIME: u32 = 75;
pub const AVPROBE_SCORE_MAX: u32 = 100;
pub const AVPROBE_PADDING_SIZE: u32 = 32;
pub const AVFMT_NOFILE: u32 = 1;
pub const AVFMT_NEEDNUMBER: u32 = 2;
pub const AVFMT_SHOW_IDS: u32 = 8;
pub const AVFMT_GLOBALHEADER: u32 = 64;
pub const AVFMT_NOTIMESTAMPS: u32 = 128;
pub const AVFMT_GENERIC_INDEX: u32 = 256;
pub const AVFMT_TS_DISCONT: u32 = 512;
pub const AVFMT_VARIABLE_FPS: u32 = 1024;
pub const AVFMT_NODIMENSIONS: u32 = 2048;
pub const AVFMT_NOSTREAMS: u32 = 4096;
pub const AVFMT_NOBINSEARCH: u32 = 8192;
pub const AVFMT_NOGENSEARCH: u32 = 16384;
pub const AVFMT_NO_BYTE_SEEK: u32 = 32768;
pub const AVFMT_ALLOW_FLUSH: u32 = 65536;
pub const AVFMT_TS_NONSTRICT: u32 = 131072;
pub const AVFMT_TS_NEGATIVE: u32 = 262144;
pub const AVFMT_SEEK_TO_PTS: u32 = 67108864;
pub const AVINDEX_KEYFRAME: u32 = 1;
pub const AVINDEX_DISCARD_FRAME: u32 = 2;
pub const AV_DISPOSITION_DEFAULT: u32 = 1;
pub const AV_DISPOSITION_DUB: u32 = 2;
pub const AV_DISPOSITION_ORIGINAL: u32 = 4;
pub const AV_DISPOSITION_COMMENT: u32 = 8;
pub const AV_DISPOSITION_LYRICS: u32 = 16;
pub const AV_DISPOSITION_KARAOKE: u32 = 32;
pub const AV_DISPOSITION_FORCED: u32 = 64;
pub const AV_DISPOSITION_HEARING_IMPAIRED: u32 = 128;
pub const AV_DISPOSITION_VISUAL_IMPAIRED: u32 = 256;
pub const AV_DISPOSITION_CLEAN_EFFECTS: u32 = 512;
pub const AV_DISPOSITION_ATTACHED_PIC: u32 = 1024;
pub const AV_DISPOSITION_TIMED_THUMBNAILS: u32 = 2048;
pub const AV_DISPOSITION_CAPTIONS: u32 = 65536;
pub const AV_DISPOSITION_DESCRIPTIONS: u32 = 131072;
pub const AV_DISPOSITION_METADATA: u32 = 262144;
pub const AV_DISPOSITION_DEPENDENT: u32 = 524288;
pub const AV_DISPOSITION_STILL_IMAGE: u32 = 1048576;
pub const AV_PTS_WRAP_IGNORE: u32 = 0;
pub const AV_PTS_WRAP_ADD_OFFSET: u32 = 1;
pub const AV_PTS_WRAP_SUB_OFFSET: i32 = -1;
pub const AVSTREAM_EVENT_FLAG_METADATA_UPDATED: u32 = 1;
pub const MAX_STD_TIMEBASES: u32 = 399;
pub const MAX_REORDER_DELAY: u32 = 16;
pub const AV_PROGRAM_RUNNING: u32 = 1;
pub const AVFMTCTX_NOHEADER: u32 = 1;
pub const AVFMTCTX_UNSEEKABLE: u32 = 2;
pub const AVFMT_FLAG_GENPTS: u32 = 1;
pub const AVFMT_FLAG_IGNIDX: u32 = 2;
pub const AVFMT_FLAG_NONBLOCK: u32 = 4;
pub const AVFMT_FLAG_IGNDTS: u32 = 8;
pub const AVFMT_FLAG_NOFILLIN: u32 = 16;
pub const AVFMT_FLAG_NOPARSE: u32 = 32;
pub const AVFMT_FLAG_NOBUFFER: u32 = 64;
pub const AVFMT_FLAG_CUSTOM_IO: u32 = 128;
pub const AVFMT_FLAG_DISCARD_CORRUPT: u32 = 256;
pub const AVFMT_FLAG_FLUSH_PACKETS: u32 = 512;
pub const AVFMT_FLAG_BITEXACT: u32 = 1024;
pub const AVFMT_FLAG_MP4A_LATM: u32 = 32768;
pub const AVFMT_FLAG_SORT_DTS: u32 = 65536;
pub const AVFMT_FLAG_PRIV_OPT: u32 = 131072;
pub const AVFMT_FLAG_KEEP_SIDE_DATA: u32 = 262144;
pub const AVFMT_FLAG_FAST_SEEK: u32 = 524288;
pub const AVFMT_FLAG_SHORTEST: u32 = 1048576;
pub const AVFMT_FLAG_AUTO_BSF: u32 = 2097152;
pub const FF_FDEBUG_TS: u32 = 1;
pub const AVFMT_EVENT_FLAG_METADATA_UPDATED: u32 = 1;
pub const AVFMT_AVOID_NEG_TS_AUTO: i32 = -1;
pub const AVFMT_AVOID_NEG_TS_MAKE_NON_NEGATIVE: u32 = 1;
pub const AVFMT_AVOID_NEG_TS_MAKE_ZERO: u32 = 2;
pub const AVSEEK_FLAG_BACKWARD: u32 = 1;
pub const AVSEEK_FLAG_BYTE: u32 = 2;
pub const AVSEEK_FLAG_ANY: u32 = 4;
pub const AVSEEK_FLAG_FRAME: u32 = 8;
pub const AVSTREAM_INIT_IN_WRITE_HEADER: u32 = 0;
pub const AVSTREAM_INIT_IN_INIT_OUTPUT: u32 = 1;
pub const AV_FRAME_FILENAME_FLAGS_MULTIPLE: u32 = 1;
pub const LIBAVFILTER_VERSION_MAJOR: u32 = 7;
pub const LIBAVFILTER_VERSION_MINOR: u32 = 57;
pub const LIBAVFILTER_VERSION_MICRO: u32 = 100;
pub const AVFILTER_FLAG_DYNAMIC_INPUTS: u32 = 1;
pub const AVFILTER_FLAG_DYNAMIC_OUTPUTS: u32 = 2;
pub const AVFILTER_FLAG_SLICE_THREADS: u32 = 4;
pub const AVFILTER_FLAG_SUPPORT_TIMELINE_GENERIC: u32 = 65536;
pub const AVFILTER_FLAG_SUPPORT_TIMELINE_INTERNAL: u32 = 131072;
pub const AVFILTER_FLAG_SUPPORT_TIMELINE: u32 = 196608;
pub const AVFILTER_THREAD_SLICE: u32 = 1;
pub const AVFILTER_CMD_FLAG_ONE: u32 = 1;
pub const AVFILTER_CMD_FLAG_FAST: u32 = 2;
pub const LIBSWRESAMPLE_VERSION_MAJOR: u32 = 3;
pub const LIBSWRESAMPLE_VERSION_MINOR: u32 = 5;
pub const LIBSWRESAMPLE_VERSION_MICRO: u32 = 100;
pub const SWR_FLAG_RESAMPLE: u32 = 1;
pub const LIBSWSCALE_VERSION_MAJOR: u32 = 5;
pub const LIBSWSCALE_VERSION_MINOR: u32 = 5;
pub const LIBSWSCALE_VERSION_MICRO: u32 = 100;
pub const SWS_FAST_BILINEAR: u32 = 1;
pub const SWS_BILINEAR: u32 = 2;
pub const SWS_BICUBIC: u32 = 4;
pub const SWS_X: u32 = 8;
pub const SWS_POINT: u32 = 16;
pub const SWS_AREA: u32 = 32;
pub const SWS_BICUBLIN: u32 = 64;
pub const SWS_GAUSS: u32 = 128;
pub const SWS_SINC: u32 = 256;
pub const SWS_LANCZOS: u32 = 512;
pub const SWS_SPLINE: u32 = 1024;
pub const SWS_SRC_V_CHR_DROP_MASK: u32 = 196608;
pub const SWS_SRC_V_CHR_DROP_SHIFT: u32 = 16;
pub const SWS_PARAM_DEFAULT: u32 = 123456;
pub const SWS_PRINT_INFO: u32 = 4096;
pub const SWS_FULL_CHR_H_INT: u32 = 8192;
pub const SWS_FULL_CHR_H_INP: u32 = 16384;
pub const SWS_DIRECT_BGR: u32 = 32768;
pub const SWS_ACCURATE_RND: u32 = 262144;
pub const SWS_BITEXACT: u32 = 524288;
pub const SWS_ERROR_DIFFUSION: u32 = 8388608;
pub const SWS_MAX_REDUCE_CUTOFF: f64 = 0.002;
pub const SWS_CS_ITU709: u32 = 1;
pub const SWS_CS_FCC: u32 = 4;
pub const SWS_CS_ITU601: u32 = 5;
pub const SWS_CS_ITU624: u32 = 5;
pub const SWS_CS_SMPTE170M: u32 = 5;
pub const SWS_CS_SMPTE240M: u32 = 7;
pub const SWS_CS_DEFAULT: u32 = 5;
pub const SWS_CS_BT2020: u32 = 9;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}

pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVDES {
    pub round_keys: [[u64; 16usize]; 3usize],
    pub triple_des: ::std::os::raw::c_int,
}

extern "C" {
    pub fn av_des_alloc() -> *mut AVDES;
}

extern "C" {
    pub fn av_des_init(
        d: *mut AVDES,
        key: *const u8,
        key_bits: ::std::os::raw::c_int,
        decrypt: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn av_des_crypt(
        d: *mut AVDES,
        dst: *mut u8,
        src: *const u8,
        count: ::std::os::raw::c_int,
        iv: *mut u8,
        decrypt: ::std::os::raw::c_int,
    );
}

extern "C" {
    pub fn av_des_mac(d: *mut AVDES, dst: *mut u8, src: *const u8, count: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVDictionaryEntry {
    pub key: *mut ::std::os::raw::c_char,
    pub value: *mut ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVDictionary {
    _unused: [u8; 0],
}

extern "C" {
    pub fn av_dict_get(
        m: *const AVDictionary,
        key: *const ::std::os::raw::c_char,
        prev: *const AVDictionaryEntry,
        flags: ::std::os::raw::c_int,
    ) -> *mut AVDictionaryEntry;
}

extern "C" {
    pub fn av_dict_count(m: *const AVDictionary) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn av_dict_set_int(
        pm: *mut *mut AVDictionary,
        key: *const ::std::os::raw::c_char,
        value: i64,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn av_dict_parse_string(
        pm: *mut *mut AVDictionary,
        str: *const ::std::os::raw::c_char,
        key_val_sep: *const ::std::os::raw::c_char,
        pairs_sep: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn av_dict_copy(
        dst: *mut *mut AVDictionary,
        src: *const AVDictionary,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn av_dict_free(m: *mut *mut AVDictionary);
}

extern "C" {
    pub fn av_dict_get_string(
        m: *const AVDictionary,
        buffer: *mut *mut ::std::os::raw::c_char,
        key_val_sep: ::std::os::raw::c_char,
        pairs_sep: ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn __errno_location() -> *mut ::std::os::raw::c_int;
}

pub type __gwchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct imaxdiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}

extern "C" {
    pub fn imaxabs(__n: intmax_t) -> intmax_t;
}

extern "C" {
    pub fn imaxdiv(__numer: intmax_t, __denom: intmax_t) -> imaxdiv_t;
}

extern "C" {
    pub fn strtoimax(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> intmax_t;
}

extern "C" {
    pub fn strtoumax(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> uintmax_t;
}

extern "C" {
    pub fn wcstoimax(
        __nptr: *const __gwchar_t,
        __endptr: *mut *mut __gwchar_t,
        __base: ::std::os::raw::c_int,
    ) -> intmax_t;
}
extern "C" {
    pub fn wcstoumax(
        __nptr: *const __gwchar_t,
        __endptr: *mut *mut __gwchar_t,
        __base: ::std::os::raw::c_int,
    ) -> uintmax_t;
}
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = u128;
pub type float_t = f32;
pub type double_t = f64;
extern "C" {
    pub fn __fpclassify(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __signbit(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __isinf(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __finite(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __isnan(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __iseqsig(__x: f64, __y: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __issignaling(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acos(__x: f64) -> f64;
}
extern "C" {
    pub fn __acos(__x: f64) -> f64;
}
extern "C" {
    pub fn asin(__x: f64) -> f64;
}
extern "C" {
    pub fn __asin(__x: f64) -> f64;
}
extern "C" {
    pub fn atan(__x: f64) -> f64;
}
extern "C" {
    pub fn __atan(__x: f64) -> f64;
}
extern "C" {
    pub fn atan2(__y: f64, __x: f64) -> f64;
}
extern "C" {
    pub fn __atan2(__y: f64, __x: f64) -> f64;
}
extern "C" {
    pub fn cos(__x: f64) -> f64;
}
extern "C" {
    pub fn __cos(__x: f64) -> f64;
}
extern "C" {
    pub fn sin(__x: f64) -> f64;
}
extern "C" {
    pub fn __sin(__x: f64) -> f64;
}
extern "C" {
    pub fn tan(__x: f64) -> f64;
}
extern "C" {
    pub fn __tan(__x: f64) -> f64;
}
extern "C" {
    pub fn cosh(__x: f64) -> f64;
}
extern "C" {
    pub fn __cosh(__x: f64) -> f64;
}
extern "C" {
    pub fn sinh(__x: f64) -> f64;
}
extern "C" {
    pub fn __sinh(__x: f64) -> f64;
}
extern "C" {
    pub fn tanh(__x: f64) -> f64;
}
extern "C" {
    pub fn __tanh(__x: f64) -> f64;
}
extern "C" {
    pub fn acosh(__x: f64) -> f64;
}
extern "C" {
    pub fn __acosh(__x: f64) -> f64;
}
extern "C" {
    pub fn asinh(__x: f64) -> f64;
}
extern "C" {
    pub fn __asinh(__x: f64) -> f64;
}
extern "C" {
    pub fn atanh(__x: f64) -> f64;
}
extern "C" {
    pub fn __atanh(__x: f64) -> f64;
}
extern "C" {
    pub fn exp(__x: f64) -> f64;
}
extern "C" {
    pub fn __exp(__x: f64) -> f64;
}
extern "C" {
    pub fn frexp(__x: f64, __exponent: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __frexp(__x: f64, __exponent: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn ldexp(__x: f64, __exponent: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __ldexp(__x: f64, __exponent: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn log(__x: f64) -> f64;
}
extern "C" {
    pub fn __log(__x: f64) -> f64;
}
extern "C" {
    pub fn log10(__x: f64) -> f64;
}
extern "C" {
    pub fn __log10(__x: f64) -> f64;
}
extern "C" {
    pub fn modf(__x: f64, __iptr: *mut f64) -> f64;
}
extern "C" {
    pub fn __modf(__x: f64, __iptr: *mut f64) -> f64;
}
extern "C" {
    pub fn expm1(__x: f64) -> f64;
}
extern "C" {
    pub fn __expm1(__x: f64) -> f64;
}
extern "C" {
    pub fn log1p(__x: f64) -> f64;
}
extern "C" {
    pub fn __log1p(__x: f64) -> f64;
}
extern "C" {
    pub fn logb(__x: f64) -> f64;
}
extern "C" {
    pub fn __logb(__x: f64) -> f64;
}
extern "C" {
    pub fn exp2(__x: f64) -> f64;
}
extern "C" {
    pub fn __exp2(__x: f64) -> f64;
}
extern "C" {
    pub fn log2(__x: f64) -> f64;
}
extern "C" {
    pub fn __log2(__x: f64) -> f64;
}
extern "C" {
    pub fn pow(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __pow(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn sqrt(__x: f64) -> f64;
}
extern "C" {
    pub fn __sqrt(__x: f64) -> f64;
}
extern "C" {
    pub fn hypot(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __hypot(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn cbrt(__x: f64) -> f64;
}
extern "C" {
    pub fn __cbrt(__x: f64) -> f64;
}
extern "C" {
    pub fn ceil(__x: f64) -> f64;
}
extern "C" {
    pub fn __ceil(__x: f64) -> f64;
}
extern "C" {
    pub fn fabs(__x: f64) -> f64;
}
extern "C" {
    pub fn __fabs(__x: f64) -> f64;
}
extern "C" {
    pub fn floor(__x: f64) -> f64;
}
extern "C" {
    pub fn __floor(__x: f64) -> f64;
}
extern "C" {
    pub fn fmod(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmod(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn isinf(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn finite(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn drem(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __drem(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn significand(__x: f64) -> f64;
}
extern "C" {
    pub fn __significand(__x: f64) -> f64;
}
extern "C" {
    pub fn copysign(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __copysign(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn nan(__tagb: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn __nan(__tagb: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn isnan(__value: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn j0(arg1: f64) -> f64;
}
extern "C" {
    pub fn __j0(arg1: f64) -> f64;
}
extern "C" {
    pub fn j1(arg1: f64) -> f64;
}
extern "C" {
    pub fn __j1(arg1: f64) -> f64;
}
extern "C" {
    pub fn jn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn __jn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn y0(arg1: f64) -> f64;
}
extern "C" {
    pub fn __y0(arg1: f64) -> f64;
}
extern "C" {
    pub fn y1(arg1: f64) -> f64;
}
extern "C" {
    pub fn __y1(arg1: f64) -> f64;
}
extern "C" {
    pub fn yn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn __yn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
extern "C" {
    pub fn erf(arg1: f64) -> f64;
}
extern "C" {
    pub fn __erf(arg1: f64) -> f64;
}
extern "C" {
    pub fn erfc(arg1: f64) -> f64;
}
extern "C" {
    pub fn __erfc(arg1: f64) -> f64;
}
extern "C" {
    pub fn lgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn __lgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn tgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn __tgamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn gamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn __gamma(arg1: f64) -> f64;
}
extern "C" {
    pub fn lgamma_r(arg1: f64, __signgamp: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __lgamma_r(arg1: f64, __signgamp: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn rint(__x: f64) -> f64;
}
extern "C" {
    pub fn __rint(__x: f64) -> f64;
}
extern "C" {
    pub fn nextafter(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __nextafter(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn nexttoward(__x: f64, __y: u128) -> f64;
}
extern "C" {
    pub fn __nexttoward(__x: f64, __y: u128) -> f64;
}
extern "C" {
    pub fn remainder(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __remainder(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn scalbn(__x: f64, __n: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __scalbn(__x: f64, __n: ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn ilogb(__x: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogb(__x: f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scalbln(__x: f64, __n: ::std::os::raw::c_long) -> f64;
}
extern "C" {
    pub fn __scalbln(__x: f64, __n: ::std::os::raw::c_long) -> f64;
}
extern "C" {
    pub fn nearbyint(__x: f64) -> f64;
}
extern "C" {
    pub fn __nearbyint(__x: f64) -> f64;
}
extern "C" {
    pub fn round(__x: f64) -> f64;
}
extern "C" {
    pub fn __round(__x: f64) -> f64;
}
extern "C" {
    pub fn trunc(__x: f64) -> f64;
}
extern "C" {
    pub fn __trunc(__x: f64) -> f64;
}
extern "C" {
    pub fn remquo(__x: f64, __y: f64, __quo: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn __remquo(__x: f64, __y: f64, __quo: *mut ::std::os::raw::c_int) -> f64;
}
extern "C" {
    pub fn lrint(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrint(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrint(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrint(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lround(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lround(__x: f64) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llround(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llround(__x: f64) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdim(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fdim(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fmax(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmax(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fmin(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn __fmin(__x: f64, __y: f64) -> f64;
}
extern "C" {
    pub fn fma(__x: f64, __y: f64, __z: f64) -> f64;
}
extern "C" {
    pub fn __fma(__x: f64, __y: f64, __z: f64) -> f64;
}
extern "C" {
    pub fn scalb(__x: f64, __n: f64) -> f64;
}
extern "C" {
    pub fn __scalb(__x: f64, __n: f64) -> f64;
}
extern "C" {
    pub fn __fpclassifyf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __signbitf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __isinff(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __finitef(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __isnanf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __iseqsigf(__x: f32, __y: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __issignalingf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acosf(__x: f32) -> f32;
}
extern "C" {
    pub fn __acosf(__x: f32) -> f32;
}
extern "C" {
    pub fn asinf(__x: f32) -> f32;
}
extern "C" {
    pub fn __asinf(__x: f32) -> f32;
}
extern "C" {
    pub fn atanf(__x: f32) -> f32;
}
extern "C" {
    pub fn __atanf(__x: f32) -> f32;
}
extern "C" {
    pub fn atan2f(__y: f32, __x: f32) -> f32;
}
extern "C" {
    pub fn __atan2f(__y: f32, __x: f32) -> f32;
}
extern "C" {
    pub fn cosf(__x: f32) -> f32;
}
extern "C" {
    pub fn __cosf(__x: f32) -> f32;
}
extern "C" {
    pub fn sinf(__x: f32) -> f32;
}
extern "C" {
    pub fn __sinf(__x: f32) -> f32;
}
extern "C" {
    pub fn tanf(__x: f32) -> f32;
}
extern "C" {
    pub fn __tanf(__x: f32) -> f32;
}
extern "C" {
    pub fn coshf(__x: f32) -> f32;
}
extern "C" {
    pub fn __coshf(__x: f32) -> f32;
}
extern "C" {
    pub fn sinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __sinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn tanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __tanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn acoshf(__x: f32) -> f32;
}
extern "C" {
    pub fn __acoshf(__x: f32) -> f32;
}
extern "C" {
    pub fn asinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __asinhf(__x: f32) -> f32;
}
extern "C" {
    pub fn atanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn __atanhf(__x: f32) -> f32;
}
extern "C" {
    pub fn expf(__x: f32) -> f32;
}
extern "C" {
    pub fn __expf(__x: f32) -> f32;
}
extern "C" {
    pub fn frexpf(__x: f32, __exponent: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __frexpf(__x: f32, __exponent: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn ldexpf(__x: f32, __exponent: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __ldexpf(__x: f32, __exponent: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn logf(__x: f32) -> f32;
}
extern "C" {
    pub fn __logf(__x: f32) -> f32;
}
extern "C" {
    pub fn log10f(__x: f32) -> f32;
}
extern "C" {
    pub fn __log10f(__x: f32) -> f32;
}
extern "C" {
    pub fn modff(__x: f32, __iptr: *mut f32) -> f32;
}
extern "C" {
    pub fn __modff(__x: f32, __iptr: *mut f32) -> f32;
}
extern "C" {
    pub fn expm1f(__x: f32) -> f32;
}
extern "C" {
    pub fn __expm1f(__x: f32) -> f32;
}
extern "C" {
    pub fn log1pf(__x: f32) -> f32;
}
extern "C" {
    pub fn __log1pf(__x: f32) -> f32;
}
extern "C" {
    pub fn logbf(__x: f32) -> f32;
}
extern "C" {
    pub fn __logbf(__x: f32) -> f32;
}
extern "C" {
    pub fn exp2f(__x: f32) -> f32;
}
extern "C" {
    pub fn __exp2f(__x: f32) -> f32;
}
extern "C" {
    pub fn log2f(__x: f32) -> f32;
}
extern "C" {
    pub fn __log2f(__x: f32) -> f32;
}
extern "C" {
    pub fn powf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __powf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn sqrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn __sqrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn hypotf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __hypotf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn cbrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn __cbrtf(__x: f32) -> f32;
}
extern "C" {
    pub fn ceilf(__x: f32) -> f32;
}
extern "C" {
    pub fn __ceilf(__x: f32) -> f32;
}
extern "C" {
    pub fn fabsf(__x: f32) -> f32;
}
extern "C" {
    pub fn __fabsf(__x: f32) -> f32;
}
extern "C" {
    pub fn floorf(__x: f32) -> f32;
}
extern "C" {
    pub fn __floorf(__x: f32) -> f32;
}
extern "C" {
    pub fn fmodf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fmodf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn isinff(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn finitef(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dremf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __dremf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn significandf(__x: f32) -> f32;
}
extern "C" {
    pub fn __significandf(__x: f32) -> f32;
}
extern "C" {
    pub fn copysignf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __copysignf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn nanf(__tagb: *const ::std::os::raw::c_char) -> f32;
}
extern "C" {
    pub fn __nanf(__tagb: *const ::std::os::raw::c_char) -> f32;
}
extern "C" {
    pub fn isnanf(__value: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn j0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn __j0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn j1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn __j1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn jnf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn __jnf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn y0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn __y0f(arg1: f32) -> f32;
}
extern "C" {
    pub fn y1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn __y1f(arg1: f32) -> f32;
}
extern "C" {
    pub fn ynf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn __ynf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
extern "C" {
    pub fn erff(arg1: f32) -> f32;
}
extern "C" {
    pub fn __erff(arg1: f32) -> f32;
}
extern "C" {
    pub fn erfcf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __erfcf(arg1: f32) -> f32;
}
extern "C" {
    pub fn lgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __lgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn tgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __tgammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn gammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn __gammaf(arg1: f32) -> f32;
}
extern "C" {
    pub fn lgammaf_r(arg1: f32, __signgamp: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __lgammaf_r(arg1: f32, __signgamp: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn rintf(__x: f32) -> f32;
}
extern "C" {
    pub fn __rintf(__x: f32) -> f32;
}
extern "C" {
    pub fn nextafterf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __nextafterf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn nexttowardf(__x: f32, __y: u128) -> f32;
}
extern "C" {
    pub fn __nexttowardf(__x: f32, __y: u128) -> f32;
}
extern "C" {
    pub fn remainderf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __remainderf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn scalbnf(__x: f32, __n: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __scalbnf(__x: f32, __n: ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn ilogbf(__x: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogbf(__x: f32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scalblnf(__x: f32, __n: ::std::os::raw::c_long) -> f32;
}
extern "C" {
    pub fn __scalblnf(__x: f32, __n: ::std::os::raw::c_long) -> f32;
}
extern "C" {
    pub fn nearbyintf(__x: f32) -> f32;
}
extern "C" {
    pub fn __nearbyintf(__x: f32) -> f32;
}
extern "C" {
    pub fn roundf(__x: f32) -> f32;
}
extern "C" {
    pub fn __roundf(__x: f32) -> f32;
}
extern "C" {
    pub fn truncf(__x: f32) -> f32;
}
extern "C" {
    pub fn __truncf(__x: f32) -> f32;
}
extern "C" {
    pub fn remquof(__x: f32, __y: f32, __quo: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn __remquof(__x: f32, __y: f32, __quo: *mut ::std::os::raw::c_int) -> f32;
}
extern "C" {
    pub fn lrintf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrintf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrintf(__x: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrintf(__x: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lroundf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lroundf(__x: f32) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llroundf(__x: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llroundf(__x: f32) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdimf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fdimf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn fmaxf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fmaxf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn fminf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn __fminf(__x: f32, __y: f32) -> f32;
}
extern "C" {
    pub fn fmaf(__x: f32, __y: f32, __z: f32) -> f32;
}
extern "C" {
    pub fn __fmaf(__x: f32, __y: f32, __z: f32) -> f32;
}
extern "C" {
    pub fn scalbf(__x: f32, __n: f32) -> f32;
}
extern "C" {
    pub fn __scalbf(__x: f32, __n: f32) -> f32;
}
extern "C" {
    pub fn __fpclassifyl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __signbitl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __isinfl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __finitel(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __isnanl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __iseqsigl(__x: u128, __y: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __issignalingl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn acosl(__x: u128) -> u128;
}
extern "C" {
    pub fn __acosl(__x: u128) -> u128;
}
extern "C" {
    pub fn asinl(__x: u128) -> u128;
}
extern "C" {
    pub fn __asinl(__x: u128) -> u128;
}
extern "C" {
    pub fn atanl(__x: u128) -> u128;
}
extern "C" {
    pub fn __atanl(__x: u128) -> u128;
}
extern "C" {
    pub fn atan2l(__y: u128, __x: u128) -> u128;
}
extern "C" {
    pub fn __atan2l(__y: u128, __x: u128) -> u128;
}
extern "C" {
    pub fn cosl(__x: u128) -> u128;
}
extern "C" {
    pub fn __cosl(__x: u128) -> u128;
}
extern "C" {
    pub fn sinl(__x: u128) -> u128;
}
extern "C" {
    pub fn __sinl(__x: u128) -> u128;
}
extern "C" {
    pub fn tanl(__x: u128) -> u128;
}
extern "C" {
    pub fn __tanl(__x: u128) -> u128;
}
extern "C" {
    pub fn coshl(__x: u128) -> u128;
}
extern "C" {
    pub fn __coshl(__x: u128) -> u128;
}
extern "C" {
    pub fn sinhl(__x: u128) -> u128;
}
extern "C" {
    pub fn __sinhl(__x: u128) -> u128;
}
extern "C" {
    pub fn tanhl(__x: u128) -> u128;
}
extern "C" {
    pub fn __tanhl(__x: u128) -> u128;
}
extern "C" {
    pub fn acoshl(__x: u128) -> u128;
}
extern "C" {
    pub fn __acoshl(__x: u128) -> u128;
}
extern "C" {
    pub fn asinhl(__x: u128) -> u128;
}
extern "C" {
    pub fn __asinhl(__x: u128) -> u128;
}
extern "C" {
    pub fn atanhl(__x: u128) -> u128;
}
extern "C" {
    pub fn __atanhl(__x: u128) -> u128;
}
extern "C" {
    pub fn expl(__x: u128) -> u128;
}
extern "C" {
    pub fn __expl(__x: u128) -> u128;
}
extern "C" {
    pub fn frexpl(__x: u128, __exponent: *mut ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn __frexpl(__x: u128, __exponent: *mut ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn ldexpl(__x: u128, __exponent: ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn __ldexpl(__x: u128, __exponent: ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn logl(__x: u128) -> u128;
}
extern "C" {
    pub fn __logl(__x: u128) -> u128;
}
extern "C" {
    pub fn log10l(__x: u128) -> u128;
}
extern "C" {
    pub fn __log10l(__x: u128) -> u128;
}
extern "C" {
    pub fn modfl(__x: u128, __iptr: *mut u128) -> u128;
}
extern "C" {
    pub fn __modfl(__x: u128, __iptr: *mut u128) -> u128;
}
extern "C" {
    pub fn expm1l(__x: u128) -> u128;
}
extern "C" {
    pub fn __expm1l(__x: u128) -> u128;
}
extern "C" {
    pub fn log1pl(__x: u128) -> u128;
}
extern "C" {
    pub fn __log1pl(__x: u128) -> u128;
}
extern "C" {
    pub fn logbl(__x: u128) -> u128;
}
extern "C" {
    pub fn __logbl(__x: u128) -> u128;
}
extern "C" {
    pub fn exp2l(__x: u128) -> u128;
}
extern "C" {
    pub fn __exp2l(__x: u128) -> u128;
}
extern "C" {
    pub fn log2l(__x: u128) -> u128;
}
extern "C" {
    pub fn __log2l(__x: u128) -> u128;
}
extern "C" {
    pub fn powl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __powl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn sqrtl(__x: u128) -> u128;
}
extern "C" {
    pub fn __sqrtl(__x: u128) -> u128;
}
extern "C" {
    pub fn hypotl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __hypotl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn cbrtl(__x: u128) -> u128;
}
extern "C" {
    pub fn __cbrtl(__x: u128) -> u128;
}
extern "C" {
    pub fn ceill(__x: u128) -> u128;
}
extern "C" {
    pub fn __ceill(__x: u128) -> u128;
}
extern "C" {
    pub fn fabsl(__x: u128) -> u128;
}
extern "C" {
    pub fn __fabsl(__x: u128) -> u128;
}
extern "C" {
    pub fn floorl(__x: u128) -> u128;
}
extern "C" {
    pub fn __floorl(__x: u128) -> u128;
}
extern "C" {
    pub fn fmodl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __fmodl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn isinfl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn finitel(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dreml(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __dreml(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn significandl(__x: u128) -> u128;
}
extern "C" {
    pub fn __significandl(__x: u128) -> u128;
}
extern "C" {
    pub fn copysignl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __copysignl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn nanl(__tagb: *const ::std::os::raw::c_char) -> u128;
}
extern "C" {
    pub fn __nanl(__tagb: *const ::std::os::raw::c_char) -> u128;
}
extern "C" {
    pub fn isnanl(__value: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn j0l(arg1: u128) -> u128;
}
extern "C" {
    pub fn __j0l(arg1: u128) -> u128;
}
extern "C" {
    pub fn j1l(arg1: u128) -> u128;
}
extern "C" {
    pub fn __j1l(arg1: u128) -> u128;
}
extern "C" {
    pub fn jnl(arg1: ::std::os::raw::c_int, arg2: u128) -> u128;
}
extern "C" {
    pub fn __jnl(arg1: ::std::os::raw::c_int, arg2: u128) -> u128;
}
extern "C" {
    pub fn y0l(arg1: u128) -> u128;
}
extern "C" {
    pub fn __y0l(arg1: u128) -> u128;
}
extern "C" {
    pub fn y1l(arg1: u128) -> u128;
}
extern "C" {
    pub fn __y1l(arg1: u128) -> u128;
}
extern "C" {
    pub fn ynl(arg1: ::std::os::raw::c_int, arg2: u128) -> u128;
}
extern "C" {
    pub fn __ynl(arg1: ::std::os::raw::c_int, arg2: u128) -> u128;
}
extern "C" {
    pub fn erfl(arg1: u128) -> u128;
}
extern "C" {
    pub fn __erfl(arg1: u128) -> u128;
}
extern "C" {
    pub fn erfcl(arg1: u128) -> u128;
}
extern "C" {
    pub fn __erfcl(arg1: u128) -> u128;
}
extern "C" {
    pub fn lgammal(arg1: u128) -> u128;
}
extern "C" {
    pub fn __lgammal(arg1: u128) -> u128;
}
extern "C" {
    pub fn tgammal(arg1: u128) -> u128;
}
extern "C" {
    pub fn __tgammal(arg1: u128) -> u128;
}
extern "C" {
    pub fn gammal(arg1: u128) -> u128;
}
extern "C" {
    pub fn __gammal(arg1: u128) -> u128;
}
extern "C" {
    pub fn lgammal_r(arg1: u128, __signgamp: *mut ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn __lgammal_r(arg1: u128, __signgamp: *mut ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn rintl(__x: u128) -> u128;
}
extern "C" {
    pub fn __rintl(__x: u128) -> u128;
}
extern "C" {
    pub fn nextafterl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __nextafterl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn nexttowardl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __nexttowardl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn remainderl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __remainderl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn scalbnl(__x: u128, __n: ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn __scalbnl(__x: u128, __n: ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn ilogbl(__x: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __ilogbl(__x: u128) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scalblnl(__x: u128, __n: ::std::os::raw::c_long) -> u128;
}
extern "C" {
    pub fn __scalblnl(__x: u128, __n: ::std::os::raw::c_long) -> u128;
}
extern "C" {
    pub fn nearbyintl(__x: u128) -> u128;
}
extern "C" {
    pub fn __nearbyintl(__x: u128) -> u128;
}
extern "C" {
    pub fn roundl(__x: u128) -> u128;
}
extern "C" {
    pub fn __roundl(__x: u128) -> u128;
}
extern "C" {
    pub fn truncl(__x: u128) -> u128;
}
extern "C" {
    pub fn __truncl(__x: u128) -> u128;
}
extern "C" {
    pub fn remquol(__x: u128, __y: u128, __quo: *mut ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn __remquol(__x: u128, __y: u128, __quo: *mut ::std::os::raw::c_int) -> u128;
}
extern "C" {
    pub fn lrintl(__x: u128) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lrintl(__x: u128) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llrintl(__x: u128) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llrintl(__x: u128) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lroundl(__x: u128) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn __lroundl(__x: u128) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llroundl(__x: u128) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn __llroundl(__x: u128) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fdiml(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __fdiml(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn fmaxl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __fmaxl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn fminl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn __fminl(__x: u128, __y: u128) -> u128;
}
extern "C" {
    pub fn fmal(__x: u128, __y: u128, __z: u128) -> u128;
}
extern "C" {
    pub fn __fmal(__x: u128, __y: u128, __z: u128) -> u128;
}
extern "C" {
    pub fn scalbl(__x: u128, __n: u128) -> u128;
}
extern "C" {
    pub fn __scalbl(__x: u128, __n: u128) -> u128;
}
extern "C" {
    pub static mut signgam: ::std::os::raw::c_int;
}
pub const FP_NAN: _bindgen_ty_1 = 0;
pub const FP_INFINITE: _bindgen_ty_1 = 1;
pub const FP_ZERO: _bindgen_ty_1 = 2;
pub const FP_SUBNORMAL: _bindgen_ty_1 = 3;
pub const FP_NORMAL: _bindgen_ty_1 = 4;
pub type _bindgen_ty_1 = u32;
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: ::std::os::raw::c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: ::std::os::raw::c_uint,
    pub __wchb: [::std::os::raw::c_char; 4usize],
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
pub type __fpos_t = _G_fpos_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
pub type __fpos64_t = _G_fpos64_t;
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::std::os::raw::c_void,
    pub __pad5: usize,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
pub type off_t = __off_t;
pub type fpos_t = __fpos_t;
extern "C" {
    pub static mut stdin: *mut FILE;
}
extern "C" {
    pub static mut stdout: *mut FILE;
}
extern "C" {
    pub static mut stderr: *mut FILE;
}
extern "C" {
    pub fn remove(__filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        __old: *const ::std::os::raw::c_char,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameat(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tempnam(
        __dir: *const ::std::os::raw::c_char,
        __pfx: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn freopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fdopen(__fd: ::std::os::raw::c_int, __modes: *const ::std::os::raw::c_char)
        -> *mut FILE;
}
extern "C" {
    pub fn fmemopen(
        __s: *mut ::std::os::raw::c_void,
        __len: usize,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(
        __bufloc: *mut *mut ::std::os::raw::c_char,
        __sizeloc: *mut usize,
    ) -> *mut FILE;
}
extern "C" {
    pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::std::os::raw::c_char,
        __modes: ::std::os::raw::c_int,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char, __size: usize);
}
extern "C" {
    pub fn setlinebuf(__stream: *mut FILE);
}
extern "C" {
    pub fn fprintf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn printf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfprintf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vprintf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn snprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: ::std::os::raw::c_ulong,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsnprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: ::std::os::raw::c_ulong,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vdprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fscanf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scanf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_fscanf"]
    pub fn fscanf1(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_scanf"]
    pub fn scanf1(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_sscanf"]
    pub fn sscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfscanf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vscanf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vfscanf"]
    pub fn vfscanf1(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vscanf"]
    pub fn vscanf1(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vsscanf"]
    pub fn vsscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar_unlocked(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(__w: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getline(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn fputs(__s: *const ::std::os::raw::c_char, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __n: ::std::os::raw::c_ulong,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __n: ::std::os::raw::c_ulong,
        __s: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn fread_unlocked(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fwrite_unlocked(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fseek(
        __stream: *mut FILE,
        __off: ::std::os::raw::c_long,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rewind(__stream: *mut FILE);
}
extern "C" {
    pub fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> __off_t;
}
extern "C" {
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr(__stream: *mut FILE);
}
extern "C" {
    pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr_unlocked(__stream: *mut FILE);
}
extern "C" {
    pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(__s: *const ::std::os::raw::c_char);
}
extern "C" {
    pub static mut sys_nerr: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub fn fileno(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn popen(
        __command: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ctermid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn flockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funlockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn __uflow(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut FILE, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
extern "C" {
    pub fn __ctype_get_mb_cur_max() -> usize;
}
extern "C" {
    pub fn atof(__nptr: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtod(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtold(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> u128;
}
extern "C" {
    pub fn strtol(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtoul(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtoll(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoull(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn l64a(__n: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn a64l(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type u_int8_t = __uint8_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
pub type register_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::std::os::raw::c_uint,
    pub __writers: ::std::os::raw::c_uint,
    pub __wrphase_futex: ::std::os::raw::c_uint,
    pub __writers_futex: ::std::os::raw::c_uint,
    pub __pad3: ::std::os::raw::c_uint,
    pub __pad4: ::std::os::raw::c_uint,
    pub __cur_writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_schar,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __bindgen_anon_1: __pthread_cond_s__bindgen_ty_1,
    pub __bindgen_anon_2: __pthread_cond_s__bindgen_ty_2,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_1 {
    pub __wseq: ::std::os::raw::c_ulonglong,
    pub __wseq32: __pthread_cond_s__bindgen_ty_1__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_1__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __pthread_cond_s__bindgen_ty_2 {
    pub __g1_start: ::std::os::raw::c_ulonglong,
    pub __g1_start32: __pthread_cond_s__bindgen_ty_2__bindgen_ty_1,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cond_s__bindgen_ty_2__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 40usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 5usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::std::os::raw::c_char; 48usize],
    pub __align: ::std::os::raw::c_longlong,
    _bindgen_union_align: [u64; 6usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [::std::os::raw::c_char; 8usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: u64,
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [::std::os::raw::c_char; 32usize],
    pub __align: ::std::os::raw::c_long,
    _bindgen_union_align: [u64; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
    _bindgen_union_align: u32,
}
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srandom(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn initstate(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn setstate(__statebuf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: ::std::os::raw::c_int,
    pub rand_deg: ::std::os::raw::c_int,
    pub rand_sep: ::std::os::raw::c_int,
    pub end_ptr: *mut i32,
}
extern "C" {
    pub fn random_r(__buf: *mut random_data, __result: *mut i32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srandom_r(
        __seed: ::std::os::raw::c_uint,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate_r(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setstate_r(
        __statebuf: *mut ::std::os::raw::c_char,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rand_r(__seed: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn erand48(__xsubi: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn jrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srand48(__seedval: ::std::os::raw::c_long);
}
extern "C" {
    pub fn seed48(__seed16v: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn lcong48(__param: *mut ::std::os::raw::c_ushort);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drand48_data {
    pub __x: [::std::os::raw::c_ushort; 3usize],
    pub __old_x: [::std::os::raw::c_ushort; 3usize],
    pub __c: ::std::os::raw::c_ushort,
    pub __init: ::std::os::raw::c_ushort,
    pub __a: ::std::os::raw::c_ulonglong,
}
extern "C" {
    pub fn drand48_r(__buffer: *mut drand48_data, __result: *mut f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn erand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand48_r(
        __seedval: ::std::os::raw::c_long,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seed48_r(
        __seed16v: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lcong48_r(
        __param: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn malloc(__size: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(
        __nmemb: ::std::os::raw::c_ulong,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn reallocarray(
        __ptr: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn free(__ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn alloca(__size: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn valloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: usize,
        __size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aligned_alloc(__alignment: usize, __size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atexit(__func: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn at_quick_exit(
        __func: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn on_exit(
        __func: ::std::option::Option<
            unsafe extern "C" fn(
                __status: ::std::os::raw::c_int,
                __arg: *mut ::std::os::raw::c_void,
            ),
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn quick_exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _Exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn putenv(__string: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __replace: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearenv() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mktemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(__template: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemps(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdtemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn system(__command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn realpath(
        __name: *const ::std::os::raw::c_char,
        __resolved: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub type __compar_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *const ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn qsort(
        __base: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    );
}
extern "C" {
    pub fn abs(__x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn labs(__x: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llabs(__x: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn div(__numer: ::std::os::raw::c_int, __denom: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: ::std::os::raw::c_long, __denom: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(
        __numer: ::std::os::raw::c_longlong,
        __denom: ::std::os::raw::c_longlong,
    ) -> lldiv_t;
}
extern "C" {
    pub fn ecvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qecvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qfcvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qgcvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ecvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fcvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qecvt_r(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qfcvt_r(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbtowc(
        __pwc: *mut wchar_t,
        __s: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut ::std::os::raw::c_char, __wchar: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut wchar_t, __s: *const ::std::os::raw::c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn wcstombs(__s: *mut ::std::os::raw::c_char, __pwcs: *const wchar_t, __n: usize) -> usize;
}
extern "C" {
    pub fn rpmatch(__response: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getsubopt(
        __optionp: *mut *mut ::std::os::raw::c_char,
        __tokens: *const *mut ::std::os::raw::c_char,
        __valuep: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getloadavg(__loadavg: *mut f64, __nelem: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memmove(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memccpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memset(
        __s: *mut ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcoll(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strxfrm(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const ::std::os::raw::c_ushort,
    pub __ctype_tolower: *const ::std::os::raw::c_int,
    pub __ctype_toupper: *const ::std::os::raw::c_int,
    pub __names: [*const ::std::os::raw::c_char; 13usize],
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
extern "C" {
    pub fn strcoll_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __l: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strxfrm_l(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
        __l: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn strdup(__s: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strndup(
        __string: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strrchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcspn(
        __s: *const ::std::os::raw::c_char,
        __reject: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strspn(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strpbrk(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strstr(
        __haystack: *const ::std::os::raw::c_char,
        __needle: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strlen(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strnlen(__string: *const ::std::os::raw::c_char, __maxlen: usize) -> usize;
}
extern "C" {
    pub fn strerror(__errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}__xpg_strerror_r"]
    pub fn strerror_r(
        __errnum: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strerror_l(
        __errnum: ::std::os::raw::c_int,
        __l: locale_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bcopy(
        __src: *const ::std::os::raw::c_void,
        __dest: *mut ::std::os::raw::c_void,
        __n: usize,
    );
}
extern "C" {
    pub fn bzero(__s: *mut ::std::os::raw::c_void, __n: ::std::os::raw::c_ulong);
}
extern "C" {
    pub fn index(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rindex(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ffs(__i: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsl(__l: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsll(__ll: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn explicit_bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
}
extern "C" {
    pub fn strsep(
        __stringp: *mut *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strsignal(__sig: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_log2(v: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_log2_16bit(v: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
extern "C" {
    pub fn av_strerror(
        errnum: ::std::os::raw::c_int,
        errbuf: *mut ::std::os::raw::c_char,
        errbuf_size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avutil_version() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn av_version_info() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avutil_configuration() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avutil_license() -> *const ::std::os::raw::c_char;
}
pub const AVMediaType_AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
pub const AVMediaType_AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
pub const AVMediaType_AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
pub const AVMediaType_AVMEDIA_TYPE_DATA: AVMediaType = 2;
pub const AVMediaType_AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
pub const AVMediaType_AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
pub const AVMediaType_AVMEDIA_TYPE_NB: AVMediaType = 5;
pub type AVMediaType = i32;
extern "C" {
    pub fn av_get_media_type_string(media_type: AVMediaType) -> *const ::std::os::raw::c_char;
}
pub const AVPictureType_AV_PICTURE_TYPE_NONE: AVPictureType = 0;
pub const AVPictureType_AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AVPictureType_AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AVPictureType_AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AVPictureType_AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AVPictureType_AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AVPictureType_AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AVPictureType_AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub type AVPictureType = u32;
extern "C" {
    pub fn av_get_picture_type_char(pict_type: AVPictureType) -> ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVRational {
    pub num: ::std::os::raw::c_int,
    pub den: ::std::os::raw::c_int,
}
extern "C" {
    pub fn av_reduce(
        dst_num: *mut ::std::os::raw::c_int,
        dst_den: *mut ::std::os::raw::c_int,
        num: i64,
        den: i64,
        max: i64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_mul_q(b: AVRational, c: AVRational) -> AVRational;
}
extern "C" {
    pub fn av_div_q(b: AVRational, c: AVRational) -> AVRational;
}
extern "C" {
    pub fn av_add_q(b: AVRational, c: AVRational) -> AVRational;
}
extern "C" {
    pub fn av_sub_q(b: AVRational, c: AVRational) -> AVRational;
}
extern "C" {
    pub fn av_d2q(d: f64, max: ::std::os::raw::c_int) -> AVRational;
}
extern "C" {
    pub fn av_nearer_q(q: AVRational, q1: AVRational, q2: AVRational) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_find_nearest_q_idx(q: AVRational, q_list: *const AVRational)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_q2intfloat(q: AVRational) -> u32;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union av_intfloat32 {
    pub i: u32,
    pub f: f32,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union av_intfloat64 {
    pub i: u64,
    pub f: f64,
    _bindgen_union_align: u64,
}
pub const AVRounding_AV_ROUND_ZERO: AVRounding = 0;
pub const AVRounding_AV_ROUND_INF: AVRounding = 1;
pub const AVRounding_AV_ROUND_DOWN: AVRounding = 2;
pub const AVRounding_AV_ROUND_UP: AVRounding = 3;
pub const AVRounding_AV_ROUND_NEAR_INF: AVRounding = 5;
pub const AVRounding_AV_ROUND_PASS_MINMAX: AVRounding = 8192;
pub type AVRounding = u32;
extern "C" {
    pub fn av_gcd(a: i64, b: i64) -> i64;
}
extern "C" {
    pub fn av_rescale(a: i64, b: i64, c: i64) -> i64;
}
extern "C" {
    pub fn av_rescale_rnd(a: i64, b: i64, c: i64, rnd: AVRounding) -> i64;
}
extern "C" {
    pub fn av_rescale_q(a: i64, bq: AVRational, cq: AVRational) -> i64;
}
extern "C" {
    pub fn av_rescale_q_rnd(a: i64, bq: AVRational, cq: AVRational, rnd: AVRounding) -> i64;
}
extern "C" {
    pub fn av_compare_ts(
        ts_a: i64,
        tb_a: AVRational,
        ts_b: i64,
        tb_b: AVRational,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_compare_mod(a: u64, b: u64, mod_: u64) -> i64;
}
extern "C" {
    pub fn av_rescale_delta(
        in_tb: AVRational,
        in_ts: i64,
        fs_tb: AVRational,
        duration: ::std::os::raw::c_int,
        last: *mut i64,
        out_tb: AVRational,
    ) -> i64;
}
extern "C" {
    pub fn av_add_stable(ts_tb: AVRational, ts: i64, inc_tb: AVRational, inc: i64) -> i64;
}
pub const AVClassCategory_AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
pub const AVClassCategory_AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
pub const AVClassCategory_AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
pub const AVClassCategory_AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
pub const AVClassCategory_AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
pub const AVClassCategory_AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
pub const AVClassCategory_AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
pub const AVClassCategory_AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
pub const AVClassCategory_AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
pub const AVClassCategory_AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
pub const AVClassCategory_AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
pub const AVClassCategory_AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
pub const AVClassCategory_AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
pub const AVClassCategory_AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
pub const AVClassCategory_AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
pub const AVClassCategory_AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
pub const AVClassCategory_AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
pub const AVClassCategory_AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
pub type AVClassCategory = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVClass {
    pub class_name: *const ::std::os::raw::c_char,
    pub item_name: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char,
    >,
    pub option: *const AVOption,
    pub version: ::std::os::raw::c_int,
    pub log_level_offset_offset: ::std::os::raw::c_int,
    pub parent_log_context_offset: ::std::os::raw::c_int,
    pub child_next: ::std::option::Option<
        unsafe extern "C" fn(
            obj: *mut ::std::os::raw::c_void,
            prev: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub child_class_next:
        ::std::option::Option<unsafe extern "C" fn(prev: *const AVClass) -> *const AVClass>,
    pub category: AVClassCategory,
    pub get_category: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void) -> AVClassCategory,
    >,
    pub query_ranges: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut *mut AVOptionRanges,
            obj: *mut ::std::os::raw::c_void,
            key: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
}
extern "C" {
    pub fn av_log(
        avcl: *mut ::std::os::raw::c_void,
        level: ::std::os::raw::c_int,
        fmt: *const ::std::os::raw::c_char,
        ...
    );
}
extern "C" {
    pub fn av_vlog(
        avcl: *mut ::std::os::raw::c_void,
        level: ::std::os::raw::c_int,
        fmt: *const ::std::os::raw::c_char,
        vl: *mut __va_list_tag,
    );
}
extern "C" {
    pub fn av_log_get_level() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_log_set_level(level: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_log_set_callback(
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: ::std::os::raw::c_int,
                arg3: *const ::std::os::raw::c_char,
                arg4: *mut __va_list_tag,
            ),
        >,
    );
}
extern "C" {
    pub fn av_log_default_callback(
        avcl: *mut ::std::os::raw::c_void,
        level: ::std::os::raw::c_int,
        fmt: *const ::std::os::raw::c_char,
        vl: *mut __va_list_tag,
    );
}
extern "C" {
    pub fn av_default_item_name(ctx: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_default_get_category(ptr: *mut ::std::os::raw::c_void) -> AVClassCategory;
}
extern "C" {
    pub fn av_log_format_line(
        ptr: *mut ::std::os::raw::c_void,
        level: ::std::os::raw::c_int,
        fmt: *const ::std::os::raw::c_char,
        vl: *mut __va_list_tag,
        line: *mut ::std::os::raw::c_char,
        line_size: ::std::os::raw::c_int,
        print_prefix: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_log_format_line2(
        ptr: *mut ::std::os::raw::c_void,
        level: ::std::os::raw::c_int,
        fmt: *const ::std::os::raw::c_char,
        vl: *mut __va_list_tag,
        line: *mut ::std::os::raw::c_char,
        line_size: ::std::os::raw::c_int,
        print_prefix: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_log_set_flags(arg: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_log_get_flags() -> ::std::os::raw::c_int;
}
pub const AVPixelFormat_AV_PIX_FMT_NONE: AVPixelFormat = -1;
pub const AVPixelFormat_AV_PIX_FMT_YUV420P: AVPixelFormat = 0;
pub const AVPixelFormat_AV_PIX_FMT_YUYV422: AVPixelFormat = 1;
pub const AVPixelFormat_AV_PIX_FMT_RGB24: AVPixelFormat = 2;
pub const AVPixelFormat_AV_PIX_FMT_BGR24: AVPixelFormat = 3;
pub const AVPixelFormat_AV_PIX_FMT_YUV422P: AVPixelFormat = 4;
pub const AVPixelFormat_AV_PIX_FMT_YUV444P: AVPixelFormat = 5;
pub const AVPixelFormat_AV_PIX_FMT_YUV410P: AVPixelFormat = 6;
pub const AVPixelFormat_AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
pub const AVPixelFormat_AV_PIX_FMT_GRAY8: AVPixelFormat = 8;
pub const AVPixelFormat_AV_PIX_FMT_MONOWHITE: AVPixelFormat = 9;
pub const AVPixelFormat_AV_PIX_FMT_MONOBLACK: AVPixelFormat = 10;
pub const AVPixelFormat_AV_PIX_FMT_PAL8: AVPixelFormat = 11;
pub const AVPixelFormat_AV_PIX_FMT_YUVJ420P: AVPixelFormat = 12;
pub const AVPixelFormat_AV_PIX_FMT_YUVJ422P: AVPixelFormat = 13;
pub const AVPixelFormat_AV_PIX_FMT_YUVJ444P: AVPixelFormat = 14;
pub const AVPixelFormat_AV_PIX_FMT_UYVY422: AVPixelFormat = 15;
pub const AVPixelFormat_AV_PIX_FMT_UYYVYY411: AVPixelFormat = 16;
pub const AVPixelFormat_AV_PIX_FMT_BGR8: AVPixelFormat = 17;
pub const AVPixelFormat_AV_PIX_FMT_BGR4: AVPixelFormat = 18;
pub const AVPixelFormat_AV_PIX_FMT_BGR4_BYTE: AVPixelFormat = 19;
pub const AVPixelFormat_AV_PIX_FMT_RGB8: AVPixelFormat = 20;
pub const AVPixelFormat_AV_PIX_FMT_RGB4: AVPixelFormat = 21;
pub const AVPixelFormat_AV_PIX_FMT_RGB4_BYTE: AVPixelFormat = 22;
pub const AVPixelFormat_AV_PIX_FMT_NV12: AVPixelFormat = 23;
pub const AVPixelFormat_AV_PIX_FMT_NV21: AVPixelFormat = 24;
pub const AVPixelFormat_AV_PIX_FMT_ARGB: AVPixelFormat = 25;
pub const AVPixelFormat_AV_PIX_FMT_RGBA: AVPixelFormat = 26;
pub const AVPixelFormat_AV_PIX_FMT_ABGR: AVPixelFormat = 27;
pub const AVPixelFormat_AV_PIX_FMT_BGRA: AVPixelFormat = 28;
pub const AVPixelFormat_AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29;
pub const AVPixelFormat_AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30;
pub const AVPixelFormat_AV_PIX_FMT_YUV440P: AVPixelFormat = 31;
pub const AVPixelFormat_AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32;
pub const AVPixelFormat_AV_PIX_FMT_YUVA420P: AVPixelFormat = 33;
pub const AVPixelFormat_AV_PIX_FMT_RGB48BE: AVPixelFormat = 34;
pub const AVPixelFormat_AV_PIX_FMT_RGB48LE: AVPixelFormat = 35;
pub const AVPixelFormat_AV_PIX_FMT_RGB565BE: AVPixelFormat = 36;
pub const AVPixelFormat_AV_PIX_FMT_RGB565LE: AVPixelFormat = 37;
pub const AVPixelFormat_AV_PIX_FMT_RGB555BE: AVPixelFormat = 38;
pub const AVPixelFormat_AV_PIX_FMT_RGB555LE: AVPixelFormat = 39;
pub const AVPixelFormat_AV_PIX_FMT_BGR565BE: AVPixelFormat = 40;
pub const AVPixelFormat_AV_PIX_FMT_BGR565LE: AVPixelFormat = 41;
pub const AVPixelFormat_AV_PIX_FMT_BGR555BE: AVPixelFormat = 42;
pub const AVPixelFormat_AV_PIX_FMT_BGR555LE: AVPixelFormat = 43;
pub const AVPixelFormat_AV_PIX_FMT_VAAPI_MOCO: AVPixelFormat = 44;
pub const AVPixelFormat_AV_PIX_FMT_VAAPI_IDCT: AVPixelFormat = 45;
pub const AVPixelFormat_AV_PIX_FMT_VAAPI_VLD: AVPixelFormat = 46;
pub const AVPixelFormat_AV_PIX_FMT_VAAPI: AVPixelFormat = 46;
pub const AVPixelFormat_AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 47;
pub const AVPixelFormat_AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 48;
pub const AVPixelFormat_AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 49;
pub const AVPixelFormat_AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 50;
pub const AVPixelFormat_AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 51;
pub const AVPixelFormat_AV_PIX_FMT_YUV444P16BE: AVPixelFormat = 52;
pub const AVPixelFormat_AV_PIX_FMT_DXVA2_VLD: AVPixelFormat = 53;
pub const AVPixelFormat_AV_PIX_FMT_RGB444LE: AVPixelFormat = 54;
pub const AVPixelFormat_AV_PIX_FMT_RGB444BE: AVPixelFormat = 55;
pub const AVPixelFormat_AV_PIX_FMT_BGR444LE: AVPixelFormat = 56;
pub const AVPixelFormat_AV_PIX_FMT_BGR444BE: AVPixelFormat = 57;
pub const AVPixelFormat_AV_PIX_FMT_YA8: AVPixelFormat = 58;
pub const AVPixelFormat_AV_PIX_FMT_Y400A: AVPixelFormat = 58;
pub const AVPixelFormat_AV_PIX_FMT_GRAY8A: AVPixelFormat = 58;
pub const AVPixelFormat_AV_PIX_FMT_BGR48BE: AVPixelFormat = 59;
pub const AVPixelFormat_AV_PIX_FMT_BGR48LE: AVPixelFormat = 60;
pub const AVPixelFormat_AV_PIX_FMT_YUV420P9BE: AVPixelFormat = 61;
pub const AVPixelFormat_AV_PIX_FMT_YUV420P9LE: AVPixelFormat = 62;
pub const AVPixelFormat_AV_PIX_FMT_YUV420P10BE: AVPixelFormat = 63;
pub const AVPixelFormat_AV_PIX_FMT_YUV420P10LE: AVPixelFormat = 64;
pub const AVPixelFormat_AV_PIX_FMT_YUV422P10BE: AVPixelFormat = 65;
pub const AVPixelFormat_AV_PIX_FMT_YUV422P10LE: AVPixelFormat = 66;
pub const AVPixelFormat_AV_PIX_FMT_YUV444P9BE: AVPixelFormat = 67;
pub const AVPixelFormat_AV_PIX_FMT_YUV444P9LE: AVPixelFormat = 68;
pub const AVPixelFormat_AV_PIX_FMT_YUV444P10BE: AVPixelFormat = 69;
pub const AVPixelFormat_AV_PIX_FMT_YUV444P10LE: AVPixelFormat = 70;
pub const AVPixelFormat_AV_PIX_FMT_YUV422P9BE: AVPixelFormat = 71;
pub const AVPixelFormat_AV_PIX_FMT_YUV422P9LE: AVPixelFormat = 72;
pub const AVPixelFormat_AV_PIX_FMT_GBRP: AVPixelFormat = 73;
pub const AVPixelFormat_AV_PIX_FMT_GBR24P: AVPixelFormat = 73;
pub const AVPixelFormat_AV_PIX_FMT_GBRP9BE: AVPixelFormat = 74;
pub const AVPixelFormat_AV_PIX_FMT_GBRP9LE: AVPixelFormat = 75;
pub const AVPixelFormat_AV_PIX_FMT_GBRP10BE: AVPixelFormat = 76;
pub const AVPixelFormat_AV_PIX_FMT_GBRP10LE: AVPixelFormat = 77;
pub const AVPixelFormat_AV_PIX_FMT_GBRP16BE: AVPixelFormat = 78;
pub const AVPixelFormat_AV_PIX_FMT_GBRP16LE: AVPixelFormat = 79;
pub const AVPixelFormat_AV_PIX_FMT_YUVA422P: AVPixelFormat = 80;
pub const AVPixelFormat_AV_PIX_FMT_YUVA444P: AVPixelFormat = 81;
pub const AVPixelFormat_AV_PIX_FMT_YUVA420P9BE: AVPixelFormat = 82;
pub const AVPixelFormat_AV_PIX_FMT_YUVA420P9LE: AVPixelFormat = 83;
pub const AVPixelFormat_AV_PIX_FMT_YUVA422P9BE: AVPixelFormat = 84;
pub const AVPixelFormat_AV_PIX_FMT_YUVA422P9LE: AVPixelFormat = 85;
pub const AVPixelFormat_AV_PIX_FMT_YUVA444P9BE: AVPixelFormat = 86;
pub const AVPixelFormat_AV_PIX_FMT_YUVA444P9LE: AVPixelFormat = 87;
pub const AVPixelFormat_AV_PIX_FMT_YUVA420P10BE: AVPixelFormat = 88;
pub const AVPixelFormat_AV_PIX_FMT_YUVA420P10LE: AVPixelFormat = 89;
pub const AVPixelFormat_AV_PIX_FMT_YUVA422P10BE: AVPixelFormat = 90;
pub const AVPixelFormat_AV_PIX_FMT_YUVA422P10LE: AVPixelFormat = 91;
pub const AVPixelFormat_AV_PIX_FMT_YUVA444P10BE: AVPixelFormat = 92;
pub const AVPixelFormat_AV_PIX_FMT_YUVA444P10LE: AVPixelFormat = 93;
pub const AVPixelFormat_AV_PIX_FMT_YUVA420P16BE: AVPixelFormat = 94;
pub const AVPixelFormat_AV_PIX_FMT_YUVA420P16LE: AVPixelFormat = 95;
pub const AVPixelFormat_AV_PIX_FMT_YUVA422P16BE: AVPixelFormat = 96;
pub const AVPixelFormat_AV_PIX_FMT_YUVA422P16LE: AVPixelFormat = 97;
pub const AVPixelFormat_AV_PIX_FMT_YUVA444P16BE: AVPixelFormat = 98;
pub const AVPixelFormat_AV_PIX_FMT_YUVA444P16LE: AVPixelFormat = 99;
pub const AVPixelFormat_AV_PIX_FMT_VDPAU: AVPixelFormat = 100;
pub const AVPixelFormat_AV_PIX_FMT_XYZ12LE: AVPixelFormat = 101;
pub const AVPixelFormat_AV_PIX_FMT_XYZ12BE: AVPixelFormat = 102;
pub const AVPixelFormat_AV_PIX_FMT_NV16: AVPixelFormat = 103;
pub const AVPixelFormat_AV_PIX_FMT_NV20LE: AVPixelFormat = 104;
pub const AVPixelFormat_AV_PIX_FMT_NV20BE: AVPixelFormat = 105;
pub const AVPixelFormat_AV_PIX_FMT_RGBA64BE: AVPixelFormat = 106;
pub const AVPixelFormat_AV_PIX_FMT_RGBA64LE: AVPixelFormat = 107;
pub const AVPixelFormat_AV_PIX_FMT_BGRA64BE: AVPixelFormat = 108;
pub const AVPixelFormat_AV_PIX_FMT_BGRA64LE: AVPixelFormat = 109;
pub const AVPixelFormat_AV_PIX_FMT_YVYU422: AVPixelFormat = 110;
pub const AVPixelFormat_AV_PIX_FMT_YA16BE: AVPixelFormat = 111;
pub const AVPixelFormat_AV_PIX_FMT_YA16LE: AVPixelFormat = 112;
pub const AVPixelFormat_AV_PIX_FMT_GBRAP: AVPixelFormat = 113;
pub const AVPixelFormat_AV_PIX_FMT_GBRAP16BE: AVPixelFormat = 114;
pub const AVPixelFormat_AV_PIX_FMT_GBRAP16LE: AVPixelFormat = 115;
pub const AVPixelFormat_AV_PIX_FMT_QSV: AVPixelFormat = 116;
pub const AVPixelFormat_AV_PIX_FMT_MMAL: AVPixelFormat = 117;
pub const AVPixelFormat_AV_PIX_FMT_D3D11VA_VLD: AVPixelFormat = 118;
pub const AVPixelFormat_AV_PIX_FMT_CUDA: AVPixelFormat = 119;
pub const AVPixelFormat_AV_PIX_FMT_0RGB: AVPixelFormat = 120;
pub const AVPixelFormat_AV_PIX_FMT_RGB0: AVPixelFormat = 121;
pub const AVPixelFormat_AV_PIX_FMT_0BGR: AVPixelFormat = 122;
pub const AVPixelFormat_AV_PIX_FMT_BGR0: AVPixelFormat = 123;
pub const AVPixelFormat_AV_PIX_FMT_YUV420P12BE: AVPixelFormat = 124;
pub const AVPixelFormat_AV_PIX_FMT_YUV420P12LE: AVPixelFormat = 125;
pub const AVPixelFormat_AV_PIX_FMT_YUV420P14BE: AVPixelFormat = 126;
pub const AVPixelFormat_AV_PIX_FMT_YUV420P14LE: AVPixelFormat = 127;
pub const AVPixelFormat_AV_PIX_FMT_YUV422P12BE: AVPixelFormat = 128;
pub const AVPixelFormat_AV_PIX_FMT_YUV422P12LE: AVPixelFormat = 129;
pub const AVPixelFormat_AV_PIX_FMT_YUV422P14BE: AVPixelFormat = 130;
pub const AVPixelFormat_AV_PIX_FMT_YUV422P14LE: AVPixelFormat = 131;
pub const AVPixelFormat_AV_PIX_FMT_YUV444P12BE: AVPixelFormat = 132;
pub const AVPixelFormat_AV_PIX_FMT_YUV444P12LE: AVPixelFormat = 133;
pub const AVPixelFormat_AV_PIX_FMT_YUV444P14BE: AVPixelFormat = 134;
pub const AVPixelFormat_AV_PIX_FMT_YUV444P14LE: AVPixelFormat = 135;
pub const AVPixelFormat_AV_PIX_FMT_GBRP12BE: AVPixelFormat = 136;
pub const AVPixelFormat_AV_PIX_FMT_GBRP12LE: AVPixelFormat = 137;
pub const AVPixelFormat_AV_PIX_FMT_GBRP14BE: AVPixelFormat = 138;
pub const AVPixelFormat_AV_PIX_FMT_GBRP14LE: AVPixelFormat = 139;
pub const AVPixelFormat_AV_PIX_FMT_YUVJ411P: AVPixelFormat = 140;
pub const AVPixelFormat_AV_PIX_FMT_BAYER_BGGR8: AVPixelFormat = 141;
pub const AVPixelFormat_AV_PIX_FMT_BAYER_RGGB8: AVPixelFormat = 142;
pub const AVPixelFormat_AV_PIX_FMT_BAYER_GBRG8: AVPixelFormat = 143;
pub const AVPixelFormat_AV_PIX_FMT_BAYER_GRBG8: AVPixelFormat = 144;
pub const AVPixelFormat_AV_PIX_FMT_BAYER_BGGR16LE: AVPixelFormat = 145;
pub const AVPixelFormat_AV_PIX_FMT_BAYER_BGGR16BE: AVPixelFormat = 146;
pub const AVPixelFormat_AV_PIX_FMT_BAYER_RGGB16LE: AVPixelFormat = 147;
pub const AVPixelFormat_AV_PIX_FMT_BAYER_RGGB16BE: AVPixelFormat = 148;
pub const AVPixelFormat_AV_PIX_FMT_BAYER_GBRG16LE: AVPixelFormat = 149;
pub const AVPixelFormat_AV_PIX_FMT_BAYER_GBRG16BE: AVPixelFormat = 150;
pub const AVPixelFormat_AV_PIX_FMT_BAYER_GRBG16LE: AVPixelFormat = 151;
pub const AVPixelFormat_AV_PIX_FMT_BAYER_GRBG16BE: AVPixelFormat = 152;
pub const AVPixelFormat_AV_PIX_FMT_XVMC: AVPixelFormat = 153;
pub const AVPixelFormat_AV_PIX_FMT_YUV440P10LE: AVPixelFormat = 154;
pub const AVPixelFormat_AV_PIX_FMT_YUV440P10BE: AVPixelFormat = 155;
pub const AVPixelFormat_AV_PIX_FMT_YUV440P12LE: AVPixelFormat = 156;
pub const AVPixelFormat_AV_PIX_FMT_YUV440P12BE: AVPixelFormat = 157;
pub const AVPixelFormat_AV_PIX_FMT_AYUV64LE: AVPixelFormat = 158;
pub const AVPixelFormat_AV_PIX_FMT_AYUV64BE: AVPixelFormat = 159;
pub const AVPixelFormat_AV_PIX_FMT_VIDEOTOOLBOX: AVPixelFormat = 160;
pub const AVPixelFormat_AV_PIX_FMT_P010LE: AVPixelFormat = 161;
pub const AVPixelFormat_AV_PIX_FMT_P010BE: AVPixelFormat = 162;
pub const AVPixelFormat_AV_PIX_FMT_GBRAP12BE: AVPixelFormat = 163;
pub const AVPixelFormat_AV_PIX_FMT_GBRAP12LE: AVPixelFormat = 164;
pub const AVPixelFormat_AV_PIX_FMT_GBRAP10BE: AVPixelFormat = 165;
pub const AVPixelFormat_AV_PIX_FMT_GBRAP10LE: AVPixelFormat = 166;
pub const AVPixelFormat_AV_PIX_FMT_MEDIACODEC: AVPixelFormat = 167;
pub const AVPixelFormat_AV_PIX_FMT_GRAY12BE: AVPixelFormat = 168;
pub const AVPixelFormat_AV_PIX_FMT_GRAY12LE: AVPixelFormat = 169;
pub const AVPixelFormat_AV_PIX_FMT_GRAY10BE: AVPixelFormat = 170;
pub const AVPixelFormat_AV_PIX_FMT_GRAY10LE: AVPixelFormat = 171;
pub const AVPixelFormat_AV_PIX_FMT_P016LE: AVPixelFormat = 172;
pub const AVPixelFormat_AV_PIX_FMT_P016BE: AVPixelFormat = 173;
pub const AVPixelFormat_AV_PIX_FMT_D3D11: AVPixelFormat = 174;
pub const AVPixelFormat_AV_PIX_FMT_GRAY9BE: AVPixelFormat = 175;
pub const AVPixelFormat_AV_PIX_FMT_GRAY9LE: AVPixelFormat = 176;
pub const AVPixelFormat_AV_PIX_FMT_GBRPF32BE: AVPixelFormat = 177;
pub const AVPixelFormat_AV_PIX_FMT_GBRPF32LE: AVPixelFormat = 178;
pub const AVPixelFormat_AV_PIX_FMT_GBRAPF32BE: AVPixelFormat = 179;
pub const AVPixelFormat_AV_PIX_FMT_GBRAPF32LE: AVPixelFormat = 180;
pub const AVPixelFormat_AV_PIX_FMT_DRM_PRIME: AVPixelFormat = 181;
pub const AVPixelFormat_AV_PIX_FMT_OPENCL: AVPixelFormat = 182;
pub const AVPixelFormat_AV_PIX_FMT_GRAY14BE: AVPixelFormat = 183;
pub const AVPixelFormat_AV_PIX_FMT_GRAY14LE: AVPixelFormat = 184;
pub const AVPixelFormat_AV_PIX_FMT_GRAYF32BE: AVPixelFormat = 185;
pub const AVPixelFormat_AV_PIX_FMT_GRAYF32LE: AVPixelFormat = 186;
pub const AVPixelFormat_AV_PIX_FMT_YUVA422P12BE: AVPixelFormat = 187;
pub const AVPixelFormat_AV_PIX_FMT_YUVA422P12LE: AVPixelFormat = 188;
pub const AVPixelFormat_AV_PIX_FMT_YUVA444P12BE: AVPixelFormat = 189;
pub const AVPixelFormat_AV_PIX_FMT_YUVA444P12LE: AVPixelFormat = 190;
pub const AVPixelFormat_AV_PIX_FMT_NV24: AVPixelFormat = 191;
pub const AVPixelFormat_AV_PIX_FMT_NV42: AVPixelFormat = 192;
pub const AVPixelFormat_AV_PIX_FMT_NB: AVPixelFormat = 193;
pub type AVPixelFormat = i32;
pub const AVColorPrimaries_AVCOL_PRI_RESERVED0: AVColorPrimaries = 0;
pub const AVColorPrimaries_AVCOL_PRI_BT709: AVColorPrimaries = 1;
pub const AVColorPrimaries_AVCOL_PRI_UNSPECIFIED: AVColorPrimaries = 2;
pub const AVColorPrimaries_AVCOL_PRI_RESERVED: AVColorPrimaries = 3;
pub const AVColorPrimaries_AVCOL_PRI_BT470M: AVColorPrimaries = 4;
pub const AVColorPrimaries_AVCOL_PRI_BT470BG: AVColorPrimaries = 5;
pub const AVColorPrimaries_AVCOL_PRI_SMPTE170M: AVColorPrimaries = 6;
pub const AVColorPrimaries_AVCOL_PRI_SMPTE240M: AVColorPrimaries = 7;
pub const AVColorPrimaries_AVCOL_PRI_FILM: AVColorPrimaries = 8;
pub const AVColorPrimaries_AVCOL_PRI_BT2020: AVColorPrimaries = 9;
pub const AVColorPrimaries_AVCOL_PRI_SMPTE428: AVColorPrimaries = 10;
pub const AVColorPrimaries_AVCOL_PRI_SMPTEST428_1: AVColorPrimaries = 10;
pub const AVColorPrimaries_AVCOL_PRI_SMPTE431: AVColorPrimaries = 11;
pub const AVColorPrimaries_AVCOL_PRI_SMPTE432: AVColorPrimaries = 12;
pub const AVColorPrimaries_AVCOL_PRI_JEDEC_P22: AVColorPrimaries = 22;
pub const AVColorPrimaries_AVCOL_PRI_NB: AVColorPrimaries = 23;
pub type AVColorPrimaries = u32;
pub const AVColorTransferCharacteristic_AVCOL_TRC_RESERVED0: AVColorTransferCharacteristic = 0;
pub const AVColorTransferCharacteristic_AVCOL_TRC_BT709: AVColorTransferCharacteristic = 1;
pub const AVColorTransferCharacteristic_AVCOL_TRC_UNSPECIFIED: AVColorTransferCharacteristic = 2;
pub const AVColorTransferCharacteristic_AVCOL_TRC_RESERVED: AVColorTransferCharacteristic = 3;
pub const AVColorTransferCharacteristic_AVCOL_TRC_GAMMA22: AVColorTransferCharacteristic = 4;
pub const AVColorTransferCharacteristic_AVCOL_TRC_GAMMA28: AVColorTransferCharacteristic = 5;
pub const AVColorTransferCharacteristic_AVCOL_TRC_SMPTE170M: AVColorTransferCharacteristic = 6;
pub const AVColorTransferCharacteristic_AVCOL_TRC_SMPTE240M: AVColorTransferCharacteristic = 7;
pub const AVColorTransferCharacteristic_AVCOL_TRC_LINEAR: AVColorTransferCharacteristic = 8;
pub const AVColorTransferCharacteristic_AVCOL_TRC_LOG: AVColorTransferCharacteristic = 9;
pub const AVColorTransferCharacteristic_AVCOL_TRC_LOG_SQRT: AVColorTransferCharacteristic = 10;
pub const AVColorTransferCharacteristic_AVCOL_TRC_IEC61966_2_4: AVColorTransferCharacteristic = 11;
pub const AVColorTransferCharacteristic_AVCOL_TRC_BT1361_ECG: AVColorTransferCharacteristic = 12;
pub const AVColorTransferCharacteristic_AVCOL_TRC_IEC61966_2_1: AVColorTransferCharacteristic = 13;
pub const AVColorTransferCharacteristic_AVCOL_TRC_BT2020_10: AVColorTransferCharacteristic = 14;
pub const AVColorTransferCharacteristic_AVCOL_TRC_BT2020_12: AVColorTransferCharacteristic = 15;
pub const AVColorTransferCharacteristic_AVCOL_TRC_SMPTE2084: AVColorTransferCharacteristic = 16;
pub const AVColorTransferCharacteristic_AVCOL_TRC_SMPTEST2084: AVColorTransferCharacteristic = 16;
pub const AVColorTransferCharacteristic_AVCOL_TRC_SMPTE428: AVColorTransferCharacteristic = 17;
pub const AVColorTransferCharacteristic_AVCOL_TRC_SMPTEST428_1: AVColorTransferCharacteristic = 17;
pub const AVColorTransferCharacteristic_AVCOL_TRC_ARIB_STD_B67: AVColorTransferCharacteristic = 18;
pub const AVColorTransferCharacteristic_AVCOL_TRC_NB: AVColorTransferCharacteristic = 19;
pub type AVColorTransferCharacteristic = u32;
pub const AVColorSpace_AVCOL_SPC_RGB: AVColorSpace = 0;
pub const AVColorSpace_AVCOL_SPC_BT709: AVColorSpace = 1;
pub const AVColorSpace_AVCOL_SPC_UNSPECIFIED: AVColorSpace = 2;
pub const AVColorSpace_AVCOL_SPC_RESERVED: AVColorSpace = 3;
pub const AVColorSpace_AVCOL_SPC_FCC: AVColorSpace = 4;
pub const AVColorSpace_AVCOL_SPC_BT470BG: AVColorSpace = 5;
pub const AVColorSpace_AVCOL_SPC_SMPTE170M: AVColorSpace = 6;
pub const AVColorSpace_AVCOL_SPC_SMPTE240M: AVColorSpace = 7;
pub const AVColorSpace_AVCOL_SPC_YCGCO: AVColorSpace = 8;
pub const AVColorSpace_AVCOL_SPC_YCOCG: AVColorSpace = 8;
pub const AVColorSpace_AVCOL_SPC_BT2020_NCL: AVColorSpace = 9;
pub const AVColorSpace_AVCOL_SPC_BT2020_CL: AVColorSpace = 10;
pub const AVColorSpace_AVCOL_SPC_SMPTE2085: AVColorSpace = 11;
pub const AVColorSpace_AVCOL_SPC_CHROMA_DERIVED_NCL: AVColorSpace = 12;
pub const AVColorSpace_AVCOL_SPC_CHROMA_DERIVED_CL: AVColorSpace = 13;
pub const AVColorSpace_AVCOL_SPC_ICTCP: AVColorSpace = 14;
pub const AVColorSpace_AVCOL_SPC_NB: AVColorSpace = 15;
pub type AVColorSpace = u32;
pub const AVColorRange_AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
pub const AVColorRange_AVCOL_RANGE_MPEG: AVColorRange = 1;
pub const AVColorRange_AVCOL_RANGE_JPEG: AVColorRange = 2;
pub const AVColorRange_AVCOL_RANGE_NB: AVColorRange = 3;
pub type AVColorRange = u32;
pub const AVChromaLocation_AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
pub const AVChromaLocation_AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVChromaLocation_AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVChromaLocation_AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVChromaLocation_AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVChromaLocation_AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVChromaLocation_AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
pub const AVChromaLocation_AVCHROMA_LOC_NB: AVChromaLocation = 7;
pub type AVChromaLocation = u32;
extern "C" {
    pub fn av_int_list_length_for_size(
        elsize: ::std::os::raw::c_uint,
        list: *const ::std::os::raw::c_void,
        term: u64,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn av_fopen_utf8(
        path: *const ::std::os::raw::c_char,
        mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn av_get_time_base_q() -> AVRational;
}
extern "C" {
    pub fn av_fourcc_make_string(
        buf: *mut ::std::os::raw::c_char,
        fourcc: u32,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_malloc(size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_mallocz(size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_malloc_array(nmemb: usize, size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_mallocz_array(nmemb: usize, size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_calloc(nmemb: usize, size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_realloc(ptr: *mut ::std::os::raw::c_void, size: usize)
        -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_reallocp(ptr: *mut ::std::os::raw::c_void, size: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_realloc_f(
        ptr: *mut ::std::os::raw::c_void,
        nelem: usize,
        elsize: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_realloc_array(
        ptr: *mut ::std::os::raw::c_void,
        nmemb: usize,
        size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_reallocp_array(
        ptr: *mut ::std::os::raw::c_void,
        nmemb: usize,
        size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_fast_realloc(
        ptr: *mut ::std::os::raw::c_void,
        size: *mut ::std::os::raw::c_uint,
        min_size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_fast_malloc(
        ptr: *mut ::std::os::raw::c_void,
        size: *mut ::std::os::raw::c_uint,
        min_size: usize,
    );
}
extern "C" {
    pub fn av_fast_mallocz(
        ptr: *mut ::std::os::raw::c_void,
        size: *mut ::std::os::raw::c_uint,
        min_size: usize,
    );
}
extern "C" {
    pub fn av_free(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn av_freep(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn av_strdup(s: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_strndup(s: *const ::std::os::raw::c_char, len: usize) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_memdup(p: *const ::std::os::raw::c_void, size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_memcpy_backptr(dst: *mut u8, back: ::std::os::raw::c_int, cnt: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_dynarray_add(
        tab_ptr: *mut ::std::os::raw::c_void,
        nb_ptr: *mut ::std::os::raw::c_int,
        elem: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn av_dynarray_add_nofree(
        tab_ptr: *mut ::std::os::raw::c_void,
        nb_ptr: *mut ::std::os::raw::c_int,
        elem: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_dynarray2_add(
        tab_ptr: *mut *mut ::std::os::raw::c_void,
        nb_ptr: *mut ::std::os::raw::c_int,
        elem_size: usize,
        elem_data: *const u8,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_max_alloc(max: usize);
}
extern "C" {
    pub fn av_display_rotation_get(matrix: *const i32) -> f64;
}
extern "C" {
    pub fn av_display_rotation_set(matrix: *mut i32, angle: f64);
}
extern "C" {
    pub fn av_display_matrix_flip(
        matrix: *mut i32,
        hflip: ::std::os::raw::c_int,
        vflip: ::std::os::raw::c_int,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVSubsampleEncryptionInfo {
    pub bytes_of_clear_data: ::std::os::raw::c_uint,
    pub bytes_of_protected_data: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVEncryptionInfo {
    pub scheme: u32,
    pub crypt_byte_block: u32,
    pub skip_byte_block: u32,
    pub key_id: *mut u8,
    pub key_id_size: u32,
    pub iv: *mut u8,
    pub iv_size: u32,
    pub subsamples: *mut AVSubsampleEncryptionInfo,
    pub subsample_count: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVEncryptionInitInfo {
    pub system_id: *mut u8,
    pub system_id_size: u32,
    pub key_ids: *mut *mut u8,
    pub num_key_ids: u32,
    pub key_id_size: u32,
    pub data: *mut u8,
    pub data_size: u32,
    pub next: *mut AVEncryptionInitInfo,
}
extern "C" {
    pub fn av_encryption_info_alloc(
        subsample_count: u32,
        key_id_size: u32,
        iv_size: u32,
    ) -> *mut AVEncryptionInfo;
}
extern "C" {
    pub fn av_encryption_info_clone(info: *const AVEncryptionInfo) -> *mut AVEncryptionInfo;
}
extern "C" {
    pub fn av_encryption_info_free(info: *mut AVEncryptionInfo);
}
extern "C" {
    pub fn av_encryption_info_get_side_data(
        side_data: *const u8,
        side_data_size: usize,
    ) -> *mut AVEncryptionInfo;
}
extern "C" {
    pub fn av_encryption_info_add_side_data(
        info: *const AVEncryptionInfo,
        side_data_size: *mut usize,
    ) -> *mut u8;
}
extern "C" {
    pub fn av_encryption_init_info_alloc(
        system_id_size: u32,
        num_key_ids: u32,
        key_id_size: u32,
        data_size: u32,
    ) -> *mut AVEncryptionInitInfo;
}
extern "C" {
    pub fn av_encryption_init_info_free(info: *mut AVEncryptionInitInfo);
}
extern "C" {
    pub fn av_encryption_init_info_get_side_data(
        side_data: *const u8,
        side_data_size: usize,
    ) -> *mut AVEncryptionInitInfo;
}
extern "C" {
    pub fn av_encryption_init_info_add_side_data(
        info: *const AVEncryptionInitInfo,
        side_data_size: *mut usize,
    ) -> *mut u8;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVExpr {
    _unused: [u8; 0],
}
extern "C" {
    pub fn av_expr_parse_and_eval(
        res: *mut f64,
        s: *const ::std::os::raw::c_char,
        const_names: *const *const ::std::os::raw::c_char,
        const_values: *const f64,
        func1_names: *const *const ::std::os::raw::c_char,
        funcs1: *const ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: f64) -> f64,
        >,
        func2_names: *const *const ::std::os::raw::c_char,
        funcs2: *const ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: f64, arg3: f64) -> f64,
        >,
        opaque: *mut ::std::os::raw::c_void,
        log_offset: ::std::os::raw::c_int,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_expr_parse(
        expr: *mut *mut AVExpr,
        s: *const ::std::os::raw::c_char,
        const_names: *const *const ::std::os::raw::c_char,
        func1_names: *const *const ::std::os::raw::c_char,
        funcs1: *const ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: f64) -> f64,
        >,
        func2_names: *const *const ::std::os::raw::c_char,
        funcs2: *const ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: f64, arg3: f64) -> f64,
        >,
        log_offset: ::std::os::raw::c_int,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_expr_eval(
        e: *mut AVExpr,
        const_values: *const f64,
        opaque: *mut ::std::os::raw::c_void,
    ) -> f64;
}
extern "C" {
    pub fn av_expr_free(e: *mut AVExpr);
}
extern "C" {
    pub fn av_strtod(
        numstr: *const ::std::os::raw::c_char,
        tail: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn av_file_map(
        filename: *const ::std::os::raw::c_char,
        bufptr: *mut *mut u8,
        size: *mut usize,
        log_offset: ::std::os::raw::c_int,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_file_unmap(bufptr: *mut u8, size: usize);
}
extern "C" {
    pub fn av_tempfile(
        prefix: *const ::std::os::raw::c_char,
        filename: *mut *mut ::std::os::raw::c_char,
        log_offset: ::std::os::raw::c_int,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVBuffer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut u8,
    pub size: ::std::os::raw::c_int,
}
extern "C" {
    pub fn av_buffer_alloc(size: ::std::os::raw::c_int) -> *mut AVBufferRef;
}
extern "C" {
    pub fn av_buffer_allocz(size: ::std::os::raw::c_int) -> *mut AVBufferRef;
}
extern "C" {
    pub fn av_buffer_create(
        data: *mut u8,
        size: ::std::os::raw::c_int,
        free: ::std::option::Option<
            unsafe extern "C" fn(opaque: *mut ::std::os::raw::c_void, data: *mut u8),
        >,
        opaque: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_int,
    ) -> *mut AVBufferRef;
}
extern "C" {
    pub fn av_buffer_default_free(opaque: *mut ::std::os::raw::c_void, data: *mut u8);
}
extern "C" {
    pub fn av_buffer_ref(buf: *mut AVBufferRef) -> *mut AVBufferRef;
}
extern "C" {
    pub fn av_buffer_unref(buf: *mut *mut AVBufferRef);
}
extern "C" {
    pub fn av_buffer_is_writable(buf: *const AVBufferRef) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_buffer_get_opaque(buf: *const AVBufferRef) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_buffer_get_ref_count(buf: *const AVBufferRef) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_buffer_make_writable(buf: *mut *mut AVBufferRef) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_buffer_realloc(
        buf: *mut *mut AVBufferRef,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVBufferPool {
    _unused: [u8; 0],
}
extern "C" {
    pub fn av_buffer_pool_init(
        size: ::std::os::raw::c_int,
        alloc: ::std::option::Option<
            unsafe extern "C" fn(size: ::std::os::raw::c_int) -> *mut AVBufferRef,
        >,
    ) -> *mut AVBufferPool;
}
extern "C" {
    pub fn av_buffer_pool_init2(
        size: ::std::os::raw::c_int,
        opaque: *mut ::std::os::raw::c_void,
        alloc: ::std::option::Option<
            unsafe extern "C" fn(
                opaque: *mut ::std::os::raw::c_void,
                size: ::std::os::raw::c_int,
            ) -> *mut AVBufferRef,
        >,
        pool_free: ::std::option::Option<unsafe extern "C" fn(opaque: *mut ::std::os::raw::c_void)>,
    ) -> *mut AVBufferPool;
}
extern "C" {
    pub fn av_buffer_pool_uninit(pool: *mut *mut AVBufferPool);
}
extern "C" {
    pub fn av_buffer_pool_get(pool: *mut AVBufferPool) -> *mut AVBufferRef;
}
pub const AVSampleFormat_AV_SAMPLE_FMT_NONE: AVSampleFormat = -1;
pub const AVSampleFormat_AV_SAMPLE_FMT_U8: AVSampleFormat = 0;
pub const AVSampleFormat_AV_SAMPLE_FMT_S16: AVSampleFormat = 1;
pub const AVSampleFormat_AV_SAMPLE_FMT_S32: AVSampleFormat = 2;
pub const AVSampleFormat_AV_SAMPLE_FMT_FLT: AVSampleFormat = 3;
pub const AVSampleFormat_AV_SAMPLE_FMT_DBL: AVSampleFormat = 4;
pub const AVSampleFormat_AV_SAMPLE_FMT_U8P: AVSampleFormat = 5;
pub const AVSampleFormat_AV_SAMPLE_FMT_S16P: AVSampleFormat = 6;
pub const AVSampleFormat_AV_SAMPLE_FMT_S32P: AVSampleFormat = 7;
pub const AVSampleFormat_AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;
pub const AVSampleFormat_AV_SAMPLE_FMT_DBLP: AVSampleFormat = 9;
pub const AVSampleFormat_AV_SAMPLE_FMT_S64: AVSampleFormat = 10;
pub const AVSampleFormat_AV_SAMPLE_FMT_S64P: AVSampleFormat = 11;
pub const AVSampleFormat_AV_SAMPLE_FMT_NB: AVSampleFormat = 12;
pub type AVSampleFormat = i32;
extern "C" {
    pub fn av_get_sample_fmt_name(sample_fmt: AVSampleFormat) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_get_sample_fmt(name: *const ::std::os::raw::c_char) -> AVSampleFormat;
}
extern "C" {
    pub fn av_get_alt_sample_fmt(
        sample_fmt: AVSampleFormat,
        planar: ::std::os::raw::c_int,
    ) -> AVSampleFormat;
}
extern "C" {
    pub fn av_get_packed_sample_fmt(sample_fmt: AVSampleFormat) -> AVSampleFormat;
}
extern "C" {
    pub fn av_get_planar_sample_fmt(sample_fmt: AVSampleFormat) -> AVSampleFormat;
}
extern "C" {
    pub fn av_get_sample_fmt_string(
        buf: *mut ::std::os::raw::c_char,
        buf_size: ::std::os::raw::c_int,
        sample_fmt: AVSampleFormat,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_get_bytes_per_sample(sample_fmt: AVSampleFormat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_sample_fmt_is_planar(sample_fmt: AVSampleFormat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_samples_get_buffer_size(
        linesize: *mut ::std::os::raw::c_int,
        nb_channels: ::std::os::raw::c_int,
        nb_samples: ::std::os::raw::c_int,
        sample_fmt: AVSampleFormat,
        align: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_samples_fill_arrays(
        audio_data: *mut *mut u8,
        linesize: *mut ::std::os::raw::c_int,
        buf: *const u8,
        nb_channels: ::std::os::raw::c_int,
        nb_samples: ::std::os::raw::c_int,
        sample_fmt: AVSampleFormat,
        align: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_samples_alloc(
        audio_data: *mut *mut u8,
        linesize: *mut ::std::os::raw::c_int,
        nb_channels: ::std::os::raw::c_int,
        nb_samples: ::std::os::raw::c_int,
        sample_fmt: AVSampleFormat,
        align: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_samples_alloc_array_and_samples(
        audio_data: *mut *mut *mut u8,
        linesize: *mut ::std::os::raw::c_int,
        nb_channels: ::std::os::raw::c_int,
        nb_samples: ::std::os::raw::c_int,
        sample_fmt: AVSampleFormat,
        align: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_samples_copy(
        dst: *mut *mut u8,
        src: *const *mut u8,
        dst_offset: ::std::os::raw::c_int,
        src_offset: ::std::os::raw::c_int,
        nb_samples: ::std::os::raw::c_int,
        nb_channels: ::std::os::raw::c_int,
        sample_fmt: AVSampleFormat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_samples_set_silence(
        audio_data: *mut *mut u8,
        offset: ::std::os::raw::c_int,
        nb_samples: ::std::os::raw::c_int,
        nb_channels: ::std::os::raw::c_int,
        sample_fmt: AVSampleFormat,
    ) -> ::std::os::raw::c_int;
}
pub const AVFrameSideDataType_AV_FRAME_DATA_PANSCAN: AVFrameSideDataType = 0;
pub const AVFrameSideDataType_AV_FRAME_DATA_A53_CC: AVFrameSideDataType = 1;
pub const AVFrameSideDataType_AV_FRAME_DATA_STEREO3D: AVFrameSideDataType = 2;
pub const AVFrameSideDataType_AV_FRAME_DATA_MATRIXENCODING: AVFrameSideDataType = 3;
pub const AVFrameSideDataType_AV_FRAME_DATA_DOWNMIX_INFO: AVFrameSideDataType = 4;
pub const AVFrameSideDataType_AV_FRAME_DATA_REPLAYGAIN: AVFrameSideDataType = 5;
pub const AVFrameSideDataType_AV_FRAME_DATA_DISPLAYMATRIX: AVFrameSideDataType = 6;
pub const AVFrameSideDataType_AV_FRAME_DATA_AFD: AVFrameSideDataType = 7;
pub const AVFrameSideDataType_AV_FRAME_DATA_MOTION_VECTORS: AVFrameSideDataType = 8;
pub const AVFrameSideDataType_AV_FRAME_DATA_SKIP_SAMPLES: AVFrameSideDataType = 9;
pub const AVFrameSideDataType_AV_FRAME_DATA_AUDIO_SERVICE_TYPE: AVFrameSideDataType = 10;
pub const AVFrameSideDataType_AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: AVFrameSideDataType = 11;
pub const AVFrameSideDataType_AV_FRAME_DATA_GOP_TIMECODE: AVFrameSideDataType = 12;
pub const AVFrameSideDataType_AV_FRAME_DATA_SPHERICAL: AVFrameSideDataType = 13;
pub const AVFrameSideDataType_AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: AVFrameSideDataType = 14;
pub const AVFrameSideDataType_AV_FRAME_DATA_ICC_PROFILE: AVFrameSideDataType = 15;
pub const AVFrameSideDataType_AV_FRAME_DATA_QP_TABLE_PROPERTIES: AVFrameSideDataType = 16;
pub const AVFrameSideDataType_AV_FRAME_DATA_QP_TABLE_DATA: AVFrameSideDataType = 17;
pub const AVFrameSideDataType_AV_FRAME_DATA_S12M_TIMECODE: AVFrameSideDataType = 18;
pub const AVFrameSideDataType_AV_FRAME_DATA_DYNAMIC_HDR_PLUS: AVFrameSideDataType = 19;
pub const AVFrameSideDataType_AV_FRAME_DATA_REGIONS_OF_INTEREST: AVFrameSideDataType = 20;
pub type AVFrameSideDataType = u32;
pub const AVActiveFormatDescription_AV_AFD_SAME: AVActiveFormatDescription = 8;
pub const AVActiveFormatDescription_AV_AFD_4_3: AVActiveFormatDescription = 9;
pub const AVActiveFormatDescription_AV_AFD_16_9: AVActiveFormatDescription = 10;
pub const AVActiveFormatDescription_AV_AFD_14_9: AVActiveFormatDescription = 11;
pub const AVActiveFormatDescription_AV_AFD_4_3_SP_14_9: AVActiveFormatDescription = 13;
pub const AVActiveFormatDescription_AV_AFD_16_9_SP_14_9: AVActiveFormatDescription = 14;
pub const AVActiveFormatDescription_AV_AFD_SP_4_3: AVActiveFormatDescription = 15;
pub type AVActiveFormatDescription = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFrameSideData {
    pub type_: AVFrameSideDataType,
    pub data: *mut u8,
    pub size: ::std::os::raw::c_int,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVRegionOfInterest {
    pub self_size: u32,
    pub top: ::std::os::raw::c_int,
    pub bottom: ::std::os::raw::c_int,
    pub left: ::std::os::raw::c_int,
    pub right: ::std::os::raw::c_int,
    pub qoffset: AVRational,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFrame {
    pub data: [*mut u8; 8usize],
    pub linesize: [::std::os::raw::c_int; 8usize],
    pub extended_data: *mut *mut u8,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub nb_samples: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
    pub key_frame: ::std::os::raw::c_int,
    pub pict_type: AVPictureType,
    pub sample_aspect_ratio: AVRational,
    pub pts: i64,
    pub pkt_pts: i64,
    pub pkt_dts: i64,
    pub coded_picture_number: ::std::os::raw::c_int,
    pub display_picture_number: ::std::os::raw::c_int,
    pub quality: ::std::os::raw::c_int,
    pub opaque: *mut ::std::os::raw::c_void,
    pub error: [u64; 8usize],
    pub repeat_pict: ::std::os::raw::c_int,
    pub interlaced_frame: ::std::os::raw::c_int,
    pub top_field_first: ::std::os::raw::c_int,
    pub palette_has_changed: ::std::os::raw::c_int,
    pub reordered_opaque: i64,
    pub sample_rate: ::std::os::raw::c_int,
    pub channel_layout: u64,
    pub buf: [*mut AVBufferRef; 8usize],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: ::std::os::raw::c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub best_effort_timestamp: i64,
    pub pkt_pos: i64,
    pub pkt_duration: i64,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: ::std::os::raw::c_int,
    pub channels: ::std::os::raw::c_int,
    pub pkt_size: ::std::os::raw::c_int,
    pub qscale_table: *mut i8,
    pub qstride: ::std::os::raw::c_int,
    pub qscale_type: ::std::os::raw::c_int,
    pub qp_table_buf: *mut AVBufferRef,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub opaque_ref: *mut AVBufferRef,
    pub crop_top: usize,
    pub crop_bottom: usize,
    pub crop_left: usize,
    pub crop_right: usize,
    pub private_ref: *mut AVBufferRef,
}
extern "C" {
    pub fn av_frame_get_best_effort_timestamp(frame: *const AVFrame) -> i64;
}
extern "C" {
    pub fn av_frame_set_best_effort_timestamp(frame: *mut AVFrame, val: i64);
}
extern "C" {
    pub fn av_frame_get_pkt_duration(frame: *const AVFrame) -> i64;
}
extern "C" {
    pub fn av_frame_set_pkt_duration(frame: *mut AVFrame, val: i64);
}
extern "C" {
    pub fn av_frame_get_pkt_pos(frame: *const AVFrame) -> i64;
}
extern "C" {
    pub fn av_frame_set_pkt_pos(frame: *mut AVFrame, val: i64);
}
extern "C" {
    pub fn av_frame_get_channel_layout(frame: *const AVFrame) -> i64;
}
extern "C" {
    pub fn av_frame_set_channel_layout(frame: *mut AVFrame, val: i64);
}
extern "C" {
    pub fn av_frame_get_channels(frame: *const AVFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_frame_set_channels(frame: *mut AVFrame, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_frame_get_sample_rate(frame: *const AVFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_frame_set_sample_rate(frame: *mut AVFrame, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_frame_get_metadata(frame: *const AVFrame) -> *mut AVDictionary;
}
extern "C" {
    pub fn av_frame_set_metadata(frame: *mut AVFrame, val: *mut AVDictionary);
}
extern "C" {
    pub fn av_frame_get_decode_error_flags(frame: *const AVFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_frame_set_decode_error_flags(frame: *mut AVFrame, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_frame_get_pkt_size(frame: *const AVFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_frame_set_pkt_size(frame: *mut AVFrame, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_frame_get_qp_table(
        f: *mut AVFrame,
        stride: *mut ::std::os::raw::c_int,
        type_: *mut ::std::os::raw::c_int,
    ) -> *mut i8;
}
extern "C" {
    pub fn av_frame_set_qp_table(
        f: *mut AVFrame,
        buf: *mut AVBufferRef,
        stride: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_frame_get_colorspace(frame: *const AVFrame) -> AVColorSpace;
}
extern "C" {
    pub fn av_frame_set_colorspace(frame: *mut AVFrame, val: AVColorSpace);
}
extern "C" {
    pub fn av_frame_get_color_range(frame: *const AVFrame) -> AVColorRange;
}
extern "C" {
    pub fn av_frame_set_color_range(frame: *mut AVFrame, val: AVColorRange);
}
extern "C" {
    pub fn av_get_colorspace_name(val: AVColorSpace) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_frame_alloc() -> *mut AVFrame;
}
extern "C" {
    pub fn av_frame_free(frame: *mut *mut AVFrame);
}
extern "C" {
    pub fn av_frame_ref(dst: *mut AVFrame, src: *const AVFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_frame_clone(src: *const AVFrame) -> *mut AVFrame;
}
extern "C" {
    pub fn av_frame_unref(frame: *mut AVFrame);
}
extern "C" {
    pub fn av_frame_move_ref(dst: *mut AVFrame, src: *mut AVFrame);
}
extern "C" {
    pub fn av_frame_get_buffer(
        frame: *mut AVFrame,
        align: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_frame_is_writable(frame: *mut AVFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_frame_make_writable(frame: *mut AVFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_frame_copy(dst: *mut AVFrame, src: *const AVFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_frame_copy_props(dst: *mut AVFrame, src: *const AVFrame) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_frame_get_plane_buffer(
        frame: *mut AVFrame,
        plane: ::std::os::raw::c_int,
    ) -> *mut AVBufferRef;
}
extern "C" {
    pub fn av_frame_new_side_data(
        frame: *mut AVFrame,
        type_: AVFrameSideDataType,
        size: ::std::os::raw::c_int,
    ) -> *mut AVFrameSideData;
}
extern "C" {
    pub fn av_frame_new_side_data_from_buf(
        frame: *mut AVFrame,
        type_: AVFrameSideDataType,
        buf: *mut AVBufferRef,
    ) -> *mut AVFrameSideData;
}
extern "C" {
    pub fn av_frame_get_side_data(
        frame: *const AVFrame,
        type_: AVFrameSideDataType,
    ) -> *mut AVFrameSideData;
}
extern "C" {
    pub fn av_frame_remove_side_data(frame: *mut AVFrame, type_: AVFrameSideDataType);
}
pub const AV_FRAME_CROP_UNALIGNED: _bindgen_ty_2 = 1;
pub type _bindgen_ty_2 = u32;
extern "C" {
    pub fn av_frame_apply_cropping(
        frame: *mut AVFrame,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_frame_side_data_name(type_: AVFrameSideDataType) -> *const ::std::os::raw::c_char;
}
pub const AVHWDeviceType_AV_HWDEVICE_TYPE_NONE: AVHWDeviceType = 0;
pub const AVHWDeviceType_AV_HWDEVICE_TYPE_VDPAU: AVHWDeviceType = 1;
pub const AVHWDeviceType_AV_HWDEVICE_TYPE_CUDA: AVHWDeviceType = 2;
pub const AVHWDeviceType_AV_HWDEVICE_TYPE_VAAPI: AVHWDeviceType = 3;
pub const AVHWDeviceType_AV_HWDEVICE_TYPE_DXVA2: AVHWDeviceType = 4;
pub const AVHWDeviceType_AV_HWDEVICE_TYPE_QSV: AVHWDeviceType = 5;
pub const AVHWDeviceType_AV_HWDEVICE_TYPE_VIDEOTOOLBOX: AVHWDeviceType = 6;
pub const AVHWDeviceType_AV_HWDEVICE_TYPE_D3D11VA: AVHWDeviceType = 7;
pub const AVHWDeviceType_AV_HWDEVICE_TYPE_DRM: AVHWDeviceType = 8;
pub const AVHWDeviceType_AV_HWDEVICE_TYPE_OPENCL: AVHWDeviceType = 9;
pub const AVHWDeviceType_AV_HWDEVICE_TYPE_MEDIACODEC: AVHWDeviceType = 10;
pub type AVHWDeviceType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVHWDeviceInternal {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVHWDeviceContext {
    pub av_class: *const AVClass,
    pub internal: *mut AVHWDeviceInternal,
    pub type_: AVHWDeviceType,
    pub hwctx: *mut ::std::os::raw::c_void,
    pub free: ::std::option::Option<unsafe extern "C" fn(ctx: *mut AVHWDeviceContext)>,
    pub user_opaque: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVHWFramesInternal {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVHWFramesContext {
    pub av_class: *const AVClass,
    pub internal: *mut AVHWFramesInternal,
    pub device_ref: *mut AVBufferRef,
    pub device_ctx: *mut AVHWDeviceContext,
    pub hwctx: *mut ::std::os::raw::c_void,
    pub free: ::std::option::Option<unsafe extern "C" fn(ctx: *mut AVHWFramesContext)>,
    pub user_opaque: *mut ::std::os::raw::c_void,
    pub pool: *mut AVBufferPool,
    pub initial_pool_size: ::std::os::raw::c_int,
    pub format: AVPixelFormat,
    pub sw_format: AVPixelFormat,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
}
extern "C" {
    pub fn av_hwdevice_find_type_by_name(name: *const ::std::os::raw::c_char) -> AVHWDeviceType;
}
extern "C" {
    pub fn av_hwdevice_get_type_name(type_: AVHWDeviceType) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_hwdevice_iterate_types(prev: AVHWDeviceType) -> AVHWDeviceType;
}
extern "C" {
    pub fn av_hwdevice_ctx_alloc(type_: AVHWDeviceType) -> *mut AVBufferRef;
}
extern "C" {
    pub fn av_hwdevice_ctx_init(ref_: *mut AVBufferRef) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_hwdevice_ctx_create(
        device_ctx: *mut *mut AVBufferRef,
        type_: AVHWDeviceType,
        device: *const ::std::os::raw::c_char,
        opts: *mut AVDictionary,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_hwdevice_ctx_create_derived(
        dst_ctx: *mut *mut AVBufferRef,
        type_: AVHWDeviceType,
        src_ctx: *mut AVBufferRef,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_hwframe_ctx_alloc(device_ctx: *mut AVBufferRef) -> *mut AVBufferRef;
}
extern "C" {
    pub fn av_hwframe_ctx_init(ref_: *mut AVBufferRef) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_hwframe_get_buffer(
        hwframe_ctx: *mut AVBufferRef,
        frame: *mut AVFrame,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_hwframe_transfer_data(
        dst: *mut AVFrame,
        src: *const AVFrame,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub const AVHWFrameTransferDirection_AV_HWFRAME_TRANSFER_DIRECTION_FROM:
    AVHWFrameTransferDirection = 0;
pub const AVHWFrameTransferDirection_AV_HWFRAME_TRANSFER_DIRECTION_TO: AVHWFrameTransferDirection =
    1;
pub type AVHWFrameTransferDirection = u32;
extern "C" {
    pub fn av_hwframe_transfer_get_formats(
        hwframe_ctx: *mut AVBufferRef,
        dir: AVHWFrameTransferDirection,
        formats: *mut *mut AVPixelFormat,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVHWFramesConstraints {
    pub valid_hw_formats: *mut AVPixelFormat,
    pub valid_sw_formats: *mut AVPixelFormat,
    pub min_width: ::std::os::raw::c_int,
    pub min_height: ::std::os::raw::c_int,
    pub max_width: ::std::os::raw::c_int,
    pub max_height: ::std::os::raw::c_int,
}
extern "C" {
    pub fn av_hwdevice_hwconfig_alloc(device_ctx: *mut AVBufferRef) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_hwdevice_get_hwframe_constraints(
        ref_: *mut AVBufferRef,
        hwconfig: *const ::std::os::raw::c_void,
    ) -> *mut AVHWFramesConstraints;
}
extern "C" {
    pub fn av_hwframe_constraints_free(constraints: *mut *mut AVHWFramesConstraints);
}
pub const AV_HWFRAME_MAP_READ: _bindgen_ty_3 = 1;
pub const AV_HWFRAME_MAP_WRITE: _bindgen_ty_3 = 2;
pub const AV_HWFRAME_MAP_OVERWRITE: _bindgen_ty_3 = 4;
pub const AV_HWFRAME_MAP_DIRECT: _bindgen_ty_3 = 8;
pub type _bindgen_ty_3 = u32;
extern "C" {
    pub fn av_hwframe_map(
        dst: *mut AVFrame,
        src: *const AVFrame,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_hwframe_ctx_create_derived(
        derived_frame_ctx: *mut *mut AVBufferRef,
        format: AVPixelFormat,
        derived_device_ctx: *mut AVBufferRef,
        source_frame_ctx: *mut AVBufferRef,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVComponentDescriptor {
    pub plane: ::std::os::raw::c_int,
    pub step: ::std::os::raw::c_int,
    pub offset: ::std::os::raw::c_int,
    pub shift: ::std::os::raw::c_int,
    pub depth: ::std::os::raw::c_int,
    pub step_minus1: ::std::os::raw::c_int,
    pub depth_minus1: ::std::os::raw::c_int,
    pub offset_plus1: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVPixFmtDescriptor {
    pub name: *const ::std::os::raw::c_char,
    pub nb_components: u8,
    pub log2_chroma_w: u8,
    pub log2_chroma_h: u8,
    pub flags: u64,
    pub comp: [AVComponentDescriptor; 4usize],
    pub alias: *const ::std::os::raw::c_char,
}
extern "C" {
    pub fn av_get_bits_per_pixel(pixdesc: *const AVPixFmtDescriptor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_get_padded_bits_per_pixel(
        pixdesc: *const AVPixFmtDescriptor,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_pix_fmt_desc_get(pix_fmt: AVPixelFormat) -> *const AVPixFmtDescriptor;
}
extern "C" {
    pub fn av_pix_fmt_desc_next(prev: *const AVPixFmtDescriptor) -> *const AVPixFmtDescriptor;
}
extern "C" {
    pub fn av_pix_fmt_desc_get_id(desc: *const AVPixFmtDescriptor) -> AVPixelFormat;
}
extern "C" {
    pub fn av_pix_fmt_get_chroma_sub_sample(
        pix_fmt: AVPixelFormat,
        h_shift: *mut ::std::os::raw::c_int,
        v_shift: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_pix_fmt_count_planes(pix_fmt: AVPixelFormat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_color_range_name(range: AVColorRange) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_color_range_from_name(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_color_primaries_name(primaries: AVColorPrimaries) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_color_primaries_from_name(
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_color_transfer_name(
        transfer: AVColorTransferCharacteristic,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_color_transfer_from_name(
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_color_space_name(space: AVColorSpace) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_color_space_from_name(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_chroma_location_name(location: AVChromaLocation) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_chroma_location_from_name(
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_get_pix_fmt(name: *const ::std::os::raw::c_char) -> AVPixelFormat;
}
extern "C" {
    pub fn av_get_pix_fmt_name(pix_fmt: AVPixelFormat) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_get_pix_fmt_string(
        buf: *mut ::std::os::raw::c_char,
        buf_size: ::std::os::raw::c_int,
        pix_fmt: AVPixelFormat,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_read_image_line2(
        dst: *mut ::std::os::raw::c_void,
        data: *mut *const u8,
        linesize: *const ::std::os::raw::c_int,
        desc: *const AVPixFmtDescriptor,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        c: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        read_pal_component: ::std::os::raw::c_int,
        dst_element_size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_read_image_line(
        dst: *mut u16,
        data: *mut *const u8,
        linesize: *const ::std::os::raw::c_int,
        desc: *const AVPixFmtDescriptor,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        c: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        read_pal_component: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_write_image_line2(
        src: *const ::std::os::raw::c_void,
        data: *mut *mut u8,
        linesize: *const ::std::os::raw::c_int,
        desc: *const AVPixFmtDescriptor,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        c: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        src_element_size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_write_image_line(
        src: *const u16,
        data: *mut *mut u8,
        linesize: *const ::std::os::raw::c_int,
        desc: *const AVPixFmtDescriptor,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        c: ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_pix_fmt_swap_endianness(pix_fmt: AVPixelFormat) -> AVPixelFormat;
}
extern "C" {
    pub fn av_get_pix_fmt_loss(
        dst_pix_fmt: AVPixelFormat,
        src_pix_fmt: AVPixelFormat,
        has_alpha: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_find_best_pix_fmt_of_2(
        dst_pix_fmt1: AVPixelFormat,
        dst_pix_fmt2: AVPixelFormat,
        src_pix_fmt: AVPixelFormat,
        has_alpha: ::std::os::raw::c_int,
        loss_ptr: *mut ::std::os::raw::c_int,
    ) -> AVPixelFormat;
}
extern "C" {
    pub fn av_image_fill_max_pixsteps(
        max_pixsteps: *mut ::std::os::raw::c_int,
        max_pixstep_comps: *mut ::std::os::raw::c_int,
        pixdesc: *const AVPixFmtDescriptor,
    );
}
extern "C" {
    pub fn av_image_get_linesize(
        pix_fmt: AVPixelFormat,
        width: ::std::os::raw::c_int,
        plane: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_image_fill_linesizes(
        linesizes: *mut ::std::os::raw::c_int,
        pix_fmt: AVPixelFormat,
        width: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_image_fill_pointers(
        data: *mut *mut u8,
        pix_fmt: AVPixelFormat,
        height: ::std::os::raw::c_int,
        ptr: *mut u8,
        linesizes: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_image_alloc(
        pointers: *mut *mut u8,
        linesizes: *mut ::std::os::raw::c_int,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        pix_fmt: AVPixelFormat,
        align: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_image_copy_plane(
        dst: *mut u8,
        dst_linesize: ::std::os::raw::c_int,
        src: *const u8,
        src_linesize: ::std::os::raw::c_int,
        bytewidth: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_image_copy(
        dst_data: *mut *mut u8,
        dst_linesizes: *mut ::std::os::raw::c_int,
        src_data: *mut *const u8,
        src_linesizes: *const ::std::os::raw::c_int,
        pix_fmt: AVPixelFormat,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_image_copy_uc_from(
        dst_data: *mut *mut u8,
        dst_linesizes: *const isize,
        src_data: *mut *const u8,
        src_linesizes: *const isize,
        pix_fmt: AVPixelFormat,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_image_fill_arrays(
        dst_data: *mut *mut u8,
        dst_linesize: *mut ::std::os::raw::c_int,
        src: *const u8,
        pix_fmt: AVPixelFormat,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        align: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_image_get_buffer_size(
        pix_fmt: AVPixelFormat,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        align: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_image_copy_to_buffer(
        dst: *mut u8,
        dst_size: ::std::os::raw::c_int,
        src_data: *const *const u8,
        src_linesize: *const ::std::os::raw::c_int,
        pix_fmt: AVPixelFormat,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        align: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_image_check_size(
        w: ::std::os::raw::c_uint,
        h: ::std::os::raw::c_uint,
        log_offset: ::std::os::raw::c_int,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_image_check_size2(
        w: ::std::os::raw::c_uint,
        h: ::std::os::raw::c_uint,
        max_pixels: i64,
        pix_fmt: AVPixelFormat,
        log_offset: ::std::os::raw::c_int,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_image_check_sar(
        w: ::std::os::raw::c_uint,
        h: ::std::os::raw::c_uint,
        sar: AVRational,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_image_fill_black(
        dst_data: *mut *mut u8,
        dst_linesize: *const isize,
        pix_fmt: AVPixelFormat,
        range: AVColorRange,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub const AVOptionType_AV_OPT_TYPE_FLAGS: AVOptionType = 0;
pub const AVOptionType_AV_OPT_TYPE_INT: AVOptionType = 1;
pub const AVOptionType_AV_OPT_TYPE_INT64: AVOptionType = 2;
pub const AVOptionType_AV_OPT_TYPE_DOUBLE: AVOptionType = 3;
pub const AVOptionType_AV_OPT_TYPE_FLOAT: AVOptionType = 4;
pub const AVOptionType_AV_OPT_TYPE_STRING: AVOptionType = 5;
pub const AVOptionType_AV_OPT_TYPE_RATIONAL: AVOptionType = 6;
pub const AVOptionType_AV_OPT_TYPE_BINARY: AVOptionType = 7;
pub const AVOptionType_AV_OPT_TYPE_DICT: AVOptionType = 8;
pub const AVOptionType_AV_OPT_TYPE_UINT64: AVOptionType = 9;
pub const AVOptionType_AV_OPT_TYPE_CONST: AVOptionType = 10;
pub const AVOptionType_AV_OPT_TYPE_IMAGE_SIZE: AVOptionType = 11;
pub const AVOptionType_AV_OPT_TYPE_PIXEL_FMT: AVOptionType = 12;
pub const AVOptionType_AV_OPT_TYPE_SAMPLE_FMT: AVOptionType = 13;
pub const AVOptionType_AV_OPT_TYPE_VIDEO_RATE: AVOptionType = 14;
pub const AVOptionType_AV_OPT_TYPE_DURATION: AVOptionType = 15;
pub const AVOptionType_AV_OPT_TYPE_COLOR: AVOptionType = 16;
pub const AVOptionType_AV_OPT_TYPE_CHANNEL_LAYOUT: AVOptionType = 17;
pub const AVOptionType_AV_OPT_TYPE_BOOL: AVOptionType = 18;
pub type AVOptionType = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AVOption {
    pub name: *const ::std::os::raw::c_char,
    pub help: *const ::std::os::raw::c_char,
    pub offset: ::std::os::raw::c_int,
    pub type_: AVOptionType,
    pub default_val: AVOption__bindgen_ty_1,
    pub min: f64,
    pub max: f64,
    pub flags: ::std::os::raw::c_int,
    pub unit: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AVOption__bindgen_ty_1 {
    pub i64: i64,
    pub dbl: f64,
    pub str: *const ::std::os::raw::c_char,
    pub q: AVRational,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVOptionRange {
    pub str: *const ::std::os::raw::c_char,
    pub value_min: f64,
    pub value_max: f64,
    pub component_min: f64,
    pub component_max: f64,
    pub is_range: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVOptionRanges {
    pub range: *mut *mut AVOptionRange,
    pub nb_ranges: ::std::os::raw::c_int,
    pub nb_components: ::std::os::raw::c_int,
}
extern "C" {
    pub fn av_opt_show2(
        obj: *mut ::std::os::raw::c_void,
        av_log_obj: *mut ::std::os::raw::c_void,
        req_flags: ::std::os::raw::c_int,
        rej_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_defaults(s: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn av_opt_set_defaults2(
        s: *mut ::std::os::raw::c_void,
        mask: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_set_options_string(
        ctx: *mut ::std::os::raw::c_void,
        opts: *const ::std::os::raw::c_char,
        key_val_sep: *const ::std::os::raw::c_char,
        pairs_sep: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_from_string(
        ctx: *mut ::std::os::raw::c_void,
        opts: *const ::std::os::raw::c_char,
        shorthand: *const *const ::std::os::raw::c_char,
        key_val_sep: *const ::std::os::raw::c_char,
        pairs_sep: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_free(obj: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn av_opt_flag_is_set(
        obj: *mut ::std::os::raw::c_void,
        field_name: *const ::std::os::raw::c_char,
        flag_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_dict(
        obj: *mut ::std::os::raw::c_void,
        options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_dict2(
        obj: *mut ::std::os::raw::c_void,
        options: *mut *mut AVDictionary,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_get_key_value(
        ropts: *mut *const ::std::os::raw::c_char,
        key_val_sep: *const ::std::os::raw::c_char,
        pairs_sep: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_uint,
        rkey: *mut *mut ::std::os::raw::c_char,
        rval: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
pub const AV_OPT_FLAG_IMPLICIT_KEY: _bindgen_ty_4 = 1;
pub type _bindgen_ty_4 = u32;
extern "C" {
    pub fn av_opt_eval_flags(
        obj: *mut ::std::os::raw::c_void,
        o: *const AVOption,
        val: *const ::std::os::raw::c_char,
        flags_out: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_eval_int(
        obj: *mut ::std::os::raw::c_void,
        o: *const AVOption,
        val: *const ::std::os::raw::c_char,
        int_out: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_eval_int64(
        obj: *mut ::std::os::raw::c_void,
        o: *const AVOption,
        val: *const ::std::os::raw::c_char,
        int64_out: *mut i64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_eval_float(
        obj: *mut ::std::os::raw::c_void,
        o: *const AVOption,
        val: *const ::std::os::raw::c_char,
        float_out: *mut f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_eval_double(
        obj: *mut ::std::os::raw::c_void,
        o: *const AVOption,
        val: *const ::std::os::raw::c_char,
        double_out: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_eval_q(
        obj: *mut ::std::os::raw::c_void,
        o: *const AVOption,
        val: *const ::std::os::raw::c_char,
        q_out: *mut AVRational,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_find(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        unit: *const ::std::os::raw::c_char,
        opt_flags: ::std::os::raw::c_int,
        search_flags: ::std::os::raw::c_int,
    ) -> *const AVOption;
}
extern "C" {
    pub fn av_opt_find2(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        unit: *const ::std::os::raw::c_char,
        opt_flags: ::std::os::raw::c_int,
        search_flags: ::std::os::raw::c_int,
        target_obj: *mut *mut ::std::os::raw::c_void,
    ) -> *const AVOption;
}
extern "C" {
    pub fn av_opt_next(
        obj: *const ::std::os::raw::c_void,
        prev: *const AVOption,
    ) -> *const AVOption;
}
extern "C" {
    pub fn av_opt_child_next(
        obj: *mut ::std::os::raw::c_void,
        prev: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_opt_child_class_next(parent: *const AVClass, prev: *const AVClass) -> *const AVClass;
}
extern "C" {
    pub fn av_opt_set(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        val: *const ::std::os::raw::c_char,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_int(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        val: i64,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_double(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        val: f64,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_q(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        val: AVRational,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_bin(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        val: *const u8,
        size: ::std::os::raw::c_int,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_image_size(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_pixel_fmt(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        fmt: AVPixelFormat,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_sample_fmt(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        fmt: AVSampleFormat,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_video_rate(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        val: AVRational,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_channel_layout(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        ch_layout: i64,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_set_dict_val(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        val: *const AVDictionary,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_get(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        search_flags: ::std::os::raw::c_int,
        out_val: *mut *mut u8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_get_int(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        search_flags: ::std::os::raw::c_int,
        out_val: *mut i64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_get_double(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        search_flags: ::std::os::raw::c_int,
        out_val: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_get_q(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        search_flags: ::std::os::raw::c_int,
        out_val: *mut AVRational,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_get_image_size(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        search_flags: ::std::os::raw::c_int,
        w_out: *mut ::std::os::raw::c_int,
        h_out: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_get_pixel_fmt(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        search_flags: ::std::os::raw::c_int,
        out_fmt: *mut AVPixelFormat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_get_sample_fmt(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        search_flags: ::std::os::raw::c_int,
        out_fmt: *mut AVSampleFormat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_get_video_rate(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        search_flags: ::std::os::raw::c_int,
        out_val: *mut AVRational,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_get_channel_layout(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        search_flags: ::std::os::raw::c_int,
        ch_layout: *mut i64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_get_dict_val(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        search_flags: ::std::os::raw::c_int,
        out_val: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_ptr(
        avclass: *const AVClass,
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_opt_freep_ranges(ranges: *mut *mut AVOptionRanges);
}
extern "C" {
    pub fn av_opt_query_ranges(
        arg1: *mut *mut AVOptionRanges,
        obj: *mut ::std::os::raw::c_void,
        key: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_copy(
        dest: *mut ::std::os::raw::c_void,
        src: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_query_ranges_default(
        arg1: *mut *mut AVOptionRanges,
        obj: *mut ::std::os::raw::c_void,
        key: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_is_set_to_default(
        obj: *mut ::std::os::raw::c_void,
        o: *const AVOption,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_is_set_to_default_by_name(
        obj: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        search_flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_opt_serialize(
        obj: *mut ::std::os::raw::c_void,
        opt_flags: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
        buffer: *mut *mut ::std::os::raw::c_char,
        key_val_sep: ::std::os::raw::c_char,
        pairs_sep: ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_gettime() -> i64;
}
extern "C" {
    pub fn av_gettime_relative() -> i64;
}
extern "C" {
    pub fn av_gettime_relative_is_monotonic() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_usleep(usec: ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
pub const AVTimecodeFlag_AV_TIMECODE_FLAG_DROPFRAME: AVTimecodeFlag = 1;
pub const AVTimecodeFlag_AV_TIMECODE_FLAG_24HOURSMAX: AVTimecodeFlag = 2;
pub const AVTimecodeFlag_AV_TIMECODE_FLAG_ALLOWNEGATIVE: AVTimecodeFlag = 4;
pub type AVTimecodeFlag = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVTimecode {
    pub start: ::std::os::raw::c_int,
    pub flags: u32,
    pub rate: AVRational,
    pub fps: ::std::os::raw::c_uint,
}
extern "C" {
    pub fn av_timecode_adjust_ntsc_framenum2(
        framenum: ::std::os::raw::c_int,
        fps: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_timecode_get_smpte_from_framenum(
        tc: *const AVTimecode,
        framenum: ::std::os::raw::c_int,
    ) -> u32;
}
extern "C" {
    pub fn av_timecode_make_string(
        tc: *const AVTimecode,
        buf: *mut ::std::os::raw::c_char,
        framenum: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_timecode_make_smpte_tc_string(
        buf: *mut ::std::os::raw::c_char,
        tcsmpte: u32,
        prevent_df: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_timecode_make_mpeg_tc_string(
        buf: *mut ::std::os::raw::c_char,
        tc25bit: u32,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_timecode_init(
        tc: *mut AVTimecode,
        rate: AVRational,
        flags: ::std::os::raw::c_int,
        frame_start: ::std::os::raw::c_int,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_timecode_init_from_string(
        tc: *mut AVTimecode,
        rate: AVRational,
        str: *const ::std::os::raw::c_char,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_timecode_check_frame_rate(rate: AVRational) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_assert0_fpu();
}
extern "C" {
    pub fn av_strstart(
        str: *const ::std::os::raw::c_char,
        pfx: *const ::std::os::raw::c_char,
        ptr: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_stristart(
        str: *const ::std::os::raw::c_char,
        pfx: *const ::std::os::raw::c_char,
        ptr: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_stristr(
        haystack: *const ::std::os::raw::c_char,
        needle: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_strnstr(
        haystack: *const ::std::os::raw::c_char,
        needle: *const ::std::os::raw::c_char,
        hay_length: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_strlcpy(
        dst: *mut ::std::os::raw::c_char,
        src: *const ::std::os::raw::c_char,
        size: usize,
    ) -> usize;
}
extern "C" {
    pub fn av_strlcat(
        dst: *mut ::std::os::raw::c_char,
        src: *const ::std::os::raw::c_char,
        size: usize,
    ) -> usize;
}
extern "C" {
    pub fn av_strlcatf(
        dst: *mut ::std::os::raw::c_char,
        size: usize,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> usize;
}
extern "C" {
    pub fn av_asprintf(fmt: *const ::std::os::raw::c_char, ...) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_d2str(d: f64) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_get_token(
        buf: *mut *const ::std::os::raw::c_char,
        term: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_strtok(
        s: *mut ::std::os::raw::c_char,
        delim: *const ::std::os::raw::c_char,
        saveptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_strcasecmp(
        a: *const ::std::os::raw::c_char,
        b: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_strncasecmp(
        a: *const ::std::os::raw::c_char,
        b: *const ::std::os::raw::c_char,
        n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_strireplace(
        str: *const ::std::os::raw::c_char,
        from: *const ::std::os::raw::c_char,
        to: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_basename(path: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_dirname(path: *mut ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_match_name(
        name: *const ::std::os::raw::c_char,
        names: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_append_path_component(
        path: *const ::std::os::raw::c_char,
        component: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub const AVEscapeMode_AV_ESCAPE_MODE_AUTO: AVEscapeMode = 0;
pub const AVEscapeMode_AV_ESCAPE_MODE_BACKSLASH: AVEscapeMode = 1;
pub const AVEscapeMode_AV_ESCAPE_MODE_QUOTE: AVEscapeMode = 2;
pub type AVEscapeMode = u32;
extern "C" {
    pub fn av_escape(
        dst: *mut *mut ::std::os::raw::c_char,
        src: *const ::std::os::raw::c_char,
        special_chars: *const ::std::os::raw::c_char,
        mode: AVEscapeMode,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_utf8_decode(
        codep: *mut i32,
        bufp: *mut *const u8,
        buf_end: *const u8,
        flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_match_list(
        name: *const ::std::os::raw::c_char,
        list: *const ::std::os::raw::c_char,
        separator: ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_sscanf(
        string: *const ::std::os::raw::c_char,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AVBlowfish {
    pub p: [u32; 18usize],
    pub s: [[u32; 256usize]; 4usize],
}
extern "C" {
    pub fn av_blowfish_alloc() -> *mut AVBlowfish;
}
extern "C" {
    pub fn av_blowfish_init(ctx: *mut AVBlowfish, key: *const u8, key_len: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_blowfish_crypt_ecb(
        ctx: *mut AVBlowfish,
        xl: *mut u32,
        xr: *mut u32,
        decrypt: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_blowfish_crypt(
        ctx: *mut AVBlowfish,
        dst: *mut u8,
        src: *const u8,
        count: ::std::os::raw::c_int,
        iv: *mut u8,
        decrypt: ::std::os::raw::c_int,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ff_pad_helper_AVBPrint {
    pub str: *mut ::std::os::raw::c_char,
    pub len: ::std::os::raw::c_uint,
    pub size: ::std::os::raw::c_uint,
    pub size_max: ::std::os::raw::c_uint,
    pub reserved_internal_buffer: [::std::os::raw::c_char; 1usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AVBPrint {
    pub str: *mut ::std::os::raw::c_char,
    pub len: ::std::os::raw::c_uint,
    pub size: ::std::os::raw::c_uint,
    pub size_max: ::std::os::raw::c_uint,
    pub reserved_internal_buffer: [::std::os::raw::c_char; 1usize],
    pub reserved_padding: [::std::os::raw::c_char; 1000usize],
}
extern "C" {
    pub fn av_bprint_init(
        buf: *mut AVBPrint,
        size_init: ::std::os::raw::c_uint,
        size_max: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn av_bprint_init_for_buffer(
        buf: *mut AVBPrint,
        buffer: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn av_bprintf(buf: *mut AVBPrint, fmt: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn av_vbprintf(
        buf: *mut AVBPrint,
        fmt: *const ::std::os::raw::c_char,
        vl_arg: *mut __va_list_tag,
    );
}
extern "C" {
    pub fn av_bprint_chars(
        buf: *mut AVBPrint,
        c: ::std::os::raw::c_char,
        n: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn av_bprint_append_data(
        buf: *mut AVBPrint,
        data: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn av_bprint_strftime(
        buf: *mut AVBPrint,
        fmt: *const ::std::os::raw::c_char,
        tm: *const tm,
    );
}
extern "C" {
    pub fn av_bprint_get_buffer(
        buf: *mut AVBPrint,
        size: ::std::os::raw::c_uint,
        mem: *mut *mut ::std::os::raw::c_uchar,
        actual_size: *mut ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn av_bprint_clear(buf: *mut AVBPrint);
}
extern "C" {
    pub fn av_bprint_finalize(
        buf: *mut AVBPrint,
        ret_str: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_bprint_escape(
        dstbuf: *mut AVBPrint,
        src: *const ::std::os::raw::c_char,
        special_chars: *const ::std::os::raw::c_char,
        mode: AVEscapeMode,
        flags: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub static av_camellia_size: ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCAMELLIA {
    _unused: [u8; 0],
}
extern "C" {
    pub fn av_camellia_alloc() -> *mut AVCAMELLIA;
}
extern "C" {
    pub fn av_camellia_init(
        ctx: *mut AVCAMELLIA,
        key: *const u8,
        key_bits: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_camellia_crypt(
        ctx: *mut AVCAMELLIA,
        dst: *mut u8,
        src: *const u8,
        count: ::std::os::raw::c_int,
        iv: *mut u8,
        decrypt: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub static av_cast5_size: ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCAST5 {
    _unused: [u8; 0],
}
extern "C" {
    pub fn av_cast5_alloc() -> *mut AVCAST5;
}
extern "C" {
    pub fn av_cast5_init(
        ctx: *mut AVCAST5,
        key: *const u8,
        key_bits: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_cast5_crypt(
        ctx: *mut AVCAST5,
        dst: *mut u8,
        src: *const u8,
        count: ::std::os::raw::c_int,
        decrypt: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_cast5_crypt2(
        ctx: *mut AVCAST5,
        dst: *mut u8,
        src: *const u8,
        count: ::std::os::raw::c_int,
        iv: *mut u8,
        decrypt: ::std::os::raw::c_int,
    );
}
pub const AVMatrixEncoding_AV_MATRIX_ENCODING_NONE: AVMatrixEncoding = 0;
pub const AVMatrixEncoding_AV_MATRIX_ENCODING_DOLBY: AVMatrixEncoding = 1;
pub const AVMatrixEncoding_AV_MATRIX_ENCODING_DPLII: AVMatrixEncoding = 2;
pub const AVMatrixEncoding_AV_MATRIX_ENCODING_DPLIIX: AVMatrixEncoding = 3;
pub const AVMatrixEncoding_AV_MATRIX_ENCODING_DPLIIZ: AVMatrixEncoding = 4;
pub const AVMatrixEncoding_AV_MATRIX_ENCODING_DOLBYEX: AVMatrixEncoding = 5;
pub const AVMatrixEncoding_AV_MATRIX_ENCODING_DOLBYHEADPHONE: AVMatrixEncoding = 6;
pub const AVMatrixEncoding_AV_MATRIX_ENCODING_NB: AVMatrixEncoding = 7;
pub type AVMatrixEncoding = u32;
extern "C" {
    pub fn av_get_channel_layout(name: *const ::std::os::raw::c_char) -> u64;
}
extern "C" {
    pub fn av_get_extended_channel_layout(
        name: *const ::std::os::raw::c_char,
        channel_layout: *mut u64,
        nb_channels: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_get_channel_layout_string(
        buf: *mut ::std::os::raw::c_char,
        buf_size: ::std::os::raw::c_int,
        nb_channels: ::std::os::raw::c_int,
        channel_layout: u64,
    );
}
extern "C" {
    pub fn av_bprint_channel_layout(
        bp: *mut AVBPrint,
        nb_channels: ::std::os::raw::c_int,
        channel_layout: u64,
    );
}
extern "C" {
    pub fn av_get_channel_layout_nb_channels(channel_layout: u64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_get_default_channel_layout(nb_channels: ::std::os::raw::c_int) -> i64;
}
extern "C" {
    pub fn av_get_channel_layout_channel_index(
        channel_layout: u64,
        channel: u64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_channel_layout_extract_channel(
        channel_layout: u64,
        index: ::std::os::raw::c_int,
    ) -> u64;
}
extern "C" {
    pub fn av_get_channel_name(channel: u64) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_get_channel_description(channel: u64) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_get_standard_channel_layout(
        index: ::std::os::raw::c_uint,
        layout: *mut u64,
        name: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
pub type av_pixelutils_sad_fn = ::std::option::Option<
    unsafe extern "C" fn(
        src1: *const u8,
        stride1: isize,
        src2: *const u8,
        stride2: isize,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn av_pixelutils_get_sad_fn(
        w_bits: ::std::os::raw::c_int,
        h_bits: ::std::os::raw::c_int,
        aligned: ::std::os::raw::c_int,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> av_pixelutils_sad_fn;
}
extern "C" {
    pub fn av_get_cpu_flags() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_force_cpu_flags(flags: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_set_cpu_flags_mask(mask: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_parse_cpu_flags(s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_parse_cpu_caps(
        flags: *mut ::std::os::raw::c_uint,
        s: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_cpu_count() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_cpu_max_align() -> usize;
}
pub const AVCodecID_AV_CODEC_ID_NONE: AVCodecID = 0;
pub const AVCodecID_AV_CODEC_ID_MPEG1VIDEO: AVCodecID = 1;
pub const AVCodecID_AV_CODEC_ID_MPEG2VIDEO: AVCodecID = 2;
pub const AVCodecID_AV_CODEC_ID_H261: AVCodecID = 3;
pub const AVCodecID_AV_CODEC_ID_H263: AVCodecID = 4;
pub const AVCodecID_AV_CODEC_ID_RV10: AVCodecID = 5;
pub const AVCodecID_AV_CODEC_ID_RV20: AVCodecID = 6;
pub const AVCodecID_AV_CODEC_ID_MJPEG: AVCodecID = 7;
pub const AVCodecID_AV_CODEC_ID_MJPEGB: AVCodecID = 8;
pub const AVCodecID_AV_CODEC_ID_LJPEG: AVCodecID = 9;
pub const AVCodecID_AV_CODEC_ID_SP5X: AVCodecID = 10;
pub const AVCodecID_AV_CODEC_ID_JPEGLS: AVCodecID = 11;
pub const AVCodecID_AV_CODEC_ID_MPEG4: AVCodecID = 12;
pub const AVCodecID_AV_CODEC_ID_RAWVIDEO: AVCodecID = 13;
pub const AVCodecID_AV_CODEC_ID_MSMPEG4V1: AVCodecID = 14;
pub const AVCodecID_AV_CODEC_ID_MSMPEG4V2: AVCodecID = 15;
pub const AVCodecID_AV_CODEC_ID_MSMPEG4V3: AVCodecID = 16;
pub const AVCodecID_AV_CODEC_ID_WMV1: AVCodecID = 17;
pub const AVCodecID_AV_CODEC_ID_WMV2: AVCodecID = 18;
pub const AVCodecID_AV_CODEC_ID_H263P: AVCodecID = 19;
pub const AVCodecID_AV_CODEC_ID_H263I: AVCodecID = 20;
pub const AVCodecID_AV_CODEC_ID_FLV1: AVCodecID = 21;
pub const AVCodecID_AV_CODEC_ID_SVQ1: AVCodecID = 22;
pub const AVCodecID_AV_CODEC_ID_SVQ3: AVCodecID = 23;
pub const AVCodecID_AV_CODEC_ID_DVVIDEO: AVCodecID = 24;
pub const AVCodecID_AV_CODEC_ID_HUFFYUV: AVCodecID = 25;
pub const AVCodecID_AV_CODEC_ID_CYUV: AVCodecID = 26;
pub const AVCodecID_AV_CODEC_ID_H264: AVCodecID = 27;
pub const AVCodecID_AV_CODEC_ID_INDEO3: AVCodecID = 28;
pub const AVCodecID_AV_CODEC_ID_VP3: AVCodecID = 29;
pub const AVCodecID_AV_CODEC_ID_THEORA: AVCodecID = 30;
pub const AVCodecID_AV_CODEC_ID_ASV1: AVCodecID = 31;
pub const AVCodecID_AV_CODEC_ID_ASV2: AVCodecID = 32;
pub const AVCodecID_AV_CODEC_ID_FFV1: AVCodecID = 33;
pub const AVCodecID_AV_CODEC_ID_4XM: AVCodecID = 34;
pub const AVCodecID_AV_CODEC_ID_VCR1: AVCodecID = 35;
pub const AVCodecID_AV_CODEC_ID_CLJR: AVCodecID = 36;
pub const AVCodecID_AV_CODEC_ID_MDEC: AVCodecID = 37;
pub const AVCodecID_AV_CODEC_ID_ROQ: AVCodecID = 38;
pub const AVCodecID_AV_CODEC_ID_INTERPLAY_VIDEO: AVCodecID = 39;
pub const AVCodecID_AV_CODEC_ID_XAN_WC3: AVCodecID = 40;
pub const AVCodecID_AV_CODEC_ID_XAN_WC4: AVCodecID = 41;
pub const AVCodecID_AV_CODEC_ID_RPZA: AVCodecID = 42;
pub const AVCodecID_AV_CODEC_ID_CINEPAK: AVCodecID = 43;
pub const AVCodecID_AV_CODEC_ID_WS_VQA: AVCodecID = 44;
pub const AVCodecID_AV_CODEC_ID_MSRLE: AVCodecID = 45;
pub const AVCodecID_AV_CODEC_ID_MSVIDEO1: AVCodecID = 46;
pub const AVCodecID_AV_CODEC_ID_IDCIN: AVCodecID = 47;
pub const AVCodecID_AV_CODEC_ID_8BPS: AVCodecID = 48;
pub const AVCodecID_AV_CODEC_ID_SMC: AVCodecID = 49;
pub const AVCodecID_AV_CODEC_ID_FLIC: AVCodecID = 50;
pub const AVCodecID_AV_CODEC_ID_TRUEMOTION1: AVCodecID = 51;
pub const AVCodecID_AV_CODEC_ID_VMDVIDEO: AVCodecID = 52;
pub const AVCodecID_AV_CODEC_ID_MSZH: AVCodecID = 53;
pub const AVCodecID_AV_CODEC_ID_ZLIB: AVCodecID = 54;
pub const AVCodecID_AV_CODEC_ID_QTRLE: AVCodecID = 55;
pub const AVCodecID_AV_CODEC_ID_TSCC: AVCodecID = 56;
pub const AVCodecID_AV_CODEC_ID_ULTI: AVCodecID = 57;
pub const AVCodecID_AV_CODEC_ID_QDRAW: AVCodecID = 58;
pub const AVCodecID_AV_CODEC_ID_VIXL: AVCodecID = 59;
pub const AVCodecID_AV_CODEC_ID_QPEG: AVCodecID = 60;
pub const AVCodecID_AV_CODEC_ID_PNG: AVCodecID = 61;
pub const AVCodecID_AV_CODEC_ID_PPM: AVCodecID = 62;
pub const AVCodecID_AV_CODEC_ID_PBM: AVCodecID = 63;
pub const AVCodecID_AV_CODEC_ID_PGM: AVCodecID = 64;
pub const AVCodecID_AV_CODEC_ID_PGMYUV: AVCodecID = 65;
pub const AVCodecID_AV_CODEC_ID_PAM: AVCodecID = 66;
pub const AVCodecID_AV_CODEC_ID_FFVHUFF: AVCodecID = 67;
pub const AVCodecID_AV_CODEC_ID_RV30: AVCodecID = 68;
pub const AVCodecID_AV_CODEC_ID_RV40: AVCodecID = 69;
pub const AVCodecID_AV_CODEC_ID_VC1: AVCodecID = 70;
pub const AVCodecID_AV_CODEC_ID_WMV3: AVCodecID = 71;
pub const AVCodecID_AV_CODEC_ID_LOCO: AVCodecID = 72;
pub const AVCodecID_AV_CODEC_ID_WNV1: AVCodecID = 73;
pub const AVCodecID_AV_CODEC_ID_AASC: AVCodecID = 74;
pub const AVCodecID_AV_CODEC_ID_INDEO2: AVCodecID = 75;
pub const AVCodecID_AV_CODEC_ID_FRAPS: AVCodecID = 76;
pub const AVCodecID_AV_CODEC_ID_TRUEMOTION2: AVCodecID = 77;
pub const AVCodecID_AV_CODEC_ID_BMP: AVCodecID = 78;
pub const AVCodecID_AV_CODEC_ID_CSCD: AVCodecID = 79;
pub const AVCodecID_AV_CODEC_ID_MMVIDEO: AVCodecID = 80;
pub const AVCodecID_AV_CODEC_ID_ZMBV: AVCodecID = 81;
pub const AVCodecID_AV_CODEC_ID_AVS: AVCodecID = 82;
pub const AVCodecID_AV_CODEC_ID_SMACKVIDEO: AVCodecID = 83;
pub const AVCodecID_AV_CODEC_ID_NUV: AVCodecID = 84;
pub const AVCodecID_AV_CODEC_ID_KMVC: AVCodecID = 85;
pub const AVCodecID_AV_CODEC_ID_FLASHSV: AVCodecID = 86;
pub const AVCodecID_AV_CODEC_ID_CAVS: AVCodecID = 87;
pub const AVCodecID_AV_CODEC_ID_JPEG2000: AVCodecID = 88;
pub const AVCodecID_AV_CODEC_ID_VMNC: AVCodecID = 89;
pub const AVCodecID_AV_CODEC_ID_VP5: AVCodecID = 90;
pub const AVCodecID_AV_CODEC_ID_VP6: AVCodecID = 91;
pub const AVCodecID_AV_CODEC_ID_VP6F: AVCodecID = 92;
pub const AVCodecID_AV_CODEC_ID_TARGA: AVCodecID = 93;
pub const AVCodecID_AV_CODEC_ID_DSICINVIDEO: AVCodecID = 94;
pub const AVCodecID_AV_CODEC_ID_TIERTEXSEQVIDEO: AVCodecID = 95;
pub const AVCodecID_AV_CODEC_ID_TIFF: AVCodecID = 96;
pub const AVCodecID_AV_CODEC_ID_GIF: AVCodecID = 97;
pub const AVCodecID_AV_CODEC_ID_DXA: AVCodecID = 98;
pub const AVCodecID_AV_CODEC_ID_DNXHD: AVCodecID = 99;
pub const AVCodecID_AV_CODEC_ID_THP: AVCodecID = 100;
pub const AVCodecID_AV_CODEC_ID_SGI: AVCodecID = 101;
pub const AVCodecID_AV_CODEC_ID_C93: AVCodecID = 102;
pub const AVCodecID_AV_CODEC_ID_BETHSOFTVID: AVCodecID = 103;
pub const AVCodecID_AV_CODEC_ID_PTX: AVCodecID = 104;
pub const AVCodecID_AV_CODEC_ID_TXD: AVCodecID = 105;
pub const AVCodecID_AV_CODEC_ID_VP6A: AVCodecID = 106;
pub const AVCodecID_AV_CODEC_ID_AMV: AVCodecID = 107;
pub const AVCodecID_AV_CODEC_ID_VB: AVCodecID = 108;
pub const AVCodecID_AV_CODEC_ID_PCX: AVCodecID = 109;
pub const AVCodecID_AV_CODEC_ID_SUNRAST: AVCodecID = 110;
pub const AVCodecID_AV_CODEC_ID_INDEO4: AVCodecID = 111;
pub const AVCodecID_AV_CODEC_ID_INDEO5: AVCodecID = 112;
pub const AVCodecID_AV_CODEC_ID_MIMIC: AVCodecID = 113;
pub const AVCodecID_AV_CODEC_ID_RL2: AVCodecID = 114;
pub const AVCodecID_AV_CODEC_ID_ESCAPE124: AVCodecID = 115;
pub const AVCodecID_AV_CODEC_ID_DIRAC: AVCodecID = 116;
pub const AVCodecID_AV_CODEC_ID_BFI: AVCodecID = 117;
pub const AVCodecID_AV_CODEC_ID_CMV: AVCodecID = 118;
pub const AVCodecID_AV_CODEC_ID_MOTIONPIXELS: AVCodecID = 119;
pub const AVCodecID_AV_CODEC_ID_TGV: AVCodecID = 120;
pub const AVCodecID_AV_CODEC_ID_TGQ: AVCodecID = 121;
pub const AVCodecID_AV_CODEC_ID_TQI: AVCodecID = 122;
pub const AVCodecID_AV_CODEC_ID_AURA: AVCodecID = 123;
pub const AVCodecID_AV_CODEC_ID_AURA2: AVCodecID = 124;
pub const AVCodecID_AV_CODEC_ID_V210X: AVCodecID = 125;
pub const AVCodecID_AV_CODEC_ID_TMV: AVCodecID = 126;
pub const AVCodecID_AV_CODEC_ID_V210: AVCodecID = 127;
pub const AVCodecID_AV_CODEC_ID_DPX: AVCodecID = 128;
pub const AVCodecID_AV_CODEC_ID_MAD: AVCodecID = 129;
pub const AVCodecID_AV_CODEC_ID_FRWU: AVCodecID = 130;
pub const AVCodecID_AV_CODEC_ID_FLASHSV2: AVCodecID = 131;
pub const AVCodecID_AV_CODEC_ID_CDGRAPHICS: AVCodecID = 132;
pub const AVCodecID_AV_CODEC_ID_R210: AVCodecID = 133;
pub const AVCodecID_AV_CODEC_ID_ANM: AVCodecID = 134;
pub const AVCodecID_AV_CODEC_ID_BINKVIDEO: AVCodecID = 135;
pub const AVCodecID_AV_CODEC_ID_IFF_ILBM: AVCodecID = 136;
pub const AVCodecID_AV_CODEC_ID_KGV1: AVCodecID = 137;
pub const AVCodecID_AV_CODEC_ID_YOP: AVCodecID = 138;
pub const AVCodecID_AV_CODEC_ID_VP8: AVCodecID = 139;
pub const AVCodecID_AV_CODEC_ID_PICTOR: AVCodecID = 140;
pub const AVCodecID_AV_CODEC_ID_ANSI: AVCodecID = 141;
pub const AVCodecID_AV_CODEC_ID_A64_MULTI: AVCodecID = 142;
pub const AVCodecID_AV_CODEC_ID_A64_MULTI5: AVCodecID = 143;
pub const AVCodecID_AV_CODEC_ID_R10K: AVCodecID = 144;
pub const AVCodecID_AV_CODEC_ID_MXPEG: AVCodecID = 145;
pub const AVCodecID_AV_CODEC_ID_LAGARITH: AVCodecID = 146;
pub const AVCodecID_AV_CODEC_ID_PRORES: AVCodecID = 147;
pub const AVCodecID_AV_CODEC_ID_JV: AVCodecID = 148;
pub const AVCodecID_AV_CODEC_ID_DFA: AVCodecID = 149;
pub const AVCodecID_AV_CODEC_ID_WMV3IMAGE: AVCodecID = 150;
pub const AVCodecID_AV_CODEC_ID_VC1IMAGE: AVCodecID = 151;
pub const AVCodecID_AV_CODEC_ID_UTVIDEO: AVCodecID = 152;
pub const AVCodecID_AV_CODEC_ID_BMV_VIDEO: AVCodecID = 153;
pub const AVCodecID_AV_CODEC_ID_VBLE: AVCodecID = 154;
pub const AVCodecID_AV_CODEC_ID_DXTORY: AVCodecID = 155;
pub const AVCodecID_AV_CODEC_ID_V410: AVCodecID = 156;
pub const AVCodecID_AV_CODEC_ID_XWD: AVCodecID = 157;
pub const AVCodecID_AV_CODEC_ID_CDXL: AVCodecID = 158;
pub const AVCodecID_AV_CODEC_ID_XBM: AVCodecID = 159;
pub const AVCodecID_AV_CODEC_ID_ZEROCODEC: AVCodecID = 160;
pub const AVCodecID_AV_CODEC_ID_MSS1: AVCodecID = 161;
pub const AVCodecID_AV_CODEC_ID_MSA1: AVCodecID = 162;
pub const AVCodecID_AV_CODEC_ID_TSCC2: AVCodecID = 163;
pub const AVCodecID_AV_CODEC_ID_MTS2: AVCodecID = 164;
pub const AVCodecID_AV_CODEC_ID_CLLC: AVCodecID = 165;
pub const AVCodecID_AV_CODEC_ID_MSS2: AVCodecID = 166;
pub const AVCodecID_AV_CODEC_ID_VP9: AVCodecID = 167;
pub const AVCodecID_AV_CODEC_ID_AIC: AVCodecID = 168;
pub const AVCodecID_AV_CODEC_ID_ESCAPE130: AVCodecID = 169;
pub const AVCodecID_AV_CODEC_ID_G2M: AVCodecID = 170;
pub const AVCodecID_AV_CODEC_ID_WEBP: AVCodecID = 171;
pub const AVCodecID_AV_CODEC_ID_HNM4_VIDEO: AVCodecID = 172;
pub const AVCodecID_AV_CODEC_ID_HEVC: AVCodecID = 173;
pub const AVCodecID_AV_CODEC_ID_FIC: AVCodecID = 174;
pub const AVCodecID_AV_CODEC_ID_ALIAS_PIX: AVCodecID = 175;
pub const AVCodecID_AV_CODEC_ID_BRENDER_PIX: AVCodecID = 176;
pub const AVCodecID_AV_CODEC_ID_PAF_VIDEO: AVCodecID = 177;
pub const AVCodecID_AV_CODEC_ID_EXR: AVCodecID = 178;
pub const AVCodecID_AV_CODEC_ID_VP7: AVCodecID = 179;
pub const AVCodecID_AV_CODEC_ID_SANM: AVCodecID = 180;
pub const AVCodecID_AV_CODEC_ID_SGIRLE: AVCodecID = 181;
pub const AVCodecID_AV_CODEC_ID_MVC1: AVCodecID = 182;
pub const AVCodecID_AV_CODEC_ID_MVC2: AVCodecID = 183;
pub const AVCodecID_AV_CODEC_ID_HQX: AVCodecID = 184;
pub const AVCodecID_AV_CODEC_ID_TDSC: AVCodecID = 185;
pub const AVCodecID_AV_CODEC_ID_HQ_HQA: AVCodecID = 186;
pub const AVCodecID_AV_CODEC_ID_HAP: AVCodecID = 187;
pub const AVCodecID_AV_CODEC_ID_DDS: AVCodecID = 188;
pub const AVCodecID_AV_CODEC_ID_DXV: AVCodecID = 189;
pub const AVCodecID_AV_CODEC_ID_SCREENPRESSO: AVCodecID = 190;
pub const AVCodecID_AV_CODEC_ID_RSCC: AVCodecID = 191;
pub const AVCodecID_AV_CODEC_ID_AVS2: AVCodecID = 192;
pub const AVCodecID_AV_CODEC_ID_Y41P: AVCodecID = 32768;
pub const AVCodecID_AV_CODEC_ID_AVRP: AVCodecID = 32769;
pub const AVCodecID_AV_CODEC_ID_012V: AVCodecID = 32770;
pub const AVCodecID_AV_CODEC_ID_AVUI: AVCodecID = 32771;
pub const AVCodecID_AV_CODEC_ID_AYUV: AVCodecID = 32772;
pub const AVCodecID_AV_CODEC_ID_TARGA_Y216: AVCodecID = 32773;
pub const AVCodecID_AV_CODEC_ID_V308: AVCodecID = 32774;
pub const AVCodecID_AV_CODEC_ID_V408: AVCodecID = 32775;
pub const AVCodecID_AV_CODEC_ID_YUV4: AVCodecID = 32776;
pub const AVCodecID_AV_CODEC_ID_AVRN: AVCodecID = 32777;
pub const AVCodecID_AV_CODEC_ID_CPIA: AVCodecID = 32778;
pub const AVCodecID_AV_CODEC_ID_XFACE: AVCodecID = 32779;
pub const AVCodecID_AV_CODEC_ID_SNOW: AVCodecID = 32780;
pub const AVCodecID_AV_CODEC_ID_SMVJPEG: AVCodecID = 32781;
pub const AVCodecID_AV_CODEC_ID_APNG: AVCodecID = 32782;
pub const AVCodecID_AV_CODEC_ID_DAALA: AVCodecID = 32783;
pub const AVCodecID_AV_CODEC_ID_CFHD: AVCodecID = 32784;
pub const AVCodecID_AV_CODEC_ID_TRUEMOTION2RT: AVCodecID = 32785;
pub const AVCodecID_AV_CODEC_ID_M101: AVCodecID = 32786;
pub const AVCodecID_AV_CODEC_ID_MAGICYUV: AVCodecID = 32787;
pub const AVCodecID_AV_CODEC_ID_SHEERVIDEO: AVCodecID = 32788;
pub const AVCodecID_AV_CODEC_ID_YLC: AVCodecID = 32789;
pub const AVCodecID_AV_CODEC_ID_PSD: AVCodecID = 32790;
pub const AVCodecID_AV_CODEC_ID_PIXLET: AVCodecID = 32791;
pub const AVCodecID_AV_CODEC_ID_SPEEDHQ: AVCodecID = 32792;
pub const AVCodecID_AV_CODEC_ID_FMVC: AVCodecID = 32793;
pub const AVCodecID_AV_CODEC_ID_SCPR: AVCodecID = 32794;
pub const AVCodecID_AV_CODEC_ID_CLEARVIDEO: AVCodecID = 32795;
pub const AVCodecID_AV_CODEC_ID_XPM: AVCodecID = 32796;
pub const AVCodecID_AV_CODEC_ID_AV1: AVCodecID = 32797;
pub const AVCodecID_AV_CODEC_ID_BITPACKED: AVCodecID = 32798;
pub const AVCodecID_AV_CODEC_ID_MSCC: AVCodecID = 32799;
pub const AVCodecID_AV_CODEC_ID_SRGC: AVCodecID = 32800;
pub const AVCodecID_AV_CODEC_ID_SVG: AVCodecID = 32801;
pub const AVCodecID_AV_CODEC_ID_GDV: AVCodecID = 32802;
pub const AVCodecID_AV_CODEC_ID_FITS: AVCodecID = 32803;
pub const AVCodecID_AV_CODEC_ID_IMM4: AVCodecID = 32804;
pub const AVCodecID_AV_CODEC_ID_PROSUMER: AVCodecID = 32805;
pub const AVCodecID_AV_CODEC_ID_MWSC: AVCodecID = 32806;
pub const AVCodecID_AV_CODEC_ID_WCMV: AVCodecID = 32807;
pub const AVCodecID_AV_CODEC_ID_RASC: AVCodecID = 32808;
pub const AVCodecID_AV_CODEC_ID_HYMT: AVCodecID = 32809;
pub const AVCodecID_AV_CODEC_ID_ARBC: AVCodecID = 32810;
pub const AVCodecID_AV_CODEC_ID_AGM: AVCodecID = 32811;
pub const AVCodecID_AV_CODEC_ID_LSCR: AVCodecID = 32812;
pub const AVCodecID_AV_CODEC_ID_VP4: AVCodecID = 32813;
pub const AVCodecID_AV_CODEC_ID_FIRST_AUDIO: AVCodecID = 65536;
pub const AVCodecID_AV_CODEC_ID_PCM_S16LE: AVCodecID = 65536;
pub const AVCodecID_AV_CODEC_ID_PCM_S16BE: AVCodecID = 65537;
pub const AVCodecID_AV_CODEC_ID_PCM_U16LE: AVCodecID = 65538;
pub const AVCodecID_AV_CODEC_ID_PCM_U16BE: AVCodecID = 65539;
pub const AVCodecID_AV_CODEC_ID_PCM_S8: AVCodecID = 65540;
pub const AVCodecID_AV_CODEC_ID_PCM_U8: AVCodecID = 65541;
pub const AVCodecID_AV_CODEC_ID_PCM_MULAW: AVCodecID = 65542;
pub const AVCodecID_AV_CODEC_ID_PCM_ALAW: AVCodecID = 65543;
pub const AVCodecID_AV_CODEC_ID_PCM_S32LE: AVCodecID = 65544;
pub const AVCodecID_AV_CODEC_ID_PCM_S32BE: AVCodecID = 65545;
pub const AVCodecID_AV_CODEC_ID_PCM_U32LE: AVCodecID = 65546;
pub const AVCodecID_AV_CODEC_ID_PCM_U32BE: AVCodecID = 65547;
pub const AVCodecID_AV_CODEC_ID_PCM_S24LE: AVCodecID = 65548;
pub const AVCodecID_AV_CODEC_ID_PCM_S24BE: AVCodecID = 65549;
pub const AVCodecID_AV_CODEC_ID_PCM_U24LE: AVCodecID = 65550;
pub const AVCodecID_AV_CODEC_ID_PCM_U24BE: AVCodecID = 65551;
pub const AVCodecID_AV_CODEC_ID_PCM_S24DAUD: AVCodecID = 65552;
pub const AVCodecID_AV_CODEC_ID_PCM_ZORK: AVCodecID = 65553;
pub const AVCodecID_AV_CODEC_ID_PCM_S16LE_PLANAR: AVCodecID = 65554;
pub const AVCodecID_AV_CODEC_ID_PCM_DVD: AVCodecID = 65555;
pub const AVCodecID_AV_CODEC_ID_PCM_F32BE: AVCodecID = 65556;
pub const AVCodecID_AV_CODEC_ID_PCM_F32LE: AVCodecID = 65557;
pub const AVCodecID_AV_CODEC_ID_PCM_F64BE: AVCodecID = 65558;
pub const AVCodecID_AV_CODEC_ID_PCM_F64LE: AVCodecID = 65559;
pub const AVCodecID_AV_CODEC_ID_PCM_BLURAY: AVCodecID = 65560;
pub const AVCodecID_AV_CODEC_ID_PCM_LXF: AVCodecID = 65561;
pub const AVCodecID_AV_CODEC_ID_S302M: AVCodecID = 65562;
pub const AVCodecID_AV_CODEC_ID_PCM_S8_PLANAR: AVCodecID = 65563;
pub const AVCodecID_AV_CODEC_ID_PCM_S24LE_PLANAR: AVCodecID = 65564;
pub const AVCodecID_AV_CODEC_ID_PCM_S32LE_PLANAR: AVCodecID = 65565;
pub const AVCodecID_AV_CODEC_ID_PCM_S16BE_PLANAR: AVCodecID = 65566;
pub const AVCodecID_AV_CODEC_ID_PCM_S64LE: AVCodecID = 67584;
pub const AVCodecID_AV_CODEC_ID_PCM_S64BE: AVCodecID = 67585;
pub const AVCodecID_AV_CODEC_ID_PCM_F16LE: AVCodecID = 67586;
pub const AVCodecID_AV_CODEC_ID_PCM_F24LE: AVCodecID = 67587;
pub const AVCodecID_AV_CODEC_ID_PCM_VIDC: AVCodecID = 67588;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_QT: AVCodecID = 69632;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_WAV: AVCodecID = 69633;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_DK3: AVCodecID = 69634;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_DK4: AVCodecID = 69635;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_WS: AVCodecID = 69636;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_SMJPEG: AVCodecID = 69637;
pub const AVCodecID_AV_CODEC_ID_ADPCM_MS: AVCodecID = 69638;
pub const AVCodecID_AV_CODEC_ID_ADPCM_4XM: AVCodecID = 69639;
pub const AVCodecID_AV_CODEC_ID_ADPCM_XA: AVCodecID = 69640;
pub const AVCodecID_AV_CODEC_ID_ADPCM_ADX: AVCodecID = 69641;
pub const AVCodecID_AV_CODEC_ID_ADPCM_EA: AVCodecID = 69642;
pub const AVCodecID_AV_CODEC_ID_ADPCM_G726: AVCodecID = 69643;
pub const AVCodecID_AV_CODEC_ID_ADPCM_CT: AVCodecID = 69644;
pub const AVCodecID_AV_CODEC_ID_ADPCM_SWF: AVCodecID = 69645;
pub const AVCodecID_AV_CODEC_ID_ADPCM_YAMAHA: AVCodecID = 69646;
pub const AVCodecID_AV_CODEC_ID_ADPCM_SBPRO_4: AVCodecID = 69647;
pub const AVCodecID_AV_CODEC_ID_ADPCM_SBPRO_3: AVCodecID = 69648;
pub const AVCodecID_AV_CODEC_ID_ADPCM_SBPRO_2: AVCodecID = 69649;
pub const AVCodecID_AV_CODEC_ID_ADPCM_THP: AVCodecID = 69650;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_AMV: AVCodecID = 69651;
pub const AVCodecID_AV_CODEC_ID_ADPCM_EA_R1: AVCodecID = 69652;
pub const AVCodecID_AV_CODEC_ID_ADPCM_EA_R3: AVCodecID = 69653;
pub const AVCodecID_AV_CODEC_ID_ADPCM_EA_R2: AVCodecID = 69654;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_EA_SEAD: AVCodecID = 69655;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_EA_EACS: AVCodecID = 69656;
pub const AVCodecID_AV_CODEC_ID_ADPCM_EA_XAS: AVCodecID = 69657;
pub const AVCodecID_AV_CODEC_ID_ADPCM_EA_MAXIS_XA: AVCodecID = 69658;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_ISS: AVCodecID = 69659;
pub const AVCodecID_AV_CODEC_ID_ADPCM_G722: AVCodecID = 69660;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_APC: AVCodecID = 69661;
pub const AVCodecID_AV_CODEC_ID_ADPCM_VIMA: AVCodecID = 69662;
pub const AVCodecID_AV_CODEC_ID_ADPCM_AFC: AVCodecID = 71680;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_OKI: AVCodecID = 71681;
pub const AVCodecID_AV_CODEC_ID_ADPCM_DTK: AVCodecID = 71682;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_RAD: AVCodecID = 71683;
pub const AVCodecID_AV_CODEC_ID_ADPCM_G726LE: AVCodecID = 71684;
pub const AVCodecID_AV_CODEC_ID_ADPCM_THP_LE: AVCodecID = 71685;
pub const AVCodecID_AV_CODEC_ID_ADPCM_PSX: AVCodecID = 71686;
pub const AVCodecID_AV_CODEC_ID_ADPCM_AICA: AVCodecID = 71687;
pub const AVCodecID_AV_CODEC_ID_ADPCM_IMA_DAT4: AVCodecID = 71688;
pub const AVCodecID_AV_CODEC_ID_ADPCM_MTAF: AVCodecID = 71689;
pub const AVCodecID_AV_CODEC_ID_ADPCM_AGM: AVCodecID = 71690;
pub const AVCodecID_AV_CODEC_ID_AMR_NB: AVCodecID = 73728;
pub const AVCodecID_AV_CODEC_ID_AMR_WB: AVCodecID = 73729;
pub const AVCodecID_AV_CODEC_ID_RA_144: AVCodecID = 77824;
pub const AVCodecID_AV_CODEC_ID_RA_288: AVCodecID = 77825;
pub const AVCodecID_AV_CODEC_ID_ROQ_DPCM: AVCodecID = 81920;
pub const AVCodecID_AV_CODEC_ID_INTERPLAY_DPCM: AVCodecID = 81921;
pub const AVCodecID_AV_CODEC_ID_XAN_DPCM: AVCodecID = 81922;
pub const AVCodecID_AV_CODEC_ID_SOL_DPCM: AVCodecID = 81923;
pub const AVCodecID_AV_CODEC_ID_SDX2_DPCM: AVCodecID = 83968;
pub const AVCodecID_AV_CODEC_ID_GREMLIN_DPCM: AVCodecID = 83969;
pub const AVCodecID_AV_CODEC_ID_MP2: AVCodecID = 86016;
pub const AVCodecID_AV_CODEC_ID_MP3: AVCodecID = 86017;
pub const AVCodecID_AV_CODEC_ID_AAC: AVCodecID = 86018;
pub const AVCodecID_AV_CODEC_ID_AC3: AVCodecID = 86019;
pub const AVCodecID_AV_CODEC_ID_DTS: AVCodecID = 86020;
pub const AVCodecID_AV_CODEC_ID_VORBIS: AVCodecID = 86021;
pub const AVCodecID_AV_CODEC_ID_DVAUDIO: AVCodecID = 86022;
pub const AVCodecID_AV_CODEC_ID_WMAV1: AVCodecID = 86023;
pub const AVCodecID_AV_CODEC_ID_WMAV2: AVCodecID = 86024;
pub const AVCodecID_AV_CODEC_ID_MACE3: AVCodecID = 86025;
pub const AVCodecID_AV_CODEC_ID_MACE6: AVCodecID = 86026;
pub const AVCodecID_AV_CODEC_ID_VMDAUDIO: AVCodecID = 86027;
pub const AVCodecID_AV_CODEC_ID_FLAC: AVCodecID = 86028;
pub const AVCodecID_AV_CODEC_ID_MP3ADU: AVCodecID = 86029;
pub const AVCodecID_AV_CODEC_ID_MP3ON4: AVCodecID = 86030;
pub const AVCodecID_AV_CODEC_ID_SHORTEN: AVCodecID = 86031;
pub const AVCodecID_AV_CODEC_ID_ALAC: AVCodecID = 86032;
pub const AVCodecID_AV_CODEC_ID_WESTWOOD_SND1: AVCodecID = 86033;
pub const AVCodecID_AV_CODEC_ID_GSM: AVCodecID = 86034;
pub const AVCodecID_AV_CODEC_ID_QDM2: AVCodecID = 86035;
pub const AVCodecID_AV_CODEC_ID_COOK: AVCodecID = 86036;
pub const AVCodecID_AV_CODEC_ID_TRUESPEECH: AVCodecID = 86037;
pub const AVCodecID_AV_CODEC_ID_TTA: AVCodecID = 86038;
pub const AVCodecID_AV_CODEC_ID_SMACKAUDIO: AVCodecID = 86039;
pub const AVCodecID_AV_CODEC_ID_QCELP: AVCodecID = 86040;
pub const AVCodecID_AV_CODEC_ID_WAVPACK: AVCodecID = 86041;
pub const AVCodecID_AV_CODEC_ID_DSICINAUDIO: AVCodecID = 86042;
pub const AVCodecID_AV_CODEC_ID_IMC: AVCodecID = 86043;
pub const AVCodecID_AV_CODEC_ID_MUSEPACK7: AVCodecID = 86044;
pub const AVCodecID_AV_CODEC_ID_MLP: AVCodecID = 86045;
pub const AVCodecID_AV_CODEC_ID_GSM_MS: AVCodecID = 86046;
pub const AVCodecID_AV_CODEC_ID_ATRAC3: AVCodecID = 86047;
pub const AVCodecID_AV_CODEC_ID_APE: AVCodecID = 86048;
pub const AVCodecID_AV_CODEC_ID_NELLYMOSER: AVCodecID = 86049;
pub const AVCodecID_AV_CODEC_ID_MUSEPACK8: AVCodecID = 86050;
pub const AVCodecID_AV_CODEC_ID_SPEEX: AVCodecID = 86051;
pub const AVCodecID_AV_CODEC_ID_WMAVOICE: AVCodecID = 86052;
pub const AVCodecID_AV_CODEC_ID_WMAPRO: AVCodecID = 86053;
pub const AVCodecID_AV_CODEC_ID_WMALOSSLESS: AVCodecID = 86054;
pub const AVCodecID_AV_CODEC_ID_ATRAC3P: AVCodecID = 86055;
pub const AVCodecID_AV_CODEC_ID_EAC3: AVCodecID = 86056;
pub const AVCodecID_AV_CODEC_ID_SIPR: AVCodecID = 86057;
pub const AVCodecID_AV_CODEC_ID_MP1: AVCodecID = 86058;
pub const AVCodecID_AV_CODEC_ID_TWINVQ: AVCodecID = 86059;
pub const AVCodecID_AV_CODEC_ID_TRUEHD: AVCodecID = 86060;
pub const AVCodecID_AV_CODEC_ID_MP4ALS: AVCodecID = 86061;
pub const AVCodecID_AV_CODEC_ID_ATRAC1: AVCodecID = 86062;
pub const AVCodecID_AV_CODEC_ID_BINKAUDIO_RDFT: AVCodecID = 86063;
pub const AVCodecID_AV_CODEC_ID_BINKAUDIO_DCT: AVCodecID = 86064;
pub const AVCodecID_AV_CODEC_ID_AAC_LATM: AVCodecID = 86065;
pub const AVCodecID_AV_CODEC_ID_QDMC: AVCodecID = 86066;
pub const AVCodecID_AV_CODEC_ID_CELT: AVCodecID = 86067;
pub const AVCodecID_AV_CODEC_ID_G723_1: AVCodecID = 86068;
pub const AVCodecID_AV_CODEC_ID_G729: AVCodecID = 86069;
pub const AVCodecID_AV_CODEC_ID_8SVX_EXP: AVCodecID = 86070;
pub const AVCodecID_AV_CODEC_ID_8SVX_FIB: AVCodecID = 86071;
pub const AVCodecID_AV_CODEC_ID_BMV_AUDIO: AVCodecID = 86072;
pub const AVCodecID_AV_CODEC_ID_RALF: AVCodecID = 86073;
pub const AVCodecID_AV_CODEC_ID_IAC: AVCodecID = 86074;
pub const AVCodecID_AV_CODEC_ID_ILBC: AVCodecID = 86075;
pub const AVCodecID_AV_CODEC_ID_OPUS: AVCodecID = 86076;
pub const AVCodecID_AV_CODEC_ID_COMFORT_NOISE: AVCodecID = 86077;
pub const AVCodecID_AV_CODEC_ID_TAK: AVCodecID = 86078;
pub const AVCodecID_AV_CODEC_ID_METASOUND: AVCodecID = 86079;
pub const AVCodecID_AV_CODEC_ID_PAF_AUDIO: AVCodecID = 86080;
pub const AVCodecID_AV_CODEC_ID_ON2AVC: AVCodecID = 86081;
pub const AVCodecID_AV_CODEC_ID_DSS_SP: AVCodecID = 86082;
pub const AVCodecID_AV_CODEC_ID_CODEC2: AVCodecID = 86083;
pub const AVCodecID_AV_CODEC_ID_FFWAVESYNTH: AVCodecID = 88064;
pub const AVCodecID_AV_CODEC_ID_SONIC: AVCodecID = 88065;
pub const AVCodecID_AV_CODEC_ID_SONIC_LS: AVCodecID = 88066;
pub const AVCodecID_AV_CODEC_ID_EVRC: AVCodecID = 88067;
pub const AVCodecID_AV_CODEC_ID_SMV: AVCodecID = 88068;
pub const AVCodecID_AV_CODEC_ID_DSD_LSBF: AVCodecID = 88069;
pub const AVCodecID_AV_CODEC_ID_DSD_MSBF: AVCodecID = 88070;
pub const AVCodecID_AV_CODEC_ID_DSD_LSBF_PLANAR: AVCodecID = 88071;
pub const AVCodecID_AV_CODEC_ID_DSD_MSBF_PLANAR: AVCodecID = 88072;
pub const AVCodecID_AV_CODEC_ID_4GV: AVCodecID = 88073;
pub const AVCodecID_AV_CODEC_ID_INTERPLAY_ACM: AVCodecID = 88074;
pub const AVCodecID_AV_CODEC_ID_XMA1: AVCodecID = 88075;
pub const AVCodecID_AV_CODEC_ID_XMA2: AVCodecID = 88076;
pub const AVCodecID_AV_CODEC_ID_DST: AVCodecID = 88077;
pub const AVCodecID_AV_CODEC_ID_ATRAC3AL: AVCodecID = 88078;
pub const AVCodecID_AV_CODEC_ID_ATRAC3PAL: AVCodecID = 88079;
pub const AVCodecID_AV_CODEC_ID_DOLBY_E: AVCodecID = 88080;
pub const AVCodecID_AV_CODEC_ID_APTX: AVCodecID = 88081;
pub const AVCodecID_AV_CODEC_ID_APTX_HD: AVCodecID = 88082;
pub const AVCodecID_AV_CODEC_ID_SBC: AVCodecID = 88083;
pub const AVCodecID_AV_CODEC_ID_ATRAC9: AVCodecID = 88084;
pub const AVCodecID_AV_CODEC_ID_HCOM: AVCodecID = 88085;
pub const AVCodecID_AV_CODEC_ID_FIRST_SUBTITLE: AVCodecID = 94208;
pub const AVCodecID_AV_CODEC_ID_DVD_SUBTITLE: AVCodecID = 94208;
pub const AVCodecID_AV_CODEC_ID_DVB_SUBTITLE: AVCodecID = 94209;
pub const AVCodecID_AV_CODEC_ID_TEXT: AVCodecID = 94210;
pub const AVCodecID_AV_CODEC_ID_XSUB: AVCodecID = 94211;
pub const AVCodecID_AV_CODEC_ID_SSA: AVCodecID = 94212;
pub const AVCodecID_AV_CODEC_ID_MOV_TEXT: AVCodecID = 94213;
pub const AVCodecID_AV_CODEC_ID_HDMV_PGS_SUBTITLE: AVCodecID = 94214;
pub const AVCodecID_AV_CODEC_ID_DVB_TELETEXT: AVCodecID = 94215;
pub const AVCodecID_AV_CODEC_ID_SRT: AVCodecID = 94216;
pub const AVCodecID_AV_CODEC_ID_MICRODVD: AVCodecID = 96256;
pub const AVCodecID_AV_CODEC_ID_EIA_608: AVCodecID = 96257;
pub const AVCodecID_AV_CODEC_ID_JACOSUB: AVCodecID = 96258;
pub const AVCodecID_AV_CODEC_ID_SAMI: AVCodecID = 96259;
pub const AVCodecID_AV_CODEC_ID_REALTEXT: AVCodecID = 96260;
pub const AVCodecID_AV_CODEC_ID_STL: AVCodecID = 96261;
pub const AVCodecID_AV_CODEC_ID_SUBVIEWER1: AVCodecID = 96262;
pub const AVCodecID_AV_CODEC_ID_SUBVIEWER: AVCodecID = 96263;
pub const AVCodecID_AV_CODEC_ID_SUBRIP: AVCodecID = 96264;
pub const AVCodecID_AV_CODEC_ID_WEBVTT: AVCodecID = 96265;
pub const AVCodecID_AV_CODEC_ID_MPL2: AVCodecID = 96266;
pub const AVCodecID_AV_CODEC_ID_VPLAYER: AVCodecID = 96267;
pub const AVCodecID_AV_CODEC_ID_PJS: AVCodecID = 96268;
pub const AVCodecID_AV_CODEC_ID_ASS: AVCodecID = 96269;
pub const AVCodecID_AV_CODEC_ID_HDMV_TEXT_SUBTITLE: AVCodecID = 96270;
pub const AVCodecID_AV_CODEC_ID_TTML: AVCodecID = 96271;
pub const AVCodecID_AV_CODEC_ID_ARIB_CAPTION: AVCodecID = 96272;
pub const AVCodecID_AV_CODEC_ID_FIRST_UNKNOWN: AVCodecID = 98304;
pub const AVCodecID_AV_CODEC_ID_TTF: AVCodecID = 98304;
pub const AVCodecID_AV_CODEC_ID_SCTE_35: AVCodecID = 98305;
pub const AVCodecID_AV_CODEC_ID_BINTEXT: AVCodecID = 100352;
pub const AVCodecID_AV_CODEC_ID_XBIN: AVCodecID = 100353;
pub const AVCodecID_AV_CODEC_ID_IDF: AVCodecID = 100354;
pub const AVCodecID_AV_CODEC_ID_OTF: AVCodecID = 100355;
pub const AVCodecID_AV_CODEC_ID_SMPTE_KLV: AVCodecID = 100356;
pub const AVCodecID_AV_CODEC_ID_DVD_NAV: AVCodecID = 100357;
pub const AVCodecID_AV_CODEC_ID_TIMED_ID3: AVCodecID = 100358;
pub const AVCodecID_AV_CODEC_ID_BIN_DATA: AVCodecID = 100359;
pub const AVCodecID_AV_CODEC_ID_PROBE: AVCodecID = 102400;
pub const AVCodecID_AV_CODEC_ID_MPEG2TS: AVCodecID = 131072;
pub const AVCodecID_AV_CODEC_ID_MPEG4SYSTEMS: AVCodecID = 131073;
pub const AVCodecID_AV_CODEC_ID_FFMETADATA: AVCodecID = 135168;
pub const AVCodecID_AV_CODEC_ID_WRAPPED_AVFRAME: AVCodecID = 135169;
pub type AVCodecID = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecDescriptor {
    pub id: AVCodecID,
    pub type_: AVMediaType,
    pub name: *const ::std::os::raw::c_char,
    pub long_name: *const ::std::os::raw::c_char,
    pub props: ::std::os::raw::c_int,
    pub mime_types: *const *const ::std::os::raw::c_char,
    pub profiles: *const AVProfile,
}
pub const AVDiscard_AVDISCARD_NONE: AVDiscard = -16;
pub const AVDiscard_AVDISCARD_DEFAULT: AVDiscard = 0;
pub const AVDiscard_AVDISCARD_NONREF: AVDiscard = 8;
pub const AVDiscard_AVDISCARD_BIDIR: AVDiscard = 16;
pub const AVDiscard_AVDISCARD_NONINTRA: AVDiscard = 24;
pub const AVDiscard_AVDISCARD_NONKEY: AVDiscard = 32;
pub const AVDiscard_AVDISCARD_ALL: AVDiscard = 48;
pub type AVDiscard = i32;
pub const AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_MAIN: AVAudioServiceType = 0;
pub const AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_EFFECTS: AVAudioServiceType = 1;
pub const AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED: AVAudioServiceType = 2;
pub const AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED: AVAudioServiceType = 3;
pub const AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_DIALOGUE: AVAudioServiceType = 4;
pub const AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_COMMENTARY: AVAudioServiceType = 5;
pub const AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_EMERGENCY: AVAudioServiceType = 6;
pub const AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_VOICE_OVER: AVAudioServiceType = 7;
pub const AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_KARAOKE: AVAudioServiceType = 8;
pub const AVAudioServiceType_AV_AUDIO_SERVICE_TYPE_NB: AVAudioServiceType = 9;
pub type AVAudioServiceType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RcOverride {
    pub start_frame: ::std::os::raw::c_int,
    pub end_frame: ::std::os::raw::c_int,
    pub qscale: ::std::os::raw::c_int,
    pub quality_factor: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVPanScan {
    pub id: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub position: [[i16; 2usize]; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCPBProperties {
    pub max_bitrate: ::std::os::raw::c_int,
    pub min_bitrate: ::std::os::raw::c_int,
    pub avg_bitrate: ::std::os::raw::c_int,
    pub buffer_size: ::std::os::raw::c_int,
    pub vbv_delay: u64,
}
pub const AVPacketSideDataType_AV_PKT_DATA_PALETTE: AVPacketSideDataType = 0;
pub const AVPacketSideDataType_AV_PKT_DATA_NEW_EXTRADATA: AVPacketSideDataType = 1;
pub const AVPacketSideDataType_AV_PKT_DATA_PARAM_CHANGE: AVPacketSideDataType = 2;
pub const AVPacketSideDataType_AV_PKT_DATA_H263_MB_INFO: AVPacketSideDataType = 3;
pub const AVPacketSideDataType_AV_PKT_DATA_REPLAYGAIN: AVPacketSideDataType = 4;
pub const AVPacketSideDataType_AV_PKT_DATA_DISPLAYMATRIX: AVPacketSideDataType = 5;
pub const AVPacketSideDataType_AV_PKT_DATA_STEREO3D: AVPacketSideDataType = 6;
pub const AVPacketSideDataType_AV_PKT_DATA_AUDIO_SERVICE_TYPE: AVPacketSideDataType = 7;
pub const AVPacketSideDataType_AV_PKT_DATA_QUALITY_STATS: AVPacketSideDataType = 8;
pub const AVPacketSideDataType_AV_PKT_DATA_FALLBACK_TRACK: AVPacketSideDataType = 9;
pub const AVPacketSideDataType_AV_PKT_DATA_CPB_PROPERTIES: AVPacketSideDataType = 10;
pub const AVPacketSideDataType_AV_PKT_DATA_SKIP_SAMPLES: AVPacketSideDataType = 11;
pub const AVPacketSideDataType_AV_PKT_DATA_JP_DUALMONO: AVPacketSideDataType = 12;
pub const AVPacketSideDataType_AV_PKT_DATA_STRINGS_METADATA: AVPacketSideDataType = 13;
pub const AVPacketSideDataType_AV_PKT_DATA_SUBTITLE_POSITION: AVPacketSideDataType = 14;
pub const AVPacketSideDataType_AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: AVPacketSideDataType = 15;
pub const AVPacketSideDataType_AV_PKT_DATA_WEBVTT_IDENTIFIER: AVPacketSideDataType = 16;
pub const AVPacketSideDataType_AV_PKT_DATA_WEBVTT_SETTINGS: AVPacketSideDataType = 17;
pub const AVPacketSideDataType_AV_PKT_DATA_METADATA_UPDATE: AVPacketSideDataType = 18;
pub const AVPacketSideDataType_AV_PKT_DATA_MPEGTS_STREAM_ID: AVPacketSideDataType = 19;
pub const AVPacketSideDataType_AV_PKT_DATA_MASTERING_DISPLAY_METADATA: AVPacketSideDataType = 20;
pub const AVPacketSideDataType_AV_PKT_DATA_SPHERICAL: AVPacketSideDataType = 21;
pub const AVPacketSideDataType_AV_PKT_DATA_CONTENT_LIGHT_LEVEL: AVPacketSideDataType = 22;
pub const AVPacketSideDataType_AV_PKT_DATA_A53_CC: AVPacketSideDataType = 23;
pub const AVPacketSideDataType_AV_PKT_DATA_ENCRYPTION_INIT_INFO: AVPacketSideDataType = 24;
pub const AVPacketSideDataType_AV_PKT_DATA_ENCRYPTION_INFO: AVPacketSideDataType = 25;
pub const AVPacketSideDataType_AV_PKT_DATA_AFD: AVPacketSideDataType = 26;
pub const AVPacketSideDataType_AV_PKT_DATA_NB: AVPacketSideDataType = 27;
pub type AVPacketSideDataType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVPacketSideData {
    pub data: *mut u8,
    pub size: ::std::os::raw::c_int,
    pub type_: AVPacketSideDataType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: i64,
    pub dts: i64,
    pub data: *mut u8,
    pub size: ::std::os::raw::c_int,
    pub stream_index: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: ::std::os::raw::c_int,
    pub duration: i64,
    pub pos: i64,
    pub convergence_duration: i64,
}
pub const AVSideDataParamChangeFlags_AV_SIDE_DATA_PARAM_CHANGE_CHANNEL_COUNT:
    AVSideDataParamChangeFlags = 1;
pub const AVSideDataParamChangeFlags_AV_SIDE_DATA_PARAM_CHANGE_CHANNEL_LAYOUT:
    AVSideDataParamChangeFlags = 2;
pub const AVSideDataParamChangeFlags_AV_SIDE_DATA_PARAM_CHANGE_SAMPLE_RATE:
    AVSideDataParamChangeFlags = 4;
pub const AVSideDataParamChangeFlags_AV_SIDE_DATA_PARAM_CHANGE_DIMENSIONS:
    AVSideDataParamChangeFlags = 8;
pub type AVSideDataParamChangeFlags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecInternal {
    _unused: [u8; 0],
}
pub const AVFieldOrder_AV_FIELD_UNKNOWN: AVFieldOrder = 0;
pub const AVFieldOrder_AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
pub const AVFieldOrder_AV_FIELD_TT: AVFieldOrder = 2;
pub const AVFieldOrder_AV_FIELD_BB: AVFieldOrder = 3;
pub const AVFieldOrder_AV_FIELD_TB: AVFieldOrder = 4;
pub const AVFieldOrder_AV_FIELD_BT: AVFieldOrder = 5;
pub type AVFieldOrder = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecContext {
    pub av_class: *const AVClass,
    pub log_level_offset: ::std::os::raw::c_int,
    pub codec_type: AVMediaType,
    pub codec: *const AVCodec,
    pub codec_id: AVCodecID,
    pub codec_tag: ::std::os::raw::c_uint,
    pub priv_data: *mut ::std::os::raw::c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut ::std::os::raw::c_void,
    pub bit_rate: i64,
    pub bit_rate_tolerance: ::std::os::raw::c_int,
    pub global_quality: ::std::os::raw::c_int,
    pub compression_level: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub flags2: ::std::os::raw::c_int,
    pub extradata: *mut u8,
    pub extradata_size: ::std::os::raw::c_int,
    pub time_base: AVRational,
    pub ticks_per_frame: ::std::os::raw::c_int,
    pub delay: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub coded_width: ::std::os::raw::c_int,
    pub coded_height: ::std::os::raw::c_int,
    pub gop_size: ::std::os::raw::c_int,
    pub pix_fmt: AVPixelFormat,
    pub draw_horiz_band: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVCodecContext,
            src: *const AVFrame,
            offset: *mut ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            type_: ::std::os::raw::c_int,
            height: ::std::os::raw::c_int,
        ),
    >,
    pub get_format: ::std::option::Option<
        unsafe extern "C" fn(s: *mut AVCodecContext, fmt: *const AVPixelFormat) -> AVPixelFormat,
    >,
    pub max_b_frames: ::std::os::raw::c_int,
    pub b_quant_factor: f32,
    pub b_frame_strategy: ::std::os::raw::c_int,
    pub b_quant_offset: f32,
    pub has_b_frames: ::std::os::raw::c_int,
    pub mpeg_quant: ::std::os::raw::c_int,
    pub i_quant_factor: f32,
    pub i_quant_offset: f32,
    pub lumi_masking: f32,
    pub temporal_cplx_masking: f32,
    pub spatial_cplx_masking: f32,
    pub p_masking: f32,
    pub dark_masking: f32,
    pub slice_count: ::std::os::raw::c_int,
    pub prediction_method: ::std::os::raw::c_int,
    pub slice_offset: *mut ::std::os::raw::c_int,
    pub sample_aspect_ratio: AVRational,
    pub me_cmp: ::std::os::raw::c_int,
    pub me_sub_cmp: ::std::os::raw::c_int,
    pub mb_cmp: ::std::os::raw::c_int,
    pub ildct_cmp: ::std::os::raw::c_int,
    pub dia_size: ::std::os::raw::c_int,
    pub last_predictor_count: ::std::os::raw::c_int,
    pub pre_me: ::std::os::raw::c_int,
    pub me_pre_cmp: ::std::os::raw::c_int,
    pub pre_dia_size: ::std::os::raw::c_int,
    pub me_subpel_quality: ::std::os::raw::c_int,
    pub me_range: ::std::os::raw::c_int,
    pub slice_flags: ::std::os::raw::c_int,
    pub mb_decision: ::std::os::raw::c_int,
    pub intra_matrix: *mut u16,
    pub inter_matrix: *mut u16,
    pub scenechange_threshold: ::std::os::raw::c_int,
    pub noise_reduction: ::std::os::raw::c_int,
    pub intra_dc_precision: ::std::os::raw::c_int,
    pub skip_top: ::std::os::raw::c_int,
    pub skip_bottom: ::std::os::raw::c_int,
    pub mb_lmin: ::std::os::raw::c_int,
    pub mb_lmax: ::std::os::raw::c_int,
    pub me_penalty_compensation: ::std::os::raw::c_int,
    pub bidir_refine: ::std::os::raw::c_int,
    pub brd_scale: ::std::os::raw::c_int,
    pub keyint_min: ::std::os::raw::c_int,
    pub refs: ::std::os::raw::c_int,
    pub chromaoffset: ::std::os::raw::c_int,
    pub mv0_threshold: ::std::os::raw::c_int,
    pub b_sensitivity: ::std::os::raw::c_int,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub chroma_sample_location: AVChromaLocation,
    pub slices: ::std::os::raw::c_int,
    pub field_order: AVFieldOrder,
    pub sample_rate: ::std::os::raw::c_int,
    pub channels: ::std::os::raw::c_int,
    pub sample_fmt: AVSampleFormat,
    pub frame_size: ::std::os::raw::c_int,
    pub frame_number: ::std::os::raw::c_int,
    pub block_align: ::std::os::raw::c_int,
    pub cutoff: ::std::os::raw::c_int,
    pub channel_layout: u64,
    pub request_channel_layout: u64,
    pub audio_service_type: AVAudioServiceType,
    pub request_sample_fmt: AVSampleFormat,
    pub get_buffer2: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVCodecContext,
            frame: *mut AVFrame,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub refcounted_frames: ::std::os::raw::c_int,
    pub qcompress: f32,
    pub qblur: f32,
    pub qmin: ::std::os::raw::c_int,
    pub qmax: ::std::os::raw::c_int,
    pub max_qdiff: ::std::os::raw::c_int,
    pub rc_buffer_size: ::std::os::raw::c_int,
    pub rc_override_count: ::std::os::raw::c_int,
    pub rc_override: *mut RcOverride,
    pub rc_max_rate: i64,
    pub rc_min_rate: i64,
    pub rc_max_available_vbv_use: f32,
    pub rc_min_vbv_overflow_use: f32,
    pub rc_initial_buffer_occupancy: ::std::os::raw::c_int,
    pub coder_type: ::std::os::raw::c_int,
    pub context_model: ::std::os::raw::c_int,
    pub frame_skip_threshold: ::std::os::raw::c_int,
    pub frame_skip_factor: ::std::os::raw::c_int,
    pub frame_skip_exp: ::std::os::raw::c_int,
    pub frame_skip_cmp: ::std::os::raw::c_int,
    pub trellis: ::std::os::raw::c_int,
    pub min_prediction_order: ::std::os::raw::c_int,
    pub max_prediction_order: ::std::os::raw::c_int,
    pub timecode_frame_start: i64,
    pub rtp_callback: ::std::option::Option<
        unsafe extern "C" fn(
            avctx: *mut AVCodecContext,
            data: *mut ::std::os::raw::c_void,
            size: ::std::os::raw::c_int,
            mb_nb: ::std::os::raw::c_int,
        ),
    >,
    pub rtp_payload_size: ::std::os::raw::c_int,
    pub mv_bits: ::std::os::raw::c_int,
    pub header_bits: ::std::os::raw::c_int,
    pub i_tex_bits: ::std::os::raw::c_int,
    pub p_tex_bits: ::std::os::raw::c_int,
    pub i_count: ::std::os::raw::c_int,
    pub p_count: ::std::os::raw::c_int,
    pub skip_count: ::std::os::raw::c_int,
    pub misc_bits: ::std::os::raw::c_int,
    pub frame_bits: ::std::os::raw::c_int,
    pub stats_out: *mut ::std::os::raw::c_char,
    pub stats_in: *mut ::std::os::raw::c_char,
    pub workaround_bugs: ::std::os::raw::c_int,
    pub strict_std_compliance: ::std::os::raw::c_int,
    pub error_concealment: ::std::os::raw::c_int,
    pub debug: ::std::os::raw::c_int,
    pub err_recognition: ::std::os::raw::c_int,
    pub reordered_opaque: i64,
    pub hwaccel: *const AVHWAccel,
    pub hwaccel_context: *mut ::std::os::raw::c_void,
    pub error: [u64; 8usize],
    pub dct_algo: ::std::os::raw::c_int,
    pub idct_algo: ::std::os::raw::c_int,
    pub bits_per_coded_sample: ::std::os::raw::c_int,
    pub bits_per_raw_sample: ::std::os::raw::c_int,
    pub lowres: ::std::os::raw::c_int,
    pub coded_frame: *mut AVFrame,
    pub thread_count: ::std::os::raw::c_int,
    pub thread_type: ::std::os::raw::c_int,
    pub active_thread_type: ::std::os::raw::c_int,
    pub thread_safe_callbacks: ::std::os::raw::c_int,
    pub execute: ::std::option::Option<
        unsafe extern "C" fn(
            c: *mut AVCodecContext,
            func: ::std::option::Option<
                unsafe extern "C" fn(
                    c2: *mut AVCodecContext,
                    arg: *mut ::std::os::raw::c_void,
                ) -> ::std::os::raw::c_int,
            >,
            arg2: *mut ::std::os::raw::c_void,
            ret: *mut ::std::os::raw::c_int,
            count: ::std::os::raw::c_int,
            size: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub execute2: ::std::option::Option<
        unsafe extern "C" fn(
            c: *mut AVCodecContext,
            func: ::std::option::Option<
                unsafe extern "C" fn(
                    c2: *mut AVCodecContext,
                    arg: *mut ::std::os::raw::c_void,
                    jobnr: ::std::os::raw::c_int,
                    threadnr: ::std::os::raw::c_int,
                ) -> ::std::os::raw::c_int,
            >,
            arg2: *mut ::std::os::raw::c_void,
            ret: *mut ::std::os::raw::c_int,
            count: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub nsse_weight: ::std::os::raw::c_int,
    pub profile: ::std::os::raw::c_int,
    pub level: ::std::os::raw::c_int,
    pub skip_loop_filter: AVDiscard,
    pub skip_idct: AVDiscard,
    pub skip_frame: AVDiscard,
    pub subtitle_header: *mut u8,
    pub subtitle_header_size: ::std::os::raw::c_int,
    pub vbv_delay: u64,
    pub side_data_only_packets: ::std::os::raw::c_int,
    pub initial_padding: ::std::os::raw::c_int,
    pub framerate: AVRational,
    pub sw_pix_fmt: AVPixelFormat,
    pub pkt_timebase: AVRational,
    pub codec_descriptor: *const AVCodecDescriptor,
    pub pts_correction_num_faulty_pts: i64,
    pub pts_correction_num_faulty_dts: i64,
    pub pts_correction_last_pts: i64,
    pub pts_correction_last_dts: i64,
    pub sub_charenc: *mut ::std::os::raw::c_char,
    pub sub_charenc_mode: ::std::os::raw::c_int,
    pub skip_alpha: ::std::os::raw::c_int,
    pub seek_preroll: ::std::os::raw::c_int,
    pub debug_mv: ::std::os::raw::c_int,
    pub chroma_intra_matrix: *mut u16,
    pub dump_separator: *mut u8,
    pub codec_whitelist: *mut ::std::os::raw::c_char,
    pub properties: ::std::os::raw::c_uint,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: ::std::os::raw::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub sub_text_format: ::std::os::raw::c_int,
    pub trailing_padding: ::std::os::raw::c_int,
    pub max_pixels: i64,
    pub hw_device_ctx: *mut AVBufferRef,
    pub hwaccel_flags: ::std::os::raw::c_int,
    pub apply_cropping: ::std::os::raw::c_int,
    pub extra_hw_frames: ::std::os::raw::c_int,
    pub discard_damaged_percentage: ::std::os::raw::c_int,
}
extern "C" {
    pub fn av_codec_get_pkt_timebase(avctx: *const AVCodecContext) -> AVRational;
}
extern "C" {
    pub fn av_codec_set_pkt_timebase(avctx: *mut AVCodecContext, val: AVRational);
}
extern "C" {
    pub fn av_codec_get_codec_descriptor(avctx: *const AVCodecContext) -> *const AVCodecDescriptor;
}
extern "C" {
    pub fn av_codec_set_codec_descriptor(
        avctx: *mut AVCodecContext,
        desc: *const AVCodecDescriptor,
    );
}
extern "C" {
    pub fn av_codec_get_codec_properties(avctx: *const AVCodecContext) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn av_codec_get_lowres(avctx: *const AVCodecContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_codec_set_lowres(avctx: *mut AVCodecContext, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_codec_get_seek_preroll(avctx: *const AVCodecContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_codec_set_seek_preroll(avctx: *mut AVCodecContext, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_codec_get_chroma_intra_matrix(avctx: *const AVCodecContext) -> *mut u16;
}
extern "C" {
    pub fn av_codec_set_chroma_intra_matrix(avctx: *mut AVCodecContext, val: *mut u16);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVProfile {
    pub profile: ::std::os::raw::c_int,
    pub name: *const ::std::os::raw::c_char,
}
pub const AV_CODEC_HW_CONFIG_METHOD_HW_DEVICE_CTX: _bindgen_ty_5 = 1;
pub const AV_CODEC_HW_CONFIG_METHOD_HW_FRAMES_CTX: _bindgen_ty_5 = 2;
pub const AV_CODEC_HW_CONFIG_METHOD_INTERNAL: _bindgen_ty_5 = 4;
pub const AV_CODEC_HW_CONFIG_METHOD_AD_HOC: _bindgen_ty_5 = 8;
pub type _bindgen_ty_5 = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecHWConfig {
    pub pix_fmt: AVPixelFormat,
    pub methods: ::std::os::raw::c_int,
    pub device_type: AVHWDeviceType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecDefault {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodec {
    pub name: *const ::std::os::raw::c_char,
    pub long_name: *const ::std::os::raw::c_char,
    pub type_: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: ::std::os::raw::c_int,
    pub supported_framerates: *const AVRational,
    pub pix_fmts: *const AVPixelFormat,
    pub supported_samplerates: *const ::std::os::raw::c_int,
    pub sample_fmts: *const AVSampleFormat,
    pub channel_layouts: *const u64,
    pub max_lowres: u8,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const ::std::os::raw::c_char,
    pub priv_data_size: ::std::os::raw::c_int,
    pub next: *mut AVCodec,
    pub init_thread_copy: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AVCodecContext) -> ::std::os::raw::c_int,
    >,
    pub update_thread_context: ::std::option::Option<
        unsafe extern "C" fn(
            dst: *mut AVCodecContext,
            src: *const AVCodecContext,
        ) -> ::std::os::raw::c_int,
    >,
    pub defaults: *const AVCodecDefault,
    pub init_static_data: ::std::option::Option<unsafe extern "C" fn(codec: *mut AVCodec)>,
    pub init: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AVCodecContext) -> ::std::os::raw::c_int,
    >,
    pub encode_sub: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AVCodecContext,
            buf: *mut u8,
            buf_size: ::std::os::raw::c_int,
            sub: *const AVSubtitle,
        ) -> ::std::os::raw::c_int,
    >,
    pub encode2: ::std::option::Option<
        unsafe extern "C" fn(
            avctx: *mut AVCodecContext,
            avpkt: *mut AVPacket,
            frame: *const AVFrame,
            got_packet_ptr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub decode: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AVCodecContext,
            outdata: *mut ::std::os::raw::c_void,
            outdata_size: *mut ::std::os::raw::c_int,
            avpkt: *mut AVPacket,
        ) -> ::std::os::raw::c_int,
    >,
    pub close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AVCodecContext) -> ::std::os::raw::c_int,
    >,
    pub send_frame: ::std::option::Option<
        unsafe extern "C" fn(
            avctx: *mut AVCodecContext,
            frame: *const AVFrame,
        ) -> ::std::os::raw::c_int,
    >,
    pub receive_packet: ::std::option::Option<
        unsafe extern "C" fn(
            avctx: *mut AVCodecContext,
            avpkt: *mut AVPacket,
        ) -> ::std::os::raw::c_int,
    >,
    pub receive_frame: ::std::option::Option<
        unsafe extern "C" fn(
            avctx: *mut AVCodecContext,
            frame: *mut AVFrame,
        ) -> ::std::os::raw::c_int,
    >,
    pub flush: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVCodecContext)>,
    pub caps_internal: ::std::os::raw::c_int,
    pub bsfs: *const ::std::os::raw::c_char,
    pub hw_configs: *mut *mut AVCodecHWConfigInternal,
}
extern "C" {
    pub fn av_codec_get_max_lowres(codec: *const AVCodec) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MpegEncContext {
    _unused: [u8; 0],
}
extern "C" {
    pub fn avcodec_get_hw_config(
        codec: *const AVCodec,
        index: ::std::os::raw::c_int,
    ) -> *const AVCodecHWConfig;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVHWAccel {
    pub name: *const ::std::os::raw::c_char,
    pub type_: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: ::std::os::raw::c_int,
    pub alloc_frame: ::std::option::Option<
        unsafe extern "C" fn(
            avctx: *mut AVCodecContext,
            frame: *mut AVFrame,
        ) -> ::std::os::raw::c_int,
    >,
    pub start_frame: ::std::option::Option<
        unsafe extern "C" fn(
            avctx: *mut AVCodecContext,
            buf: *const u8,
            buf_size: u32,
        ) -> ::std::os::raw::c_int,
    >,
    pub decode_params: ::std::option::Option<
        unsafe extern "C" fn(
            avctx: *mut AVCodecContext,
            type_: ::std::os::raw::c_int,
            buf: *const u8,
            buf_size: u32,
        ) -> ::std::os::raw::c_int,
    >,
    pub decode_slice: ::std::option::Option<
        unsafe extern "C" fn(
            avctx: *mut AVCodecContext,
            buf: *const u8,
            buf_size: u32,
        ) -> ::std::os::raw::c_int,
    >,
    pub end_frame: ::std::option::Option<
        unsafe extern "C" fn(avctx: *mut AVCodecContext) -> ::std::os::raw::c_int,
    >,
    pub frame_priv_data_size: ::std::os::raw::c_int,
    pub decode_mb: ::std::option::Option<unsafe extern "C" fn(s: *mut MpegEncContext)>,
    pub init: ::std::option::Option<
        unsafe extern "C" fn(avctx: *mut AVCodecContext) -> ::std::os::raw::c_int,
    >,
    pub uninit: ::std::option::Option<
        unsafe extern "C" fn(avctx: *mut AVCodecContext) -> ::std::os::raw::c_int,
    >,
    pub priv_data_size: ::std::os::raw::c_int,
    pub caps_internal: ::std::os::raw::c_int,
    pub frame_params: ::std::option::Option<
        unsafe extern "C" fn(
            avctx: *mut AVCodecContext,
            hw_frames_ctx: *mut AVBufferRef,
        ) -> ::std::os::raw::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVPicture {
    pub data: [*mut u8; 8usize],
    pub linesize: [::std::os::raw::c_int; 8usize],
}
pub const AVSubtitleType_SUBTITLE_NONE: AVSubtitleType = 0;
pub const AVSubtitleType_SUBTITLE_BITMAP: AVSubtitleType = 1;
pub const AVSubtitleType_SUBTITLE_TEXT: AVSubtitleType = 2;
pub const AVSubtitleType_SUBTITLE_ASS: AVSubtitleType = 3;
pub type AVSubtitleType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVSubtitleRect {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub w: ::std::os::raw::c_int,
    pub h: ::std::os::raw::c_int,
    pub nb_colors: ::std::os::raw::c_int,
    pub pict: AVPicture,
    pub data: [*mut u8; 4usize],
    pub linesize: [::std::os::raw::c_int; 4usize],
    pub type_: AVSubtitleType,
    pub text: *mut ::std::os::raw::c_char,
    pub ass: *mut ::std::os::raw::c_char,
    pub flags: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVSubtitle {
    pub format: u16,
    pub start_display_time: u32,
    pub end_display_time: u32,
    pub num_rects: ::std::os::raw::c_uint,
    pub rects: *mut *mut AVSubtitleRect,
    pub pts: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecParameters {
    pub codec_type: AVMediaType,
    pub codec_id: AVCodecID,
    pub codec_tag: u32,
    pub extradata: *mut u8,
    pub extradata_size: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
    pub bit_rate: i64,
    pub bits_per_coded_sample: ::std::os::raw::c_int,
    pub bits_per_raw_sample: ::std::os::raw::c_int,
    pub profile: ::std::os::raw::c_int,
    pub level: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub sample_aspect_ratio: AVRational,
    pub field_order: AVFieldOrder,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub video_delay: ::std::os::raw::c_int,
    pub channel_layout: u64,
    pub channels: ::std::os::raw::c_int,
    pub sample_rate: ::std::os::raw::c_int,
    pub block_align: ::std::os::raw::c_int,
    pub frame_size: ::std::os::raw::c_int,
    pub initial_padding: ::std::os::raw::c_int,
    pub trailing_padding: ::std::os::raw::c_int,
    pub seek_preroll: ::std::os::raw::c_int,
}
extern "C" {
    pub fn av_codec_iterate(opaque: *mut *mut ::std::os::raw::c_void) -> *const AVCodec;
}
extern "C" {
    pub fn av_codec_next(c: *const AVCodec) -> *mut AVCodec;
}
extern "C" {
    pub fn avcodec_version() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn avcodec_configuration() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avcodec_license() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avcodec_register(codec: *mut AVCodec);
}
extern "C" {
    pub fn avcodec_register_all();
}
extern "C" {
    pub fn avcodec_alloc_context3(codec: *const AVCodec) -> *mut AVCodecContext;
}
extern "C" {
    pub fn avcodec_free_context(avctx: *mut *mut AVCodecContext);
}
extern "C" {
    pub fn avcodec_get_context_defaults3(
        s: *mut AVCodecContext,
        codec: *const AVCodec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_get_class() -> *const AVClass;
}
extern "C" {
    pub fn avcodec_get_frame_class() -> *const AVClass;
}
extern "C" {
    pub fn avcodec_get_subtitle_rect_class() -> *const AVClass;
}
extern "C" {
    pub fn avcodec_copy_context(
        dest: *mut AVCodecContext,
        src: *const AVCodecContext,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_parameters_alloc() -> *mut AVCodecParameters;
}
extern "C" {
    pub fn avcodec_parameters_free(par: *mut *mut AVCodecParameters);
}
extern "C" {
    pub fn avcodec_parameters_copy(
        dst: *mut AVCodecParameters,
        src: *const AVCodecParameters,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_parameters_from_context(
        par: *mut AVCodecParameters,
        codec: *const AVCodecContext,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_parameters_to_context(
        codec: *mut AVCodecContext,
        par: *const AVCodecParameters,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_open2(
        avctx: *mut AVCodecContext,
        codec: *const AVCodec,
        options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_close(avctx: *mut AVCodecContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avsubtitle_free(sub: *mut AVSubtitle);
}
extern "C" {
    pub fn av_packet_alloc() -> *mut AVPacket;
}
extern "C" {
    pub fn av_packet_clone(src: *const AVPacket) -> *mut AVPacket;
}
extern "C" {
    pub fn av_packet_free(pkt: *mut *mut AVPacket);
}
extern "C" {
    pub fn av_init_packet(pkt: *mut AVPacket);
}
extern "C" {
    pub fn av_new_packet(pkt: *mut AVPacket, size: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_shrink_packet(pkt: *mut AVPacket, size: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_grow_packet(
        pkt: *mut AVPacket,
        grow_by: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_packet_from_data(
        pkt: *mut AVPacket,
        data: *mut u8,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_dup_packet(pkt: *mut AVPacket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_copy_packet(dst: *mut AVPacket, src: *const AVPacket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_copy_packet_side_data(
        dst: *mut AVPacket,
        src: *const AVPacket,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_free_packet(pkt: *mut AVPacket);
}
extern "C" {
    pub fn av_packet_new_side_data(
        pkt: *mut AVPacket,
        type_: AVPacketSideDataType,
        size: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn av_packet_add_side_data(
        pkt: *mut AVPacket,
        type_: AVPacketSideDataType,
        data: *mut u8,
        size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_packet_shrink_side_data(
        pkt: *mut AVPacket,
        type_: AVPacketSideDataType,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_packet_get_side_data(
        pkt: *const AVPacket,
        type_: AVPacketSideDataType,
        size: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn av_packet_merge_side_data(pkt: *mut AVPacket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_packet_split_side_data(pkt: *mut AVPacket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_packet_side_data_name(type_: AVPacketSideDataType) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_packet_pack_dictionary(
        dict: *mut AVDictionary,
        size: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn av_packet_unpack_dictionary(
        data: *const u8,
        size: ::std::os::raw::c_int,
        dict: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_packet_free_side_data(pkt: *mut AVPacket);
}
extern "C" {
    pub fn av_packet_ref(dst: *mut AVPacket, src: *const AVPacket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_packet_unref(pkt: *mut AVPacket);
}
extern "C" {
    pub fn av_packet_move_ref(dst: *mut AVPacket, src: *mut AVPacket);
}
extern "C" {
    pub fn av_packet_copy_props(dst: *mut AVPacket, src: *const AVPacket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_packet_make_refcounted(pkt: *mut AVPacket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_packet_make_writable(pkt: *mut AVPacket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_packet_rescale_ts(pkt: *mut AVPacket, tb_src: AVRational, tb_dst: AVRational);
}
extern "C" {
    pub fn avcodec_find_decoder(id: AVCodecID) -> *mut AVCodec;
}
extern "C" {
    pub fn avcodec_find_decoder_by_name(name: *const ::std::os::raw::c_char) -> *mut AVCodec;
}
extern "C" {
    pub fn avcodec_default_get_buffer2(
        s: *mut AVCodecContext,
        frame: *mut AVFrame,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_align_dimensions(
        s: *mut AVCodecContext,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn avcodec_align_dimensions2(
        s: *mut AVCodecContext,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
        linesize_align: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn avcodec_enum_to_chroma_pos(
        xpos: *mut ::std::os::raw::c_int,
        ypos: *mut ::std::os::raw::c_int,
        pos: AVChromaLocation,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_chroma_pos_to_enum(
        xpos: ::std::os::raw::c_int,
        ypos: ::std::os::raw::c_int,
    ) -> AVChromaLocation;
}
extern "C" {
    pub fn avcodec_decode_audio4(
        avctx: *mut AVCodecContext,
        frame: *mut AVFrame,
        got_frame_ptr: *mut ::std::os::raw::c_int,
        avpkt: *const AVPacket,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_decode_video2(
        avctx: *mut AVCodecContext,
        picture: *mut AVFrame,
        got_picture_ptr: *mut ::std::os::raw::c_int,
        avpkt: *const AVPacket,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_decode_subtitle2(
        avctx: *mut AVCodecContext,
        sub: *mut AVSubtitle,
        got_sub_ptr: *mut ::std::os::raw::c_int,
        avpkt: *mut AVPacket,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_send_packet(
        avctx: *mut AVCodecContext,
        avpkt: *const AVPacket,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_receive_frame(
        avctx: *mut AVCodecContext,
        frame: *mut AVFrame,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_send_frame(
        avctx: *mut AVCodecContext,
        frame: *const AVFrame,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_receive_packet(
        avctx: *mut AVCodecContext,
        avpkt: *mut AVPacket,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_get_hw_frames_parameters(
        avctx: *mut AVCodecContext,
        device_ref: *mut AVBufferRef,
        hw_pix_fmt: AVPixelFormat,
        out_frames_ref: *mut *mut AVBufferRef,
    ) -> ::std::os::raw::c_int;
}
pub const AVPictureStructure_AV_PICTURE_STRUCTURE_UNKNOWN: AVPictureStructure = 0;
pub const AVPictureStructure_AV_PICTURE_STRUCTURE_TOP_FIELD: AVPictureStructure = 1;
pub const AVPictureStructure_AV_PICTURE_STRUCTURE_BOTTOM_FIELD: AVPictureStructure = 2;
pub const AVPictureStructure_AV_PICTURE_STRUCTURE_FRAME: AVPictureStructure = 3;
pub type AVPictureStructure = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecParserContext {
    pub priv_data: *mut ::std::os::raw::c_void,
    pub parser: *mut AVCodecParser,
    pub frame_offset: i64,
    pub cur_offset: i64,
    pub next_frame_offset: i64,
    pub pict_type: ::std::os::raw::c_int,
    pub repeat_pict: ::std::os::raw::c_int,
    pub pts: i64,
    pub dts: i64,
    pub last_pts: i64,
    pub last_dts: i64,
    pub fetch_timestamp: ::std::os::raw::c_int,
    pub cur_frame_start_index: ::std::os::raw::c_int,
    pub cur_frame_offset: [i64; 4usize],
    pub cur_frame_pts: [i64; 4usize],
    pub cur_frame_dts: [i64; 4usize],
    pub flags: ::std::os::raw::c_int,
    pub offset: i64,
    pub cur_frame_end: [i64; 4usize],
    pub key_frame: ::std::os::raw::c_int,
    pub convergence_duration: i64,
    pub dts_sync_point: ::std::os::raw::c_int,
    pub dts_ref_dts_delta: ::std::os::raw::c_int,
    pub pts_dts_delta: ::std::os::raw::c_int,
    pub cur_frame_pos: [i64; 4usize],
    pub pos: i64,
    pub last_pos: i64,
    pub duration: ::std::os::raw::c_int,
    pub field_order: AVFieldOrder,
    pub picture_structure: AVPictureStructure,
    pub output_picture_number: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub coded_width: ::std::os::raw::c_int,
    pub coded_height: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecParser {
    pub codec_ids: [::std::os::raw::c_int; 5usize],
    pub priv_data_size: ::std::os::raw::c_int,
    pub parser_init: ::std::option::Option<
        unsafe extern "C" fn(s: *mut AVCodecParserContext) -> ::std::os::raw::c_int,
    >,
    pub parser_parse: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVCodecParserContext,
            avctx: *mut AVCodecContext,
            poutbuf: *mut *const u8,
            poutbuf_size: *mut ::std::os::raw::c_int,
            buf: *const u8,
            buf_size: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub parser_close: ::std::option::Option<unsafe extern "C" fn(s: *mut AVCodecParserContext)>,
    pub split: ::std::option::Option<
        unsafe extern "C" fn(
            avctx: *mut AVCodecContext,
            buf: *const u8,
            buf_size: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub next: *mut AVCodecParser,
}
extern "C" {
    pub fn av_parser_iterate(opaque: *mut *mut ::std::os::raw::c_void) -> *const AVCodecParser;
}
extern "C" {
    pub fn av_parser_next(c: *const AVCodecParser) -> *mut AVCodecParser;
}
extern "C" {
    pub fn av_register_codec_parser(parser: *mut AVCodecParser);
}
extern "C" {
    pub fn av_parser_init(codec_id: ::std::os::raw::c_int) -> *mut AVCodecParserContext;
}
extern "C" {
    pub fn av_parser_parse2(
        s: *mut AVCodecParserContext,
        avctx: *mut AVCodecContext,
        poutbuf: *mut *mut u8,
        poutbuf_size: *mut ::std::os::raw::c_int,
        buf: *const u8,
        buf_size: ::std::os::raw::c_int,
        pts: i64,
        dts: i64,
        pos: i64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_parser_change(
        s: *mut AVCodecParserContext,
        avctx: *mut AVCodecContext,
        poutbuf: *mut *mut u8,
        poutbuf_size: *mut ::std::os::raw::c_int,
        buf: *const u8,
        buf_size: ::std::os::raw::c_int,
        keyframe: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_parser_close(s: *mut AVCodecParserContext);
}
extern "C" {
    pub fn avcodec_find_encoder(id: AVCodecID) -> *mut AVCodec;
}
extern "C" {
    pub fn avcodec_find_encoder_by_name(name: *const ::std::os::raw::c_char) -> *mut AVCodec;
}
extern "C" {
    pub fn avcodec_encode_audio2(
        avctx: *mut AVCodecContext,
        avpkt: *mut AVPacket,
        frame: *const AVFrame,
        got_packet_ptr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_encode_video2(
        avctx: *mut AVCodecContext,
        avpkt: *mut AVPacket,
        frame: *const AVFrame,
        got_packet_ptr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_encode_subtitle(
        avctx: *mut AVCodecContext,
        buf: *mut u8,
        buf_size: ::std::os::raw::c_int,
        sub: *const AVSubtitle,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avpicture_alloc(
        picture: *mut AVPicture,
        pix_fmt: AVPixelFormat,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avpicture_free(picture: *mut AVPicture);
}
extern "C" {
    pub fn avpicture_fill(
        picture: *mut AVPicture,
        ptr: *const u8,
        pix_fmt: AVPixelFormat,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avpicture_layout(
        src: *const AVPicture,
        pix_fmt: AVPixelFormat,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        dest: *mut ::std::os::raw::c_uchar,
        dest_size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avpicture_get_size(
        pix_fmt: AVPixelFormat,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_picture_copy(
        dst: *mut AVPicture,
        src: *const AVPicture,
        pix_fmt: AVPixelFormat,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_picture_crop(
        dst: *mut AVPicture,
        src: *const AVPicture,
        pix_fmt: AVPixelFormat,
        top_band: ::std::os::raw::c_int,
        left_band: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_picture_pad(
        dst: *mut AVPicture,
        src: *const AVPicture,
        height: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        pix_fmt: AVPixelFormat,
        padtop: ::std::os::raw::c_int,
        padbottom: ::std::os::raw::c_int,
        padleft: ::std::os::raw::c_int,
        padright: ::std::os::raw::c_int,
        color: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_get_chroma_sub_sample(
        pix_fmt: AVPixelFormat,
        h_shift: *mut ::std::os::raw::c_int,
        v_shift: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn avcodec_pix_fmt_to_codec_tag(pix_fmt: AVPixelFormat) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn avcodec_get_pix_fmt_loss(
        dst_pix_fmt: AVPixelFormat,
        src_pix_fmt: AVPixelFormat,
        has_alpha: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_find_best_pix_fmt_of_list(
        pix_fmt_list: *const AVPixelFormat,
        src_pix_fmt: AVPixelFormat,
        has_alpha: ::std::os::raw::c_int,
        loss_ptr: *mut ::std::os::raw::c_int,
    ) -> AVPixelFormat;
}
extern "C" {
    pub fn avcodec_find_best_pix_fmt_of_2(
        dst_pix_fmt1: AVPixelFormat,
        dst_pix_fmt2: AVPixelFormat,
        src_pix_fmt: AVPixelFormat,
        has_alpha: ::std::os::raw::c_int,
        loss_ptr: *mut ::std::os::raw::c_int,
    ) -> AVPixelFormat;
}
extern "C" {
    pub fn avcodec_find_best_pix_fmt2(
        dst_pix_fmt1: AVPixelFormat,
        dst_pix_fmt2: AVPixelFormat,
        src_pix_fmt: AVPixelFormat,
        has_alpha: ::std::os::raw::c_int,
        loss_ptr: *mut ::std::os::raw::c_int,
    ) -> AVPixelFormat;
}
extern "C" {
    pub fn avcodec_default_get_format(
        s: *mut AVCodecContext,
        fmt: *const AVPixelFormat,
    ) -> AVPixelFormat;
}
extern "C" {
    pub fn av_get_codec_tag_string(
        buf: *mut ::std::os::raw::c_char,
        buf_size: usize,
        codec_tag: ::std::os::raw::c_uint,
    ) -> usize;
}
extern "C" {
    pub fn avcodec_string(
        buf: *mut ::std::os::raw::c_char,
        buf_size: ::std::os::raw::c_int,
        enc: *mut AVCodecContext,
        encode: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_get_profile_name(
        codec: *const AVCodec,
        profile: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avcodec_profile_name(
        codec_id: AVCodecID,
        profile: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avcodec_default_execute(
        c: *mut AVCodecContext,
        func: ::std::option::Option<
            unsafe extern "C" fn(
                c2: *mut AVCodecContext,
                arg2: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        arg: *mut ::std::os::raw::c_void,
        ret: *mut ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_default_execute2(
        c: *mut AVCodecContext,
        func: ::std::option::Option<
            unsafe extern "C" fn(
                c2: *mut AVCodecContext,
                arg2: *mut ::std::os::raw::c_void,
                arg1: ::std::os::raw::c_int,
                arg2: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        arg: *mut ::std::os::raw::c_void,
        ret: *mut ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_fill_audio_frame(
        frame: *mut AVFrame,
        nb_channels: ::std::os::raw::c_int,
        sample_fmt: AVSampleFormat,
        buf: *const u8,
        buf_size: ::std::os::raw::c_int,
        align: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_flush_buffers(avctx: *mut AVCodecContext);
}
extern "C" {
    pub fn av_get_bits_per_sample(codec_id: AVCodecID) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_get_pcm_codec(fmt: AVSampleFormat, be: ::std::os::raw::c_int) -> AVCodecID;
}
extern "C" {
    pub fn av_get_exact_bits_per_sample(codec_id: AVCodecID) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_get_audio_frame_duration(
        avctx: *mut AVCodecContext,
        frame_bytes: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_get_audio_frame_duration2(
        par: *mut AVCodecParameters,
        frame_bytes: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVBitStreamFilterContext {
    pub priv_data: *mut ::std::os::raw::c_void,
    pub filter: *const AVBitStreamFilter,
    pub parser: *mut AVCodecParserContext,
    pub next: *mut AVBitStreamFilterContext,
    pub args: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVBSFInternal {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVBSFContext {
    pub av_class: *const AVClass,
    pub filter: *const AVBitStreamFilter,
    pub internal: *mut AVBSFInternal,
    pub priv_data: *mut ::std::os::raw::c_void,
    pub par_in: *mut AVCodecParameters,
    pub par_out: *mut AVCodecParameters,
    pub time_base_in: AVRational,
    pub time_base_out: AVRational,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVBitStreamFilter {
    pub name: *const ::std::os::raw::c_char,
    pub codec_ids: *const AVCodecID,
    pub priv_class: *const AVClass,
    pub priv_data_size: ::std::os::raw::c_int,
    pub init: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut AVBSFContext) -> ::std::os::raw::c_int,
    >,
    pub filter: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut AVBSFContext, pkt: *mut AVPacket) -> ::std::os::raw::c_int,
    >,
    pub close: ::std::option::Option<unsafe extern "C" fn(ctx: *mut AVBSFContext)>,
    pub flush: ::std::option::Option<unsafe extern "C" fn(ctx: *mut AVBSFContext)>,
}
extern "C" {
    pub fn av_register_bitstream_filter(bsf: *mut AVBitStreamFilter);
}
extern "C" {
    pub fn av_bitstream_filter_init(
        name: *const ::std::os::raw::c_char,
    ) -> *mut AVBitStreamFilterContext;
}
extern "C" {
    pub fn av_bitstream_filter_filter(
        bsfc: *mut AVBitStreamFilterContext,
        avctx: *mut AVCodecContext,
        args: *const ::std::os::raw::c_char,
        poutbuf: *mut *mut u8,
        poutbuf_size: *mut ::std::os::raw::c_int,
        buf: *const u8,
        buf_size: ::std::os::raw::c_int,
        keyframe: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_bitstream_filter_close(bsf: *mut AVBitStreamFilterContext);
}
extern "C" {
    pub fn av_bitstream_filter_next(f: *const AVBitStreamFilter) -> *const AVBitStreamFilter;
}
extern "C" {
    pub fn av_bsf_get_by_name(name: *const ::std::os::raw::c_char) -> *const AVBitStreamFilter;
}
extern "C" {
    pub fn av_bsf_iterate(opaque: *mut *mut ::std::os::raw::c_void) -> *const AVBitStreamFilter;
}
extern "C" {
    pub fn av_bsf_next(opaque: *mut *mut ::std::os::raw::c_void) -> *const AVBitStreamFilter;
}
extern "C" {
    pub fn av_bsf_alloc(
        filter: *const AVBitStreamFilter,
        ctx: *mut *mut AVBSFContext,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_bsf_init(ctx: *mut AVBSFContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_bsf_send_packet(ctx: *mut AVBSFContext, pkt: *mut AVPacket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_bsf_receive_packet(
        ctx: *mut AVBSFContext,
        pkt: *mut AVPacket,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_bsf_flush(ctx: *mut AVBSFContext);
}
extern "C" {
    pub fn av_bsf_free(ctx: *mut *mut AVBSFContext);
}
extern "C" {
    pub fn av_bsf_get_class() -> *const AVClass;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVBSFList {
    _unused: [u8; 0],
}
extern "C" {
    pub fn av_bsf_list_alloc() -> *mut AVBSFList;
}
extern "C" {
    pub fn av_bsf_list_free(lst: *mut *mut AVBSFList);
}
extern "C" {
    pub fn av_bsf_list_append(lst: *mut AVBSFList, bsf: *mut AVBSFContext)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_bsf_list_append2(
        lst: *mut AVBSFList,
        bsf_name: *const ::std::os::raw::c_char,
        options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_bsf_list_finalize(
        lst: *mut *mut AVBSFList,
        bsf: *mut *mut AVBSFContext,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_bsf_list_parse_str(
        str: *const ::std::os::raw::c_char,
        bsf: *mut *mut AVBSFContext,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_bsf_get_null_filter(bsf: *mut *mut AVBSFContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_fast_padded_malloc(
        ptr: *mut ::std::os::raw::c_void,
        size: *mut ::std::os::raw::c_uint,
        min_size: usize,
    );
}
extern "C" {
    pub fn av_fast_padded_mallocz(
        ptr: *mut ::std::os::raw::c_void,
        size: *mut ::std::os::raw::c_uint,
        min_size: usize,
    );
}
extern "C" {
    pub fn av_xiphlacing(
        s: *mut ::std::os::raw::c_uchar,
        v: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn av_register_hwaccel(hwaccel: *mut AVHWAccel);
}
extern "C" {
    pub fn av_hwaccel_next(hwaccel: *const AVHWAccel) -> *mut AVHWAccel;
}
pub const AVLockOp_AV_LOCK_CREATE: AVLockOp = 0;
pub const AVLockOp_AV_LOCK_OBTAIN: AVLockOp = 1;
pub const AVLockOp_AV_LOCK_RELEASE: AVLockOp = 2;
pub const AVLockOp_AV_LOCK_DESTROY: AVLockOp = 3;
pub type AVLockOp = u32;
extern "C" {
    pub fn av_lockmgr_register(
        cb: ::std::option::Option<
            unsafe extern "C" fn(
                mutex: *mut *mut ::std::os::raw::c_void,
                op: AVLockOp,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_get_type(codec_id: AVCodecID) -> AVMediaType;
}
extern "C" {
    pub fn avcodec_get_name(id: AVCodecID) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avcodec_is_open(s: *mut AVCodecContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_codec_is_encoder(codec: *const AVCodec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_codec_is_decoder(codec: *const AVCodec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avcodec_descriptor_get(id: AVCodecID) -> *const AVCodecDescriptor;
}
extern "C" {
    pub fn avcodec_descriptor_next(prev: *const AVCodecDescriptor) -> *const AVCodecDescriptor;
}
extern "C" {
    pub fn avcodec_descriptor_get_by_name(
        name: *const ::std::os::raw::c_char,
    ) -> *const AVCodecDescriptor;
}
extern "C" {
    pub fn av_cpb_properties_alloc(size: *mut usize) -> *mut AVCPBProperties;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tm {
    pub tm_sec: ::std::os::raw::c_int,
    pub tm_min: ::std::os::raw::c_int,
    pub tm_hour: ::std::os::raw::c_int,
    pub tm_mday: ::std::os::raw::c_int,
    pub tm_mon: ::std::os::raw::c_int,
    pub tm_year: ::std::os::raw::c_int,
    pub tm_wday: ::std::os::raw::c_int,
    pub tm_yday: ::std::os::raw::c_int,
    pub tm_isdst: ::std::os::raw::c_int,
    pub tm_gmtoff: ::std::os::raw::c_long,
    pub tm_zone: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigevent {
    _unused: [u8; 0],
}
extern "C" {
    pub fn clock() -> clock_t;
}
extern "C" {
    pub fn time(__timer: *mut time_t) -> time_t;
}
extern "C" {
    pub fn difftime(__time1: time_t, __time0: time_t) -> f64;
}
extern "C" {
    pub fn mktime(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn strftime(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: usize,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
    ) -> usize;
}
extern "C" {
    pub fn strftime_l(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: usize,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
        __loc: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn gmtime(__timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn localtime(__timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn asctime(__tp: *const tm) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ctime(__timer: *const time_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn asctime_r(
        __tp: *const tm,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ctime_r(
        __timer: *const time_t,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub static mut __tzname: [*mut ::std::os::raw::c_char; 2usize];
}
extern "C" {
    pub static mut __daylight: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut __timezone: ::std::os::raw::c_long;
}
extern "C" {
    pub static mut tzname: [*mut ::std::os::raw::c_char; 2usize];
}
extern "C" {
    pub fn tzset();
}
extern "C" {
    pub static mut daylight: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut timezone: ::std::os::raw::c_long;
}
extern "C" {
    pub fn stime(__when: *const time_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timegm(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn timelocal(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn dysize(__year: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_getres(__clock_id: clockid_t, __res: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_settime(__clock_id: clockid_t, __tp: *const timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_nanosleep(
        __clock_id: clockid_t,
        __flags: ::std::os::raw::c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_getcpuclockid(__pid: pid_t, __clock_id: *mut clockid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_create(
        __clock_id: clockid_t,
        __evp: *mut sigevent,
        __timerid: *mut timer_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_delete(__timerid: timer_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_settime(
        __timerid: timer_t,
        __flags: ::std::os::raw::c_int,
        __value: *const itimerspec,
        __ovalue: *mut itimerspec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_gettime(__timerid: timer_t, __value: *mut itimerspec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_getoverrun(__timerid: timer_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timespec_get(
        __ts: *mut timespec,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVIOInterruptCB {
    pub callback: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub opaque: *mut ::std::os::raw::c_void,
}
pub const AVIODirEntryType_AVIO_ENTRY_UNKNOWN: AVIODirEntryType = 0;
pub const AVIODirEntryType_AVIO_ENTRY_BLOCK_DEVICE: AVIODirEntryType = 1;
pub const AVIODirEntryType_AVIO_ENTRY_CHARACTER_DEVICE: AVIODirEntryType = 2;
pub const AVIODirEntryType_AVIO_ENTRY_DIRECTORY: AVIODirEntryType = 3;
pub const AVIODirEntryType_AVIO_ENTRY_NAMED_PIPE: AVIODirEntryType = 4;
pub const AVIODirEntryType_AVIO_ENTRY_SYMBOLIC_LINK: AVIODirEntryType = 5;
pub const AVIODirEntryType_AVIO_ENTRY_SOCKET: AVIODirEntryType = 6;
pub const AVIODirEntryType_AVIO_ENTRY_FILE: AVIODirEntryType = 7;
pub const AVIODirEntryType_AVIO_ENTRY_SERVER: AVIODirEntryType = 8;
pub const AVIODirEntryType_AVIO_ENTRY_SHARE: AVIODirEntryType = 9;
pub const AVIODirEntryType_AVIO_ENTRY_WORKGROUP: AVIODirEntryType = 10;
pub type AVIODirEntryType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVIODirEntry {
    pub name: *mut ::std::os::raw::c_char,
    pub type_: ::std::os::raw::c_int,
    pub utf8: ::std::os::raw::c_int,
    pub size: i64,
    pub modification_timestamp: i64,
    pub access_timestamp: i64,
    pub status_change_timestamp: i64,
    pub user_id: i64,
    pub group_id: i64,
    pub filemode: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVIODirContext {
    pub url_context: *mut URLContext,
}
pub const AVIODataMarkerType_AVIO_DATA_MARKER_HEADER: AVIODataMarkerType = 0;
pub const AVIODataMarkerType_AVIO_DATA_MARKER_SYNC_POINT: AVIODataMarkerType = 1;
pub const AVIODataMarkerType_AVIO_DATA_MARKER_BOUNDARY_POINT: AVIODataMarkerType = 2;
pub const AVIODataMarkerType_AVIO_DATA_MARKER_UNKNOWN: AVIODataMarkerType = 3;
pub const AVIODataMarkerType_AVIO_DATA_MARKER_TRAILER: AVIODataMarkerType = 4;
pub const AVIODataMarkerType_AVIO_DATA_MARKER_FLUSH_POINT: AVIODataMarkerType = 5;
pub type AVIODataMarkerType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVIOContext {
    pub av_class: *const AVClass,
    pub buffer: *mut ::std::os::raw::c_uchar,
    pub buffer_size: ::std::os::raw::c_int,
    pub buf_ptr: *mut ::std::os::raw::c_uchar,
    pub buf_end: *mut ::std::os::raw::c_uchar,
    pub opaque: *mut ::std::os::raw::c_void,
    pub read_packet: ::std::option::Option<
        unsafe extern "C" fn(
            opaque: *mut ::std::os::raw::c_void,
            buf: *mut u8,
            buf_size: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub write_packet: ::std::option::Option<
        unsafe extern "C" fn(
            opaque: *mut ::std::os::raw::c_void,
            buf: *mut u8,
            buf_size: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub seek: ::std::option::Option<
        unsafe extern "C" fn(
            opaque: *mut ::std::os::raw::c_void,
            offset: i64,
            whence: ::std::os::raw::c_int,
        ) -> i64,
    >,
    pub pos: i64,
    pub eof_reached: ::std::os::raw::c_int,
    pub write_flag: ::std::os::raw::c_int,
    pub max_packet_size: ::std::os::raw::c_int,
    pub checksum: ::std::os::raw::c_ulong,
    pub checksum_ptr: *mut ::std::os::raw::c_uchar,
    pub update_checksum: ::std::option::Option<
        unsafe extern "C" fn(
            checksum: ::std::os::raw::c_ulong,
            buf: *const u8,
            size: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_ulong,
    >,
    pub error: ::std::os::raw::c_int,
    pub read_pause: ::std::option::Option<
        unsafe extern "C" fn(
            opaque: *mut ::std::os::raw::c_void,
            pause: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub read_seek: ::std::option::Option<
        unsafe extern "C" fn(
            opaque: *mut ::std::os::raw::c_void,
            stream_index: ::std::os::raw::c_int,
            timestamp: i64,
            flags: ::std::os::raw::c_int,
        ) -> i64,
    >,
    pub seekable: ::std::os::raw::c_int,
    pub maxsize: i64,
    pub direct: ::std::os::raw::c_int,
    pub bytes_read: i64,
    pub seek_count: ::std::os::raw::c_int,
    pub writeout_count: ::std::os::raw::c_int,
    pub orig_buffer_size: ::std::os::raw::c_int,
    pub short_seek_threshold: ::std::os::raw::c_int,
    pub protocol_whitelist: *const ::std::os::raw::c_char,
    pub protocol_blacklist: *const ::std::os::raw::c_char,
    pub write_data_type: ::std::option::Option<
        unsafe extern "C" fn(
            opaque: *mut ::std::os::raw::c_void,
            buf: *mut u8,
            buf_size: ::std::os::raw::c_int,
            type_: AVIODataMarkerType,
            time: i64,
        ) -> ::std::os::raw::c_int,
    >,
    pub ignore_boundary_point: ::std::os::raw::c_int,
    pub current_type: AVIODataMarkerType,
    pub last_time: i64,
    pub short_seek_get: ::std::option::Option<
        unsafe extern "C" fn(opaque: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub written: i64,
    pub buf_ptr_max: *mut ::std::os::raw::c_uchar,
    pub min_packet_size: ::std::os::raw::c_int,
}
extern "C" {
    pub fn avio_find_protocol_name(
        url: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avio_check(
        url: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avpriv_io_move(
        url_src: *const ::std::os::raw::c_char,
        url_dst: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avpriv_io_delete(url: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_open_dir(
        s: *mut *mut AVIODirContext,
        url: *const ::std::os::raw::c_char,
        options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_read_dir(
        s: *mut AVIODirContext,
        next: *mut *mut AVIODirEntry,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_close_dir(s: *mut *mut AVIODirContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_free_directory_entry(entry: *mut *mut AVIODirEntry);
}
extern "C" {
    pub fn avio_alloc_context(
        buffer: *mut ::std::os::raw::c_uchar,
        buffer_size: ::std::os::raw::c_int,
        write_flag: ::std::os::raw::c_int,
        opaque: *mut ::std::os::raw::c_void,
        read_packet: ::std::option::Option<
            unsafe extern "C" fn(
                opaque: *mut ::std::os::raw::c_void,
                buf: *mut u8,
                buf_size: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        write_packet: ::std::option::Option<
            unsafe extern "C" fn(
                opaque: *mut ::std::os::raw::c_void,
                buf: *mut u8,
                buf_size: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        seek: ::std::option::Option<
            unsafe extern "C" fn(
                opaque: *mut ::std::os::raw::c_void,
                offset: i64,
                whence: ::std::os::raw::c_int,
            ) -> i64,
        >,
    ) -> *mut AVIOContext;
}
extern "C" {
    pub fn avio_context_free(s: *mut *mut AVIOContext);
}
extern "C" {
    pub fn avio_w8(s: *mut AVIOContext, b: ::std::os::raw::c_int);
}
extern "C" {
    pub fn avio_write(
        s: *mut AVIOContext,
        buf: *const ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn avio_wl64(s: *mut AVIOContext, val: u64);
}
extern "C" {
    pub fn avio_wb64(s: *mut AVIOContext, val: u64);
}
extern "C" {
    pub fn avio_wl32(s: *mut AVIOContext, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn avio_wb32(s: *mut AVIOContext, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn avio_wl24(s: *mut AVIOContext, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn avio_wb24(s: *mut AVIOContext, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn avio_wl16(s: *mut AVIOContext, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn avio_wb16(s: *mut AVIOContext, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn avio_put_str(
        s: *mut AVIOContext,
        str: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_put_str16le(
        s: *mut AVIOContext,
        str: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_put_str16be(
        s: *mut AVIOContext,
        str: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_write_marker(s: *mut AVIOContext, time: i64, type_: AVIODataMarkerType);
}
extern "C" {
    pub fn avio_seek(s: *mut AVIOContext, offset: i64, whence: ::std::os::raw::c_int) -> i64;
}
extern "C" {
    pub fn avio_skip(s: *mut AVIOContext, offset: i64) -> i64;
}
extern "C" {
    pub fn avio_size(s: *mut AVIOContext) -> i64;
}
extern "C" {
    pub fn avio_feof(s: *mut AVIOContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_printf(
        s: *mut AVIOContext,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_flush(s: *mut AVIOContext);
}
extern "C" {
    pub fn avio_read(
        s: *mut AVIOContext,
        buf: *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_read_partial(
        s: *mut AVIOContext,
        buf: *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_r8(s: *mut AVIOContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_rl16(s: *mut AVIOContext) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn avio_rl24(s: *mut AVIOContext) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn avio_rl32(s: *mut AVIOContext) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn avio_rl64(s: *mut AVIOContext) -> u64;
}
extern "C" {
    pub fn avio_rb16(s: *mut AVIOContext) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn avio_rb24(s: *mut AVIOContext) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn avio_rb32(s: *mut AVIOContext) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn avio_rb64(s: *mut AVIOContext) -> u64;
}
extern "C" {
    pub fn avio_get_str(
        pb: *mut AVIOContext,
        maxlen: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buflen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_get_str16le(
        pb: *mut AVIOContext,
        maxlen: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buflen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_get_str16be(
        pb: *mut AVIOContext,
        maxlen: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buflen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_open(
        s: *mut *mut AVIOContext,
        url: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_open2(
        s: *mut *mut AVIOContext,
        url: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        int_cb: *const AVIOInterruptCB,
        options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_close(s: *mut AVIOContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_closep(s: *mut *mut AVIOContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_open_dyn_buf(s: *mut *mut AVIOContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_get_dyn_buf(s: *mut AVIOContext, pbuffer: *mut *mut u8) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_close_dyn_buf(s: *mut AVIOContext, pbuffer: *mut *mut u8) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_enum_protocols(
        opaque: *mut *mut ::std::os::raw::c_void,
        output: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avio_pause(h: *mut AVIOContext, pause: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_seek_time(
        h: *mut AVIOContext,
        stream_index: ::std::os::raw::c_int,
        timestamp: i64,
        flags: ::std::os::raw::c_int,
    ) -> i64;
}
extern "C" {
    pub fn avio_read_to_bprint(
        h: *mut AVIOContext,
        pb: *mut AVBPrint,
        max_size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_accept(s: *mut AVIOContext, c: *mut *mut AVIOContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avio_handshake(c: *mut AVIOContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_get_packet(
        s: *mut AVIOContext,
        pkt: *mut AVPacket,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_append_packet(
        s: *mut AVIOContext,
        pkt: *mut AVPacket,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecTag {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVProbeData {
    pub filename: *const ::std::os::raw::c_char,
    pub buf: *mut ::std::os::raw::c_uchar,
    pub buf_size: ::std::os::raw::c_int,
    pub mime_type: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVOutputFormat {
    pub name: *const ::std::os::raw::c_char,
    pub long_name: *const ::std::os::raw::c_char,
    pub mime_type: *const ::std::os::raw::c_char,
    pub extensions: *const ::std::os::raw::c_char,
    pub audio_codec: AVCodecID,
    pub video_codec: AVCodecID,
    pub subtitle_codec: AVCodecID,
    pub flags: ::std::os::raw::c_int,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub next: *mut AVOutputFormat,
    pub priv_data_size: ::std::os::raw::c_int,
    pub write_header: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AVFormatContext) -> ::std::os::raw::c_int,
    >,
    pub write_packet: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AVFormatContext,
            pkt: *mut AVPacket,
        ) -> ::std::os::raw::c_int,
    >,
    pub write_trailer: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AVFormatContext) -> ::std::os::raw::c_int,
    >,
    pub interleave_packet: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AVFormatContext,
            out: *mut AVPacket,
            in_: *mut AVPacket,
            flush: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub query_codec: ::std::option::Option<
        unsafe extern "C" fn(
            id: AVCodecID,
            std_compliance: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_output_timestamp: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVFormatContext,
            stream: ::std::os::raw::c_int,
            dts: *mut i64,
            wall: *mut i64,
        ),
    >,
    pub control_message: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVFormatContext,
            type_: ::std::os::raw::c_int,
            data: *mut ::std::os::raw::c_void,
            data_size: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub write_uncoded_frame: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AVFormatContext,
            stream_index: ::std::os::raw::c_int,
            frame: *mut *mut AVFrame,
            flags: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_device_list: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVFormatContext,
            device_list: *mut AVDeviceInfoList,
        ) -> ::std::os::raw::c_int,
    >,
    pub create_device_capabilities: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVFormatContext,
            caps: *mut AVDeviceCapabilitiesQuery,
        ) -> ::std::os::raw::c_int,
    >,
    pub free_device_capabilities: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVFormatContext,
            caps: *mut AVDeviceCapabilitiesQuery,
        ) -> ::std::os::raw::c_int,
    >,
    pub data_codec: AVCodecID,
    pub init: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AVFormatContext) -> ::std::os::raw::c_int,
    >,
    pub deinit: ::std::option::Option<unsafe extern "C" fn(arg1: *mut AVFormatContext)>,
    pub check_bitstream: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AVFormatContext,
            pkt: *const AVPacket,
        ) -> ::std::os::raw::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVInputFormat {
    pub name: *const ::std::os::raw::c_char,
    pub long_name: *const ::std::os::raw::c_char,
    pub flags: ::std::os::raw::c_int,
    pub extensions: *const ::std::os::raw::c_char,
    pub codec_tag: *const *const AVCodecTag,
    pub priv_class: *const AVClass,
    pub mime_type: *const ::std::os::raw::c_char,
    pub next: *mut AVInputFormat,
    pub raw_codec_id: ::std::os::raw::c_int,
    pub priv_data_size: ::std::os::raw::c_int,
    pub read_probe: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const AVProbeData) -> ::std::os::raw::c_int,
    >,
    pub read_header: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AVFormatContext) -> ::std::os::raw::c_int,
    >,
    pub read_packet: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AVFormatContext,
            pkt: *mut AVPacket,
        ) -> ::std::os::raw::c_int,
    >,
    pub read_close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AVFormatContext) -> ::std::os::raw::c_int,
    >,
    pub read_seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AVFormatContext,
            stream_index: ::std::os::raw::c_int,
            timestamp: i64,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub read_timestamp: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVFormatContext,
            stream_index: ::std::os::raw::c_int,
            pos: *mut i64,
            pos_limit: i64,
        ) -> i64,
    >,
    pub read_play: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AVFormatContext) -> ::std::os::raw::c_int,
    >,
    pub read_pause: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AVFormatContext) -> ::std::os::raw::c_int,
    >,
    pub read_seek2: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVFormatContext,
            stream_index: ::std::os::raw::c_int,
            min_ts: i64,
            ts: i64,
            max_ts: i64,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_device_list: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVFormatContext,
            device_list: *mut AVDeviceInfoList,
        ) -> ::std::os::raw::c_int,
    >,
    pub create_device_capabilities: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVFormatContext,
            caps: *mut AVDeviceCapabilitiesQuery,
        ) -> ::std::os::raw::c_int,
    >,
    pub free_device_capabilities: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVFormatContext,
            caps: *mut AVDeviceCapabilitiesQuery,
        ) -> ::std::os::raw::c_int,
    >,
}
pub const AVStreamParseType_AVSTREAM_PARSE_NONE: AVStreamParseType = 0;
pub const AVStreamParseType_AVSTREAM_PARSE_FULL: AVStreamParseType = 1;
pub const AVStreamParseType_AVSTREAM_PARSE_HEADERS: AVStreamParseType = 2;
pub const AVStreamParseType_AVSTREAM_PARSE_TIMESTAMPS: AVStreamParseType = 3;
pub const AVStreamParseType_AVSTREAM_PARSE_FULL_ONCE: AVStreamParseType = 4;
pub const AVStreamParseType_AVSTREAM_PARSE_FULL_RAW: AVStreamParseType = 5;
pub type AVStreamParseType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVIndexEntry {
    pub pos: i64,
    pub timestamp: i64,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    pub min_distance: ::std::os::raw::c_int,
}
impl AVIndexEntry {
    #[inline]
    pub fn flags(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_flags(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn size(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 30u8) as u32) }
    }
    #[inline]
    pub fn set_size(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 30u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        flags: ::std::os::raw::c_int,
        size: ::std::os::raw::c_int,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u32> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let flags: u32 = unsafe { ::std::mem::transmute(flags) };
            flags as u64
        });
        __bindgen_bitfield_unit.set(2usize, 30u8, {
            let size: u32 = unsafe { ::std::mem::transmute(size) };
            size as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVStreamInternal {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVStream {
    pub index: ::std::os::raw::c_int,
    pub id: ::std::os::raw::c_int,
    pub codec: *mut AVCodecContext,
    pub priv_data: *mut ::std::os::raw::c_void,
    pub time_base: AVRational,
    pub start_time: i64,
    pub duration: i64,
    pub nb_frames: i64,
    pub disposition: ::std::os::raw::c_int,
    pub discard: AVDiscard,
    pub sample_aspect_ratio: AVRational,
    pub metadata: *mut AVDictionary,
    pub avg_frame_rate: AVRational,
    pub attached_pic: AVPacket,
    pub side_data: *mut AVPacketSideData,
    pub nb_side_data: ::std::os::raw::c_int,
    pub event_flags: ::std::os::raw::c_int,
    pub r_frame_rate: AVRational,
    pub recommended_encoder_configuration: *mut ::std::os::raw::c_char,
    pub codecpar: *mut AVCodecParameters,
    pub info: *mut AVStream__bindgen_ty_1,
    pub pts_wrap_bits: ::std::os::raw::c_int,
    pub first_dts: i64,
    pub cur_dts: i64,
    pub last_IP_pts: i64,
    pub last_IP_duration: ::std::os::raw::c_int,
    pub probe_packets: ::std::os::raw::c_int,
    pub codec_info_nb_frames: ::std::os::raw::c_int,
    pub need_parsing: AVStreamParseType,
    pub parser: *mut AVCodecParserContext,
    pub last_in_packet_buffer: *mut AVPacketList,
    pub probe_data: AVProbeData,
    pub pts_buffer: [i64; 17usize],
    pub index_entries: *mut AVIndexEntry,
    pub nb_index_entries: ::std::os::raw::c_int,
    pub index_entries_allocated_size: ::std::os::raw::c_uint,
    pub stream_identifier: ::std::os::raw::c_int,
    pub program_num: ::std::os::raw::c_int,
    pub pmt_version: ::std::os::raw::c_int,
    pub pmt_stream_idx: ::std::os::raw::c_int,
    pub interleaver_chunk_size: i64,
    pub interleaver_chunk_duration: i64,
    pub request_probe: ::std::os::raw::c_int,
    pub skip_to_keyframe: ::std::os::raw::c_int,
    pub skip_samples: ::std::os::raw::c_int,
    pub start_skip_samples: i64,
    pub first_discard_sample: i64,
    pub last_discard_sample: i64,
    pub nb_decoded_frames: ::std::os::raw::c_int,
    pub mux_ts_offset: i64,
    pub pts_wrap_reference: i64,
    pub pts_wrap_behavior: ::std::os::raw::c_int,
    pub update_initial_durations_done: ::std::os::raw::c_int,
    pub pts_reorder_error: [i64; 17usize],
    pub pts_reorder_error_count: [u8; 17usize],
    pub last_dts_for_order_check: i64,
    pub dts_ordered: u8,
    pub dts_misordered: u8,
    pub inject_global_side_data: ::std::os::raw::c_int,
    pub display_aspect_ratio: AVRational,
    pub internal: *mut AVStreamInternal,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVStream__bindgen_ty_1 {
    pub last_dts: i64,
    pub duration_gcd: i64,
    pub duration_count: ::std::os::raw::c_int,
    pub rfps_duration_sum: i64,
    pub duration_error: *mut [[f64; 399usize]; 2usize],
    pub codec_info_duration: i64,
    pub codec_info_duration_fields: i64,
    pub frame_delay_evidence: ::std::os::raw::c_int,
    pub found_decoder: ::std::os::raw::c_int,
    pub last_duration: i64,
    pub fps_first_dts: i64,
    pub fps_first_dts_idx: ::std::os::raw::c_int,
    pub fps_last_dts: i64,
    pub fps_last_dts_idx: ::std::os::raw::c_int,
}
extern "C" {
    pub fn av_stream_get_r_frame_rate(s: *const AVStream) -> AVRational;
}
extern "C" {
    pub fn av_stream_set_r_frame_rate(s: *mut AVStream, r: AVRational);
}
extern "C" {
    pub fn av_stream_get_recommended_encoder_configuration(
        s: *const AVStream,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_stream_set_recommended_encoder_configuration(
        s: *mut AVStream,
        configuration: *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn av_stream_get_parser(s: *const AVStream) -> *mut AVCodecParserContext;
}
extern "C" {
    pub fn av_stream_get_end_pts(st: *const AVStream) -> i64;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVProgram {
    pub id: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub discard: AVDiscard,
    pub stream_index: *mut ::std::os::raw::c_uint,
    pub nb_stream_indexes: ::std::os::raw::c_uint,
    pub metadata: *mut AVDictionary,
    pub program_num: ::std::os::raw::c_int,
    pub pmt_pid: ::std::os::raw::c_int,
    pub pcr_pid: ::std::os::raw::c_int,
    pub pmt_version: ::std::os::raw::c_int,
    pub start_time: i64,
    pub end_time: i64,
    pub pts_wrap_reference: i64,
    pub pts_wrap_behavior: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVChapter {
    pub id: ::std::os::raw::c_int,
    pub time_base: AVRational,
    pub start: i64,
    pub end: i64,
    pub metadata: *mut AVDictionary,
}
pub type av_format_control_message = ::std::option::Option<
    unsafe extern "C" fn(
        s: *mut AVFormatContext,
        type_: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
        data_size: usize,
    ) -> ::std::os::raw::c_int,
>;
pub type AVOpenCallback = ::std::option::Option<
    unsafe extern "C" fn(
        s: *mut AVFormatContext,
        pb: *mut *mut AVIOContext,
        url: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        int_cb: *const AVIOInterruptCB,
        options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int,
>;
pub const AVDurationEstimationMethod_AVFMT_DURATION_FROM_PTS: AVDurationEstimationMethod = 0;
pub const AVDurationEstimationMethod_AVFMT_DURATION_FROM_STREAM: AVDurationEstimationMethod = 1;
pub const AVDurationEstimationMethod_AVFMT_DURATION_FROM_BITRATE: AVDurationEstimationMethod = 2;
pub type AVDurationEstimationMethod = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFormatInternal {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AVFormatContext {
    pub av_class: *const AVClass,
    pub iformat: *mut AVInputFormat,
    pub oformat: *mut AVOutputFormat,
    pub priv_data: *mut ::std::os::raw::c_void,
    pub pb: *mut AVIOContext,
    pub ctx_flags: ::std::os::raw::c_int,
    pub nb_streams: ::std::os::raw::c_uint,
    pub streams: *mut *mut AVStream,
    pub filename: [::std::os::raw::c_char; 1024usize],
    pub url: *mut ::std::os::raw::c_char,
    pub start_time: i64,
    pub duration: i64,
    pub bit_rate: i64,
    pub packet_size: ::std::os::raw::c_uint,
    pub max_delay: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub probesize: i64,
    pub max_analyze_duration: i64,
    pub key: *const u8,
    pub keylen: ::std::os::raw::c_int,
    pub nb_programs: ::std::os::raw::c_uint,
    pub programs: *mut *mut AVProgram,
    pub video_codec_id: AVCodecID,
    pub audio_codec_id: AVCodecID,
    pub subtitle_codec_id: AVCodecID,
    pub max_index_size: ::std::os::raw::c_uint,
    pub max_picture_buffer: ::std::os::raw::c_uint,
    pub nb_chapters: ::std::os::raw::c_uint,
    pub chapters: *mut *mut AVChapter,
    pub metadata: *mut AVDictionary,
    pub start_time_realtime: i64,
    pub fps_probe_size: ::std::os::raw::c_int,
    pub error_recognition: ::std::os::raw::c_int,
    pub interrupt_callback: AVIOInterruptCB,
    pub debug: ::std::os::raw::c_int,
    pub max_interleave_delta: i64,
    pub strict_std_compliance: ::std::os::raw::c_int,
    pub event_flags: ::std::os::raw::c_int,
    pub max_ts_probe: ::std::os::raw::c_int,
    pub avoid_negative_ts: ::std::os::raw::c_int,
    pub ts_id: ::std::os::raw::c_int,
    pub audio_preload: ::std::os::raw::c_int,
    pub max_chunk_duration: ::std::os::raw::c_int,
    pub max_chunk_size: ::std::os::raw::c_int,
    pub use_wallclock_as_timestamps: ::std::os::raw::c_int,
    pub avio_flags: ::std::os::raw::c_int,
    pub duration_estimation_method: AVDurationEstimationMethod,
    pub skip_initial_bytes: i64,
    pub correct_ts_overflow: ::std::os::raw::c_uint,
    pub seek2any: ::std::os::raw::c_int,
    pub flush_packets: ::std::os::raw::c_int,
    pub probe_score: ::std::os::raw::c_int,
    pub format_probesize: ::std::os::raw::c_int,
    pub codec_whitelist: *mut ::std::os::raw::c_char,
    pub format_whitelist: *mut ::std::os::raw::c_char,
    pub internal: *mut AVFormatInternal,
    pub io_repositioned: ::std::os::raw::c_int,
    pub video_codec: *mut AVCodec,
    pub audio_codec: *mut AVCodec,
    pub subtitle_codec: *mut AVCodec,
    pub data_codec: *mut AVCodec,
    pub metadata_header_padding: ::std::os::raw::c_int,
    pub opaque: *mut ::std::os::raw::c_void,
    pub control_message_cb: av_format_control_message,
    pub output_ts_offset: i64,
    pub dump_separator: *mut u8,
    pub data_codec_id: AVCodecID,
    pub open_cb: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVFormatContext,
            p: *mut *mut AVIOContext,
            url: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            int_cb: *const AVIOInterruptCB,
            options: *mut *mut AVDictionary,
        ) -> ::std::os::raw::c_int,
    >,
    pub protocol_whitelist: *mut ::std::os::raw::c_char,
    pub io_open: ::std::option::Option<
        unsafe extern "C" fn(
            s: *mut AVFormatContext,
            pb: *mut *mut AVIOContext,
            url: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            options: *mut *mut AVDictionary,
        ) -> ::std::os::raw::c_int,
    >,
    pub io_close:
        ::std::option::Option<unsafe extern "C" fn(s: *mut AVFormatContext, pb: *mut AVIOContext)>,
    pub protocol_blacklist: *mut ::std::os::raw::c_char,
    pub max_streams: ::std::os::raw::c_int,
    pub skip_estimate_duration_from_pts: ::std::os::raw::c_int,
}
extern "C" {
    pub fn av_format_get_probe_score(s: *const AVFormatContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_format_get_video_codec(s: *const AVFormatContext) -> *mut AVCodec;
}
extern "C" {
    pub fn av_format_set_video_codec(s: *mut AVFormatContext, c: *mut AVCodec);
}
extern "C" {
    pub fn av_format_get_audio_codec(s: *const AVFormatContext) -> *mut AVCodec;
}
extern "C" {
    pub fn av_format_set_audio_codec(s: *mut AVFormatContext, c: *mut AVCodec);
}
extern "C" {
    pub fn av_format_get_subtitle_codec(s: *const AVFormatContext) -> *mut AVCodec;
}
extern "C" {
    pub fn av_format_set_subtitle_codec(s: *mut AVFormatContext, c: *mut AVCodec);
}
extern "C" {
    pub fn av_format_get_data_codec(s: *const AVFormatContext) -> *mut AVCodec;
}
extern "C" {
    pub fn av_format_set_data_codec(s: *mut AVFormatContext, c: *mut AVCodec);
}
extern "C" {
    pub fn av_format_get_metadata_header_padding(
        s: *const AVFormatContext,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_format_set_metadata_header_padding(s: *mut AVFormatContext, c: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_format_get_opaque(s: *const AVFormatContext) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn av_format_set_opaque(s: *mut AVFormatContext, opaque: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn av_format_get_control_message_cb(s: *const AVFormatContext)
        -> av_format_control_message;
}
extern "C" {
    pub fn av_format_set_control_message_cb(
        s: *mut AVFormatContext,
        callback: av_format_control_message,
    );
}
extern "C" {
    pub fn av_format_get_open_cb(s: *const AVFormatContext) -> AVOpenCallback;
}
extern "C" {
    pub fn av_format_set_open_cb(s: *mut AVFormatContext, callback: AVOpenCallback);
}
extern "C" {
    pub fn av_format_inject_global_side_data(s: *mut AVFormatContext);
}
extern "C" {
    pub fn av_fmt_ctx_get_duration_estimation_method(
        ctx: *const AVFormatContext,
    ) -> AVDurationEstimationMethod;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVPacketList {
    pub pkt: AVPacket,
    pub next: *mut AVPacketList,
}
extern "C" {
    pub fn avformat_version() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn avformat_configuration() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avformat_license() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn av_register_all();
}
extern "C" {
    pub fn av_register_input_format(format: *mut AVInputFormat);
}
extern "C" {
    pub fn av_register_output_format(format: *mut AVOutputFormat);
}
extern "C" {
    pub fn avformat_network_init() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avformat_network_deinit() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_iformat_next(f: *const AVInputFormat) -> *mut AVInputFormat;
}
extern "C" {
    pub fn av_oformat_next(f: *const AVOutputFormat) -> *mut AVOutputFormat;
}
extern "C" {
    pub fn av_muxer_iterate(opaque: *mut *mut ::std::os::raw::c_void) -> *const AVOutputFormat;
}
extern "C" {
    pub fn av_demuxer_iterate(opaque: *mut *mut ::std::os::raw::c_void) -> *const AVInputFormat;
}
extern "C" {
    pub fn avformat_alloc_context() -> *mut AVFormatContext;
}
extern "C" {
    pub fn avformat_free_context(s: *mut AVFormatContext);
}
extern "C" {
    pub fn avformat_get_class() -> *const AVClass;
}
extern "C" {
    pub fn avformat_new_stream(s: *mut AVFormatContext, c: *const AVCodec) -> *mut AVStream;
}
extern "C" {
    pub fn av_stream_add_side_data(
        st: *mut AVStream,
        type_: AVPacketSideDataType,
        data: *mut u8,
        size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_stream_new_side_data(
        stream: *mut AVStream,
        type_: AVPacketSideDataType,
        size: ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn av_stream_get_side_data(
        stream: *const AVStream,
        type_: AVPacketSideDataType,
        size: *mut ::std::os::raw::c_int,
    ) -> *mut u8;
}
extern "C" {
    pub fn av_new_program(s: *mut AVFormatContext, id: ::std::os::raw::c_int) -> *mut AVProgram;
}
extern "C" {
    pub fn avformat_alloc_output_context2(
        ctx: *mut *mut AVFormatContext,
        oformat: *mut AVOutputFormat,
        format_name: *const ::std::os::raw::c_char,
        filename: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_find_input_format(short_name: *const ::std::os::raw::c_char) -> *mut AVInputFormat;
}
extern "C" {
    pub fn av_probe_input_format(
        pd: *mut AVProbeData,
        is_opened: ::std::os::raw::c_int,
    ) -> *mut AVInputFormat;
}
extern "C" {
    pub fn av_probe_input_format2(
        pd: *mut AVProbeData,
        is_opened: ::std::os::raw::c_int,
        score_max: *mut ::std::os::raw::c_int,
    ) -> *mut AVInputFormat;
}
extern "C" {
    pub fn av_probe_input_format3(
        pd: *mut AVProbeData,
        is_opened: ::std::os::raw::c_int,
        score_ret: *mut ::std::os::raw::c_int,
    ) -> *mut AVInputFormat;
}
extern "C" {
    pub fn av_probe_input_buffer2(
        pb: *mut AVIOContext,
        fmt: *mut *mut AVInputFormat,
        url: *const ::std::os::raw::c_char,
        logctx: *mut ::std::os::raw::c_void,
        offset: ::std::os::raw::c_uint,
        max_probe_size: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_probe_input_buffer(
        pb: *mut AVIOContext,
        fmt: *mut *mut AVInputFormat,
        url: *const ::std::os::raw::c_char,
        logctx: *mut ::std::os::raw::c_void,
        offset: ::std::os::raw::c_uint,
        max_probe_size: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avformat_open_input(
        ps: *mut *mut AVFormatContext,
        url: *const ::std::os::raw::c_char,
        fmt: *mut AVInputFormat,
        options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_demuxer_open(ic: *mut AVFormatContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avformat_find_stream_info(
        ic: *mut AVFormatContext,
        options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_find_program_from_stream(
        ic: *mut AVFormatContext,
        last: *mut AVProgram,
        s: ::std::os::raw::c_int,
    ) -> *mut AVProgram;
}
extern "C" {
    pub fn av_program_add_stream_index(
        ac: *mut AVFormatContext,
        progid: ::std::os::raw::c_int,
        idx: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn av_find_best_stream(
        ic: *mut AVFormatContext,
        type_: AVMediaType,
        wanted_stream_nb: ::std::os::raw::c_int,
        related_stream: ::std::os::raw::c_int,
        decoder_ret: *mut *mut AVCodec,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_read_frame(s: *mut AVFormatContext, pkt: *mut AVPacket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_seek_frame(
        s: *mut AVFormatContext,
        stream_index: ::std::os::raw::c_int,
        timestamp: i64,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avformat_seek_file(
        s: *mut AVFormatContext,
        stream_index: ::std::os::raw::c_int,
        min_ts: i64,
        ts: i64,
        max_ts: i64,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avformat_flush(s: *mut AVFormatContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_read_play(s: *mut AVFormatContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_read_pause(s: *mut AVFormatContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avformat_close_input(s: *mut *mut AVFormatContext);
}
extern "C" {
    pub fn avformat_write_header(
        s: *mut AVFormatContext,
        options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avformat_init_output(
        s: *mut AVFormatContext,
        options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_write_frame(s: *mut AVFormatContext, pkt: *mut AVPacket) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_interleaved_write_frame(
        s: *mut AVFormatContext,
        pkt: *mut AVPacket,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_write_uncoded_frame(
        s: *mut AVFormatContext,
        stream_index: ::std::os::raw::c_int,
        frame: *mut AVFrame,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_interleaved_write_uncoded_frame(
        s: *mut AVFormatContext,
        stream_index: ::std::os::raw::c_int,
        frame: *mut AVFrame,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_write_uncoded_frame_query(
        s: *mut AVFormatContext,
        stream_index: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_write_trailer(s: *mut AVFormatContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_guess_format(
        short_name: *const ::std::os::raw::c_char,
        filename: *const ::std::os::raw::c_char,
        mime_type: *const ::std::os::raw::c_char,
    ) -> *mut AVOutputFormat;
}
extern "C" {
    pub fn av_guess_codec(
        fmt: *mut AVOutputFormat,
        short_name: *const ::std::os::raw::c_char,
        filename: *const ::std::os::raw::c_char,
        mime_type: *const ::std::os::raw::c_char,
        type_: AVMediaType,
    ) -> AVCodecID;
}
extern "C" {
    pub fn av_get_output_timestamp(
        s: *mut AVFormatContext,
        stream: ::std::os::raw::c_int,
        dts: *mut i64,
        wall: *mut i64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_hex_dump(f: *mut FILE, buf: *const u8, size: ::std::os::raw::c_int);
}
extern "C" {
    pub fn av_hex_dump_log(
        avcl: *mut ::std::os::raw::c_void,
        level: ::std::os::raw::c_int,
        buf: *const u8,
        size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_pkt_dump2(
        f: *mut FILE,
        pkt: *const AVPacket,
        dump_payload: ::std::os::raw::c_int,
        st: *const AVStream,
    );
}
extern "C" {
    pub fn av_pkt_dump_log2(
        avcl: *mut ::std::os::raw::c_void,
        level: ::std::os::raw::c_int,
        pkt: *const AVPacket,
        dump_payload: ::std::os::raw::c_int,
        st: *const AVStream,
    );
}
extern "C" {
    pub fn av_codec_get_id(
        tags: *const *const AVCodecTag,
        tag: ::std::os::raw::c_uint,
    ) -> AVCodecID;
}
extern "C" {
    pub fn av_codec_get_tag(
        tags: *const *const AVCodecTag,
        id: AVCodecID,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn av_codec_get_tag2(
        tags: *const *const AVCodecTag,
        id: AVCodecID,
        tag: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_find_default_stream_index(s: *mut AVFormatContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_index_search_timestamp(
        st: *mut AVStream,
        timestamp: i64,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_add_index_entry(
        st: *mut AVStream,
        pos: i64,
        timestamp: i64,
        size: ::std::os::raw::c_int,
        distance: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_url_split(
        proto: *mut ::std::os::raw::c_char,
        proto_size: ::std::os::raw::c_int,
        authorization: *mut ::std::os::raw::c_char,
        authorization_size: ::std::os::raw::c_int,
        hostname: *mut ::std::os::raw::c_char,
        hostname_size: ::std::os::raw::c_int,
        port_ptr: *mut ::std::os::raw::c_int,
        path: *mut ::std::os::raw::c_char,
        path_size: ::std::os::raw::c_int,
        url: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn av_dump_format(
        ic: *mut AVFormatContext,
        index: ::std::os::raw::c_int,
        url: *const ::std::os::raw::c_char,
        is_output: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn av_get_frame_filename2(
        buf: *mut ::std::os::raw::c_char,
        buf_size: ::std::os::raw::c_int,
        path: *const ::std::os::raw::c_char,
        number: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_get_frame_filename(
        buf: *mut ::std::os::raw::c_char,
        buf_size: ::std::os::raw::c_int,
        path: *const ::std::os::raw::c_char,
        number: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_filename_number_test(
        filename: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_sdp_create(
        ac: *mut *mut AVFormatContext,
        n_files: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_match_ext(
        filename: *const ::std::os::raw::c_char,
        extensions: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avformat_query_codec(
        ofmt: *const AVOutputFormat,
        codec_id: AVCodecID,
        std_compliance: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avformat_get_riff_video_tags() -> *const AVCodecTag;
}
extern "C" {
    pub fn avformat_get_riff_audio_tags() -> *const AVCodecTag;
}
extern "C" {
    pub fn avformat_get_mov_video_tags() -> *const AVCodecTag;
}
extern "C" {
    pub fn avformat_get_mov_audio_tags() -> *const AVCodecTag;
}
extern "C" {
    pub fn av_guess_sample_aspect_ratio(
        format: *mut AVFormatContext,
        stream: *mut AVStream,
        frame: *mut AVFrame,
    ) -> AVRational;
}
extern "C" {
    pub fn av_guess_frame_rate(
        ctx: *mut AVFormatContext,
        stream: *mut AVStream,
        frame: *mut AVFrame,
    ) -> AVRational;
}
extern "C" {
    pub fn avformat_match_stream_specifier(
        s: *mut AVFormatContext,
        st: *mut AVStream,
        spec: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avformat_queue_attached_pictures(s: *mut AVFormatContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_apply_bitstream_filters(
        codec: *mut AVCodecContext,
        pkt: *mut AVPacket,
        bsfc: *mut AVBitStreamFilterContext,
    ) -> ::std::os::raw::c_int;
}
pub const AVTimebaseSource_AVFMT_TBCF_AUTO: AVTimebaseSource = -1;
pub const AVTimebaseSource_AVFMT_TBCF_DECODER: AVTimebaseSource = 0;
pub const AVTimebaseSource_AVFMT_TBCF_DEMUXER: AVTimebaseSource = 1;
pub const AVTimebaseSource_AVFMT_TBCF_R_FRAMERATE: AVTimebaseSource = 2;
pub type AVTimebaseSource = i32;
extern "C" {
    pub fn avformat_transfer_internal_stream_timing_info(
        ofmt: *const AVOutputFormat,
        ost: *mut AVStream,
        ist: *const AVStream,
        copy_tb: AVTimebaseSource,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_stream_get_codec_timebase(st: *const AVStream) -> AVRational;
}
extern "C" {
    pub fn avdevice_version() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn avdevice_configuration() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avdevice_license() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avdevice_register_all();
}
extern "C" {
    pub fn av_input_audio_device_next(d: *mut AVInputFormat) -> *mut AVInputFormat;
}
extern "C" {
    pub fn av_input_video_device_next(d: *mut AVInputFormat) -> *mut AVInputFormat;
}
extern "C" {
    pub fn av_output_audio_device_next(d: *mut AVOutputFormat) -> *mut AVOutputFormat;
}
extern "C" {
    pub fn av_output_video_device_next(d: *mut AVOutputFormat) -> *mut AVOutputFormat;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVDeviceRect {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
}
pub const AVAppToDevMessageType_AV_APP_TO_DEV_NONE: AVAppToDevMessageType = 1313820229;
pub const AVAppToDevMessageType_AV_APP_TO_DEV_WINDOW_SIZE: AVAppToDevMessageType = 1195724621;
pub const AVAppToDevMessageType_AV_APP_TO_DEV_WINDOW_REPAINT: AVAppToDevMessageType = 1380274241;
pub const AVAppToDevMessageType_AV_APP_TO_DEV_PAUSE: AVAppToDevMessageType = 1346458912;
pub const AVAppToDevMessageType_AV_APP_TO_DEV_PLAY: AVAppToDevMessageType = 1347174745;
pub const AVAppToDevMessageType_AV_APP_TO_DEV_TOGGLE_PAUSE: AVAppToDevMessageType = 1346458964;
pub const AVAppToDevMessageType_AV_APP_TO_DEV_SET_VOLUME: AVAppToDevMessageType = 1398165324;
pub const AVAppToDevMessageType_AV_APP_TO_DEV_MUTE: AVAppToDevMessageType = 541939028;
pub const AVAppToDevMessageType_AV_APP_TO_DEV_UNMUTE: AVAppToDevMessageType = 1431131476;
pub const AVAppToDevMessageType_AV_APP_TO_DEV_TOGGLE_MUTE: AVAppToDevMessageType = 1414354260;
pub const AVAppToDevMessageType_AV_APP_TO_DEV_GET_VOLUME: AVAppToDevMessageType = 1196838732;
pub const AVAppToDevMessageType_AV_APP_TO_DEV_GET_MUTE: AVAppToDevMessageType = 1196250452;
pub type AVAppToDevMessageType = u32;
pub const AVDevToAppMessageType_AV_DEV_TO_APP_NONE: AVDevToAppMessageType = 1313820229;
pub const AVDevToAppMessageType_AV_DEV_TO_APP_CREATE_WINDOW_BUFFER: AVDevToAppMessageType =
    1111708229;
pub const AVDevToAppMessageType_AV_DEV_TO_APP_PREPARE_WINDOW_BUFFER: AVDevToAppMessageType =
    1112560197;
pub const AVDevToAppMessageType_AV_DEV_TO_APP_DISPLAY_WINDOW_BUFFER: AVDevToAppMessageType =
    1111771475;
pub const AVDevToAppMessageType_AV_DEV_TO_APP_DESTROY_WINDOW_BUFFER: AVDevToAppMessageType =
    1111770451;
pub const AVDevToAppMessageType_AV_DEV_TO_APP_BUFFER_OVERFLOW: AVDevToAppMessageType = 1112491596;
pub const AVDevToAppMessageType_AV_DEV_TO_APP_BUFFER_UNDERFLOW: AVDevToAppMessageType = 1112884812;
pub const AVDevToAppMessageType_AV_DEV_TO_APP_BUFFER_READABLE: AVDevToAppMessageType = 1112687648;
pub const AVDevToAppMessageType_AV_DEV_TO_APP_BUFFER_WRITABLE: AVDevToAppMessageType = 1113018912;
pub const AVDevToAppMessageType_AV_DEV_TO_APP_MUTE_STATE_CHANGED: AVDevToAppMessageType =
    1129141588;
pub const AVDevToAppMessageType_AV_DEV_TO_APP_VOLUME_LEVEL_CHANGED: AVDevToAppMessageType =
    1129729868;
pub type AVDevToAppMessageType = u32;
extern "C" {
    pub fn avdevice_app_to_dev_control_message(
        s: *mut AVFormatContext,
        type_: AVAppToDevMessageType,
        data: *mut ::std::os::raw::c_void,
        data_size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avdevice_dev_to_app_control_message(
        s: *mut AVFormatContext,
        type_: AVDevToAppMessageType,
        data: *mut ::std::os::raw::c_void,
        data_size: usize,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVDeviceCapabilitiesQuery {
    pub av_class: *const AVClass,
    pub device_context: *mut AVFormatContext,
    pub codec: AVCodecID,
    pub sample_format: AVSampleFormat,
    pub pixel_format: AVPixelFormat,
    pub sample_rate: ::std::os::raw::c_int,
    pub channels: ::std::os::raw::c_int,
    pub channel_layout: i64,
    pub window_width: ::std::os::raw::c_int,
    pub window_height: ::std::os::raw::c_int,
    pub frame_width: ::std::os::raw::c_int,
    pub frame_height: ::std::os::raw::c_int,
    pub fps: AVRational,
}
extern "C" {
    pub static mut av_device_capabilities: [AVOption; 0usize];
}
extern "C" {
    pub fn avdevice_capabilities_create(
        caps: *mut *mut AVDeviceCapabilitiesQuery,
        s: *mut AVFormatContext,
        device_options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avdevice_capabilities_free(
        caps: *mut *mut AVDeviceCapabilitiesQuery,
        s: *mut AVFormatContext,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVDeviceInfo {
    pub device_name: *mut ::std::os::raw::c_char,
    pub device_description: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVDeviceInfoList {
    pub devices: *mut *mut AVDeviceInfo,
    pub nb_devices: ::std::os::raw::c_int,
    pub default_device: ::std::os::raw::c_int,
}
extern "C" {
    pub fn avdevice_list_devices(
        s: *mut AVFormatContext,
        device_list: *mut *mut AVDeviceInfoList,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avdevice_free_list_devices(device_list: *mut *mut AVDeviceInfoList);
}
extern "C" {
    pub fn avdevice_list_input_sources(
        device: *mut AVInputFormat,
        device_name: *const ::std::os::raw::c_char,
        device_options: *mut AVDictionary,
        device_list: *mut *mut AVDeviceInfoList,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avdevice_list_output_sinks(
        device: *mut AVOutputFormat,
        device_name: *const ::std::os::raw::c_char,
        device_options: *mut AVDictionary,
        device_list: *mut *mut AVDeviceInfoList,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_version() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn avfilter_configuration() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avfilter_license() -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFilterPad {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFilterFormats {
    _unused: [u8; 0],
}
extern "C" {
    pub fn avfilter_pad_count(pads: *const AVFilterPad) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_pad_get_name(
        pads: *const AVFilterPad,
        pad_idx: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn avfilter_pad_get_type(
        pads: *const AVFilterPad,
        pad_idx: ::std::os::raw::c_int,
    ) -> AVMediaType;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFilter {
    pub name: *const ::std::os::raw::c_char,
    pub description: *const ::std::os::raw::c_char,
    pub inputs: *const AVFilterPad,
    pub outputs: *const AVFilterPad,
    pub priv_class: *const AVClass,
    pub flags: ::std::os::raw::c_int,
    pub preinit: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut AVFilterContext) -> ::std::os::raw::c_int,
    >,
    pub init: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut AVFilterContext) -> ::std::os::raw::c_int,
    >,
    pub init_dict: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut AVFilterContext,
            options: *mut *mut AVDictionary,
        ) -> ::std::os::raw::c_int,
    >,
    pub uninit: ::std::option::Option<unsafe extern "C" fn(ctx: *mut AVFilterContext)>,
    pub query_formats: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AVFilterContext) -> ::std::os::raw::c_int,
    >,
    pub priv_size: ::std::os::raw::c_int,
    pub flags_internal: ::std::os::raw::c_int,
    pub next: *mut AVFilter,
    pub process_command: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AVFilterContext,
            cmd: *const ::std::os::raw::c_char,
            arg: *const ::std::os::raw::c_char,
            res: *mut ::std::os::raw::c_char,
            res_len: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub init_opaque: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut AVFilterContext,
            opaque: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub activate: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut AVFilterContext) -> ::std::os::raw::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFilterInternal {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFilterContext {
    pub av_class: *const AVClass,
    pub filter: *const AVFilter,
    pub name: *mut ::std::os::raw::c_char,
    pub input_pads: *mut AVFilterPad,
    pub inputs: *mut *mut AVFilterLink,
    pub nb_inputs: ::std::os::raw::c_uint,
    pub output_pads: *mut AVFilterPad,
    pub outputs: *mut *mut AVFilterLink,
    pub nb_outputs: ::std::os::raw::c_uint,
    pub priv_: *mut ::std::os::raw::c_void,
    pub graph: *mut AVFilterGraph,
    pub thread_type: ::std::os::raw::c_int,
    pub internal: *mut AVFilterInternal,
    pub command_queue: *mut AVFilterCommand,
    pub enable_str: *mut ::std::os::raw::c_char,
    pub enable: *mut ::std::os::raw::c_void,
    pub var_values: *mut f64,
    pub is_disabled: ::std::os::raw::c_int,
    pub hw_device_ctx: *mut AVBufferRef,
    pub nb_threads: ::std::os::raw::c_int,
    pub ready: ::std::os::raw::c_uint,
    pub extra_hw_frames: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AVFilterLink {
    pub src: *mut AVFilterContext,
    pub srcpad: *mut AVFilterPad,
    pub dst: *mut AVFilterContext,
    pub dstpad: *mut AVFilterPad,
    pub type_: AVMediaType,
    pub w: ::std::os::raw::c_int,
    pub h: ::std::os::raw::c_int,
    pub sample_aspect_ratio: AVRational,
    pub channel_layout: u64,
    pub sample_rate: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
    pub time_base: AVRational,
    pub in_formats: *mut AVFilterFormats,
    pub out_formats: *mut AVFilterFormats,
    pub in_samplerates: *mut AVFilterFormats,
    pub out_samplerates: *mut AVFilterFormats,
    pub in_channel_layouts: *mut AVFilterChannelLayouts,
    pub out_channel_layouts: *mut AVFilterChannelLayouts,
    pub request_samples: ::std::os::raw::c_int,
    pub init_state: AVFilterLink__bindgen_ty_1,
    pub graph: *mut AVFilterGraph,
    pub current_pts: i64,
    pub current_pts_us: i64,
    pub age_index: ::std::os::raw::c_int,
    pub frame_rate: AVRational,
    pub partial_buf: *mut AVFrame,
    pub partial_buf_size: ::std::os::raw::c_int,
    pub min_samples: ::std::os::raw::c_int,
    pub max_samples: ::std::os::raw::c_int,
    pub channels: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_uint,
    pub frame_count_in: i64,
    pub frame_count_out: i64,
    pub frame_pool: *mut ::std::os::raw::c_void,
    pub frame_wanted_out: ::std::os::raw::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub reserved: [::std::os::raw::c_char; 61440usize],
}
pub const AVFilterLink_AVLINK_UNINIT: AVFilterLink__bindgen_ty_1 = 0;
pub const AVFilterLink_AVLINK_STARTINIT: AVFilterLink__bindgen_ty_1 = 1;
pub const AVFilterLink_AVLINK_INIT: AVFilterLink__bindgen_ty_1 = 2;
pub type AVFilterLink__bindgen_ty_1 = u32;
extern "C" {
    pub fn avfilter_link(
        src: *mut AVFilterContext,
        srcpad: ::std::os::raw::c_uint,
        dst: *mut AVFilterContext,
        dstpad: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_link_free(link: *mut *mut AVFilterLink);
}
extern "C" {
    pub fn avfilter_link_get_channels(link: *mut AVFilterLink) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_link_set_closed(link: *mut AVFilterLink, closed: ::std::os::raw::c_int);
}
extern "C" {
    pub fn avfilter_config_links(filter: *mut AVFilterContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_process_command(
        filter: *mut AVFilterContext,
        cmd: *const ::std::os::raw::c_char,
        arg: *const ::std::os::raw::c_char,
        res: *mut ::std::os::raw::c_char,
        res_len: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn av_filter_iterate(opaque: *mut *mut ::std::os::raw::c_void) -> *const AVFilter;
}
extern "C" {
    pub fn avfilter_register_all();
}
extern "C" {
    pub fn avfilter_register(filter: *mut AVFilter) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_next(prev: *const AVFilter) -> *const AVFilter;
}
extern "C" {
    pub fn avfilter_get_by_name(name: *const ::std::os::raw::c_char) -> *const AVFilter;
}
extern "C" {
    pub fn avfilter_init_str(
        ctx: *mut AVFilterContext,
        args: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_init_dict(
        ctx: *mut AVFilterContext,
        options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_free(filter: *mut AVFilterContext);
}
extern "C" {
    pub fn avfilter_insert_filter(
        link: *mut AVFilterLink,
        filt: *mut AVFilterContext,
        filt_srcpad_idx: ::std::os::raw::c_uint,
        filt_dstpad_idx: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_get_class() -> *const AVClass;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFilterGraphInternal {
    _unused: [u8; 0],
}
pub type avfilter_action_func = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut AVFilterContext,
        arg: *mut ::std::os::raw::c_void,
        jobnr: ::std::os::raw::c_int,
        nb_jobs: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type avfilter_execute_func = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: *mut AVFilterContext,
        func: avfilter_action_func,
        arg: *mut ::std::os::raw::c_void,
        ret: *mut ::std::os::raw::c_int,
        nb_jobs: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFilterGraph {
    pub av_class: *const AVClass,
    pub filters: *mut *mut AVFilterContext,
    pub nb_filters: ::std::os::raw::c_uint,
    pub scale_sws_opts: *mut ::std::os::raw::c_char,
    pub resample_lavr_opts: *mut ::std::os::raw::c_char,
    pub thread_type: ::std::os::raw::c_int,
    pub nb_threads: ::std::os::raw::c_int,
    pub internal: *mut AVFilterGraphInternal,
    pub opaque: *mut ::std::os::raw::c_void,
    pub execute: avfilter_execute_func,
    pub aresample_swr_opts: *mut ::std::os::raw::c_char,
    pub sink_links: *mut *mut AVFilterLink,
    pub sink_links_count: ::std::os::raw::c_int,
    pub disable_auto_convert: ::std::os::raw::c_uint,
}
extern "C" {
    pub fn avfilter_graph_alloc() -> *mut AVFilterGraph;
}
extern "C" {
    pub fn avfilter_graph_alloc_filter(
        graph: *mut AVFilterGraph,
        filter: *const AVFilter,
        name: *const ::std::os::raw::c_char,
    ) -> *mut AVFilterContext;
}
extern "C" {
    pub fn avfilter_graph_get_filter(
        graph: *mut AVFilterGraph,
        name: *const ::std::os::raw::c_char,
    ) -> *mut AVFilterContext;
}
extern "C" {
    pub fn avfilter_graph_create_filter(
        filt_ctx: *mut *mut AVFilterContext,
        filt: *const AVFilter,
        name: *const ::std::os::raw::c_char,
        args: *const ::std::os::raw::c_char,
        opaque: *mut ::std::os::raw::c_void,
        graph_ctx: *mut AVFilterGraph,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_graph_set_auto_convert(
        graph: *mut AVFilterGraph,
        flags: ::std::os::raw::c_uint,
    );
}
pub const AVFILTER_AUTO_CONVERT_ALL: _bindgen_ty_6 = 0;
pub const AVFILTER_AUTO_CONVERT_NONE: _bindgen_ty_6 = -1;
pub type _bindgen_ty_6 = i32;
extern "C" {
    pub fn avfilter_graph_config(
        graphctx: *mut AVFilterGraph,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_graph_free(graph: *mut *mut AVFilterGraph);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFilterInOut {
    pub name: *mut ::std::os::raw::c_char,
    pub filter_ctx: *mut AVFilterContext,
    pub pad_idx: ::std::os::raw::c_int,
    pub next: *mut AVFilterInOut,
}
extern "C" {
    pub fn avfilter_inout_alloc() -> *mut AVFilterInOut;
}
extern "C" {
    pub fn avfilter_inout_free(inout: *mut *mut AVFilterInOut);
}
extern "C" {
    pub fn avfilter_graph_parse(
        graph: *mut AVFilterGraph,
        filters: *const ::std::os::raw::c_char,
        inputs: *mut AVFilterInOut,
        outputs: *mut AVFilterInOut,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_graph_parse_ptr(
        graph: *mut AVFilterGraph,
        filters: *const ::std::os::raw::c_char,
        inputs: *mut *mut AVFilterInOut,
        outputs: *mut *mut AVFilterInOut,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_graph_parse2(
        graph: *mut AVFilterGraph,
        filters: *const ::std::os::raw::c_char,
        inputs: *mut *mut AVFilterInOut,
        outputs: *mut *mut AVFilterInOut,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_graph_send_command(
        graph: *mut AVFilterGraph,
        target: *const ::std::os::raw::c_char,
        cmd: *const ::std::os::raw::c_char,
        arg: *const ::std::os::raw::c_char,
        res: *mut ::std::os::raw::c_char,
        res_len: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_graph_queue_command(
        graph: *mut AVFilterGraph,
        target: *const ::std::os::raw::c_char,
        cmd: *const ::std::os::raw::c_char,
        arg: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        ts: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn avfilter_graph_dump(
        graph: *mut AVFilterGraph,
        options: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn avfilter_graph_request_oldest(graph: *mut AVFilterGraph) -> ::std::os::raw::c_int;
}
pub const SwrDitherType_SWR_DITHER_NONE: SwrDitherType = 0;
pub const SwrDitherType_SWR_DITHER_RECTANGULAR: SwrDitherType = 1;
pub const SwrDitherType_SWR_DITHER_TRIANGULAR: SwrDitherType = 2;
pub const SwrDitherType_SWR_DITHER_TRIANGULAR_HIGHPASS: SwrDitherType = 3;
pub const SwrDitherType_SWR_DITHER_NS: SwrDitherType = 64;
pub const SwrDitherType_SWR_DITHER_NS_LIPSHITZ: SwrDitherType = 65;
pub const SwrDitherType_SWR_DITHER_NS_F_WEIGHTED: SwrDitherType = 66;
pub const SwrDitherType_SWR_DITHER_NS_MODIFIED_E_WEIGHTED: SwrDitherType = 67;
pub const SwrDitherType_SWR_DITHER_NS_IMPROVED_E_WEIGHTED: SwrDitherType = 68;
pub const SwrDitherType_SWR_DITHER_NS_SHIBATA: SwrDitherType = 69;
pub const SwrDitherType_SWR_DITHER_NS_LOW_SHIBATA: SwrDitherType = 70;
pub const SwrDitherType_SWR_DITHER_NS_HIGH_SHIBATA: SwrDitherType = 71;
pub const SwrDitherType_SWR_DITHER_NB: SwrDitherType = 72;
pub type SwrDitherType = u32;
pub const SwrEngine_SWR_ENGINE_SWR: SwrEngine = 0;
pub const SwrEngine_SWR_ENGINE_SOXR: SwrEngine = 1;
pub const SwrEngine_SWR_ENGINE_NB: SwrEngine = 2;
pub type SwrEngine = u32;
pub const SwrFilterType_SWR_FILTER_TYPE_CUBIC: SwrFilterType = 0;
pub const SwrFilterType_SWR_FILTER_TYPE_BLACKMAN_NUTTALL: SwrFilterType = 1;
pub const SwrFilterType_SWR_FILTER_TYPE_KAISER: SwrFilterType = 2;
pub type SwrFilterType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SwrContext {
    _unused: [u8; 0],
}
extern "C" {
    pub fn swr_get_class() -> *const AVClass;
}
extern "C" {
    pub fn swr_alloc() -> *mut SwrContext;
}
extern "C" {
    pub fn swr_init(s: *mut SwrContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swr_is_initialized(s: *mut SwrContext) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swr_alloc_set_opts(
        s: *mut SwrContext,
        out_ch_layout: i64,
        out_sample_fmt: AVSampleFormat,
        out_sample_rate: ::std::os::raw::c_int,
        in_ch_layout: i64,
        in_sample_fmt: AVSampleFormat,
        in_sample_rate: ::std::os::raw::c_int,
        log_offset: ::std::os::raw::c_int,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> *mut SwrContext;
}
extern "C" {
    pub fn swr_free(s: *mut *mut SwrContext);
}
extern "C" {
    pub fn swr_close(s: *mut SwrContext);
}
extern "C" {
    pub fn swr_convert(
        s: *mut SwrContext,
        out: *mut *mut u8,
        out_count: ::std::os::raw::c_int,
        in_: *mut *const u8,
        in_count: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swr_next_pts(s: *mut SwrContext, pts: i64) -> i64;
}
extern "C" {
    pub fn swr_set_compensation(
        s: *mut SwrContext,
        sample_delta: ::std::os::raw::c_int,
        compensation_distance: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swr_set_channel_mapping(
        s: *mut SwrContext,
        channel_map: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swr_build_matrix(
        in_layout: u64,
        out_layout: u64,
        center_mix_level: f64,
        surround_mix_level: f64,
        lfe_mix_level: f64,
        rematrix_maxval: f64,
        rematrix_volume: f64,
        matrix: *mut f64,
        stride: ::std::os::raw::c_int,
        matrix_encoding: AVMatrixEncoding,
        log_ctx: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swr_set_matrix(
        s: *mut SwrContext,
        matrix: *const f64,
        stride: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swr_drop_output(
        s: *mut SwrContext,
        count: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swr_inject_silence(
        s: *mut SwrContext,
        count: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swr_get_delay(s: *mut SwrContext, base: i64) -> i64;
}
extern "C" {
    pub fn swr_get_out_samples(
        s: *mut SwrContext,
        in_samples: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swresample_version() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn swresample_configuration() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn swresample_license() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn swr_convert_frame(
        swr: *mut SwrContext,
        output: *mut AVFrame,
        input: *const AVFrame,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swr_config_frame(
        swr: *mut SwrContext,
        out: *const AVFrame,
        in_: *const AVFrame,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn swscale_version() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn swscale_configuration() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn swscale_license() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn sws_getCoefficients(colorspace: ::std::os::raw::c_int) -> *const ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SwsVector {
    pub coeff: *mut f64,
    pub length: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SwsFilter {
    pub lumH: *mut SwsVector,
    pub lumV: *mut SwsVector,
    pub chrH: *mut SwsVector,
    pub chrV: *mut SwsVector,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SwsContext {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sws_isSupportedInput(pix_fmt: AVPixelFormat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sws_isSupportedOutput(pix_fmt: AVPixelFormat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sws_isSupportedEndiannessConversion(pix_fmt: AVPixelFormat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sws_alloc_context() -> *mut SwsContext;
}
extern "C" {
    pub fn sws_init_context(
        sws_context: *mut SwsContext,
        srcFilter: *mut SwsFilter,
        dstFilter: *mut SwsFilter,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sws_freeContext(swsContext: *mut SwsContext);
}
extern "C" {
    pub fn sws_getContext(
        srcW: ::std::os::raw::c_int,
        srcH: ::std::os::raw::c_int,
        srcFormat: AVPixelFormat,
        dstW: ::std::os::raw::c_int,
        dstH: ::std::os::raw::c_int,
        dstFormat: AVPixelFormat,
        flags: ::std::os::raw::c_int,
        srcFilter: *mut SwsFilter,
        dstFilter: *mut SwsFilter,
        param: *const f64,
    ) -> *mut SwsContext;
}
extern "C" {
    pub fn sws_scale(
        c: *mut SwsContext,
        srcSlice: *const *const u8,
        srcStride: *const ::std::os::raw::c_int,
        srcSliceY: ::std::os::raw::c_int,
        srcSliceH: ::std::os::raw::c_int,
        dst: *const *mut u8,
        dstStride: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sws_setColorspaceDetails(
        c: *mut SwsContext,
        inv_table: *const ::std::os::raw::c_int,
        srcRange: ::std::os::raw::c_int,
        table: *const ::std::os::raw::c_int,
        dstRange: ::std::os::raw::c_int,
        brightness: ::std::os::raw::c_int,
        contrast: ::std::os::raw::c_int,
        saturation: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sws_getColorspaceDetails(
        c: *mut SwsContext,
        inv_table: *mut *mut ::std::os::raw::c_int,
        srcRange: *mut ::std::os::raw::c_int,
        table: *mut *mut ::std::os::raw::c_int,
        dstRange: *mut ::std::os::raw::c_int,
        brightness: *mut ::std::os::raw::c_int,
        contrast: *mut ::std::os::raw::c_int,
        saturation: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sws_allocVec(length: ::std::os::raw::c_int) -> *mut SwsVector;
}
extern "C" {
    pub fn sws_getGaussianVec(variance: f64, quality: f64) -> *mut SwsVector;
}
extern "C" {
    pub fn sws_scaleVec(a: *mut SwsVector, scalar: f64);
}
extern "C" {
    pub fn sws_normalizeVec(a: *mut SwsVector, height: f64);
}
extern "C" {
    pub fn sws_getConstVec(c: f64, length: ::std::os::raw::c_int) -> *mut SwsVector;
}
extern "C" {
    pub fn sws_getIdentityVec() -> *mut SwsVector;
}
extern "C" {
    pub fn sws_convVec(a: *mut SwsVector, b: *mut SwsVector);
}
extern "C" {
    pub fn sws_addVec(a: *mut SwsVector, b: *mut SwsVector);
}
extern "C" {
    pub fn sws_subVec(a: *mut SwsVector, b: *mut SwsVector);
}
extern "C" {
    pub fn sws_shiftVec(a: *mut SwsVector, shift: ::std::os::raw::c_int);
}
extern "C" {
    pub fn sws_cloneVec(a: *mut SwsVector) -> *mut SwsVector;
}
extern "C" {
    pub fn sws_printVec2(
        a: *mut SwsVector,
        log_ctx: *mut AVClass,
        log_level: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn sws_freeVec(a: *mut SwsVector);
}
extern "C" {
    pub fn sws_getDefaultFilter(
        lumaGBlur: f32,
        chromaGBlur: f32,
        lumaSharpen: f32,
        chromaSharpen: f32,
        chromaHShift: f32,
        chromaVShift: f32,
        verbose: ::std::os::raw::c_int,
    ) -> *mut SwsFilter;
}
extern "C" {
    pub fn sws_freeFilter(filter: *mut SwsFilter);
}
extern "C" {
    pub fn sws_getCachedContext(
        context: *mut SwsContext,
        srcW: ::std::os::raw::c_int,
        srcH: ::std::os::raw::c_int,
        srcFormat: AVPixelFormat,
        dstW: ::std::os::raw::c_int,
        dstH: ::std::os::raw::c_int,
        dstFormat: AVPixelFormat,
        flags: ::std::os::raw::c_int,
        srcFilter: *mut SwsFilter,
        dstFilter: *mut SwsFilter,
        param: *const f64,
    ) -> *mut SwsContext;
}
extern "C" {
    pub fn sws_convertPalette8ToPacked32(
        src: *const u8,
        dst: *mut u8,
        num_pixels: ::std::os::raw::c_int,
        palette: *const u8,
    );
}
extern "C" {
    pub fn sws_convertPalette8ToPacked24(
        src: *const u8,
        dst: *mut u8,
        num_pixels: ::std::os::raw::c_int,
        palette: *const u8,
    );
}
extern "C" {
    pub fn sws_get_class() -> *const AVClass;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVCodecHWConfigInternal {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct URLContext {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFilterCommand {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFilterChannelLayouts {
    pub _address: u8,
}