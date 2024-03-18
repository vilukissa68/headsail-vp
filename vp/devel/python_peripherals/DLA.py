import math

NAME = "DLA"

# Memory banks
MEMORY_BANK_SIZE = 0x8000
NO_MEMORY_BANKS = 16

# Register map
BASE_ADDR = 0x1000
MEM_SIZE = 0x68
REG_WIDTH = 32

# Status register
STATUS_ADDR = 0x0 
BUF_DONE_OFFSET = 0x0
MAC_DONE_OFFSET = 0x1
PP_DONE_OFFSET = 0x2
DMA_IRQ_OFFSET = 0x3

# Control register
CTRL_ADDR = 0x4 
CPU_FE_OFFSET = 0x0
HP_RST_OFFSET = 0x4
SW_IRQ_OFFSET = 0x8

# Buffer control
BUF_CTRL = 0x8
CONV_MODE_OFFSET = 0x0
READ_A_VALID_OFFSET = 0x4
READ_B_VALID_OFFSET = 0x4 

# Mac control
MAC_CTRL = 0xC
SIMD_SELECT_OFFSET= 0x1
MAC_CLIP_OFFSET = 0x8

# PP control
PP_CTRL = 0x10
ACTIVE_MODE_OFFSET = 0x0
RELU_OFFSET = 0x2
MAX_OFFSET = 0x4
PP_SELECT_OFFSET = 0x6
POOL_MODE_OFFSET = 0x7
ROUNDING_OFFSET = 0x9
CTRL_VLD_OFFSET = 0xA
PP_CLIP_OFFSET = 0x10

# Buffer input
BUF_INPUT = 0x14
BUF_WIDTH_OFFSET = 0x0
BUF_HEIGHT_OFFSET = 0x9
BUF_CHANNELS_OFFSET = 0x12

# Buffer kernel 0
BUF_KERNEL_0 = 0x18
BUF_KERNEL_0_WIDTH_OFFSET = 0x0
BUF_KERNEL_0_HEIGHT_OFFSET = 0x4
BUF_KERNEL_0_S_CHANNELS_OFFSET = 0x8

# Buffer kernel 1
BUF_KERNEL_1 = 0x1C
BUF_KERNEL_1_NUM_OFFSET = 0x0

# Buffer padding
BUF_PAD = 0x20
BUF_PAD_TOP_OFFSET = 0x0
BUF_PAD_RIGHT_OFFSET = 0x4
BUF_PAD_BOTTOM_OFFSET = 0x8
BUF_PAD_LEFT_OFFSET = 0xC
BUF_PAD_VALUE_OFFSET = 0x10

# Buffer stride
BUF_STRIDE = 0x24 
BUF_STRIDE_X_OFFSET = 0x0
BUF_STRIDE_Y_OFFSET = 0x10

# PP input
PP_INPUT = 0x28
PP_INPUT_WIDTH_OFFSET = 0x0
PP_INPUT_HEIGHT_OFFSET = 0x10

# Buffer data bank
BUF_DATA_BANK = 0x2C
BUF_DATA_BANK_A_OFFSET = 0x0
BUF_DATA_BANK_B_OFFSET = 0x10

# Buffer data wait A
BUF_DATA_WAIT_A = 0x30
BUF_DATA_WAIT_A_OFFSET = 0x0

# Buffer data wait B
BUF_DATA_WAIT_B = 0x34
BUF_DATA_WAIT_B_OFFSET = 0x0

# Buffer pipe stall
BUF_PIPE_STALL_STALL_CYCLES = 0x38
BUF_PIPE_STALL_STALL_CYCLES_OFFSET = 0x0

# Power control
POWER_CTRL = 0x4C
POWER_CTRL_DOWN_0_OFFSET = 0x0
POWER_CTRL_DOWN_1_OFFSET = 0x1
POWER_CTRL_DOWN_2_OFFSET = 0x2
POWER_CTRL_ISO_OFFSET = 0x3

# Power status
POWER_STAT = 0x50
POWER_STAT_ACK_0_OFFSET = 0x0
POWER_STAT_ACK_1_OFFSET = 0x1
POWER_STAT_ACK_2_OFFSET = 0x2

# DMA control
DMA_CTRL = 0x44
DMA_CTRL_READ_EVENT_OFFSET = 0x0
DMA_CTRL_WRITE_EVENT_OFFSET = 0x0

# DMA padding
DMA_PAD_CONFIG = 0x48
DMA_PAD_CONFIG_OFFSET = 0x0

