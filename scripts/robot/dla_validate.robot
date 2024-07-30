*** Variables ***
${SCRIPT}                       ${CURDIR}/../resc/1_hpc.resc
${CPU}                          sysbus.cpu_hpc0
${UART}                         sysbus.apb_uart_0
${DATA_DIR}                     ${CURDIR}/../../examples/hpc/dla-driver/examples/test_data
${BIN}                          ${CURDIR}/../../examples/hpc/target/riscv64imac-unknown-none-elf/release/examples/validate
${TINY_DIN}                     ${DATA_DIR}/tiny_test_din.mem
${TINY_WGT}                     ${DATA_DIR}/tiny_test_wgt.mem
${TINY_DOUT}                    ${DATA_DIR}/tiny_test_dout.mem
${CONV_16x16x16_3x3_DIN}        ${DATA_DIR}/conv_16x16x16_3x3_din.mem
${CONV_16x16x16_3x3_WGT}        ${DATA_DIR}/conv_16x16x16_3x3_wgt.mem
${CONV_16x16x16_3x3_DOUT}       ${DATA_DIR}/conv_16x16x16_3x3_dout.mem
${BIAS}                         ${DATA_DIR}/bias.mem
${BIAS_DOUT}                    ${DATA_DIR}/bias_test.out

*** Settings ***
Suite Setup     Setup
Suite Teardown  Teardown
Test Teardown   Test Teardown
Resource        ${RENODEKEYWORDS}
Library         ${CURDIR}/UartLibrary.py      /tmp/uart0     9600

*** Keywords ***
Create Machine
    Execute Script              ${SCRIPT}

*** Test Cases ***
Runs DLA validation tests and prints on uart
    Create Machine
    Create Terminal Tester      ${UART}

    Execute Command             set bin @${BIN}
    Execute Command             sysbus LoadELF $bin false true ${CPU}
    Start Emulation

    Wait For Line On Uart       din
    Read File And Write Mem To Uart     ${TINY_DIN}
    Wait For Line On Uart       wgt
    Read File And Write Mem To Uart     ${TINY_WGT}
    Wait For Line On Uart       dout
    Read File And Write Mem To Uart     ${TINY_DOUT}

    Wait For Line On Uart       din
    Read File And Write Mem To Uart     ${CONV_16x16x16_3x3_DIN}
    Wait For Line On Uart       wgt
    Read File And Write Mem To Uart     ${CONV_16x16x16_3x3_WGT}
    Wait For Line On Uart       dout
    Read File And Write Mem To Uart     ${CONV_16x16x16_3x3_DOUT}

    Wait For Line On Uart       din
    Read File And Write Mem To Uart     ${CONV_16x16x16_3x3_DIN}
    Wait For Line On Uart       wgt
    Read File And Write Mem To Uart     ${CONV_16x16x16_3x3_WGT}
    Wait For Line On Uart       dout
    Read File And Write Mem To Uart     ${BIAS_DOUT}
    Wait For Line On Uart       bias
    Read File And Write Mem To Uart     ${BIAS}

    Wait For Line On Uart       All tests succesful!
