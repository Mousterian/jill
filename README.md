# jill
Simple rust bindings for Jack Audio on Raspbian.

Why another rusty jack FFI wrapper library?

At the moment I want to target raspberry pi, rust-jack doesn't compile on ARM. 

I'd fix it but there's no documentation on how they generated the bindings.

Also when people wrap APIs they seem to be in an awful hurry to redesign the API, 
which to me seems to be an impediment to understanding how the API works.

I'm not against object oriented wrappers for C APIs but I'd like to start with 
something simpler first to get my head around Jack. 

If this starts to look a lot like rust-jack then I might do it properly and port rust-jack to ARM.
