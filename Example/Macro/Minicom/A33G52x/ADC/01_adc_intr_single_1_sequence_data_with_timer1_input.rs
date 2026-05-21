# A. Description
#    A list of commands here configures ADC and generates single capture interrupt.
#
# B. Preparation
#    Connecting target port with a suitable instrument.
#
# C. Prerequisite Example (abov_example_config.h)
#    1. ADC
#    2. Timer1
#    3. PCU/GPIO
#
# D. Default Port
#    1. ADC0    : PA0 (AN0)
#               : PA1 (AN1)
#               : PA2 (AN2)
#    2. Timer10 : PB0 (Output)
#                 PA6 (Capture Input/External Clock Input)
#
# E. Port Connection
#    1. PA0 (AN0) <----> PB0 (Timer10 Output)
#
# For more information, read a user's manual of the target device carefully.
#
# ADC0
# 1. Channel                    : 0
#
# 2. Clock                      : [ 0 p 255 ]
#    Source                     : p (Periperhal Clock)
#    Source Divide              : 255 (Divide + 1)
#
# 3. Config                     : [ config 0 i s i 1 a 0 ]
#    Operation                  : i (Interrupt)
#    Mode                       : s (Single)
#    Reference Level            : i (Internal VDD)
#    Sequence Count             : 1
#    Base Trigger Source        : a (ADC Instant Start)
#    Sampling Time              : 0 (No Sampling Time available so default 0)
#
# 4. Seq                        : [ 0 0 0 a 0 ]
#    Sequence Number            : 0
#    Input Channel              : 0
#    Trigger Source             : a (ADC Instant Start)
#    Trigger Source Number      : 0 (Ignored)
#
# Timer10
# 1. Channel                   : 0 (Timer10)
#
# 2. Clock                     : [ 0 p 16 9 ]
#    Source                    : p (PCLK : Peripherl Clock)
#    Source Divide             : 16
#    Timer1 Pre-Divide         : 999 (MCCR Clock) / (Pre-Divide + 1)
#
# 3. Config                    : [ 0 i p h 10 10 ]
#    Operation                 : i (Interrupt)
#    Mode                      : p (Period)
#    Output Port Polarity      : h (High)
#    A Data                    : 10
#    B Data                    : 10
#
# PCU (PAx)
# 1. Port Group                : 0 (PCU Port A)
#
# 2. Port                      : [ 0 0 a 3 ] [ 0 1 a 3 ] [ 0 2 a 3 ]
#    Pin Number                : 0 / 1 / 2
#    Pin Mode                  : a (Alternative)
#    Alternative               : 3 (AN0/AN1/AN2)
# 
# 3. Port                      : [ 0 0 i a ] [ 0 1 i a ] [ 0 2 i a ]
#    Pin Number                : 0 / 1 / 2
#    Pin Mode                  : i (Input)
#    Pin Input Mode            : a (Analog Input)

# PCU (PAx)
send ""

send "cm pcu"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 0 a 3"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 1 a 3"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 2 a 3"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 0 i a"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 1 i a"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}

send "port 0 2 i a"
expect {
    "<PCU> # "
    break
    timeout 5 goto end
}


# Timer10
send "cm timer1"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "clk 0 p 16 9"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "config 0 i p h 10 10"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "log off"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<TIMER1> # "
    break
    timeout 5 goto end
}

# ADC0
send "cm adc"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "uninit 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "init 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "clk 0 p 255"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "config 0 i s i 1 a 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "seq 0 0 0 a 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

send "start 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

sleep 2 
send "dump 0"
expect {
    "<ADC> # "
    break
    timeout 5 goto end
}

end:
