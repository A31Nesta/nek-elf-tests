.global return67
.type return67, @function
.align 4

return67:
    entry a1, 32    /* Setup register window (allocates 32 bytes on stack) */
    movi a2, 67     /* a2 is the standard return register in Windowed ABI */
    retw.n          /* Return and restore previous register window */
