/**
 *******************************************************************************
 * @file        system_a33xxxx.h
 * @author      ABOV R&D Division
 * @brief       CMSIS Cortex-M3 Device Peripheral Access Layer for A33xxxx
 *
 * Copyright 2022 ABOV Semiconductor Co.,Ltd. All rights reserved.
 *
 * This file is licensed under terms that are found in the LICENSE file
 * located at Document directory.
 * If this file is delivered or shared without applicable license terms,
 * the terms of the BSD-3-Clause license shall be applied.
 * Reference: https://opensource.org/licenses/BSD-3-Clause
 ******************************************************************************/

#include <stdint.h>

typedef void(*VECTOR_TABLE_Type)(void);

extern uint32_t		SystemCoreClock;
extern uint32_t		SystemPeriClock;
extern uint32_t         SystemDelayMSCount;     /**< System Delay Milli-second Count */
extern uint32_t         SystemDelayUSCount;     /**< System Delay Micro-second Count */


/*----------------------------------------------------------------------------
  Clock functions
 *----------------------------------------------------------------------------*/
void SystemInit (void);
void SystemCoreClockUpdate (void) ;           /* Get Core Clock Frequency      */
extern void SystemDelayMS(uint32_t ms);
extern void SystemDelayUS(uint32_t us);
