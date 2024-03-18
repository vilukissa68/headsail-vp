*** Variables ***
${SCRIPT}                       ${CURDIR}/../1_hpc.resc
${CPU}                          sysbus.cpu_hpc
${UART}                         sysbus.apb_uart_0
${BIN}                          ${CURDIR}/../../examples/hello-dla/target/riscv64imac-unknown-none-elf/debug/examples/dla

*** Settings ***
Suite Setup     Setup
Suite Teardown  Teardown
Test Teardown   Test Teardown
Resource        ${RENODEKEYWORDS}

*** Keywords ***
Create Machine
    Execute Script              ${SCRIPT}

*** Test Cases ***
Hello DLA works
    Create Machine
    Create Terminal Tester      ${UART}

    Execute Command             set bin @${BIN}
    Execute Command             sysbus LoadELF $bin false true ${CPU}
    Start Emulation

    Wait For Line On Uart       Hello world!
