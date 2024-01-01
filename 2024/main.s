	.file	"main.c"
	.text
	.def	___main;	.scl	2;	.type	32;	.endef
	.section .rdata,"dr"
	.align 4
LC0:
	.ascii "Generating a public/private New Year's card.\0"
LC1:
	.ascii "Happy New Year!!\0"
	.align 4
LC2:
	.ascii "Looking forward to another great year!!\0"
	.align 4
LC3:
	.ascii "2023 has been saved in your memories at C:/User/you/memories.\0"
	.align 4
LC4:
	.ascii "Enter the file in which to save the New Year's wishes (//.ssh/new_year_wishes): c:/Users/you/.ssh/new_year_wishes\0"
	.align 4
LC5:
	.ascii "Wishes for health, happiness, and success in the New Year have been generated.\0"
LC6:
	.ascii "The key fingerprint is:\0"
	.align 4
LC7:
	.ascii "<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3 newyear@2024\0"
	.align 4
LC8:
	.ascii "The New Year's card randomart image is:\0"
LC9:
	.ascii "+----[ HNY 2024]------+\0"
LC10:
	.ascii "|            .==*..   |\0"
LC11:
	.ascii "|           .+===++*  |\0"
LC12:
	.ascii "|    +ooooo+ =====+*  |\0"
LC13:
	.ascii "|   o0oBo000o ==*=+   |\0"
LC14:
	.ascii "|  0.oBEo.0o.o ***    |\0"
LC15:
	.ascii "| ....Bo...=o..       |\0"
LC16:
	.ascii "|......0........      |\0"
LC17:
	.ascii "|............o...     |\0"
LC18:
	.ascii "|.................    |\0"
LC19:
	.ascii "|..................   |\0"
LC20:
	.ascii "+---------------------+\0"
LC21:
	.ascii "by rxxuzi\0"
	.text
	.globl	_main
	.def	_main;	.scl	2;	.type	32;	.endef
_main:
LFB14:
	.cfi_startproc
	pushl	%ebp
	.cfi_def_cfa_offset 8
	.cfi_offset 5, -8
	movl	%esp, %ebp
	.cfi_def_cfa_register 5
	andl	$-16, %esp
	subl	$16, %esp
	call	___main
	movl	$LC0, (%esp)
	call	_puts
	movl	$3, (%esp)
	call	_sleep
	movl	$LC1, (%esp)
	call	_puts
	movl	$LC2, (%esp)
	call	_puts
	movl	$1, (%esp)
	call	_sleep
	movl	$LC3, (%esp)
	call	_puts
	movl	$1, (%esp)
	call	_sleep
	movl	$LC4, (%esp)
	call	_puts
	movl	$1, (%esp)
	call	_sleep
	movl	$LC5, (%esp)
	call	_puts
	movl	$2, (%esp)
	call	_sleep
	movl	$LC6, (%esp)
	call	_puts
	movl	$LC7, (%esp)
	call	_puts
	movl	$2, (%esp)
	call	_sleep
	movl	$LC8, (%esp)
	call	_puts
	movl	$4, (%esp)
	call	_sleep
	movl	$LC9, (%esp)
	call	_puts
	movl	$LC10, (%esp)
	call	_puts
	movl	$LC11, (%esp)
	call	_puts
	movl	$LC12, (%esp)
	call	_puts
	movl	$LC13, (%esp)
	call	_puts
	movl	$LC14, (%esp)
	call	_puts
	movl	$LC15, (%esp)
	call	_puts
	movl	$LC16, (%esp)
	call	_puts
	movl	$LC17, (%esp)
	call	_puts
	movl	$LC18, (%esp)
	call	_puts
	movl	$LC19, (%esp)
	call	_puts
	movl	$LC20, (%esp)
	call	_puts
	movl	$2, (%esp)
	call	_sleep
	movl	$LC21, (%esp)
	call	_printf
	movl	$0, %eax
	leave
	.cfi_restore 5
	.cfi_def_cfa 4, 4
	ret
	.cfi_endproc
LFE14:
	.ident	"GCC: (i686-posix-dwarf-rev0, Built by MinGW-W64 project) 8.1.0"
	.def	_puts;	.scl	2;	.type	32;	.endef
	.def	_sleep;	.scl	2;	.type	32;	.endef
	.def	_printf;	.scl	2;	.type	32;	.endef
