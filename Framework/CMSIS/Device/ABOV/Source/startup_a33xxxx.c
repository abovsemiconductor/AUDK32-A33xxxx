/**
 *******************************************************************************
 * @file        startup_a33xxxx.c
 * @author      ABOV R&D Division
 * @brief       CMSIS-Core(M) Device Startup File for a Cortex-M3 Device
 *
 * Copyright 2022 ABOV Semiconductor Co.,Ltd. All rights reserved.
 *
 * This file is licensed under terms that are found in the LICENSE file
 * located at Document directory.
 * If this file is delivered or shared without applicable license terms,
 * the terms of the BSD-3-Clause license shall be applied.
 * Reference: https://opensource.org/licenses/BSD-3-Clause
 ******************************************************************************/

#include "abov_config.h"

/*----------------------------------------------------------------------------
  Internal Macros
 *----------------------------------------------------------------------------*/
#if (CONFIG_EMUL_JTAG_CONNECTION == 1)
#define PRV_CHIPSET_REG_DHCSR  0xE000EDF0
#endif

/*----------------------------------------------------------------------------
  External References
 *----------------------------------------------------------------------------*/
extern uint32_t __INITIAL_SP;

#if (CONFIG_EMUL_JTAG_CONNECTION == 1)
int8_t g_bIsARMisDebugMode = 0;
int8_t g_bIsARMisDebugConnectionChecked = 0;
#endif

extern __NO_RETURN void __PROGRAM_START(void);

/*----------------------------------------------------------------------------
  Internal References
 *----------------------------------------------------------------------------*/
__NO_RETURN void Reset_Handler  (void);
            void Default_Handler(void);

/*----------------------------------------------------------------------------
  Exception / Interrupt Handler
 *----------------------------------------------------------------------------*/
/* Exceptions */
void NMI_Handler            (void) __attribute__ ((weak, alias("Default_Handler")));
void HardFault_Handler      (void) __attribute__ ((weak));
void MemManage_Handler      (void) __attribute__ ((weak, alias("Default_Handler")));
void BusFault_Handler       (void) __attribute__ ((weak, alias("Default_Handler")));
void UsageFault_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void SVC_Handler            (void) __attribute__ ((weak, alias("Default_Handler")));
void DebugMon_Handler       (void) __attribute__ ((weak, alias("Default_Handler")));
void PendSV_Handler         (void) __attribute__ ((weak, alias("Default_Handler")));
void SysTick_Handler        (void) __attribute__ ((weak, alias("Default_Handler")));

void Interrupt0_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt1_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt2_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt3_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt4_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt5_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt6_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt7_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt8_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt9_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));

void Interrupt10_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt11_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt12_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt13_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt14_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt15_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt16_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt17_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt18_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt19_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));

void Interrupt20_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt21_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt22_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt23_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt24_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt25_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt26_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt27_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt28_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt29_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));

void Interrupt30_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt31_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt32_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt33_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt34_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt35_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt36_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt37_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt38_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt39_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));

void Interrupt40_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt41_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt42_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt43_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt44_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt45_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt46_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt47_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt48_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt49_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));

void Interrupt50_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt51_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt52_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt53_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt54_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt55_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt56_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt57_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt58_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt59_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));

void Interrupt60_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt61_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt62_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt63_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt64_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt65_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt66_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt67_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt68_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt69_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));

void Interrupt70_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt71_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt72_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt73_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt74_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt75_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt76_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt77_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt78_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt79_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));

void Interrupt80_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt81_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt82_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt83_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt84_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt85_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt86_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt87_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt88_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
void Interrupt89_Handler     (void) __attribute__ ((weak, alias("Default_Handler")));
/*----------------------------------------------------------------------------
  Exception / Interrupt Vector table
 *----------------------------------------------------------------------------*/

#if defined ( __GNUC__ )
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpedantic"
#endif

