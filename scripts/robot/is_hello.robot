*** Variables ***
${SCRIPT}                       ${CURDIR}/../resc/1_hpc.resc
${CPU}                          sysbus.cpu_hpc0
${UART}                         sysbus.apb_uart_0
${BIN}                          ${CURDIR}/../../examples/hpc-c/hello-hpc/build/hello-hpc

*** Settings ***
Suite Setup     Setup
Suite Teardown  Teardown
Test Teardown   Test Teardown
Resource        ${RENODEKEYWORDS}

*** Keywords ***
Create Machine
    Execute Script              ${SCRIPT}

*** Test Cases ***
Binary prints "Hello World!" on UART0
    Create Machine
    Create Terminal Tester      ${UART}

    Execute Command             set bin @${BIN}
    Execute Command             sysbus LoadELF $bin false true ${CPU}
    Start Emulation

    Wait For Line On Uart       Hello world!