# MAC_SAT_MAX
MAC_SAT_MAX = 0x54
MAC_SAT_MAX_OFFSET = 0x0

# MAC_SAT_MIN
MAC_SAT_MIN = 0x58
MAC_SAT_MIN_OFFSET = 0x0

# PP_AXI_READ
PP_AXI_READ = 0x60
PP_AXI_READ_ADDRESS_OFFSET = 0x00

# Handshake
HANDSHAKE = 0x64
HANDSHAKE_BUFFER_VALID_OFFSET = 0x0
HANDSHAKE_MAC_VALID_OFFSET = 0x1
HANDSHAKE_POOL_VALID_OFFSET = 0x2
HANDSHAKE_ACTIVE_VALID_OFFSET = 0x3
HANDSHAKE_BUFFER_ENABLE_OFFSET = 0x4
HANDSHAKE_MAC_ENABLE_OFFSET =  0x5
HANDSHAKE_ACTIVE_ENABLE_OFFSET = 0x6
HANDSHAKE_POOL_ENABLE_OFFSET = 0x7
HANDSHAKE_BIAS_ENABLE_OFFSET = 0x8
HANDSHAKE_BYPASS_ENABLE_OFFSET = 0x9

class MemoryBank:
    def __init__(self, size):
        self.size = size

class DLA:
    def __init__(self):
        self.mem = [0x00] * MEM_SIZE # Memory initalizaed
        self.name = "DLA"

        # Initialize memory banks
        self.banks = [MemoryBank(MEMORY_BANK_SIZE) for _ in range(0, NO_MEMORY_BANKS)]

    # Perform conv2d to value written
    def conv2d(self):
        return

    # ReLU
    def relu(self, x):
        """Perform ReLU for input,
        relu is defined as (x + |x|) / 2, when x > 0 else 0
        """
        if x > 0:
            return (x + abs(x))/2
        else:
            return 0

    # Bias
    def bias(self):
        return

    def matmul(self, A, B):
        """Perform matrix multiplication AxB=C

        Params:
        A: Matrix A in form of [[Int]]
        B: Matrix B in form of [[Int]]

        Returns:
        C: Matrix C in form of [[Int]]
        """
        C = []
        for i in range(0,len(A)):
            temp=[]
            for j in range(0,len(B[0])):
                elem = 0
                for k in range(0,len(A[0])):
                    elem += A[i][k]*B[k][j]
                temp.append(elem)
            C.append(temp)
        return C

    def set_register(self, register, offset, width, value):
        """Set value of specific registers offset

        Params:
        register: interger of modified register
        offset: integer representing bit offset to start modification from
        width: width of the area in memory modified starting from offset
        value: value to be written to register
        """
        start = offset
        end = offset + width - 1
        self.mem[register] = self.mem[register] & ~((2 ** (end-start)) << (end))
        self.mem[register] = self.mem[register] | value << (offset)

    def get_registers(self):
        """Get all registers"""
        return self.mem

    def print_register(self, register):
        """Print specific register in b format

        Params:
        register: integer representing register to print
        """
        print("{0:x}: {1}".format(register, bin(self.mem[register])))

    def print_registers(self):
        """Print all registers"""
        for (i, reg) in enumerate(self.get_registers()):
            print("{:x}: {:b}".format(i, reg))

    def get_memory_banks(self):
        return self.banks

#    Logic    #
# ----------- #
#     API     #

def write(request, dla):
    self.NoisyLog("Writing request: %s at 0x%x, value 0x%x" % (str(request.type), request.offset, request.value))
    dla.mem[request.offset] = request.value

def read(request, dla):
    self.NoisyLog("Reading request: %s at 0x%x, value 0x%x" % (str(request.type), request.offset, request.value))
    request.value = dla.mem[request.offset]

if __name__ == "__main__":
    print("Running as independent module")
    dla = DLA()
    dla.print_registers()
    dla.set_register(HANDSHAKE, 3, 4, 3)
    dla.print_register(HANDSHAKE)
    A = [[1,2,3], [4,5,6], [7,8,9]]
    B = [[1,1,1], [2,2,2], [3,3,3]]
    print(dla.matmul(A,B))

else:
    if request.isInit:
        dla = DLA()
        self.NoisyLog("%s initialized" % NAME)
    elif request.isRead:
        read(request, dla)
    elif request.isWrite:
        write(request, dla)
    else:
        self.NoisyLog("Bad request: %s at 0x%x, value 0x%x" % (str(request.type), request.offset, request.value))