extern const VECTOR_TABLE_Type __VECTOR_TABLE[240];
       const VECTOR_TABLE_Type __VECTOR_TABLE[240] __VECTOR_TABLE_ATTRIBUTE =
{
  (VECTOR_TABLE_Type)(&__INITIAL_SP),       /*     Initial Stack Pointer */
  Reset_Handler,                            /*     Reset Handler */
  NMI_Handler,                              /* -14 NMI Handler */
  HardFault_Handler,                        /* -13 Hard Fault Handler */
  MemManage_Handler,                        /* -12 MPU Fault Handler */
  BusFault_Handler,                         /* -11 Bus Fault Handler */
  UsageFault_Handler,                       /* -10 Usage Fault Handler */
  0,                                        /*     Reserved */
  0,                                        /*     Reserved */
  0,                                        /*     Reserved */
  0,                                        /*     Reserved */
  SVC_Handler,                              /*  -5 SVC Handler */
  DebugMon_Handler,                         /*  -4 Debug Monitor Handler */
  0,                                        /*     Reserved */
  PendSV_Handler,                           /*  -2 PendSV Handler */
  SysTick_Handler,                          /*  -1 SysTick Handler */

  /* Interrupts */
  Interrupt0_Handler,                       /*   0 Interrupt 0 */
  Interrupt1_Handler,                       /*   1 Interrupt 1 */
  Interrupt2_Handler,                       /*   2 Interrupt 2 */
  Interrupt3_Handler,                       /*   3 Interrupt 3 */
  Interrupt4_Handler,                       /*   4 Interrupt 4 */
  Interrupt5_Handler,                       /*   5 Interrupt 5 */
  Interrupt6_Handler,                       /*   6 Interrupt 6 */
  Interrupt7_Handler,                       /*   7 Interrupt 7 */
  Interrupt8_Handler,                       /*   8 Interrupt 8 */
  Interrupt9_Handler,                        /*   9 Interrupt 9 */
  Interrupt10_Handler,                       /*   0 Interrupt 0 */
  Interrupt11_Handler,                       /*   1 Interrupt 1 */
  Interrupt12_Handler,                       /*   2 Interrupt 2 */
  Interrupt13_Handler,                       /*   3 Interrupt 3 */
  Interrupt14_Handler,                       /*   4 Interrupt 4 */
  Interrupt15_Handler,                       /*   5 Interrupt 5 */
  Interrupt16_Handler,                       /*   6 Interrupt 6 */
  Interrupt17_Handler,                       /*   7 Interrupt 7 */
  Interrupt18_Handler,                       /*   8 Interrupt 8 */
  Interrupt19_Handler,                       /*   9 Interrupt 9 */
  Interrupt20_Handler,                       /*   0 Interrupt 0 */
  Interrupt21_Handler,                       /*   1 Interrupt 1 */
  Interrupt22_Handler,                       /*   2 Interrupt 2 */
  Interrupt23_Handler,                       /*   3 Interrupt 3 */
  Interrupt24_Handler,                       /*   4 Interrupt 4 */
  Interrupt25_Handler,                       /*   5 Interrupt 5 */
  Interrupt26_Handler,                       /*   6 Interrupt 6 */
  Interrupt27_Handler,                       /*   7 Interrupt 7 */
  Interrupt28_Handler,                       /*   8 Interrupt 8 */
  Interrupt29_Handler,                        /*   9 Interrupt 9 */
  Interrupt30_Handler,                       /*   0 Interrupt 0 */
  Interrupt31_Handler,                       /*   1 Interrupt 1 */
  Interrupt32_Handler,                       /*   2 Interrupt 2 */
  Interrupt33_Handler,                       /*   3 Interrupt 3 */
  Interrupt34_Handler,                       /*   4 Interrupt 4 */
  Interrupt35_Handler,                       /*   5 Interrupt 5 */
  Interrupt36_Handler,                       /*   6 Interrupt 6 */
  Interrupt37_Handler,                       /*   7 Interrupt 7 */
  Interrupt38_Handler,                       /*   8 Interrupt 8 */
  Interrupt39_Handler,                        /*   9 Interrupt 9 */
  Interrupt40_Handler,                       /*   0 Interrupt 0 */
  Interrupt41_Handler,                       /*   1 Interrupt 1 */
  Interrupt42_Handler,                       /*   2 Interrupt 2 */
  Interrupt43_Handler,                       /*   3 Interrupt 3 */
  Interrupt44_Handler,                       /*   4 Interrupt 4 */
  Interrupt45_Handler,                       /*   5 Interrupt 5 */
  Interrupt46_Handler,                       /*   6 Interrupt 6 */
  Interrupt47_Handler,                       /*   7 Interrupt 7 */
  Interrupt48_Handler,                       /*   8 Interrupt 8 */
  Interrupt49_Handler,                        /*   9 Interrupt 9 */
  Interrupt50_Handler,                       /*   0 Interrupt 0 */
  Interrupt51_Handler,                       /*   1 Interrupt 1 */
  Interrupt52_Handler,                       /*   2 Interrupt 2 */
  Interrupt53_Handler,                       /*   3 Interrupt 3 */
  Interrupt54_Handler,                       /*   4 Interrupt 4 */
  Interrupt55_Handler,                       /*   5 Interrupt 5 */
  Interrupt56_Handler,                       /*   6 Interrupt 6 */
  Interrupt57_Handler,                       /*   7 Interrupt 7 */
  Interrupt58_Handler,                       /*   8 Interrupt 8 */
  Interrupt59_Handler,                        /*   9 Interrupt 9 */
  Interrupt60_Handler,                       /*   0 Interrupt 0 */
  Interrupt61_Handler,                       /*   1 Interrupt 1 */
  Interrupt62_Handler,                       /*   2 Interrupt 2 */
  Interrupt63_Handler,                       /*   3 Interrupt 3 */
  Interrupt64_Handler,                       /*   4 Interrupt 4 */
  Interrupt65_Handler,                       /*   5 Interrupt 5 */
  Interrupt66_Handler,                       /*   6 Interrupt 6 */
  Interrupt67_Handler,                       /*   7 Interrupt 7 */
  Interrupt68_Handler,                       /*   8 Interrupt 8 */
  Interrupt69_Handler,                       /*   9 Interrupt 9 */
  Interrupt70_Handler,                       /*   0 Interrupt 0 */
  Interrupt71_Handler,                       /*   1 Interrupt 1 */
  Interrupt72_Handler,                       /*   2 Interrupt 2 */
  Interrupt73_Handler,                       /*   3 Interrupt 3 */
  Interrupt74_Handler,                       /*   4 Interrupt 4 */
  Interrupt75_Handler,                       /*   5 Interrupt 5 */
  Interrupt76_Handler,                       /*   6 Interrupt 6 */
  Interrupt77_Handler,                       /*   7 Interrupt 7 */
  Interrupt78_Handler,                       /*   8 Interrupt 8 */
  Interrupt79_Handler,                       /*   9 Interrupt 9 */
  Interrupt80_Handler,                       /*   0 Interrupt 0 */
  Interrupt81_Handler,                       /*   1 Interrupt 1 */
  Interrupt82_Handler,                       /*   2 Interrupt 2 */
  Interrupt83_Handler,                       /*   3 Interrupt 3 */
  Interrupt84_Handler,                       /*   4 Interrupt 4 */
  Interrupt85_Handler,                       /*   5 Interrupt 5 */
  Interrupt86_Handler,                       /*   6 Interrupt 6 */
  Interrupt87_Handler,                       /*   7 Interrupt 7 */
  Interrupt88_Handler,                       /*   8 Interrupt 8 */
  Interrupt89_Handler                        /*   9 Interrupt 9 */
                                            /* Interrupts 10 .. 223 are left out */
};

