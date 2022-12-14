// Exception vectors, unused space (curr_el_sp0 exceptions) used for RVBAR
.section .text.exceptions
.global vector_table

vector_table:
.set VBAR, vector_table
    b _boot_cores

.org (VBAR + 0x200)
curr_el_spx_sync:
    b synchronous_handler
    
.org (VBAR + 0x280)
curr_el_spx_irq:
    b irq_handler

.org (VBAR + 0x300)
curr_el_spx_fiq:
    b fiq_handler

.org (VBAR + 0x380)
curr_el_spx_serror:
    b system_error_handler