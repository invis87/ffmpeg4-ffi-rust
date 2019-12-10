#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
#![allow(safe_packed_borrows)]
//! Experimental, alternative FFI bindings.
//! 
//! Experimenting with semi manual API bindings here, since it seems as though
//! `ffmpeg_dev::sys` is too big for RLS. Or rather, because RLS sometimes
//! really, really, really sucks.
//! 
//! Notes:
//! * **Be advised, exported API yet not stable. Prefer directly using symbols from the `ffmpeg_dev::sys` module over this one.**
//! * Also note that this will be be incomplete.
//! 
//! UPDATE:
//! * Things have improved a bit since removing autogenerated layout tests from `ffmpeg_dev::sys`.
//! * **Consider this module deprecated for now.**

// pub mod avcodec;
// pub mod avdevice;
// pub mod avfilter;
// pub mod avformat;
// pub mod avutil;
// pub mod avresample;
// pub mod swresample;
// pub mod swscale;

