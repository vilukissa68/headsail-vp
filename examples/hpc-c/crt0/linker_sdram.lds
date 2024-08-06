/* Copyright (c) 2020 SiFive Inc. */
/* SPDX-License-Identifier: Apache-2.0 */
/*                                     */
/* Copyright (c) 2024 Soc-Hub */
/* SPDX-License-Identifier: Apache-2.0*/
/*                                     */
/* Modifications: */
/* Applied original linker structure for singular memory region.*/
OUTPUT_ARCH("riscv")

ENTRY(_enter)

MEMORY
{
    ram (airwx) : ORIGIN = 0x100000000, LENGTH = 0x70000000
}

SECTIONS
{
    /* ALLOCATION SIZE
     * 
     * Offset that when added to the origin address of the memory, gives us the 
     * top of the stack. Stack grows downwards below this address.
     */
    _allocation_size = 256M;

    /* MINIMUM STACK SIZE
     * 
     * This is the minimum guaranteed size we want to leave available for the stack.
     * It defines the upper limit of the heap, so that the heap can never grow and  
     * occupy memory space reserved for the stack. The heap starts after the last 
     * allocated section and grows upwards.
     */
    _minimum_stack_size = 4M;
    

    /* RAM SECTION
     *
     * The following sections contain data which lives in read-only memory, if
     * such memory is present in the design, for the entire duration of program
     * execution.
     */
    .init : {
        /* The _enter symbol is placed in the .text.init.enter section
         * and must be placed at the beginning of the program */
        KEEP (*(.text.init.enter))
        KEEP (*(.text.init.*))
        KEEP (*(SORT_NONE(.init)))
    } >ram

    .fini : {
        KEEP (*(SORT_NONE(.fini)))
    } >ram

    .preinit_array : ALIGN(8) {
        PROVIDE_HIDDEN (_preinit_array_start = .);
        KEEP (*(.preinit_array))
        PROVIDE_HIDDEN (_preinit_array_end = .);
    } >ram

    .init_array : ALIGN(8) {
        PROVIDE_HIDDEN (__init_array_start = .);
        KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))
        KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))
        PROVIDE_HIDDEN (__init_array_end = .);
        PROVIDE_HIDDEN ( metal_constructors_start = .);
        KEEP (*(SORT_BY_INIT_PRIORITY(.metal.init_array.*)));
        KEEP (*(.metal.init_array));
        PROVIDE_HIDDEN ( metal_constructors_end = .);
    } >ram

    .fini_array : ALIGN(8) {
        PROVIDE_HIDDEN (__fini_array_start = .);
        KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))
        KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))
        PROVIDE_HIDDEN (__fini_array_end = .);
        KEEP (*(SORT_BY_INIT_PRIORITY(.metal.fini_array.*)));
        KEEP (*(.metal.fini_array));
        PROVIDE_HIDDEN ( metal_destructors_end = .);
    } >ram

    .ctors : {
        KEEP (*crtbegin.o(.ctors))
        KEEP (*crtbegin?.o(.ctors))
        KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .ctors))
        KEEP (*(SORT(.ctors.*)))
        KEEP (*(.ctors))
        KEEP (*(.metal.ctors .metal.ctors.*))
    } >ram

    .dtors : {
        KEEP (*crtbegin.o(.dtors))
        KEEP (*crtbegin?.o(.dtors))
        KEEP (*(EXCLUDE_FILE (*crtend.o *crtend?.o ) .dtors))
        KEEP (*(SORT(.dtors.*)))
        KEEP (*(.dtors))
        KEEP (*(.metal.dtors .metal.dtors.*))
    } >ram

    .rodata : {
        *(.rdata)
        *(.rodata .rodata.*)
        *(.gnu.linkonce.r.*)
        . = ALIGN(8);
        *(.srodata.cst16)
        *(.srodata.cst8)
        *(.srodata.cst4)
        *(.srodata.cst2)
        *(.srodata .srodata.*)
    } >ram

    /* LIM SECTION
     *
     * The following sections contain data which is copied fram read-only
     * memory into a loosely integrated memory (LIM), which is shared with L2
     * cache, during pre-main program initialization.
     *
     * Generally, the data copied into the LIM should be performance-critical
     * functions which benefit fram low instruction-fetch latency.
     */
    .lim : ALIGN(8) {
        *(.lim .lim.*)
    } >ram

    /* TEXT SECTION
     *
     * The following section contains the code of the program
     */
    .text : {
        *(.text.unlikely .text.unlikely.*)
        *(.text.startup .text.startup.*)
        *(.text .text.*)
        *(.gnu.linkonce.t.*)
    } >ram

    /* RAM SECTION
     *
     * The following sections contain data which is copied fram read-only
     * memory into a read-write-capable memory such as data tightly-integrated
     * memory (DTIM) or another main memory, as well as the BSS, stack, and
     * heap.
     *
     * You might notice that .data, .tdata, .tbss, .tbss_space, and .bss all
     * have an apparently unnecessary ALIGN at their top. This is because
     * the implementation of _start in Freedom Metal libgloss depends on the
     * ADDR and LOADADDR being 8-byte aligned.
     */
    .data : ALIGN(8) {
        *(.data .data.*)
        *(.gnu.linkonce.d.*)
        . = ALIGN(8);
        PROVIDE( _global_pointer$ = . + 0x800 );
        *(.sdata .sdata.* .sdata2.*)
        *(.gnu.linkonce.s.*)
    } >ram

    .tdata : ALIGN(8) {
        PROVIDE( _tls_base = . );
	*(.tdata .tdata.* .gnu.linkonce.td.*)
    } >ram

    .tbss : ALIGN(8) {
	*(.tbss .tbss.* .gnu.linkonce.tb.*)
	*(.tcommon .tcommon.*)
	PROVIDE( _tls_end = . );
    } >ram
    PROVIDE( _tbss_size = SIZEOF(.tbss) );

    .tbss_space : ALIGN(8) {
	. = . + _tbss_size;
    } >ram

    .bss (NOLOAD): ALIGN(8) {
        *(.sbss*)
        *(.gnu.linkonce.sb.*)
        *(.bss .bss.*)
        *(.gnu.linkonce.b.*)
        *(COMMON)
    } >ram

    PROVIDE( _bss_source_start = LOADADDR(.tbss) );
    PROVIDE( _bss_target_start = ADDR(.tbss) );
    PROVIDE( _bss_target_end = ADDR(.bss) + SIZEOF(.bss) );

    /*
     * HEAP AND STACK MANAGEMENT
     *
     * The stack is set to the top and it grows downwards. The top of the 
     * address space is calculated using the base address + max memory 
     * size. In this case, the memory size has been set to 256MB. The end 
     * of the stack is calculated so we can guarantee a minimum stack size 
     * with regards to the heap. In other words, the stack can grow indefi-
     * nitely towards the heap, but the heap has to leave a minimum amount 
     * of space available for the stack.
     *
     * The heap starts just after the BSS section and grows upwards towards
     * the stack. The heap should grow until it has reached the stack's mi-
     * nimum allocated space.
     *
     *  ORIGIN + _allocation_size ====> +-----------+ <- stack top
     *                                  |           |
     *                                  |           |
     *                                  |unallocated|
     *                                  |   space   | <- stack bottom
     *                                  |           |
     *                                  |           |
     *                                  |           |
     *                                  |           |
     *                                  |           |
     *                                  |           |
     *                                  +-----------+ <- heap start
     *                                  | .bss      |
     *                                  +-----------+
     *                                  | .data     |
     *                                  +-----------+
     *                                  |.text      |
     *                                  |           |
     *                                  |           |
     *                                  |           |
     *  ORIGIN =======================> +-----------+
     *
     */

    /* Symbol that exports the start of the heap. */
    PROVIDE( _heap_start = . );

    /* 
     * End of minimum stack space. Stack grows downwards and can exceeed 
     * this limit, but the heap cannot grow higher than this.
     */
    PROVIDE( _stack_bottom = ORIGIN(ram) + _allocation_size - _minimum_stack_size);
    PROVIDE( _heap_end = _stack_bottom );

    /* Top of the stack. */
    PROVIDE( _stack_top = ORIGIN(ram) + _allocation_size);

    /* C++ exception handling information is
     * not useful with our current runtime environment,
     * and it consumes flash space. Discard it until
     * we have something that can use it
     */
    /DISCARD/ : {
	*(.eh_frame .eh_frame.*)
    }
}