/*----------------------------------------------------------------------------
  Reset Handler called on controller reset
 *----------------------------------------------------------------------------*/
__NO_RETURN void Reset_Handler(void)
{
  SystemInit();                             /* CMSIS System Initialization */
  __PROGRAM_START();                        /* Enter PreMain (C library entry point) */
}

/*----------------------------------------------------------------------------
  Hard Fault Handler
 *----------------------------------------------------------------------------*/
void HardFault_Handler(void)
{
#if (CONFIG_EMUL_JTAG_CONNECTION == 1)
  int* pn32Reg = (int*)PRV_CHIPSET_REG_DHCSR;

  /* Get Debugger connection */
  if (g_bIsARMisDebugConnectionChecked)
  {
     if (*(pn32Reg) & 0x01)
     {
        g_bIsARMisDebugMode = 1;
     }

     g_bIsARMisDebugConnectionChecked = 0;
     return;
  }
#endif

  while(1);
}

/*----------------------------------------------------------------------------
  Default Handler for Exceptions / Interrupts
 *----------------------------------------------------------------------------*/
void Default_Handler(void)
{
  while(1);
}

/*----------------------------------------------------------------------------
  Private API to detect debugger connection
 *----------------------------------------------------------------------------*/
#define PRV_CHIPSET_INVALID_ADD   0x99999999UL
#define PRV_CHIPSET_INVALID_DATA  0x50

#if (CONFIG_EMUL_JTAG_CONNECTION == 1)
void PRV_CHIPSET_SetDebuggerConnectionStatus (void)
{
    uint8_t *un8pAddr = (uint8_t *)PRV_CHIPSET_INVALID_ADD;

    g_bIsARMisDebugConnectionChecked = 1;

    /* Fire Hard-Fault */
    *un8pAddr = PRV_CHIPSET_INVALID_DATA;
    return;
}

int8_t PRV_CHIPSET_GetDebuggerConnectionStatus (void)
{
    return g_bIsARMisDebugMode;
}
#endif

